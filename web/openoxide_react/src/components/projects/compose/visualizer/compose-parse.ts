import {load as yamlLoad} from 'js-yaml';

// ─── Types ────────────────────────────────────────────────────────────────────

export interface ComposeService extends Record<string, unknown> {
	name: string;
	image: string;
	dependsOn: string[];
	envVars: Record<string, string>;
	volumes: string[];
	ports: string[];
}

// ─── YAML parser ─────────────────────────────────────────────────────────────

export function parseComposeFile(yaml: string): ComposeService[] {
	if (!yaml || !yaml.trim()) return [];
	try {
		let cleanYaml = yaml.trim();
		if (cleanYaml.startsWith('```')) {
			cleanYaml = cleanYaml
				.replace(/^```[a-zA-Z]*\n?/, '')
				.replace(/\n?```$/, '')
				.trim();
		}
		if (cleanYaml.includes('\\n') && !cleanYaml.includes('\n')) {
			cleanYaml = cleanYaml.replace(/\\n/g, '\n');
		}

		let doc: any = null;
		try {
			doc = yamlLoad(cleanYaml);
		} catch {
			try {
				doc = JSON.parse(cleanYaml);
			} catch {}
		}

		if (!doc) return [];
		if (typeof doc === 'string') {
			try {
				doc = JSON.parse(doc);
			} catch {
				try {
					doc = yamlLoad(doc);
				} catch {}
			}
		}

		const svcs = resolveServicesSection(doc);
		if (typeof svcs !== 'object' || svcs === null || Array.isArray(svcs))
			return [];

		return Object.entries(svcs).map(([name, s]: [string, any]) =>
			mapService(name, s),
		);
	} catch (e) {
		console.warn('[ComposeVisualizer] YAML parse error:', e);
		return [];
	}
}

/** Locate the services map, inferring it when there's no explicit key. */
function resolveServicesSection(doc: any) {
	let svcs = doc?.services;
	if (
		!svcs &&
		typeof doc === 'object' &&
		doc !== null &&
		!Array.isArray(doc)
	) {
		const possible: Record<string, any> = {};
		Object.entries(doc).forEach(([k, v]: [string, any]) => {
			const reserved = [
				'version',
				'volumes',
				'networks',
				'configs',
				'secrets',
			];
			if (
				!reserved.includes(k) &&
				typeof v === 'object' &&
				v !== null &&
				!Array.isArray(v) &&
				(v.image ||
					v.build ||
					v.ports ||
					v.environment ||
					v.depends_on ||
					v.volumes)
			) {
				possible[k] = v;
			}
		});
		if (Object.keys(possible).length > 0) svcs = possible;
	}
	return svcs;
}

function mapService(name: string, s: any): ComposeService {
	let dependsOn: string[] = [];
	if (Array.isArray(s?.depends_on)) dependsOn = s.depends_on.map(String);
	else if (s?.depends_on && typeof s.depends_on === 'object')
		dependsOn = Object.keys(s.depends_on);

	const envVars = readEnvVars(s?.environment ?? s?.env ?? {});
	const volumes = (Array.isArray(s?.volumes) ? s.volumes : []).map(
		(v: any) => {
			const str = typeof v === 'string' ? v : (v?.source ?? String(v));
			const parts = str.split(':');
			return parts.length > 1 ? parts[1] : parts[0];
		},
	);
	const ports = (Array.isArray(s?.ports) ? s.ports : []).map((p: any) =>
		typeof p === 'object' ? `${p.target ?? p.published}` : String(p),
	);

	return {
		name,
		image: imageFor(s) || 'container',
		dependsOn,
		envVars,
		volumes,
		ports,
	};
}

function readEnvVars(envRaw: any): Record<string, string> {
	const envVars: Record<string, string> = {};
	if (Array.isArray(envRaw)) {
		envRaw.forEach((line: any) => {
			const str = String(line);
			const idx = str.indexOf('=');
			if (idx >= 0) envVars[str.slice(0, idx)] = str.slice(idx + 1);
			else envVars[str] = '';
		});
	} else if (envRaw && typeof envRaw === 'object') {
		Object.entries(envRaw).forEach(([k, v]) => {
			envVars[k] = v == null ? '' : String(v);
		});
	}
	return envVars;
}

function imageFor(s: any): string {
	if (s?.image) return s.image;
	if (!s?.build) return '';
	if (typeof s.build === 'string') return `build: ${s.build}`;
	if (typeof s.build === 'object' && s.build?.context)
		return `build: ${s.build.context}`;
	return 'build';
}
