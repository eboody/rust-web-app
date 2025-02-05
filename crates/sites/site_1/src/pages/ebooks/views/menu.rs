use crate::{pages::ebooks, prelude::*};
use lib_core::model::EbooksTranslations;

pub struct Menu {
    pub ebooks: Vec<EbooksTranslations>,
}

impl Render for Menu {
    fn render(&self) -> Markup {
        html! {
          .ebooks-container {
            section {
              .grid-auto-fit {
                @for ebook in &self.ebooks {
                  (ebooks::Card { ebook })
                }
              }
            }
          }
          (css())
        }
    }
}

pub async fn get_menu() -> Result<Markup> {
    //let ebooks = EbooksTranslationsService::find().await;
    //
    //let ebooks = ebooks?;

    //Ok(Menu { ebooks }.render())
    Ok(html! {})
}

css! {
  .grid-auto-fit {
    display: grid;
    gap: var(--size-fluid-3);
    grid-template-columns: repeat(auto-fit, minmax(min(35ch, 100%), 1fr));

    container: grid-auto-fit / inline-size;
  }
}
