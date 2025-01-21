#!/usr/bin/env node

const fs = require("fs");
const path = require("path");
const { Schema } = require("prosemirror-model");
const { MarkdownParser } = require("prosemirror-markdown");
const markdownIt = require("markdown-it");
const { schema: basicSchema } = require("prosemirror-schema-basic");
const { addListNodes } = require("prosemirror-schema-list");

// Extend the basic schema to include list nodes and horizontal_rule
const schema = new Schema({
  nodes: addListNodes(basicSchema.spec.nodes, "paragraph block*", "block")
    .update("image", {
      inline: true,
      group: "inline",
      draggable: true,
      attrs: {
        src: { default: null }, // Make src optional just like other attrs
        alt: { default: null },
        title: { default: null },
      },
      parseDOM: [{
        tag: "img[src]",
        getAttrs: (dom) => ({
          src: dom.getAttribute("src"),
          alt: dom.getAttribute("alt"),
          title: dom.getAttribute("title"),
        }),
      }],
      toDOM: (node) => ["img", node.attrs],
    })
    .update("horizontal_rule", {
      group: "block",
      parseDOM: [{ tag: "hr" }],
      toDOM: () => ["hr"],
    }),
  marks: basicSchema.spec.marks,
});

// Markdown parser using ProseMirror schema with lists, images, and horizontal_rule
const mdParser = new MarkdownParser(schema, markdownIt(), {
  paragraph: { block: "paragraph" },
  heading: { block: "heading", getAttrs: (tok) => ({ level: +tok.tag.slice(1) }) },
  list_item: { block: "list_item" },
  bullet_list: { block: "bullet_list" },
  ordered_list: { block: "ordered_list", getAttrs: (tok) => ({ order: +tok.attrGet("start") || 1 }) },
  code_block: { block: "code_block" },
  blockquote: { block: "blockquote" },
  hr: { node: "horizontal_rule" },  // Changed from horizontal_rule to hr
  text: { node: "text" },
  hardbreak: { node: "hard_break" },
  em: { mark: "em" },
  strong: { mark: "strong" },
  link: { mark: "link", getAttrs: (tok) => ({ href: tok.attrGet("href"), title: tok.attrGet("title") }) },
  code_inline: { mark: "code" },
  image: {
    node: "image",
    getAttrs: (tok) => ({
      src: tok.attrGet("src"),
      alt: tok.attrGet("alt"),
      title: tok.attrGet("title"),
    }),
  },
});

function markdownToProseMirror(markdown) {
  const doc = mdParser.parse(markdown);
  return doc.toJSON();
}

function convertMarkdownFileToProseMirror(inputFile, outputDir) {
  fs.readFile(inputFile, "utf8", (err, markdown) => {
    if (err) {
      console.error(`Error reading file ${inputFile}:`, err);
      process.exit(1);
    }

    const inputFileName = path.basename(inputFile, path.extname(inputFile));
    const outputFile = path.join(outputDir, `${inputFileName}.json`);
    const prosemirrorJson = markdownToProseMirror(markdown);

    fs.writeFile(outputFile, JSON.stringify(prosemirrorJson, null, 2), (err) => {
      if (err) {
        console.error(`Error writing file ${outputFile}:`, err);
        process.exit(1);
      }
      console.log(`Converted ${inputFile} to ${outputFile}`);
    });
  });
}

if (require.main === module) {
  const args = process.argv.slice(2);

  if (args.length !== 1) {
    console.error("Usage: mtp.js <markdown_string>");
    process.exit(1);
  }

  const [markdown] = args;

  try {
    const prosemirrorJson = markdownToProseMirror(markdown);
    console.log(JSON.stringify(prosemirrorJson, null, 2));
  } catch (err) {
    console.error("Error processing Markdown:", err);
    process.exit(1);
  }
}
