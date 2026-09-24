/**
 * Seeds a template's `[variables]` table (template.toml) into the .env-style
 * KEY=VALUE lines the compose engine interpolates into `${VAR}` references in
 * docker-compose.yml. Supported value syntax:
 * - `"${password:N}"` → random N-char secret
 * - `"${domain}"` / unknown refs → empty string (filled later on the compose domains/env tab)
 * - `"${OTHER_KEY}"` (bare or embedded) → the resolved value of that variable
 */
const VARIABLE_LINE = /^\s*([A-Za-z_][A-Za-z0-9_]*)\s*=\s*"([^"]*)"\s*$/;
const REFERENCE = /\$\{([A-Za-z_][A-Za-z0-9_:.]*)\}/g;

function randomSecret(length: number) {
	const bytes = new Uint8Array(length);
	crypto.getRandomValues(bytes);
	const alphabet =
		'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
	return Array.from(bytes, b => alphabet[b % alphabet.length]).join('');
}

/** Extracts the `KEY = "value"` pairs of the `[variables]` TOML table (until the next table header). */
function readVariablesSection(toml: string) {
	const entries: Array<[string, string]> = [];
	let inVariables = false;
	for (const line of toml.split('\n')) {
		const header = /^\[([^\]]+)\]/.exec(line);
		if (header) {
			inVariables = header[1] === 'variables';
			continue;
		}
		if (!inVariables) continue;
		const match = VARIABLE_LINE.exec(line);
		if (match) entries.push([match[1], match[2]]);
	}
	return entries;
}

export function parseTemplateEnv(
	toml: string | undefined,
): string | undefined {
	if (!toml) return undefined;
	const entries = readVariablesSection(toml);
	if (entries.length === 0) return undefined;

	const resolved = new Map<string, string>();
	for (const [key, raw] of entries) {
		const password = /^\$\{password:(\d+)\}$/.exec(raw);
		resolved.set(key, password ? randomSecret(Number(password[1])) : raw);
	}
	// Second pass: expand ${REF}s against the pass-1 values (passwords stay generated); unresolvable placeholders (${domain}) become empty.
	for (const [key] of entries) {
		const current = resolved.get(key) ?? '';
		const expanded = current.replace(REFERENCE, (whole, name: string) => {
			if (name.includes(':')) return whole;
			return resolved.get(name) ?? '';
		});
		resolved.set(key, expanded);
	}
	return entries
		.map(([key]) => `${key}=${resolved.get(key) ?? ''}`)
		.join('\n');
}
