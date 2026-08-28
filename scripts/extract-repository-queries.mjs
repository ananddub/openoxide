import { mkdir, readFile, readdir, rm, writeFile } from "node:fs/promises";
import path from "node:path";

const sourceDir = "src/db/repository";
const outputDir = "db/queries";
const outputPrefix = "";
const generatedMarker = "-- BEGIN GENERATED REPOSITORY QUERIES";
const excludedRepositories = new Set(["mod", "atlas_schema_revisions"]);

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

await mkdir(outputDir, { recursive: true });
// Repository extraction now targets the default db/queries directory directly.
const dynamic = new Set();
let queryCount = 0;

for (const file of (await readdir(sourceDir)).filter((name) => name.endsWith(".rs")).sort()) {
  const source = await readFile(path.join(sourceDir, file), "utf8");
  const repository = file.replace(/\.rs$/, "");
  if (excludedRepositories.has(repository)) continue;
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

  const stalePattern = new RegExp(`^${repository}_[0-9]+\\.sql$`);
  for (const stale of (await readdir(outputDir)).filter((name) => stalePattern.test(name))) {
    await rm(path.join(outputDir, stale));
  }
  // Once a repository has migrated to generated Flash queries it contains no
  // SQL literals. Its canonical query file is then the source of truth and
  // must not be replaced with an empty generated section.
  if (entries.length === 0) continue;

  const outputFile = path.join(outputDir, `${outputPrefix}${repository}.sql`);
  let canonical = "";
  try {
    canonical = await readFile(outputFile, "utf8");
  } catch {
    // The repository does not have a hand-maintained query section yet.
  }
  if (canonical.startsWith("-- Extracted from src/db/repository/")) canonical = "";
  const markerIndex = canonical.indexOf(generatedMarker);
  if (markerIndex >= 0) canonical = canonical.slice(0, markerIndex);
  canonical = canonical.trimEnd();

  const canonicalNames = new Set(
    [...canonical.matchAll(/^-- name:\s+([^\s]+)/gm)].map((match) => match[1]),
  );
  const generatedEntries = entries.filter((entry) => !canonicalNames.has(entry.name));

  const header = canonical
    ? `${generatedMarker}\n-- Extracted from src/db/repository/${file}. Do not edit this generated section directly.\n\n`
    : `-- Extracted from src/db/repository/${file}. Do not edit repository SQL directly.\n\n`;
  const body = generatedEntries.map((entry) => `-- name: ${entry.name} ${entry.command}\n${entry.sql}\n`).join("\n");
  const separator = canonical ? "\n\n" : "";
  await writeFile(outputFile, `${canonical}${separator}${header}${body}`);

  queryCount += generatedEntries.length;
}

await writeFile(
  path.join(outputDir, `${outputPrefix}DYNAMIC_QUERIES.md`),
  `# Dynamic repository queries\n\nThese queries require typed builders or explicit variants.\n\n${[...dynamic]
    .sort()
    .map((item) => `- \`${item}\``)
    .join("\n")}\n`,
);
console.log(`Extracted ${queryCount} static queries; ${dynamic.size} dynamic queries require manual variants.`);
