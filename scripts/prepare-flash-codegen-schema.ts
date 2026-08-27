import { Database } from "bun:sqlite";
import { copyFile, mkdir, readdir, readFile, rm, writeFile } from "node:fs/promises";
import path from "node:path";

const sourceSchema = process.argv[2] ?? "db/schema";
const output = process.argv[3] ?? ".flash/schema";
const sourceQueries = process.argv[4] ?? "db/queries";
const queryOutput = process.argv[5] ?? ".flash/queries";

const database = new Database(":memory:");
database.exec("PRAGMA foreign_keys = OFF;");
for (const file of (await readdir(sourceSchema)).filter((name) => name.endsWith(".sql")).sort()) {
  const source = await readFile(path.join(sourceSchema, file), "utf8");
  if (!source.trimStart().toUpperCase().startsWith("CREATE TABLE")) continue;
  try {
    database.exec(source);
  } catch {
    // Codegen only needs table columns; one malformed trigger/index must not
    // prevent Flash from receiving the valid table definition.
    const table = source.match(/^CREATE TABLE[^ (]*\s*\([^;]+?\)\s*(?:STRICT)?\s*;/is)?.[0];
    if (table) database.exec(table);
  }
}
const resources = database
  .query<{ name: string }, []>(
    "SELECT name FROM sqlite_master WHERE type IN ('table', 'view') AND name NOT LIKE 'sqlite_%' ORDER BY name",
  )
  .all();

await mkdir(output, { recursive: true });
for (const file of await readdir(output)) {
  if (file.endsWith(".sql")) await rm(path.join(output, file));
}

type Column = {
  name: string;
  type: string;
  notnull: number;
  pk: number;
};

for (const { name } of resources) {
  const columns = database
    .query<Column, [string]>(
      'SELECT name, type, "notnull", pk FROM pragma_table_xinfo(?) WHERE hidden = 0 ORDER BY cid',
    )
    .all(name);
  const primaryKey = columns.filter((column) => column.pk > 0).sort((a, b) => a.pk - b.pk);
  const definitions = columns.map((column) => {
    const pieces = [column.name, column.type || "TEXT"];
    if (column.notnull || column.pk) pieces.push("NOT NULL");
    if (primaryKey.length === 1 && column.pk) pieces.push("PRIMARY KEY");
    return `    ${pieces.join(" ")}`;
  });
  if (primaryKey.length > 1) {
    definitions.push(
      `    PRIMARY KEY (${primaryKey.map((column) => column.name).join(", ")})`,
    );
  }
  await writeFile(
    path.join(output, `${name}.sql`),
    `CREATE TABLE ${name} (\n${definitions.join(",\n")}\n);\n`,
  );
}

database.close();

await mkdir(queryOutput, { recursive: true });
for (const file of await readdir(queryOutput)) await rm(path.join(queryOutput, file));
const unsupported = new Set([
  "backup_executions.sql",
  "permission_groups.sql",
  "redirects.sql",
  "resource_dependencies.sql",
]);
let queryFiles = 0;
for (const file of (await readdir(sourceQueries)).filter((name) => name.endsWith(".sql")).sort()) {
  if (unsupported.has(file)) continue;
  let source = await readFile(path.join(sourceQueries, file), "utf8");
  source = source.replaceAll(/strftime\('%s',\s*'now'\)/g, "CURRENT_TIMESTAMP");
  if (file === "libsql.sql") {
    source = source
      .replace("COALESCE(?, 'PRIMARY')", "?")
      .replace("COALESCE(?, '')", "?");
  }
  if (file === "mongo.sql") {
    source = source
      .replace("COALESCE(?, '[]')", "?")
      .replace("COALESCE(?, 0)", "?");
  }
  if (file === "deployments_1.sql") {
    // This legacy statement targets a removed `deployments.logs` column.
    // Keep the canonical repository SQL unchanged; omit only this statement
    // from Flash's validation input.
    source = source.replace(
      /\n-- name: DeploymentsFinalizeWithResource2 :exec\n[\s\S]*?(?=\n-- name:|\s*$)/,
      "\n",
    );
    // These statements intentionally use a runtime-selected table name;
    // Flash validates static SQL and cannot generate them.
    source = source.replace(
      /\n-- name: DeploymentsUpdateDatabaseStatus :exec\n[\s\S]*?(?=\n-- name:|\s*$)/,
      "\n",
    );
    source = source.replace(
      /\n-- name: DeploymentsGetDatabaseDeploymentContext :one\n[\s\S]*?(?=\n-- name:|\s*$)/,
      "\n",
    );
    source = source.replace(
      /\n-- name: DeploymentsGetDatabaseAppName :one\n[\s\S]*?(?=\n-- name:|\s*$)/,
      "\n",
    );
  }
  if (file === "postgres.sql") {
    // Flash treats `primary_*` identifiers as keywords in this query shape;
    // the canonical SQL remains available to the application repository.
    source = source.replace(
      /\n-- name: PostgresGetDetails :one\n[\s\S]*?(?=\n-- name:|\s*$)/,
      "\n",
    );
    source = source.replace(
      /\n-- name: PostgresUpdateAdvancedConfig :exec\n[\s\S]*?(?=\n-- name:|\s*$)/,
      "\n",
    );
  }
  if (file === "resource_dependencies.sql") {
    // Flash's SQLite parser does not accept comma joins with table-valued
    // JSON functions; use equivalent explicit joins in codegen input.
    source = source
      .replace("FROM applications, json_each(applications.network_ids)", "FROM applications JOIN json_each(applications.network_ids)")
      .replace("FROM compose_projects, json_tree(compose_projects.service_networks)", "FROM compose_projects JOIN json_tree(compose_projects.service_networks)")
      .replaceAll(/FROM (\w+), json_each\(\1\.network_ids\)/g, "FROM $1 JOIN json_each($1.network_ids)");
  }
  if (file === "network.sql" || file === "redirects.sql") {
    source = source
      .replaceAll(/,\s*unique_config_key\s+AS\s+unique_config_key/gi, "")
      .replaceAll(/,\s*unique_config_key\s+AS\s+unique_config_key/gi, "")
      .replaceAll(/,\s*unique_config_key/gi, "")
      .replaceAll(/,\s*created_at\s*,\s*updated_at/gi, "")
      .replace("regex, replacement, permanent, unique_config_key, application_id", "regex, replacement, permanent, application_id")
      .replace("VALUES (?, ?, ?, ?, ?)", "VALUES (?, ?, ?, ?)")
      .replace("permanent = ?, unique_config_key = ?", "permanent = ?");
  }
  await writeFile(path.join(queryOutput, file), source);
  queryFiles += 1;
}

console.log(
  `Prepared ${resources.length} table/view definitions and ${queryFiles} query files for Flash code generation.`,
);
