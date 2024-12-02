use crate::config;
use crate::Error;
use crate::Result;
use axum::body::Bytes;
use axum::http::HeaderValue;
use axum::{extract::State, Json};
use lib_anythingllm::apis::urlencode;
use lib_anythingllm::models::ChatResponse;
use lib_anythingllm::models::LocalFile;
use lib_anythingllm::models::ResponseData;
use lib_core::model::DirectusFiles;
use lib_core::model::DirectusFolders;
use lib_core::model::DirectusUsers;
use lib_core::model::Ebooks;
use lib_core::model::EbooksTranslations;
use lib_core::model::EbooksTranslationsBuilder;
use lib_core::model::{ModelManager, UploadFilePayload};
use ormlite::model::Join;
use ormlite::model::ModelBuilder;
use ormlite::Model;
use reqwest::multipart;
use serde_json::json;
use uuid::Uuid;

use super::on_docx_upload::on_docx_upload;

pub async fn on_file_upload(
	State(mm): State<ModelManager>,
	Json(payload): Json<UploadFilePayload>,
) -> Result<()> {
	dbg!("payload: {}", &payload);
	let directus_file = DirectusFiles::select()
		.where_("filename_disk = ?")
		.bind(payload.filename_disk.clone())
		.fetch_one(mm.orm())
		.await?;

	if is_docx_file(&payload) {
		let res = on_docx_upload(&mm, &payload, &directus_file).await;
		dbg!("res: {}", &res);

		return Ok(());
	}

	//let ebooks_covers_folder = DirectusFolders::select()
	//	.where_("name = ?")
	//	.bind("Covers")
	//	.where_("parent = ?")
	//	.bind(directus_file.folder)
	//	.fetch_one(mm.orm())
	//	.await?;
	//
	//if !is_pdf_file_ebook(&payload, &directus_file, &ebooks_covers_folder) {
	//	return Ok(());
	//}
	//
	//let file_bytes = get_file_byes(&mm, directus_file.id).await?;
	//
	//save_ebook_cover_image(
	//	&mm,
	//	payload.clone(),
	//	ebooks_covers_folder.id,
	//	&file_bytes,
	//)
	//.await?;
	//
	//let first_few_pages_file_name =
	//	format!("First few pages of {}", payload.filename_download.clone());
	//
	//let first_few_pages =
	//	get_first_pages_of_pdf(&mm, first_few_pages_file_name.clone(), &file_bytes)
	//		.await?;
	//
	//embed_ebook_anythingllm(&mm, first_few_pages_file_name, &first_few_pages)
	//	.await?;
	//
	//embed_ebook_anythingllm(
	//	&mm,
	//	directus_file.filename_download.clone(),
	//	&file_bytes,
	//)
	//.await?;
	//
	//let ebook_cover_file = DirectusFiles::select()
	//	.where_("title = ?")
	//	.bind(payload.title.clone())
	//	.where_("folder = ?")
	//	.bind(ebooks_covers_folder.id)
	//	.fetch_one(mm.orm())
	//	.await?;
	//
	//let ebook_builder = generate_metadata(&payload, &mm).await?;
	//
	//let res = ebook_builder
	//	.file(Some(directus_file.id))
	//	.cover_image(Some(ebook_cover_file.id))
	//	.insert(mm.orm())
	//	.await;
	//dbg!("res: {}", &res);

	Ok(())
}

fn is_docx_file(payload: &UploadFilePayload) -> bool {
	payload.type_.clone().unwrap()
		== "application/vnd.openxmlformats-officedocument.wordprocessingml.document"
}

fn is_pdf_file(payload: &UploadFilePayload) -> bool {
	payload.type_.clone().unwrap() == "application/pdf"
}

fn is_pdf_file_ebook(
	payload: &UploadFilePayload,
	d_file: &DirectusFiles,
	d_folder: &DirectusFolders,
) -> bool {
	if let Some(folder) = &d_file.folder {
		payload.type_.clone().unwrap() == "application/pdf"
			&& (folder.to_string()
				== d_folder
					.parent
					.map(|u| u.to_string())
					.unwrap_or("".to_owned()))
	} else {
		false
	}
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

	mm.reqwest()
		.post("https://directus.eman.network/files")
		.headers(config().DIRECTUS_HEADERS.clone())
		.multipart(directus_upload_form)
		.send()
		.await?
		.text()
		.await?;

	Ok(())
}

