use crate::prelude::*;

#[derive(Debug)]
pub struct Card<'a> {
  title: &'a str,
  content: &'a str,
}

impl Render for Card<'_> {
  fn render(&self) -> maud::Markup {
    html! {
      .card {
        .card-header {
          h3 {
            (self.title)
          }
        }
        .card-body {
          p {
            (self.content)
          }
        }
      }
      (css())
    }
  }
}

css! {
  .card {
    background: var(--surface-1);
    border-radius: 0.5rem;
    box-shadow: rgba(0, 0, 0, 0.1) 0px 2px 8px;
    overflow: hidden;

    .card-header {
      background: var(--primary);
      color: white;
      padding: 1rem;
    }

    .card-body {
      padding: 1rem;
    }
  }
}
