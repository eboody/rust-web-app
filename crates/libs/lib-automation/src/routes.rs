use crate::on_file_upload::on_file_upload;
use axum::{extract::State, routing::post, Json, Router};
use lib_core::model::{ModelManager, UploadFilePayload};

pub fn routes(mm: ModelManager) -> Router {
	let fake_payload = UploadFilePayload {
    storage: "local".to_owned(),
    filename_disk: "7010e2a9-8ed6-4868-91d4-a8513993617e.docx".to_owned(),
    filename_download: "av-Pimpernel-jf.docx".to_owned(),
    title: "Av Pimpernel Jf".to_owned(),
    type_: Some(
        "application/vnd.openxmlformats-officedocument.wordprocessingml.document".to_owned(),
    ),
    folder: Some(
			uuid::uuid!(
        "4f4950c9-f16f-4ddc-af8a-9f4329a5fdf5")
    ),
    uploaded_by: None,
    created_on: None,
    modified_by: None,
    modified_on: None,
    charset: None,
    filesize: Some(
        24451,
    ),
    width: None,
    height: None,
    duration: None,
    embed: None,
    description: None,
    location: None,
    tags: None,
    metadata: None,
    focal_point_x: None,
    focal_point_y: None,
    tus_id: None,
    tus_data: None,
    uploaded_on: Some(
        "2024-12-01T16:53:20.141Z".to_owned(),
    ),
};

	let mm_clone = mm.clone();

	//spawn blocking tokio thread so i can so async in this function
	tokio::spawn(async move {
		let res = on_file_upload(State(mm_clone), Json(fake_payload)).await;
		println!("{:?}", res);
	});

	Router::new()
		.route("/on_file_upload", post(on_file_upload))
		.route(
			"/check",
			post(|| async {
				println!("check");
				"OK"
			}),
		)
		.with_state(mm)
}
