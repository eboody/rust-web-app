#!/usr/bin/env node

const fs = require("fs");
const path = require("path");
const { Schema } = require("prosemirror-model");
const { MarkdownParser } = require("prosemirror-markdown");
const markdownIt = require("markdown-it");
const { schema: basicSchema } = require("prosemirror-schema-basic");
const { addListNodes } = require("prosemirror-schema-list");

// Extend the basic schema to include list nodes
const schema = new Schema({
  nodes: addListNodes(basicSchema.spec.nodes, "paragraph block*", "block"),
  marks: basicSchema.spec.marks,
});

// Markdown parser using ProseMirror schema with lists
const mdParser = new MarkdownParser(schema, markdownIt(), {
  paragraph: { block: "paragraph" },
  heading: { block: "heading", getAttrs: (tok) => ({ level: +tok.tag.slice(1) }) },
  list_item: { block: "list_item" },
  bullet_list: { block: "bullet_list" },
  ordered_list: { block: "ordered_list", getAttrs: (tok) => ({ order: +tok.attrGet("start") || 1 }) },
  code_block: { block: "code_block" },
  blockquote: { block: "blockquote" },
  horizontal_rule: { node: "horizontal_rule" },
  text: { node: "text" },
  hardbreak: { node: "hard_break" },
  em: { mark: "em" },
  strong: { mark: "strong" },
  link: { mark: "link", getAttrs: (tok) => ({ href: tok.attrGet("href"), title: tok.attrGet("title") }) },
  code_inline: { mark: "code" },
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

// Main CLI logic
if (require.main === module) {
  const args = process.argv.slice(2);

  if (args.length !== 2) {
    console.error("Usage: mtp.js <input-file.md> <output-dir>");
    process.exit(1);
  }

  const [inputFile, outputDir] = args;

  // Ensure output directory exists
  if (!fs.existsSync(outputDir)) {
    console.error(`Error: Output directory ${outputDir} does not exist.`);
    process.exit(1);
  }

  convertMarkdownFileToProseMirror(inputFile, outputDir);
}
