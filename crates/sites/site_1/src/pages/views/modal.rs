use crate::prelude::*;

#[allow(dead_code)]
pub enum Modal<'a> {
  Info { title: &'a str, body: &'a str },
  Warning { title: &'a str, body: &'a str },
  Error { title: &'a str, body: &'a str },
}
impl Render for Modal<'_> {
  fn render(&self) -> maud::Markup {
    let uuid = uuid::Uuid::new_v4();
    html! {
      .modal id={"modal-" (uuid) } {
        .modal-header {
          h2 {
           (match self {
              Modal::Info { title, .. } => title,
              Modal::Warning { title, .. } => title,
              Modal::Error { title, .. } => title,
            })
          }
          .close-modal.icon toast-id=(uuid) {
           (icon::Close)
          }
        }
        .modal-body {
          p {
           (match self {
              Modal::Info { body, .. } => body,
              Modal::Warning { body, .. } => body,
              Modal::Error { body, .. } => body,
            })
          }
        }
        .modal-footer {
          button class="btn" {
               "Close"
            }
        }
      }
      (js())
      (css())
    }
  }
}
js! {
  me(".btn").on("click", (ev) => {
    halt(ev);
    let modalId = "#" + me(ev).closest(".modal").id;
    let el = any(modalId);
    el.fadeOut(null, 30);
  });
  me(".close-modal").on("click", (ev) => {
    halt(ev);
    let modalId = "#" + me(ev).attribute("toast-id");
    let el = any(modalId);
    el.fadeOut(null, 30);
  });
}
css! {
  .modal {
    position: fixed;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    background: var(--surface-1);
    border-radius: 0.5rem;
    box-shadow: 0 4px 30px rgba(0, 0, 0, 0.1);
    width: 400px;
    .modal-header {
      display: flex;
      justify-content: space-between;
      padding: 1rem;
      background-color: var(--primary);
      border-top-left-radius: 0.5rem;
      border-top-right-radius: 0.5rem;
      .close-modal {
        cursor: pointer;
      }
    }
    .modal-body {
      padding: 1rem;
    }
    .modal-footer {
      display: flex;
      justify-content: flex-end;
      padding: 1rem;
      .btn {
        cursor: pointer;
        background: var(--primary);
        color: white;
        border: none;
        padding: 0.5rem 1rem;
        border-radius: 0.25rem;
        transition: background 0.3s;
        &:hover {
          background: var(--primary-dark);
        }
      }
    }
  }
}
