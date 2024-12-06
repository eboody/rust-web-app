mod endnote_converter;
mod error;
mod structs;

pub use error::{Error, Result};

use encoding_rs::UTF_8;
use lib_core::model::{directus::*, ModelManager};
use std::{
	fs::{self, File},
	io::Write,
	path::Path,
	process::Command,
	thread::sleep,
	time::Duration,
};

pub use endnote_converter::*;
use ormlite::{types::Uuid, Model};
pub use structs::*;

pub fn convert_docx_to_md(input_path: &Path, output_path: &Path) -> Result<()> {
	let html_output = Command::new("./scripts/mammoth.js/bin/mammoth")
		.arg("--style-map=scripts/mammoth.js/style-map.mammoth")
		.arg(input_path)
		.output()?;

	if !html_output.status.success() {
		return Err(Error::MammothFailed(format!(
			"Failed to run mammoth on {:?}: {}",
			input_path,
			String::from_utf8_lossy(&html_output.stderr)
		)));
	}

	let (html_string, _, had_errors) = UTF_8.decode(&html_output.stdout);
	if had_errors {
		eprintln!("There were encoding errors in the HTML output");
	}

	let markdown =
		htmd::convert(&html_string).expect("Failed to convert HTML to Markdown");
	let markdown = markdown.replace(
		r#"
    
    [↑]"#,
		" [↑]",
	);
	let markdown_bytes = markdown.as_bytes();

	let mut file = File::create(output_path)?;
	file.write_all(markdown_bytes)?;

	Ok(())
}

pub fn get_content_from_file<T: AsRef<Path>>(
	file_path: T,
) -> std::result::Result<Content, Box<dyn std::error::Error>> {
	let content = fs::read_to_string(file_path)?;
	let document: Content = json::from_str(&content)?;
	Ok(document)
}

pub fn get_content_from_string(
	content: &str,
) -> std::result::Result<Content, Box<dyn std::error::Error>> {
	let document: Content = json::from_str(content)?;
	Ok(document)
}

pub fn extract_heading(document: &mut Content) -> Option<String> {
	let heading = document.content.as_ref().and_then(|nodes| {
		nodes.iter().find_map(|node| {
			if matches!(node.type_, NodeType::Heading) {
				node.content.as_ref().map(|content| {
					content
						.iter()
						.filter_map(|node| node.text.clone())
						.collect::<String>()
				})
			} else {
				None
			}
		})
	});

	if let Some(ref mut nodes) = document.content {
		nodes.retain(|node| !matches!(node.type_, NodeType::Heading));
	}

	heading
}

