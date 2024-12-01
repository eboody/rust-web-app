#!/usr/bin/env node

const { DOMParser } = require("prosemirror-model");
const { schema } = require("prosemirror-schema-basic");
const fs = require("fs");
const { JSDOM } = require("jsdom");
const path = require("path");

function htmlToProseMirror(html) {
  const dom = new JSDOM(html);
  const document = dom.window.document;
  const parser = DOMParser.fromSchema(schema);
  const doc = parser.parse(document.body);
  return doc.toJSON();
}

function convertHtmlFileToProseMirror(inputFile, outputDir) {
  fs.readFile(inputFile, "utf8", (err, html) => {
    if (err) {
      console.error(`Error reading file ${inputFile}:`, err);
      process.exit(1);
    }

    const inputFileName = path.basename(inputFile, path.extname(inputFile));
    const outputFile = path.join(outputDir, `${inputFileName}.json`);
    const footnotesFile = path.join(outputDir, `footnotes_${inputFileName}.json`);

    let olSplit = html.split("<ol>");
    let pre = olSplit[0];
    let post = olSplit[1];

    const prosemirrorJson = htmlToProseMirror(pre);
    const footnotes = htmlToProseMirror(post);

    fs.writeFile(outputFile, JSON.stringify(prosemirrorJson, null, 2), (err) => {
      if (err) {
        console.error(`Error writing file ${outputFile}:`, err);
        process.exit(1);
      }
      console.log(`Converted ${inputFile} to ${outputFile}`);
    });

    fs.writeFile(footnotesFile, JSON.stringify(footnotes, null, 2), (err) => {
      if (err) {
        console.error(`Error writing footnotes file:`, err);
        process.exit(1);
      }
      console.log(`Saved footnotes to ${footnotesFile}`);
    });
  });
}

if (require.main === module) {
  const args = process.argv.slice(2);

  if (args.length !== 2) {
    console.error("Usage: my-js-program <input-file.html> <output-dir>");
    process.exit(1);
  }

  const [inputFile, outputDir] = args;

  if (!fs.existsSync(outputDir)) {
    console.error(`Error: Output directory ${outputDir} does not exist.`);
    process.exit(1);
  }

  convertHtmlFileToProseMirror(inputFile, outputDir);
}
