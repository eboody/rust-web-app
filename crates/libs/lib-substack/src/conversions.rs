#![allow(unused)]
use super::{
  error::{Error, Result},
  prose_mirror::*,
};

use encoding_rs::UTF_8;
use std::{
  fs::{self, File},
  io::Write,
  ops::Deref,
  path::Path,
  process::Command,
};

pub fn convert_docx_to_md(input_path: &Path, output_path: &Path) -> Result<()> {
  let html_output = Command::new("./scripts/mammoth.js/bin/mammoth")
    .arg("--style-map=scripts/mammoth.js/style-map.mammoth")
    .arg(input_path)
    .output()?;

  if !html_output.status.success() {
    return Err(Error::MammothFailed(format!(
      "Failed to run mammoth on {:?}: {}",
      input_path,
      String::from_utf8_lossy(&html_output.stderr)
    )));
  }

  let (html_string, _, had_errors) = UTF_8.decode(&html_output.stdout);
  if had_errors {
    eprintln!("There were encoding errors in the HTML output");
  }

  let markdown =
    htmd::convert(&html_string).expect("Failed to convert HTML to Markdown");
  let markdown = markdown
    .replace(
      r#"

[↑]"#,
      " [↑]",
    )
    .replace(".  .", ". ");
  let markdown_bytes = markdown.as_bytes();

  let mut file = File::create(output_path)?;
  file.write_all(markdown_bytes)?;

  Ok(())
}

pub fn get_content_from_file<T: AsRef<Path>>(
  file_path: T,
) -> std::result::Result<Node, Box<dyn std::error::Error>> {
  let content = fs::read_to_string(file_path)?;
  let document: Node = json::from_str(&content)?;
  Ok(document)
}

pub fn get_content_from_string(
  content: &str,
) -> std::result::Result<Node, Box<dyn std::error::Error>> {
  let document: Node = json::from_str(content)?;
  Ok(document)
}

pub fn extract_heading(document: &mut Node) -> Option<String> {
  let heading = document.content.as_ref().and_then(|nodes| {
    nodes.iter().find_map(|node| {
      if matches!(node.type_, NodeType::Heading) {
        node.content.as_ref().map(|content| {
          content
            .iter()
            .filter_map(|node| node.text.clone())
            .collect::<String>()
        })
      } else {
        None
      }
    })
  });

  if let Some(ref mut nodes) = document.content {
    nodes.retain(|node| !matches!(node.type_, NodeType::Heading));
  }

  heading
}

pub fn md_to_prosemirror(md: &str) -> Result<Document> {
  let prosemirror_output = Command::new("./scripts/to-prosemirror/mdtp.js")
    .arg(md)
    .output()
    .expect("Failed to run mdtp.js");

  // Print debug output
  if !prosemirror_output.stderr.is_empty() {
    //eprintln!(
    //  "Debug output: {}",
    //  String::from_utf8_lossy(&prosemirror_output.stderr)
    //);
  }

  if !prosemirror_output.status.success() {
    //eprintln!(
    //  "Process failed with output: {}",
    //  String::from_utf8_lossy(&prosemirror_output.stdout)
    //);
    return Err(Error::ProseMirrorFailed);
  }

  let prosemirror_string = String::from_utf8_lossy(&prosemirror_output.stdout);
  //eprintln!("ProseMirror JSON output: {}", prosemirror_string);

  let doc: Document =
    json::from_str(&prosemirror_string).expect("Failed to parse ProseMirror JSON");
  Ok(doc)
}
const FOOTNOTE_REGEX: &str = r"#_?([a-zA-Z]{2,3})(note|ref)?-?(\d+)";

