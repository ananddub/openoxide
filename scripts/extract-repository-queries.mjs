import { mkdir, readFile, readdir, writeFile } from "node:fs/promises";
import path from "node:path";

const sourceDir = "src/db/repository";
const outputDir = "db/queries";
const outputPrefix = "";
const maxLines = 180;

const pascal = (value) =>
  value
    .split(/[^a-zA-Z0-9]+/)
    .filter(Boolean)
    .map((part) => part[0].toUpperCase() + part.slice(1))
    .join("");

function readString(source, start) {
  if (source[start] === "r") {
    const match = source.slice(start).match(/^r(#{0,16})"/);
    if (match) {
      const marker = `"${match[1]}`;
      const contentStart = start + match[0].length;
      const end = source.indexOf(marker, contentStart);
      if (end >= 0) return { value: source.slice(contentStart, end), end: end + marker.length };
    }
  }
  if (source[start] !== '"') return null;
  let index = start + 1;
  while (index < source.length) {
    if (source[index] === "\\") index += 2;
    else if (source[index++] === '"') break;
  }
  const literal = source.slice(start, index).replace(/\\\r?\n\s*/g, "");
  try {
    return { value: JSON.parse(literal), end: index };
  } catch {
    return { value: literal.slice(1, -1), end: index };
  }
}

function matchingBrace(source, opening) {
  let depth = 1;
  for (let index = opening + 1; index < source.length; index += 1) {
    const string = readString(source, index);
    if (string) {
      index = string.end - 1;
      continue;
    }
    if (source.startsWith("//", index)) {
      index = source.indexOf("\n", index);
      if (index < 0) return source.length;
    } else if (source.startsWith("/*", index)) {
      index = source.indexOf("*/", index + 2) + 1;
    } else if (source[index] === "{") depth += 1;
    else if (source[index] === "}" && --depth === 0) return index;
  }
  return source.length;
}

function methods(source) {
  const found = [];
  const pattern = /(?:pub(?:\([^)]*\))?\s+)?async\s+fn\s+([a-zA-Z0-9_]+)/g;
  for (const match of source.matchAll(pattern)) {
    const opening = source.indexOf("{", match.index + match[0].length);
    const end = matchingBrace(source, opening);
    found.push({ name: match[1], body: source.slice(opening + 1, end) });
    pattern.lastIndex = end + 1;
  }
  return found;
}

function sqlStrings(body) {
  const values = [];
  for (let index = 0; index < body.length; index += 1) {
    const string = readString(body, index);
    if (!string) continue;
    index = string.end - 1;
    const sql = string.value.trim();
    if (/^(SELECT|WITH|INSERT|UPDATE|DELETE|REPLACE|PRAGMA)\b/i.test(sql)) values.push(sql);
  }
  return [...new Set(values)];
}

function cleanSql(sql) {
  return sql
    .replace(/\bAS\s+"([a-zA-Z_][a-zA-Z0-9_]*)(?:[!?])?(?::\s*[^"]+)?"/gi, "AS $1")
    .replace(/\s+$/gm, "")
    .trim()
    .replace(/;?$/, ";");
}

function command(sql, body) {
  if (/\bRETURNING\b/i.test(sql)) return ":one";
  if (/^(INSERT|UPDATE|DELETE|REPLACE)\b/i.test(sql)) return ":exec";
  return /\.fetch_all\s*\(/.test(body) ? ":many" : ":one";
}

function chunks(entries) {
  const result = [];
  let current = [];
  let lines = 2;
  for (const entry of entries) {
    const size = entry.sql.split("\n").length + 3;
    if (current.length && lines + size > maxLines) {
      result.push(current);
      current = [];
      lines = 2;
    }
    current.push(entry);
    lines += size;
  }
  if (current.length) result.push(current);
  return result;
}

await mkdir(outputDir, { recursive: true });
// Repository extraction now targets the default db/queries directory directly.
const dynamic = new Set();
let queryCount = 0;

for (const file of (await readdir(sourceDir)).filter((name) => name.endsWith(".rs")).sort()) {
  const source = await readFile(path.join(sourceDir, file), "utf8");
  const repository = file.replace(/\.rs$/, "");
  const entries = [];
  for (const method of methods(source)) {
    const statements = sqlStrings(method.body);
    statements.forEach((statement, index) => {
      if (/\{[a-zA-Z_][a-zA-Z0-9_]*\}/.test(statement)) {
        dynamic.add(`${repository}::${method.name} (formatted SQL)`);
        return;
      }
      const suffix = statements.length > 1 ? `${index + 1}` : "";
      entries.push({
        name: `${pascal(repository)}${pascal(method.name)}${suffix}`,
        command: command(statement, method.body),
        sql: cleanSql(statement),
      });
    });
    if (/QueryBuilder|AssertSqlSafe/.test(method.body)) {
      dynamic.add(`${repository}::${method.name} (runtime-built SQL)`);
    }
  }
  const parts = chunks(entries);
  for (const [index, part] of parts.entries()) {
    const suffix = parts.length > 1 ? `_${index + 1}` : "";
    const header = `-- Extracted from src/db/repository/${file}. Do not edit repository SQL directly.\n\n`;
    const body = part.map((entry) => `-- name: ${entry.name} ${entry.command}\n${entry.sql}\n`).join("\n");
    await writeFile(path.join(outputDir, `${outputPrefix}${repository}${suffix}.sql`), header + body);
    queryCount += part.length;
  }
}

await writeFile(
  path.join(outputDir, `${outputPrefix}DYNAMIC_QUERIES.md`),
  `# Dynamic repository queries\n\nThese queries require typed builders or explicit variants.\n\n${[...dynamic]
    .sort()
    .map((item) => `- \`${item}\``)
    .join("\n")}\n`,
);
console.log(`Extracted ${queryCount} static queries; ${dynamic.size} dynamic queries require manual variants.`);