pub async fn export_to_substack(mm: &ModelManager, article_id: Uuid) -> Result<()> {
	let english_article = ArticlesTranslations::select()
		.join(ArticlesTranslations::article())
		.where_("languages_code = ?")
		.bind(Language::English.to_string())
		.where_("articles_id = ?")
		.bind(article_id)
		.fetch_one(mm.orm())
		.await?;
	let doc = md_to_prosemirror(
		&english_article
			.content
			.ok_or_else(|| Error::NoArticleContent)?,
	)?;

	let mut doc = doc.into();
	transform_to_substack_format(&mut doc);

	let payload = DraftRequest {
		audience: "everyone".to_string(),
		draft_body: json::to_string(&doc).unwrap(),
		draft_bylines: vec![DraftByline {
			id: 292604153,
			is_guest: false,
		}],
		draft_podcast_duration: None,
		draft_podcast_preview_upload_id: None,
		draft_podcast_upload_id: None,
		draft_podcast_url: "".to_string(),
		draft_section_id: None,
		draft_subtitle: "".to_string(),
		draft_title: english_article.title.ok_or_else(|| Error::NoArticleTitle)?,
		draft_video_upload_id: None,
		draft_voiceover_upload_id: None,
		section_chosen: false,
		r#type: "newsletter".to_string(),
	};

	let mut headers = reqwest::header::HeaderMap::new();
	headers.insert("accept", "application/json".parse().unwrap());
	headers.insert("content-type", "application/json".parse().unwrap());
	headers.insert("user-agent", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/128.0.0.0 Safari/537.36".parse().unwrap());
	headers.insert("cookie", "ab_experiment_sampled=\"false\"; ab_testing_id=\"08ed65a0-9a42-4070-a9b0-a513f220ee66\"; cookie_storage_key=05a051ab-3608-4165-b0fb-57028fc8d6b5; substack.sid=s%3AAXkgh2skDfIuUkpmVQrvrNW5c_PW8fb_.4I39nNwU2k7LTZ%2F2V61EYyNAeLksqJvOk6EIyAK5%2FyY; ajs_anonymous_id=\"b6bf0691-177b-4411-b6fa-fee8b5767bdd\"; substack.lli=1; __cf_bm=jGEhBtEFPt1UfN.4xBhXs7xgRUrerOTA3wnirL7s7UU-1732980009-1.0.1.1-OaPCDJ0K3yBpbom97WYOoMrf20Aoh6phruERFayEBq5GOelsPjhNs4gUVXz4wiyaE_TvPt3NOdfnReY9jNAnkA; visit_id={\"id\":\"962508ff-2f87-421d-96ed-bafcd2db690c\",\"timestamp\":\"2024-11-30T15:25:11.807Z\",\"utm_source\":\"substack\"}; AWSALBTG=RtLsRiLD88Ab2ywUwrRCGeZWBbRQGYPc3dVEPcSJe++l04pvwm22ggzYyOKE5uNnt6JEOrqzEBQ3zR3JV7eLc+fooxjqjzzzzdz5Z/JEDciXukVeq16GQie2vSNv4knI9YATiy6ge9sOjXw6JOZmUG0ugQVe7K/ON5Ic3AU66tyu; AWSALBTGCORS=RtLsRiLD88Ab2ywUwrRCGeZWBbRQGYPc3dVEPcSJe++l04pvwm22ggzYyOKE5uNnt6JEOrqzEBQ3zR3JV7eLc+fooxjqjzzzzdz5Z/JEDciXukVeq16GQie2vSNv4knI9YATiy6ge9sOjXw6JOZmUG0ugQVe7K/ON5Ic3AU66tyu".parse().unwrap());

	let response = mm
		.reqwest()
		.post("https://theobjectivestandard.substack.com/api/v1/drafts")
		.headers(headers.clone())
		.json(&payload)
		.send()
		.await?;

	let draft_response = response.json::<DraftResponse>().await?;

	println!("response: {:?}", draft_response);

	sleep(Duration::from_secs(25));

	let response = mm
		.reqwest()
		.delete(format!(
			"https://theobjectivestandard.substack.com/api/v1/drafts/{}",
			draft_response.id
		))
		.headers(headers)
		.send()
		.await?;

	dbg!("response: {}", &response);
	todo!()
}

pub fn md_to_prosemirror(md: &str) -> Result<Document> {
	let prosemirror_output = Command::new("./scripts/to-prosemirror/mdtp.js")
		.arg(md)
		.output()?;
	if !prosemirror_output.status.success() {
		return Err(Error::ProseMirrorFailed);
	}

	let (prosemirror_string, _, _) = UTF_8.decode(&prosemirror_output.stdout);

	let doc: Document = json::from_str(&prosemirror_string)?;
	Ok(doc)
}

//#[tokio::main]
//async fn main() -> Result<(), Box<dyn std::error::Error>> {
//	let client = Client::new();
//	let project_dir = env!("CARGO_MANIFEST_DIR");
//
//	let mut doc = get_content_from_file(format!("{project_dir}/AWW-Atlas-PM.json"))?;
//
//	transform_to_substack_format(&mut doc);
//
//	let heading = extract_heading(&mut doc);
//	dbg!("doc: {}", &doc);
//	dbg!("heading: {}", &heading);
//
//	let payload = DraftRequest {
//		audience: "everyone".to_string(),
//		draft_body: json::to_string(&doc).unwrap(),
//		draft_bylines: vec![DraftByline {
//			id: 292604153,
//			is_guest: false,
//		}],
//		draft_podcast_duration: None,
//		draft_podcast_preview_upload_id: None,
//		draft_podcast_upload_id: None,
//		draft_podcast_url: "".to_string(),
//		draft_section_id: None,
//		draft_subtitle: "".to_string(),
//		draft_title: heading.unwrap_or("".to_string()),
//		draft_video_upload_id: None,
//		draft_voiceover_upload_id: None,
//		section_chosen: false,
//		r#type: "newsletter".to_string(),
//	};
//
//	dbg!("payload: {}", &payload);
//
//	let mut headers = reqwest::header::HeaderMap::new();
//	headers.insert("accept", "application/json".parse().unwrap());
//	headers.insert("content-type", "application/json".parse().unwrap());
//	headers.insert("user-agent", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/128.0.0.0 Safari/537.36".parse().unwrap());
//	headers.insert("cookie", "ab_experiment_sampled=\"false\"; ab_testing_id=\"08ed65a0-9a42-4070-a9b0-a513f220ee66\"; cookie_storage_key=05a051ab-3608-4165-b0fb-57028fc8d6b5; substack.sid=s%3AAXkgh2skDfIuUkpmVQrvrNW5c_PW8fb_.4I39nNwU2k7LTZ%2F2V61EYyNAeLksqJvOk6EIyAK5%2FyY; ajs_anonymous_id=\"b6bf0691-177b-4411-b6fa-fee8b5767bdd\"; substack.lli=1; __cf_bm=jGEhBtEFPt1UfN.4xBhXs7xgRUrerOTA3wnirL7s7UU-1732980009-1.0.1.1-OaPCDJ0K3yBpbom97WYOoMrf20Aoh6phruERFayEBq5GOelsPjhNs4gUVXz4wiyaE_TvPt3NOdfnReY9jNAnkA; visit_id={\"id\":\"962508ff-2f87-421d-96ed-bafcd2db690c\",\"timestamp\":\"2024-11-30T15:25:11.807Z\",\"utm_source\":\"substack\"}; AWSALBTG=RtLsRiLD88Ab2ywUwrRCGeZWBbRQGYPc3dVEPcSJe++l04pvwm22ggzYyOKE5uNnt6JEOrqzEBQ3zR3JV7eLc+fooxjqjzzzzdz5Z/JEDciXukVeq16GQie2vSNv4knI9YATiy6ge9sOjXw6JOZmUG0ugQVe7K/ON5Ic3AU66tyu; AWSALBTGCORS=RtLsRiLD88Ab2ywUwrRCGeZWBbRQGYPc3dVEPcSJe++l04pvwm22ggzYyOKE5uNnt6JEOrqzEBQ3zR3JV7eLc+fooxjqjzzzzdz5Z/JEDciXukVeq16GQie2vSNv4knI9YATiy6ge9sOjXw6JOZmUG0ugQVe7K/ON5Ic3AU66tyu".parse().unwrap());
//
//	let response = client
//		.post("https://theobjectivestandard.substack.com/api/v1/drafts")
//		.headers(headers.clone())
//		.json(&payload)
//		.send()
//		.await?;
//
//	let draft_response = response.json::<DraftResponse>().await?;
//
//	println!("response: {:?}", draft_response);
//
//	sleep(Duration::from_secs(25));
//
//	let response = client
//		.delete(format!(
//			"https://theobjectivestandard.substack.com/api/v1/drafts/{}",
//			draft_response.id
//		))
//		.headers(headers)
//		.send()
//		.await?;
//
//	dbg!("response: {}", &response);
//	Ok(())
//}
