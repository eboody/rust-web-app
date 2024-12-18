use crate::prelude::*;
use futures::stream::{self, StreamExt};
use model::directus;
use tracing::{debug, info, warn};

#[derive(Debug)]
struct SectionDiff {
  to_create: Vec<directus::Sections>,
  to_update: Vec<(directus::Sections, lib_substack::Section)>,
  to_delete: Vec<lib_substack::Section>,
}

pub async fn sync_sections(mm: &ModelManager) -> Result<()> {
  // Fetch sections from both systems
  let directus_sections = directus::Sections::select().fetch_all(mm.orm()).await?;
  let substack_sections = lib_substack::Section::list(mm.reqwest()).await?;

  info!(
    "Syncing sections - Directus: {}, Substack: {}",
    directus_sections.len(),
    substack_sections.len()
  );

  // Analyze what needs to be created, updated, or deleted
  let diff = analyze_sections_diff(&directus_sections, &substack_sections);

  debug!(
    "Section diff - To create: {}, To update: {}, To delete: {}",
    diff.to_create.len(),
    diff.to_update.len(),
    diff.to_delete.len()
  );

  // Create new sections
  let create_results = stream::iter(diff.to_create)
    .map(|section| async move {
      let new_section = lib_substack::Section::new(
        section.display_string.clone(),
        section.description.clone(),
      )
      .set_default_on(false);

      match new_section.create(mm.reqwest()).await {
        Ok(created) => {
          info!("Created section: {}", section.display_string);
          section
            .update_partial()
            .substack_id(created.id)
            .update(mm.orm())
            .await?;
          Ok((section, created))
        }
        Err(e) => {
          warn!("Failed to create section {}: {}", section.display_string, e);
          Err(e)
        }
      }
    })
    .buffer_unordered(5)
    .collect::<Vec<_>>()
    .await;

  // Update existing sections
  let update_results = stream::iter(diff.to_update)
    .map(|(directus_section, substack_section)| async move {
      let updated_section = lib_substack::Section::new(
        directus_section.display_string.clone(),
        directus_section.description.clone(),
      )
      .set_default_on(false);

      match updated_section
        .update(mm.reqwest(), substack_section.id.unwrap_or_default())
        .await
      {
        Ok(updated) => {
          info!("Updated section: {}", directus_section.display_string);
          Ok((directus_section, updated))
        }
        Err(e) => {
          warn!(
            "Failed to update section {}: {}",
            directus_section.display_string, e
          );
          Err(e)
        }
      }
    })
    .buffer_unordered(5)
    .collect::<Vec<_>>()
    .await;

  // Delete sections that don't exist in Directus
  let delete_results = stream::iter(diff.to_delete)
    .map(|section| async move {
      if let Some(id) = section.id {
        match section.delete(mm.reqwest(), id).await {
          Ok(_) => {
            info!("Deleted section: {}", section.name);
            Ok(section)
          }
          Err(e) => {
            warn!("Failed to delete section {}: {}", section.name, e);
            Err(e)
          }
        }
      } else {
        warn!("Cannot delete section {} - no ID", section.name);
        Err(lib_substack::Error::MissingField("section.id".into()))
      }
    })
    .buffer_unordered(5)
    .collect::<Vec<_>>()
    .await;

  info!(
    "Sync completed - Created: {}, Updated: {}, Deleted: {}",
    create_results.len(),
    update_results.len(),
    delete_results.len()
  );

  Ok(())
}

fn analyze_sections_diff(
  directus_sections: &[directus::Sections],
  substack_sections: &[lib_substack::Section],
) -> SectionDiff {
  let mut to_create = Vec::new();
  let mut to_update = Vec::new();
  let mut to_delete = Vec::new();

  // Find sections to create or update
  for directus_section in directus_sections {
    let matching_substack = substack_sections.iter().find(|s| {
      s.name == directus_section.display_string
        || s.slug
          == Some(
            directus_section
              .display_string
              .to_lowercase()
              .replace(' ', "-"),
          )
    });

    match matching_substack {
      Some(substack_section) => {
        if needs_update(directus_section, substack_section) {
          to_update.push((directus_section.clone(), substack_section.clone()));
        }
      }
      None => {
        to_create.push(directus_section.clone());
      }
    }
  }

  // Find sections to delete (exist in Substack but not in Directus)
  for substack_section in substack_sections {
    let exists_in_directus = directus_sections.iter().any(|d| {
      d.display_string == substack_section.name
        || d.display_string.to_lowercase().replace(' ', "-")
          == substack_section.slug.as_deref().unwrap_or_default()
    });

    if !exists_in_directus {
      to_delete.push(substack_section.clone());
    }
  }

  SectionDiff {
    to_create,
    to_update,
    to_delete,
  }
}

fn needs_update(
  directus: &directus::Sections,
  substack: &lib_substack::Section,
) -> bool {
  // Compare description strings directly
  Some(&directus.description) != Some(&substack.description)
}