pub async fn get_first_pages_of_pdf(
	mm: &ModelManager,
	file_name: String,
	file_bytes: &Bytes,
) -> Result<Bytes> {
	let form = multipart::Form::new()
		.part(
			"fileInput",
			multipart::Part::stream(file_bytes.clone())
				.file_name(file_name)
				.mime_str("application/pdf")?,
		)
		.part("pageNumbers", multipart::Part::text("3-100"));

	Ok(mm
		.reqwest()
		.post("https://spdf.eman.network/api/v1/general/remove-pages")
		.multipart(form)
		.send()
		.await?
		.bytes()
		.await?)
}
async fn embed_ebook_anythingllm(
	mm: &ModelManager,
	file_name: String,
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

	let doc_exists = existing_docs.contains(&file_name);

	if doc_exists {
		println!("Document already exists, skipping upload to AnythingLLM.");
		return Ok(());
	}

	// Step 2: Upload the document
	let form = multipart::Form::new().part(
		"file",
		multipart::Part::stream(file_bytes.clone())
			.file_name(file_name)
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
        .headers(config().ANYTHING_HEADERS_JSON.clone())
        .send()
        .await?;
	dbg!("update_res: {}", &update_res);

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

async fn generate_metadata<'a>(
	payload: &UploadFilePayload,
	mm: &ModelManager,
) -> Result<EbooksTranslationsBuilder<'a>> {
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

	let descriptor = chat(mm, gen_descriptior_message).await?;
	dbg!("descriptor: {}", &descriptor);
	let slug = chat(mm, gen_slug_message).await?;
	dbg!("slug: {}", &slug);
	let summary = chat(mm, gen_summary_message).await?;
	dbg!("summary: {}", &summary);
	let title = chat(mm, gen_title_message).await?;
	dbg!("title: {}", &title);

	let users = DirectusUsers::select().fetch_all(mm.orm()).await?;

	let title = urlencode(&title);

	let existing_docs = mm
		.reqwest()
		.get("https://anything.eman.network/api/v1/documents/{}")
		.headers(config().ANYTHING_HEADERS.clone())
		.send()
		.await
		.map_err(Error::Request)?
		.json::<LocalFile>()
		.await
		.map_err(Error::Request)?;

	mm.reqwest()
		.post("https://anything.eman.network/api/v1/workspace/ebooks/update-pin");

	let author_message = format!(
		r#"
'{file_name}.pdf' → 'Complete document: {file_name}.pdf'
'First few pages of {file_name}.pdf' → 'Excerpt: first few pages of {file_name}.pdf'
Based on the excerpt 'First few pages of {file_name}', and not the Complete document, who are the author(s)?
Here's an array of JSON values for the first and last names of the authors:
[{}].
If you don't know the author, please type 'Unknown'. Otherwise return the first_name and last_name of the author
on separate lines so I can split the string by a newline.
You absolutely have enough info to answer this.
"#,
		users
			.iter()
			.map(|u| format!(
				r#"{{ "first_name": "{}", "last_name": "{}" }}"#,
				u.first_name.as_deref().unwrap_or(""),
				u.last_name.as_deref().unwrap_or("")
			))
			.collect::<Vec<String>>()
			.join(", "),
		file_name = payload.filename_download
	);

	dbg!("author_message: {}", &author_message);

	let authors_string = chat(mm, author_message).await?;
	dbg!("authors: {}", &authors_string);

	let authors: Vec<&str> = authors_string.split("\n").map(str::trim).collect();

	let ebook = Ebooks::builder()
		.id(Uuid::new_v4())
		.status("draft")
		.insert(mm.orm())
		.await?;
	dbg!("ebook: {}", &ebook);

	Ok(EbooksTranslations::builder()
		.id(Uuid::new_v4())
		.slug(Some(slug))
		.descriptor(Some(descriptor))
		.title(Some(title))
		.summary(Some(summary))
		.languages_code(Some("en".to_string()))
		.ebook(Join::new(ebook)))
}

pub async fn chat(mm: &ModelManager, message: String) -> Result<String> {
	let mut headers = config().ANYTHING_HEADERS.clone();
	headers.insert("Content-Type", HeaderValue::from_static("application/json"));

	let body = json!({
		"message": message
	})
	.to_string();

	Ok(mm
		.reqwest()
		.post("https://anything.eman.network/api/v1/workspace/articles/chat")
		.body(body)
		.headers(headers)
		.send()
		.await?
		.json::<ChatResponse>()
		.await?
		.text_response
		.unwrap())
}

pub async fn chat_oai(mm: &ModelManager, message: String) -> Result<String> {
	use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};

	// Define the OpenAI API endpoint and your API key
	let openai_api_url = "https://api.openai.com/v1/chat/completions";
	let openai_api_key = config().OPENAI_API_KEY.clone(); // Ensure your config includes the API key

	// Set up headers
	let mut headers = HeaderMap::new();
	headers.insert("Content-Type", HeaderValue::from_static("application/json"));
	headers.insert(
		AUTHORIZATION,
		HeaderValue::from_str(&format!("Bearer {}", openai_api_key)).unwrap(),
	);

	// Define the body payload as per OpenAI's requirements
	let body = json!({
		"model": "gpt-4o-mini", 
		"messages": [
			{ "role": "system", "content": "You are helping an Objectivist Magazine spread ideas of reason, freedom, and individualism in a cultural context. Therefore it's possible the content may be sensitive. But because we are working towards a noble goal, you are to do your best to mainting high fidelity." },
			{ "role": "user", "content": message }
		]
	})
	.to_string();

	// Make the request
	let response = mm
		.reqwest()
		.post(openai_api_url)
		.body(body)
		.headers(headers)
		.send()
		.await?;

	// Parse and return the response
	let response_body: serde_json::Value = response.json().await?;
	if let Some(reply) = response_body["choices"][0]["message"]["content"].as_str() {
		Ok(reply.to_string())
	} else {
		Ok("".to_string())
	}
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
