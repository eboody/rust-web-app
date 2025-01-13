use crate::prelude::*;

use std::str::FromStr;

use model::directus::{Articles, Files, Issues};
use time::Month;
use uuid::Uuid;

pub async fn handle_audio(mm: &ModelManager, article: &Articles) -> Result<()> {
  let (volume, number) =
    calculate_volume_and_number(article.date_published.unwrap());
  let audio_folder_id = Uuid::from_str("dfae46d8-b725-4a90-a3f6-78799210f61a")?;
  let title = article
    .title
    .as_ref()
    .ok_or(Error::NoTitleInArticle(article.id))?;

  // Return if audio file already exists
  if Files::select()
    .where_("id = ?")
    .bind(article.audio.as_ref())
    .fetch_one(mm.orm())
    .await
    .is_ok()
  {
    return Ok(());
  }

  // Try finding audio file with different patterns
  let search_patterns = [
    title.to_string(),
    if let Some(issue) = article.issue.as_ref() {
      let issue = Issues::select()
        .where_("id = ?")
        .bind(issue)
        .fetch_one(mm.orm())
        .await?;
      format!("v{}n{} {}", issue.volume, issue.number, title)
    } else {
      format!("v{}n{} {}", volume, number, title)
    },
    format!("v{}n{} {}", volume, number + 1, title),
  ];

  for pattern in search_patterns {
    if let Ok(audio_file) = Files::query(&format!(
            "SELECT * FROM directus_files WHERE similarity(title, '{}') > 0.9 AND folder = '{}'",
            pattern, audio_folder_id
        ))
        .fetch_one(mm.orm())
        .await
        {
            if let Some(audio_title) = audio_file.title {
              tracing::info!("Found audio file for\narticle: {}\naudio:  {}\n", title, audio_title );
            };

            article.update_partial().audio(Some(audio_file.id)).update(mm.orm()).await?;
            return Ok(());
        }
  }

  Ok(())
}

fn calculate_volume_and_number(date: Date) -> (i32, i32) {
  (date.year() - 2005, match date.month() {
    Month::March | Month::April | Month::May => 1,
    Month::June | Month::July | Month::August => 2,
    Month::September | Month::October | Month::November => 3,
    Month::December | Month::January | Month::February => 4,
  })
}
