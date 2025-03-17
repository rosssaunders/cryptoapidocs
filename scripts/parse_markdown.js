const fs = require('fs');
const path = require('path');
const unified = require('unified');
const parse = require('remark-parse');
const stringify = require('remark-stringify');
const rehype = require('remark-rehype');
const format = require('rehype-format');

const docsDir = path.join(__dirname, '../docs');

function readMarkdownFiles(dir) {
    return fs.readdirSync(dir).filter(file => file.endsWith('.md')).map(file => path.join(dir, file));
}

function processMarkdownFile(filePath) {
    const content = fs.readFileSync(filePath, 'utf8');
    return unified()
        .use(parse)
        .use(rehype)
        .use(format)
        .use(stringify)
        .processSync(content)
        .toString();
}

function writeMarkdownFile(filePath, content) {
    fs.writeFileSync(filePath, content, 'utf8');
}

function cleanUpMarkdownFiles() {
    const files = readMarkdownFiles(docsDir);
    files.forEach(file => {
        const cleanedContent = processMarkdownFile(file);
        writeMarkdownFile(file, cleanedContent);
    });
}

cleanUpMarkdownFiles();
