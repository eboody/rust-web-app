use crate::prelude::*;
use axum::{Json, extract::State};
use lib_core::model::{ModelManager, UploadFilePayload};
use serde::Serialize;
use uuid::Uuid;

use super::ebook::on_ebook_upload;

#[derive(Debug, Serialize, PartialEq)]
pub enum MimeType {
    #[serde(rename = "application/vnd.openxmlformats-officedocument.wordprocessingml.document")]
    Docx,

    #[serde(rename = "application/pdf")]
    Pdf,
}

pub enum ContentType {
    Article,
    Ebook,
    #[allow(unused)]
    Image(Box<ContentType>),
}

impl ContentType {
    pub fn folder(&self) -> Uuid {
        match self {
            ContentType::Article => {
                uuid::uuid!("4f4950c9-f16f-4ddc-af8a-9f4329a5fdf5")
            }
            ContentType::Ebook => {
                uuid::uuid!("94849b26-f3f4-446a-9a80-ab1aa535f400")
            }
            ContentType::Image(content_type) => match **content_type {
                ContentType::Article => {
                    uuid::uuid!("64690736-ce67-4ad9-85cf-aab23184edad")
                }
                ContentType::Ebook => {
                    uuid::uuid!("9fd1e097-3794-4b53-995a-27222c74edad")
                }
                ContentType::Image(_) => {
                    unreachable!()
                }
            },
        }
    }
}

pub trait Content {
    fn mime_type(&self) -> BoxResult<MimeType>;
    fn content_type(&self) -> BoxResult<ContentType>;
}

impl Content for UploadFilePayload {
    fn mime_type(&self) -> BoxResult<MimeType> {
        let mime_type = if self.type_
            == "application/vnd.openxmlformats-officedocument.wordprocessingml.document"
        {
            MimeType::Docx
        } else if self.type_ == "application/pdf" {
            MimeType::Pdf
        } else {
            let err = Error::UnknownMimeType(self.clone());

            let boxed_err: Box<Error> = Box::new(err);

            return Err(boxed_err);
        };

        Ok(mime_type)
    }

    fn content_type(&self) -> BoxResult<ContentType> {
        match self.mime_type()? {
            MimeType::Docx => {
                if self.folder == ContentType::Article.folder() {
                    Ok(ContentType::Article)
                } else {
                    Err(Box::new(Error::UnknownContentType(self.clone())))
                }
            }
            MimeType::Pdf => {
                if self.folder == ContentType::Ebook.folder() {
                    Ok(ContentType::Ebook)
                } else {
                    Err(Box::new(Error::UnknownContentType(self.clone())))
                }
            }
        }
    }
}

pub async fn on_file_upload(
    State(mm): State<ModelManager>,
    Json(payload): Json<UploadFilePayload>,
) -> Result<()> {
    dbg!("payload: {}", &payload);

    match payload.content_type()? {
        //ContentType::Article => on_article_upload(&mm, &payload).await?,
        ContentType::Ebook => on_ebook_upload(&mm, &payload).await?,
        _ => {
            unreachable!()
        }
    }

    Ok(())
}
