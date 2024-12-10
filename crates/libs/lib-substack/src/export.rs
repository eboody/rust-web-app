use std::{thread::sleep, time::Duration};

use super::conversions::*;
use crate::{
	audiences::Audience,
	config::config,
	drafts::{DraftBody, DraftType},
	prose_mirror::Node,
};
use lib_core::model::directus::{
	Articles, ArticlesSubstackStatus, SubsctackArticleStatus,
};

use crate::api::drafts::{DraftByline, DraftRequest, DraftResponse};

use crate::prelude::*;

pub async fn export_draft(mm: &ModelManager, article_id: Uuid) -> Result<()> {
	let article = Articles::select()
		.where_("id = ?")
		.bind(article_id)
		.fetch_one(mm.orm())
		.await?;

	let content = article
		.body
		.as_deref()
		.ok_or_else(|| Error::NoArticleContent)?;

	let re = regex::Regex::new(r"<!--.*?-->").unwrap();
	let content = re.replace_all(content, "").to_string();

	let endnotes = article.endnotes.as_deref().unwrap_or("");

	let doc = md_to_prosemirror(&content)?;

	let mut doc: Node = doc.into();
	transform_to_substack_format(&mut doc);

	let endnotes = md_to_prosemirror(endnotes)?;
	let mut endnotes = transform_endnotes_for_substack(&endnotes.into());

	//append the endnotes to the prosemirror doc
	if let Some(mut c) = doc.content.clone() {
		c.append(&mut endnotes);
		doc.content = Some(c);
	}

	let payload = DraftRequest {
		audience: Audience::Everyone,
		draft_body: DraftBody(json::to_string(&doc).unwrap()),
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
		draft_title: (article.title)
			.as_ref()
			.ok_or_else(|| Error::NoArticleTitle)?
			.to_owned(),
		draft_video_upload_id: None,
		draft_voiceover_upload_id: None,
		section_chosen: false,
		r#type: DraftType::Newsletter,
	};

	let response = mm
		.reqwest()
		.post("https://theobjectivestandard.substack.com/api/v1/drafts")
		.headers(config().HEADERS.clone())
		.json(&payload)
		.send()
		.await?;

	let draft_response = response.json::<DraftResponse>().await?;

	let substack_status = ArticlesSubstackStatus {
		id: Uuid::new_v4(),
		articles_id: article_id,
		substack_id: draft_response.id,
		status: SubsctackArticleStatus::Draft,
		date_updated: OffsetDateTime::now_utc(),
		message: "Successfully exported to substack".to_owned().into(),
		sort: None,
	}
	.insert(mm.orm())
	.on_conflict(OnConflict::do_update_on_pkey("articles_id"))
	.await?;

	article
		.update_partial()
		.substack_status(Some(substack_status.id))
		.date_updated(Some(OffsetDateTime::now_utc()))
		.update(mm.orm())
		.await?;

	dbg!("substack_status: {}", &substack_status);

	sleep(Duration::from_secs(25));

	let response = mm
		.reqwest()
		.delete(format!(
			"https://theobjectivestandard.substack.com/api/v1/drafts/{}",
			draft_response.id
		))
		.headers(config().HEADERS.clone())
		.send()
		.await?;
	dbg!("response: {}", &response);

	let substack_status_id = substack_status.id;
	let res = substack_status.delete(mm.orm()).await;

	if res.is_err() {
		dbg!(
			"Failed to delete articles_substack_status: {}\n{}",
			substack_status_id,
			&res
		);
	}

	println!("deleted");
	Ok(())
}

#[allow(unused)]
pub async fn delete_draft(
	mm: &ModelManager,
	substack_status: ArticlesSubstackStatus,
) -> Result<()> {
	let substack_status = substack_status.delete(mm.orm()).await;
	dbg!("substack_status: {}", &substack_status);
	//
	//let response = mm
	//	.reqwest()
	//	.delete(format!(
	//		"https://theobjectivestandard.substack.com/api/v1/drafts/{}",
	//		draft_response.id
	//	))
	//	.headers(headers)
	//	.send()
	//	.await?;

	Ok(())
}
