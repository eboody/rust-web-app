use crate::config;
use crate::Error;
use crate::Result;
use axum::body::Bytes;
use axum::http::HeaderValue;
use axum::{extract::State, Json};
use lib_anythingllm::models::ChatResponse;
use lib_anythingllm::models::ResponseData;
use lib_core::model::DirectusFiles;
use lib_core::model::DirectusFolders;
use lib_core::model::Ebook;
use lib_core::model::Ebooks;
use lib_core::model::{ModelManager, UploadFilePayload};
use ormlite::Model;
use reqwest::header::CONTENT_TYPE;
use reqwest::multipart;
use serde_json::json;
use uuid::Uuid;

pub async fn on_ebook_file_upload(
	State(mm): State<ModelManager>,
	Json(payload): Json<UploadFilePayload>,
) -> Result<()> {
	let directus_file = DirectusFiles::select()
		.where_("filename_disk = ?")
		.bind(payload.filename_disk.clone())
		.fetch_one(mm.orm())
		.await?;

	let ebooks_covers_folder = DirectusFolders::select()
		.where_("name = ?")
		.bind("Covers")
		.where_("parent = ?")
		.bind(directus_file.folder)
		.fetch_one(mm.orm())
		.await?;

	if !is_pdf_file(&payload, &directus_file, &ebooks_covers_folder) {
		return Ok(());
	}

	//let file_bytes = get_file_byes(&mm, directus_file.id).await?;

	//save_ebook_cover_image(
	//	&mm,
	//	payload.clone(),
	//	ebooks_covers_folder.id,
	//	&file_bytes,
	//)
	//.await?;

	//embed_ebook_anythingllm(&mm, &directus_file, &file_bytes).await?;

	generate_metadata(&payload, &mm).await?;

	Ok(())
}

fn is_pdf_file(
	payload: &UploadFilePayload,
	d_file: &DirectusFiles,
	d_folder: &DirectusFolders,
) -> bool {
	payload.type_.clone().unwrap() == "application/pdf"
		&& (d_file.folder.to_string()
			== d_folder
				.parent
				.map(|u| u.to_string())
				.unwrap_or("".to_owned()))
}

pub async fn get_file_byes(mm: &ModelManager, d_id: Uuid) -> Result<Bytes> {
	Ok(mm
		.reqwest()
		.get(format!("{}/assets/{}", config().DIRECTUS_URL, d_id))
		.headers(config().DIRECTUS_HEADERS.clone())
		.send()
		.await?
		.bytes()
		.await?)
}

pub async fn save_ebook_cover_image(
	mm: &ModelManager,
	payload: UploadFilePayload,
	covers_folder_id: Uuid,
	file_bytes: &Bytes,
) -> Result<()> {
	let form = multipart::Form::new()
		.part(
			"fileInput",
			multipart::Part::stream(file_bytes.clone())
				.file_name(payload.filename_disk.clone())
				.mime_str("application/pdf")?,
		)
		.part("pageNumbers", multipart::Part::text("2-100"));

	let first_page_pdf = mm
		.reqwest()
		.post("https://spdf.eman.network/api/v1/general/remove-pages")
		.multipart(form)
		.send()
		.await?
		.bytes()
		.await?;

	let form = multipart::Form::new()
		.part(
			"fileInput",
			multipart::Part::stream(first_page_pdf.clone())
				.file_name(payload.filename_disk.clone())
				.mime_str("application/pdf")?,
		)
		.part("imageFormat", multipart::Part::text("jpeg"))
		.part("singleOrMultiple", multipart::Part::text("single"))
		.part("colorType", multipart::Part::text("color"))
		.part("dpi", multipart::Part::text("400"));

	let cover_image = mm
		.reqwest()
		.post("https://spdf.eman.network/api/v1/convert/pdf/img")
		.multipart(form)
		.send()
		.await?
		.bytes()
		.await?;

	let directus_upload_form = multipart::Form::new()
		.part("title", multipart::Part::text(payload.title.clone()))
		.part(
			"folder",
			multipart::Part::text(covers_folder_id.to_string()),
		)
		.part(
			"file",
			multipart::Part::stream(cover_image.clone())
				.file_name(format!("{}.jpeg", payload.title.clone()))
				.mime_str("image/jpeg")?,
		);

	let upload_res = mm
		.reqwest()
		.post("https://directus.eman.network/files")
		.headers(config().DIRECTUS_HEADERS.clone())
		.multipart(directus_upload_form)
		.send()
		.await?
		.text()
		.await?;

	dbg!("upload_res: {}", &upload_res);
	Ok(())
}