pub fn transform_to_substack_format(node: &mut Node) {
  if let NodeType::Text = node.type_ {
    if let Some(marks) = &node.marks {
      if let Some(link_mark) =
        marks.iter().find(|mark| mark.mark_type == MarkType::Link)
      {
        if let Some(attrs) = &link_mark.attrs {
          if let Some(href) = &attrs.href {
            let re = regex::Regex::new(FOOTNOTE_REGEX).unwrap();
            if let Some(captures) = re.captures(href) {
              if let Some(number) = captures.get(3) {
                //replace node with a footnote anchor node
                *node = Node {
                  type_: NodeType::FootnoteAnchor,
                  attrs: Some(Attributes {
                    number: Some(number.as_str().parse::<u32>().unwrap()),
                    ..Default::default()
                  }),
                  marks: None,
                  text: None,
                  content: None,
                };
              }
            }
          }
        }
      }
    }
  } else if node.type_ == NodeType::OrderedList {
    //were in the endnotes section
    if node.content.is_some() {
      let footnotes = transform_to_substack_footnotes_ol(node);
      node.content = Some(footnotes);
    }
  }

  // recursively hanbdle child nodes
  if let Some(children) = &mut node.content {
    for (index, child) in children.iter_mut().enumerate() {
      transform_to_substack_format(child);
      if child.type_ == NodeType::OrderedList {
        let mut footnotes = child.content.as_ref().unwrap().clone();
        children.remove(index);
        children.append(&mut footnotes);
        return;
      }
    }
  }
}

pub fn transform_to_substack_footnotes_ol(input: &Node) -> Vec<Node> {
  let mut new_content: Vec<Node> = Vec::new();
  let mut footnote_number = 1;

  let mut iter = input.content.as_ref().unwrap().iter().peekable();

  for list_item in input.content.as_ref().unwrap() {
    for paragraph in list_item.content.as_ref().unwrap() {
      //check if the crrent paragraph is followed by a "↑" paragraph
      if is_citation_paragraph(paragraph) {
        let mut citation_content = paragraph.clone().content.unwrap();
        citation_content.pop();
        // create a footnote
        let footnote = Node {
          type_: NodeType::Footnote,
          attrs: Some(Attributes {
            number: Some(footnote_number),
            ..Default::default()
          }),
          content: Some(citation_content),
          ..Default::default()
        };

        new_content.push(footnote);
        footnote_number += 1;

        // skip citation paragraph
        iter.next();
      } else {
        new_content.push(paragraph.clone());
      }
    }
  }

  new_content
}

pub fn transform_endnotes_for_substack(input: &Node) -> Vec<Node> {
  let mut new_content: Vec<Node> = Vec::new();
  let mut footnote_number = 1;

  let mut iter = input.content.as_ref().unwrap().iter().peekable();

  for node in input.content.as_ref().unwrap() {
    //check if the crrent paragraph is supposed to be a node
    if is_citation_paragraph(node) {
      let mut citation_content = node.clone().content.unwrap();
      citation_content.remove(0);
      if let Some(text_node) = citation_content.clone().first_mut() {
        text_node.text = text_node.clone().text.map(|t| t.trim().to_string());
      }

      // create a footnote
      let footnote = Node {
        type_: NodeType::Footnote,
        attrs: Some(Attributes {
          number: Some(footnote_number),
          ..Default::default()
        }),
        content: Some(citation_content),
        ..Default::default()
      };

      new_content.push(footnote);
      footnote_number += 1;
    } else {
      new_content.push(node.clone());
    }
  }

  new_content
}

fn is_citation_paragraph(node: &Node) -> bool {
  let is_paragraph_with_content =
    node.type_ == NodeType::Paragraph && node.content.is_some();

  if !is_paragraph_with_content {
    return false;
  }

  if let Some(paragraph_content) = &node.content
    && let Some(first_node) = &paragraph_content.first()
    && let Some(marks) = first_node.marks.as_ref()
    && let Some(first_mark) = marks.first()
    && let Some(attrs) = &first_mark.attrs
    && let Some(href) = &attrs.href
    && let Some(captures) = regex::Regex::new(FOOTNOTE_REGEX).unwrap().captures(href)
  {
    true
  } else {
    false
  }
}
