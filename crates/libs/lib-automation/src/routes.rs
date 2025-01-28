use crate::directus::tasks;
use crate::prelude::*;
use axum::{Json, Router, extract::State, routing::post};
use lib_core::model::Articles;

pub fn routes(mm: ModelManager) -> Router {
    let mm_clone = mm.clone();
    tokio::spawn(async move { fix_drafts(mm_clone).await });

    Router::new()
        .route("/draft", post(export_draft).delete(delete_draft))
        //.route("/on_file_upload", post(directus::on_file_upload))
        .route(
            "/item_update",
            post(crate::directus::events::on_item_update),
        )
        .route("/check", post(|| async { "OK" }))
        .with_state(mm)
}

async fn fix_drafts(mm: ModelManager) {
    tasks::sync_sections(&mm).await.unwrap();

    let ss_drafts = model::SubstackDraft::select()
        .fetch_all(mm.orm())
        .await
        .expect("Failed to fetch drafts");

    for ss_draft in ss_drafts {
        let draft_article_id = ss_draft.articles_id;

        if Articles::select()
            .where_("id = ?")
            .bind(draft_article_id)
            .fetch_one(mm.orm())
            .await
            .is_err()
        {
            println!("Deleting draft: {:#?}", ss_draft.draft_title);
            //lib_substack::drafts::Request::delete(mm.reqwest(), ss_draft.substack_draft_id)
            //    .await
            //    .expect("Failed to delete draft");
            //
            //ss_draft.delete(mm.orm()).await.unwrap();
        }
    }
}

