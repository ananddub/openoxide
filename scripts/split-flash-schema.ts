import { Database } from "bun:sqlite";
import { mkdir, readdir, rm, writeFile } from "node:fs/promises";
import path from "node:path";

const databasePath = process.argv[2];
const output = process.argv[3] ?? "db/schema";

if (!databasePath) {
  throw new Error("usage: bun scripts/split-flash-schema.ts <sqlite-db> [output-dir]");
}

const database = new Database(databasePath, { readonly: true });
const tables = database
  .query<{ name: string }, []>(
    "SELECT name FROM sqlite_master WHERE type = 'table' AND name NOT LIKE 'sqlite_%' ORDER BY name",
  )
  .all()
  .map(({ name }) => name);

const dependencies = new Map(
  tables.map((table) => [
    table,
    new Set(
      database
        .query<{ table: string }, [string]>(
          'SELECT DISTINCT "table" FROM pragma_foreign_key_list(?)',
        )
        .all(table)
        .map((row) => row.table)
        .filter((dependency) => dependency !== table && tables.includes(dependency)),
    ),
  ]),
);

const ordered: string[] = [];
const pending = new Set(tables);
while (pending.size) {
  const ready = [...pending]
    .filter((table) => [...dependencies.get(table)!].every((dependency) => !pending.has(dependency)))
    .sort();
  const selected = ready.length ? ready : [[...pending].sort()[0]];
  for (const table of selected) {
    pending.delete(table);
    ordered.push(table);
  }
}

await mkdir(output, { recursive: true });
for (const file of await readdir(output)) {
  if (file.endsWith(".sql")) await rm(path.join(output, file));
}

const objects = database.query<{ sql: string }, [string]>(`
  SELECT sql FROM sqlite_master
  WHERE tbl_name = ? AND sql IS NOT NULL AND type IN ('table', 'index', 'trigger')
  ORDER BY CASE type WHEN 'table' THEN 0 WHEN 'index' THEN 1 ELSE 2 END, name
`);

for (const [index, table] of ordered.entries()) {
  const body = objects
    .all(table)
    .map(({ sql }) => `${sql.trim().replace(/;?$/, ";")}\n`)
    .join("\n");
  const number = String(index + 1).padStart(3, "0");
  await writeFile(path.join(output, `${number}_${table}.sql`), body);
}

const views = database
  .query<{ name: string; sql: string }, []>(
    "SELECT name, sql FROM sqlite_master WHERE type = 'view' ORDER BY name",
  )
  .all();
for (const [index, view] of views.entries()) {
  const number = String(ordered.length + index + 1).padStart(3, "0");
  await writeFile(
    path.join(output, `${number}_${view.name}_view.sql`),
    `${view.sql.trim().replace(/;?$/, ";")}\n`,
  );
}

database.close();
console.log(`Split schema into ${ordered.length} table files and ${views.length} view files.`);
