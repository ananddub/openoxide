export const PROVIDER_LABELS: Record<string, string> = {
	aws: 'AWS S3',
	s3: 'AWS S3',
	r2: 'Cloudflare R2',
	cloudflare: 'Cloudflare R2',
	minio: 'MinIO',
	backblaze: 'Backblaze B2',
};

export function providerLabel(provider?: string): string {
	if (!provider) return 'S3 Compatible';
	return PROVIDER_LABELS[provider.toLowerCase()] ?? provider.toUpperCase();
}

export const PROVIDER_FILTERS = [
	'All',
	'AWS S3',
	'Cloudflare R2',
	'MinIO',
	'S3 Compatible',
] as const;
export type ProviderFilter = (typeof PROVIDER_FILTERS)[number];

export const PROVIDERS = [
	{id: 'aws', name: 'AWS S3'},
	{id: 'r2', name: 'Cloudflare R2'},
	{id: 'minio', name: 'MinIO'},
	{id: 'digitalocean', name: 'DigitalOcean Spaces'},
	{id: 'wasabi', name: 'Wasabi'},
	{id: 'custom', name: 'Custom S3 Compatible'},
];

/** Map any free-form provider string to a known PROVIDERS id. */
export function matchProvider(raw?: string): string {
	if (!raw) return 'aws';
	const lower = raw.trim().toLowerCase();

	const foundById = PROVIDERS.find(p => p.id.toLowerCase() === lower);
	if (foundById) return foundById.id;
	const foundByName = PROVIDERS.find(p => p.name.toLowerCase() === lower);
	if (foundByName) return foundByName.id;

	if (lower.includes('r2') || lower.includes('cloudflare')) return 'r2';
	if (lower.includes('minio')) return 'minio';
	if (
		lower.includes('digital') ||
		lower.includes('ocean') ||
		lower.includes('spaces')
	)
		return 'digitalocean';
	if (lower.includes('wasabi')) return 'wasabi';
	if (lower.includes('custom') || lower.includes('other')) return 'custom';
	if (lower.includes('aws') || lower.includes('amazon') || lower === 's3')
		return 'aws';
	return 'custom';
}