async fn embed_ebook_anythingllm(
	mm: &ModelManager,
	d_file: &DirectusFiles,
	file_bytes: &Bytes,
) -> Result<()> {
	// Step 1: Check if the document already exists
	let existing_docs = mm
		.reqwest()
		.get("https://anything.eman.network/api/v1/documents")
		.headers(config().ANYTHING_HEADERS.clone())
		.send()
		.await
		.map_err(Error::Request)?
		.text()
		.await
		.map_err(Error::Request)?;

	dbg!("existing_docs: {}", &existing_docs);

	let doc_exists = existing_docs.contains(&d_file.filename_download);

	if doc_exists {
		println!("Document already exists, skipping upload.");
		return Ok(());
	}

	// Step 2: Upload the document
	let form = multipart::Form::new().part(
		"file",
		multipart::Part::stream(file_bytes.clone())
			.file_name(d_file.filename_download.clone())
			.mime_str("application/pdf")
			.map_err(Error::Request)?,
	);

	let upload_res = mm
		.reqwest()
		.post("https://anything.eman.network/api/v1/document/upload")
		.multipart(form)
		.headers(config().ANYTHING_HEADERS.clone())
		.send()
		.await
		.map_err(Error::Request)?;

	if !upload_res.status().is_success() {
		let error_body = upload_res
			.text()
			.await
			.unwrap_or_else(|_| "Unknown error".to_string());
		return Err(Error::WrongFormat(format!(
			"Failed to upload document. Response: {}",
			error_body
		)));
	}

	let response_data: ResponseData = upload_res.json().await?;

	let from_location = response_data
		.documents
		.first()
		.ok_or_else(|| {
			Error::WrongFormat("No documents returned in upload response".to_owned())
		})?
		.location
		.clone();

	// Step 3: Move the document
	let to_location = from_location.replace("custom-documents", "Ebooks");

	let move_body = json!({
		"files": [
			{ "from": from_location, "to": to_location }
		]
	});

	let move_res = mm
		.reqwest()
		.post("https://anything.eman.network/api/v1/document/move-files")
		.body(move_body.to_string())
		.headers(config().ANYTHING_HEADERS_JSON.clone())
		.send()
		.await?;

	if !move_res.status().is_success() {
		let error_body = move_res
			.text()
			.await
			.unwrap_or_else(|_| "Unknown error".to_string());
		return Err(Error::WrongFormat(format!(
			"Failed to move document. Response: {}",
			error_body
		)));
	}

	// Step 4: Update embeddings
	let update_embeddings_body = json!({
		"adds": [to_location]
	});

	let update_res = mm
        .reqwest()
        .post("https://anything.eman.network/api/v1/workspace/ebooks/update-embeddings")
        .body(update_embeddings_body.to_string())
        .headers(headers)
        .send()
        .await?;

	if !update_res.status().is_success() {
		let error_body = update_res
			.text()
			.await
			.unwrap_or_else(|_| "Unknown error".to_string());
		return Err(Error::WrongFormat(format!(
			"Failed to update embeddings. Response: {}",
			error_body
		)));
	}

	println!(
		"Successfully uploaded, moved, and updated embeddings for the document."
	);
	Ok(())
}

async fn generate_metadata(
	payload: &UploadFilePayload,
	mm: &ModelManager,
) -> Result<()> {
	let gen_descriptior_message = format!(
		r"Generate a descriptor 
		(A very SHORT 1-sentence description) 
		for {}. 
		DO NOT MAKE IT LONG. And make it compelling...
		thought provoking if possible but enticing",
		payload.title
	);

	let gen_slug_message = format!(
		r"
		Generate a slug for the ebook {}.
		",
		payload.title
	);

	let gen_summary_message = format!(
		r"
		Generate a summary for the ebook {}.
		",
		payload.title
	);

	let gen_title_message = format!(
		r"
		Generate a title for the ebook {}.
		",
		payload.title
	);

	let descriptor = chat(mm, gen_descriptior_message).await?.text_response;
	let slug = chat(mm, gen_slug_message).await?.text_response;
	let summary = chat(mm, gen_summary_message).await?.text_response;
	let title = chat(mm, gen_title_message).await?.text_response;

	mm.reqwest()
		.post("https://directus.eman.network/items/ebooks")
		.headers(config().DIRECTUS_HEADERS_JSON.clone())
		.body(
			json!({
				"status": "published"
			})
			.to_string(),
		)
		.send()
		.await?;

	Ok(())
}

async fn chat(mm: &ModelManager, message: String) -> Result<ChatResponse> {
	let mut headers = config().ANYTHING_AUTH.clone();
	headers.insert("Content-Type", HeaderValue::from_static("application/json"));

	let body = json!({
		"message": message
	})
	.to_string();

	Ok(mm
		.reqwest()
		.post("https://anything.eman.network/api/v1/workspace/ebooks/chat")
		.body(body)
		.headers(headers)
		.send()
		.await?
		.json::<ChatResponse>()
		.await?)
}

//payload: UploadFilePayload {
//    storage: "local",
//    filename_disk: Some(
//        "4a3f35ec-7569-4050-9214-9f4fc92b5d12.pdf",
//    ),
//    filename_download: "Doe.pdf",
//    title: Some(
//        "Doe",
//    ),
//    type_: Some(
//        "application/pdf",
//    ),
//    folder: None,
//    uploaded_by: None,
//    created_on: None,
//    modified_by: None,
//    modified_on: None,
//    charset: None,
//    filesize: Some(
//        6724408,
//    ),
//    width: None,
//    height: None,
//    duration: None,
//    embed: None,
//    description: None,
//    location: None,
//    tags: None,
//    metadata: None,
//    focal_point_x: None,
//    focal_point_y: None,
//    tus_id: None,
//    tus_data: None,
//    uploaded_on: Some(
//        "2024-11-28T01:17:38.257Z",
//    ),
//}
