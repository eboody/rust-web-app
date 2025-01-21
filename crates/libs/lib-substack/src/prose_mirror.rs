#![allow(unused)]
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Document {
    #[serde(rename = "type")]
    pub doc_type: NodeType,
    pub content: Vec<Node>,
}

impl From<Document> for Node {
    fn from(doc: Document) -> Self {
        Node {
            type_: doc.doc_type,
            content: Some(doc.content),
            ..Default::default()
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize, Default)]
pub struct Node {
    #[serde(rename = "type")]
    pub type_: NodeType,
    pub attrs: Option<Attributes>,
    pub content: Option<Vec<Node>>,
    pub marks: Option<Vec<Mark>>,
    pub text: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Serialize, Default)]
pub struct Attributes {
    pub level: Option<u8>,
    pub href: Option<String>,
    pub src: Option<String>,
    pub title: Option<String>,
    pub number: Option<u32>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Mark {
    #[serde(rename = "type")]
    pub mark_type: MarkType,
    pub attrs: Option<Attributes>,
}

#[derive(Debug, Deserialize, PartialEq, Clone, Serialize, Display, Default)]
#[serde(rename_all = "snake_case")]
pub enum NodeType {
    Doc,
    Heading,
    #[default]
    Paragraph,
    Text,
    Blockquote,
    Link,
    #[serde(rename = "footnoteAnchor")]
    FootnoteAnchor,
    Footnote,
    HardBreak,
    OrderedList,
    ListItem,
    BulletList,
    Image,
    HorizontalRule,
    Other(String),
}

#[derive(Debug, Deserialize, Clone, Serialize, Display, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MarkType {
    Em,
    Strong,
    Link,
    Other(String),
}
