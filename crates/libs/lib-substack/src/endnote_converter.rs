use crate::{Attributes, Content, MarkType, NodeType};

pub fn transform_to_substack_format(node: &mut Content) {
	if let NodeType::Text = node.type_ {
		if let Some(marks) = &node.marks {
			if let Some(link_mark) =
				marks.iter().find(|mark| mark.mark_type == MarkType::Link)
			{
				if let Some(attrs) = &link_mark.attrs {
					if let Some(href) = &attrs.href {
						let re =
							regex::Regex::new(r"#(end|foot)note-(\d+)").unwrap();
						if let Some(captures) = re.captures(href) {
							if let Some(number) = captures.get(2) {
								//replace node with a footnote anchor node
								*node = Content {
									type_: NodeType::FootnoteAnchor,
									attrs: Some(Attributes {
										number: Some(
											number.as_str().parse::<u32>().unwrap(),
										),
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
			let footnotes = transform_to_substack_footnotes(node);
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

fn transform_to_substack_footnotes(input: &Content) -> Vec<Content> {
	let mut new_content: Vec<Content> = Vec::new();
	let mut footnote_number = 1;

	let mut iter = input.content.as_ref().unwrap().iter().peekable();

	for list_item in input.content.as_ref().unwrap() {
		for paragraph in list_item.content.as_ref().unwrap() {
			//check if the crrent paragraph is followed by a "↑" paragraph
			if is_citation_paragraph(paragraph) {
				let mut citation_content = paragraph.clone().content.unwrap();
				citation_content.pop();
				// create a footnote
				let footnote = Content {
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

fn is_citation_paragraph(paragraph: &Content) -> bool {
	if paragraph.content.is_some() {
		if let Some(paragraph_content) = &paragraph.content {
			if let Some(last_node) = paragraph_content.last() {
				if last_node.type_ == NodeType::Text {
					if let Some(text) = &last_node.text {
						return text == "↑";
					}
				}
			}
		}
	}
	false
}
