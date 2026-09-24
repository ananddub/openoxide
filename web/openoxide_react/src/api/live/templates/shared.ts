import {useConnectQuery} from '../query';

/** Template catalog registry (Cloudflare Worker + D1). Build-time override: `VITE_TEMPLATE_REGISTRY_URL`. */
export const TEMPLATE_REGISTRY_BASE =
	(import.meta.env.VITE_TEMPLATE_REGISTRY_URL as string | undefined) ??
	'https://openoxide-templates-worker.duanand6.workers.dev';

/** One row of the registry's `GET /meta.json` catalog list. */
export type TemplateMeta = {
	id: string;
	name: string;
	version: string;
	description: string;
	logo: string;
	links: Record<string, string>;
	tags: string[];
};

/** `GET /blueprints/:id/:filename` URL for a template file (docker-compose.yml, template.toml, logo images). */
export function templateFileUrl(templateId: string, filename: string) {
	return `${TEMPLATE_REGISTRY_BASE}/blueprints/${encodeURIComponent(templateId)}/${encodeURIComponent(filename)}`;
}

/** Full catalog list; immutable-ish (worker caches 1 h), so the default 60 s staleTime is already generous. */
export function useTemplateList() {
	return useConnectQuery<TemplateMeta[]>(
		['templates', 'catalog'],
		async () => {
			const response = await fetch(`${TEMPLATE_REGISTRY_BASE}/meta.json`);
			if (!response.ok)
				throw new Error(`Template registry failed: ${response.status}`);
			return (await response.json()) as TemplateMeta[];
		},
	);
}

/** Raw text of one template file (docker-compose.yml / template.toml), fetched lazily per template. */
export function useTemplateFile(templateId: string, filename: string) {
	return useConnectQuery<string>(
		['templates', 'file', templateId, filename],
		async () => {
			const response = await fetch(templateFileUrl(templateId, filename));
			if (!response.ok)
				throw new Error(`Template file failed: ${response.status}`);
			return response.text();
		},
		{enabled: Boolean(templateId)},
	);
}
