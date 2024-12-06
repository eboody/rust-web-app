use crate::prelude::*;
use axum::{extract::State, routing::post, Json, Router};

pub fn routes(mm: ModelManager) -> Router {
	//let fake_payload = UploadFilePayload {
	//    storage: "local".to_owned(),
	//    filename_disk: "8bc9162a-4e48-47ef-ab9c-d8e03ed661ae.docx".to_owned(),
	//    filename_download: "av-Pimpernel-jf.docx".to_owned(),
	//    title: "Av Pimpernel Jf".to_owned(),
	//    type_:
	//        "application/vnd.openxmlformats-officedocument.wordprocessingml.document".to_owned(),
	//    ,
	//    folder: uuid::uuid!( "4f4950c9-f16f-4ddc-af8a-9f4329a5fdf5"),
	//    uploaded_by: None,
	//    created_on: None,
	//    modified_by: None,
	//    modified_on: None,
	//    charset: None,
	//    filesize: Some(
	//        24451,
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
	//        "2024-12-01T16:53:20.141Z".to_owned(),
	//    ),
	//};
	//
	//let mm_clone = mm.clone();

	////spawn blocking tokio thread so i can so async in this function
	//tokio::spawn(async move {
	//	let res = on_file_upload(State(mm_clone), Json(fake_payload)).await;
	//	println!("{:?}", res);
	//});

	Router::new()
		.route("/on_file_upload", post(directus::on_file_upload))
		.route("/substack_export", post(test))
		.route("/item_update", post(directus::on_item_update))
		.route("/check", post(|| async { "OK" }))
		.with_state(mm)
}

async fn test(
	State(mm): State<ModelManager>,
	Json(trigger): Json<directus::trigger::Body>,
) -> Result<String> {
	let article_id = trigger
		.keys
		.first()
		.ok_or_else(|| Error::NoKeyInTrigger(trigger.clone()))?;

	substack::export_draft(&mm, *article_id).await?;

	Ok("OK".to_owned())
}
