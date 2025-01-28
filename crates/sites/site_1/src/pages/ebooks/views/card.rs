use crate::prelude::*;
use lib_core::model::EbooksTranslations;

pub struct Card<'a> {
    pub ebook: &'a EbooksTranslations,
}

impl Render for Card<'_> {
    fn render(&self) -> Markup {
        card(self.ebook)
    }
}

fn card(ebook: &EbooksTranslations) -> Markup {
    html! {
      article.card
      id=(&ebook.id)
      hx_patch=(format!("https://tosapp.eman.network/ebooks/{}", ebook.id))
      data-umami-event="ebook-card-clicked"
      //data-umami-event-ebook=(ebook.id)
      //data-umami-event-name=(ebook.name)
      hx_trigger="mouseover" {
        //(ebooks::Cover3D{ ebook })
        //h2 .book-name { (ebook.name) }
        //p.subtext { (ebook.sub_text.clone().unwrap_or("".to_owned())) }
        //(Button::Primary { href: ebook.get_file_download(), text: "Download".to_owned() })
        //(js())
      }
      (css())
    }
}

js! {
  me().on("click", (ev) => {
    me(ev).send("ebook-card-clicked", { ebook_id: me(ev).id });
  });

  me().on("ebook-card-clicked", (ev) => console.log(ev));
}

css! {
  me {
    article.card {
      box-shadow:
        rgba(17, 12, 46, 0.03) 0px 48px 100px 0px,
        rgba(17, 17, 26, 0.05) 0px 1px 0px;

      padding-bottom: var(--size-7);
      padding-top: var(--size-5);
      @media (max-width: 30rem) {
        padding: var(--size-2);
        padding-bottom: var(--size-3);
      }
      padding-inline: var(--size-7);
      transition:
        transform 0.3s,
        box-shadow 0.5s;
      text-align: center;
      display: flex;
      flex-direction: column;
      align-items: center;
      justify-content: space-between;
      gap: var(--size-3);
      overlflow: hidden;
    }

    article.card:hover {
      box-shadow:
        rgba(17, 12, 46, 0.2) 0px 48px 100px 0px,
        rgba(17, 17, 26, 0.2) 0px 1px 0px;
    }
    .book-name {
      font-size: var(--font-size-4);
      font-weight: 700;
      font-family: "Capitolina";
    }
    .subtext {
      font-size: var(--font-size-1);
      font-weight: 400;
    }
    article.card:has(> img) {
      border: 2px solid var(--clr-primary-300);
      box-shadow:
        rgba(0, 0, 0, 0.12) 0px 1px 3px,
        rgba(0, 0, 0, 0.24) 0px 1px 2px;

      @container grid-auto-fit (inline-size > calc(30ch * 2 + 1rem)) {
        grid-column: span 2;

        display: grid;
        grid-template-columns: subgrid;
        gap: 0;

        > img {
          grid-column: 2;
          grid-row: 1 / 4;
        }
      }

      @container grid-auto-fit (inline-size > calc(30ch * 4 + 3rem)) {
        grid-column: span 2;
        grid-row: span 2;

        > :not(img) {
          grid-column: 1 / -1;
        }

        > img {
          grid-column: 1 / -1;
          grid-row: 1;
        }
      }
    }
  }
}
