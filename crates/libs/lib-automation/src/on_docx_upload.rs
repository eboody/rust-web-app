#![allow(unused_imports)]
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

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
use lib_core::model::ArticlesTranslations;
use lib_core::model::DirectusFiles;
use lib_core::model::DirectusFolders;
use lib_core::model::DirectusUsers;
use lib_core::model::Ebooks;
use lib_core::model::EbooksTranslations;
use lib_core::model::EbooksTranslationsBuilder;
use lib_core::model::{ModelManager, UploadFilePayload};
use lib_substack::convert_docx_to_md;
use ormlite::model::Join;
use ormlite::model::ModelBuilder;
use ormlite::Model;
use reqwest::multipart;
use serde_json::json;
use uuid::Uuid;

pub async fn on_docx_upload(
	mm: &ModelManager,
	payload: &UploadFilePayload,
	directus_file: &DirectusFiles,
) -> Result<()> {
	// first we want to download the file
	let file_id = directus_file.id;
	let directus_url = format!("{}/assets/{}", &config().DIRECTUS_URL, file_id);
	let response = mm.reqwest().get(&directus_url).send().await?;
	let bytes = response.bytes().await?;
	let mut file = File::create("output.docx")?;
	file.write_all(&bytes)?;

	// get absolute path of the file
	let input_file_path = Path::new("output.docx").canonicalize()?;
	let input_file_path = input_file_path.as_path();

	let output_file_path = Path::new("output.md");

	dbg!("input_file_path: {}", &input_file_path);
	// now we want to convert the docx to md
	println!("\n\n\n\n\n");
	convert_docx_to_md(input_file_path, output_file_path)?;
	println!("\n\n\n\n\n");

	// now we want to read the md file
	let content = fs::read_to_string(output_file_path)?;
	dbg!("content: {}", &content);

	// then we'll create a new articles_translations item
	//let art_tr = ArticlesTranslations {
	//	id: Uuid::new_v4(),
	//	languages_code: Some("en".to_owned()),
	//	title: Some(payload.title.clone()),
	//	content: Some(content.clone()),
	//	..Default::default()
	//};
	// then we'll put the md in the content field of that item
	// then we'll delete both the docx and the md
	println!("\n\n\n\n\n");
	todo!()
}