//#[allow(unused)]
//async fn fix_substack_articles(mm: ModelManager, articles: Vec<model::Articles>) {
//    tokio::spawn(async move {
//        tasks::sync_sections(&mm).await.unwrap();
//
//        let articles = Articles::select()
//            .join(Articles::author())
//            .where_("articles.status = ?")
//            .bind(Status::Published)
//            .where_("articles.issue IS NULL")
//            .order_by("articles.date_updated", Direction::Asc)
//            .fetch_all(mm.orm())
//            .await
//            .expect("Failed to fetch articles");
//
//        tokio::spawn(async move {
//            let mm = mm.clone();
//
//            let regex = Regex::new(
//                r"https?://(?:www\.)?theobjectivestandard\.com/(?:[^/\s)]+/)*([^/\s)]+)",
//            )
//            .unwrap();
//
//            for mut article in articles {
//                if let Some(body) = article.body {
//                    let body = regex.replace_all(&body, "/p/$1");
//
//                    article.body = Some(body.to_string());
//
//                    article
//                        .update_partial()
//                        .body(Some(body.to_string()))
//                        .update(mm.orm())
//                        .await
//                        .expect("Failed to update article body with /p/slugs");
//                }
//
//                let Some(mut ss_draft) = model::SubstackDraft::select()
//                    .where_("articles_id = ?")
//                    .bind(article.id)
//                    .fetch_one(mm.orm())
//                    .await
//                    .ok()
//                else {
//                    continue;
//                };
//
//                println!("\n\n\n\nARTICLE: {:#?}", article.title);
//                println!("DRAFT: {:#?}", ss_draft.id);
//
//                if let Some(first_name) = &article.author.first_name
//                    && let Some(last_name) = &article.author.last_name
//                {
//                    let res = lib_substack::drafts::Request::get(
//                        mm.reqwest(),
//                        ss_draft.substack_draft_id,
//                    )
//                    .await
//                    .expect("Failed to fetch draft");
//
//                    let body = res.draft_body;
//
//                    let mut req = lib_substack::drafts::Request {
//                        draft_body: article
//                            .as_ref()
//                            .try_into()
//                            .expect("Failed to convert article"),
//                        draft_subtitle: format!("By {} {}", first_name, last_name),
//                        post_date: Some(
//                            article
//                                .date_published
//                                .expect("Article has no date published")
//                                .with_time(Time::MIDNIGHT)
//                                .assume_utc(),
//                        ),
//                        draft_title: article.title.clone().expect("Article has no title"),
//                        audience: res.audience,
//                        cover_image: None,
//                        description: res.description,
//                        draft_bylines: vec![],
//                        draft_podcast_duration: res.draft_podcast_duration,
//                        draft_podcast_preview_upload_id: res.draft_podcast_preview_upload_id,
//                        draft_podcast_upload_id: res.draft_podcast_upload_id,
//                        draft_podcast_url: "".to_owned(),
//                        type_: lib_substack::post::Type::Newsletter,
//                        draft_section_id: None,
//                        draft_video_upload_id: res.draft_video_upload_id,
//                        draft_voiceover_upload_id: res.draft_voiceover_upload_id,
//                        section_chosen: false,
//                        social_title: res.social_title,
//                    };
//
//                    if let Some(section_id) = article.section {
//                        let section = model::Sections::select()
//                            .where_("id = ?")
//                            .bind(section_id)
//                            .fetch_one(mm.orm())
//                            .await
//                            .unwrap_or_else(|_| {
//                                panic!(
//                                    "Failed to fetch section: {:#?} for article: {:#?}",
//                                    article.section, article.title
//                                )
//                            });
//
//                        ss_draft = ss_draft
//                            .update_partial()
//                            .section_chosen(Some(true))
//                            .section_id(section.substack_id)
//                            .update(mm.orm())
//                            .await
//                            .expect("Failed to update draft");
//
//                        req.draft_section_id = section.substack_id;
//                        req.section_chosen = true;
//                    }
//
//                    let cover_image_file = if let Ok(file) = model::Files::select()
//                        .where_("id = ?")
//                        .bind(article.featured_image)
//                        .fetch_one(mm.orm())
//                        .await
//                    {
//                        file
//                    } else {
//                        model::Files::select()
//                            .where_("id = ?")
//                            .bind(
//                                Uuid::from_str("a8fd08ec-82bf-484e-a032-fea9a7892878")
//                                    .expect("Invalid UUID"),
//                            )
//                            .fetch_one(mm.orm())
//                            .await
//                            .expect("Failed to fetch fallback image")
//                    };
//
//                    let substack_cover_image = model::SubstackCoverImage::select()
//                        .where_("substack_draft = ?")
//                        .bind(ss_draft.id)
//                        .fetch_one(mm.orm())
//                        .await
//                        .unwrap_or_else(|_| {
//                            panic!(
//                                "Failed to fetch cover image for draft: {:#?}",
//                                ss_draft.draft_title
//                            )
//                        });
//
//                    ss_draft = ss_draft
//                        .update_partial()
//                        .cover_image(Some(substack_cover_image.url.clone().to_string()))
//                        .update(mm.orm())
//                        .await
//                        .expect("Failed to update draft");
//
//                    req.cover_image = Some(substack_cover_image.url.clone());
//
//                    if let Some(featured_image) = article.featured_image {
//                        article.body = article
//                            .body
//                            .map(|b| format!("![]({})\n\n{}", substack_cover_image.url, b));
//
//                        req.draft_body = article
//                            .as_ref()
//                            .try_into()
//                            .expect("Failed to convert article");
//                    }
//
//                    lib_substack::drafts::Request::publish(
//                        mm.reqwest(),
//                        ss_draft.substack_draft_id,
//                        lib_substack::drafts::PublishArgs {
//                            send: false,
//                            share_automatically: false,
//                        },
//                    )
//                    .await
//                    .expect("Failed to publish draft");
//
//                    let updated_post = req
//                        .put(mm.reqwest(), ss_draft.substack_draft_id)
//                        .await
//                        .inspect_err(|e| {
//                            println!("ERROR: {:#?}", e);
//                            die!("Failed to update draft");
//                        })
//                        .expect("Failed to update draft");
//
//                    ss_draft
//                        .update_partial()
//                        .draft_body(
//                            json::from_str(&updated_post.draft_body).expect("Failed to parse body"),
//                        )
//                        .subtitle(updated_post.subtitle)
//                        .draft_subtitle(Some(updated_post.draft_subtitle))
//                        .date_updated(Some(time::OffsetDateTime::now_utc()))
//                        .update(mm.orm())
//                        .await
//                        .unwrap();
//
//                    article
//                        .update_partial()
//                        .date_updated(Some(time::OffsetDateTime::now_utc()))
//                        .update(mm.orm())
//                        .await
//                        .unwrap();
//                }
//
//                //let substack_draft = model::SubstackDraft::select()
//                //    .where_("articles_id = ?")
//                //    .bind(article.id)
//                //    .fetch_one(mm.orm())
//                //    .await;
//                //
//                //if substack_draft.is_ok() {
//                //    continue;
//                //}
//                //
//                //let res = tasks::substack::drafts::create_iterative()
//                //    .model_manager(&mm)
//                //    .root_article_id(article.id)
//                //    .should_publish(true)
//                //    .call()
//                //    .await;
//                //
//                //println!("res: {:#?}", res);
//                //
//                //let _ = res.inspect_err(|e| {
//                //    println!("ERROR: {:#?}", e);
//                //    die!("Failed to create draft");
//                //});
//
//                //let substack_draft = model::SubstackDraft::select()
//                //    .where_("articles_id = ?")
//                //    .bind(article.id)
//                //    .fetch_one(mm.orm())
//                //    .await;
//
//                //if let Ok(substack_draft) = substack_draft {
//                //    let res = lib_substack::drafts::Request::publish(
//                //        mm.reqwest(),
//                //        substack_draft.substack_draft_id,
//                //        lib_substack::drafts::PublishArgs {
//                //            send: false,
//                //            share_automatically: false,
//                //        },
//                //    )
//                //    .await;
//                //} else {
//                //    tracing::warn!(
//                //        "Failed to publish draft: {:?} {:#?}",
//                //        article.id,
//                //        article.title
//                //    );
//                //
//                //    let mut file = std::fs::OpenOptions::new()
//                //        .create(true)
//                //        .append(true)
//                //        .open("articles.txt")
//                //        .unwrap();
//                //
//                //    let article_info = format!("Article: {}\n", article.id);
//                //
//                //    file.write_all(article_info.as_bytes()).unwrap();
//                //}
//            }
//            println!("DONE WITH SYNC");
//        });
//
//        //let res = tasks::create_substack_draft(&mm, article.id).await;
//        //
//        //if let Ok(draft) = res {
//        //  tokio::time::sleep(std::time::Duration::from_secs(10)).await;
//        //
//        //  let related_articles = lib_core::model::RelatedArticles::select()
//        //    .where_("articles_id = ?")
//        //    .bind(article.id)
//        //    .fetch_all(mm.orm())
//        //    .await
//        //    .expect("Failed to fetch related articles");
//        //
//        //  for related_article in related_articles {
//        //    let article = lib_core::model::Articles::select()
//        //      .where_("id = ?")
//        //      .bind(related_article.related_articles_id)
//        //      .fetch_one(mm.orm())
//        //      .await
//        //      .unwrap();
//        //
//        //    let related_draft = lib_core::model::SubstackDraft::select()
//        //      .where_("articles_id = ?")
//        //      .bind(article.id)
//        //      .fetch_one(mm.orm())
//        //      .await
//        //      .expect("Failed to fetch related draft");
//        //
//        //    tasks::delete_substack_draft(
//        //      &mm,
//        //      article.id,
//        //      related_draft.substack_draft_id,
//        //    )
//        //    .await
//        //    .expect("Failed to delete related draft");
//        //  }
//        //
//        //  tasks::delete_substack_draft(&mm, article.id, draft.substack_draft_id)
//        //    .await
//        //    .unwrap();
//        //}
//        //
//        //
//        //if article.tags.is_none() {
//        //  directus::tasks::add_tags(&mm, &article).await.unwrap();
//        //}
//        //
//        //if article.section.is_none() {
//        //  directus::tasks::select_section(&mm, &article)
//        //    .await
//        //    .unwrap();
//        //}
//        //
//        //if article.subtitle.is_none() {
//        //  directus::tasks::add_subtitle(&mm, &article).await.unwrap();
//        //}
//        //
//        //directus::tasks::handle_images(&mm, &article).await.unwrap();
//        //directus::tasks::handle_videos(&mm, &article).await.unwrap();
//    });
//}

async fn export_draft(
    State(mm): State<ModelManager>,
    Json(trigger): Json<crate::directus::trigger::Request>,
) -> Result<String> {
    let trigger = trigger.body;

    println!("Exporting article to substack");

    let Some(article_id) = trigger.keys.first() else {
        return Err(Error::NoKeyInTrigger(trigger.clone()));
    };

    tasks::substack::drafts::create_iterative()
        .model_manager(&mm)
        .root_article_id(*article_id)
        .should_publish(true)
        .call()
        .await?;

    Ok("OK".to_owned())
}

async fn delete_draft(
    State(mm): State<ModelManager>,
    Json(trigger): Json<crate::directus::trigger::Request>,
) -> Result<String> {
    let trigger = trigger.body;

    println!("Exporting article to substack");

    let Some(article_id) = trigger.keys.first() else {
        return Err(Error::NoKeyInTrigger(trigger.clone()));
    };

    tasks::substack::drafts::delete(&mm, *article_id).await?;

    Ok("OK".to_owned())
}
