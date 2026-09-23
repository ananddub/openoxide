import {KeyRound} from 'lucide-react';
import {
	AwsIcon,
	AzureIcon,
	DopplerIcon,
	HashicorpVaultIcon,
	InfisicalIcon,
	ScalewayIcon,
} from '#/components/icons/provider-icons';
import type {ProviderOption} from '#/components/shared/provider-type-select';

// Mirrors `VaultProviderPayload` (backend VaultProviderDto wire shape).
export interface VaultProviderItem {
	id: number;
	name: string;
	provider_type: string;
	api_url: string;
	auth_token: string;
	namespace?: string;
	config_json?: string;
	organization_id: number;
	created_at: number;
	updated_at: number;
	assignments?: Array<{project_id: number; environment_ids: number[]}>;
}

export type VaultProviderType =
	| 'HASHICORP'
	| 'INFISICAL'
	| 'DOPPLER'
	| 'AWS'
	| 'AWS_PARAMETER_STORE'
	| 'SCALEWAY'
	| 'AZURE';

export function VaultProviderIcon({
	type,
	className = 'size-7 shrink-0',
}: {
	type: string;
	className?: string;
}) {
	switch (type.toUpperCase()) {
		case 'HASHICORP':
			return <HashicorpVaultIcon className={className} />;
		case 'DOPPLER':
			return <DopplerIcon className={className} />;
		case 'INFISICAL':
			return <InfisicalIcon className={className} />;
		case 'AWS':
		case 'AWS_PARAMETER_STORE':
			return <AwsIcon className={className} />;
		case 'AZURE':
			return <AzureIcon className={className} />;
		case 'SCALEWAY':
			return <ScalewayIcon className={className} />;
		default:
			return <KeyRound className={className} />;
	}
}

const VAULT_PROVIDER_LABELS: Record<string, string> = {
	HASHICORP: 'HashiCorp Vault',
	DOPPLER: 'Doppler',
	INFISICAL: 'Infisical',
	AWS: 'AWS Secrets Manager',
	AWS_PARAMETER_STORE: 'AWS Parameter Store',
	AZURE: 'Azure Key Vault',
	SCALEWAY: 'Scaleway Secret Manager',
};

export function getVaultProviderLabel(type: string) {
	return VAULT_PROVIDER_LABELS[type.toUpperCase()] ?? type;
}

/** Env-var interpolation snippet a deployment uses to read a secret. */
export const vaultReferenceSyntax = (name: string, secret = '<secret>') =>
	`\${{vault.${name || '<name>'}.${secret}}}`;

/** "3 projects" / "All projects" — how wide a provider is visible. */
export function vaultScopeLabel(provider: VaultProviderItem) {
	const count = provider.assignments?.length ?? 0;
	if (!count) return 'All projects';
	return `${count} project${count > 1 ? 's' : ''}`;
}

export const vaultProviderOptions: ProviderOption[] = [
	'HASHICORP',
	'DOPPLER',
	'INFISICAL',
	'AWS',
	'AWS_PARAMETER_STORE',
	'AZURE',
	'SCALEWAY',
].map(value => ({
	value,
	label:
		value === 'HASHICORP'
			? 'HashiCorp Vault / OpenBao'
			: getVaultProviderLabel(value),
	icon: <VaultProviderIcon type={value} className="size-4 shrink-0" />,
}));

// [field name, label, placeholder] triples per provider type live in
// vault-provider-fields.tsx (VAULT_FIELD_DEFINITIONS).

export function buildVaultConfig(
	formType: VaultProviderType,
	mount: string,
	field: (name: string) => string,
): Record<string, string | undefined> {
	if (formType === 'HASHICORP') return {mount: mount.trim() || 'secret'};
	if (formType === 'DOPPLER')
		return {
			project: field('project') || undefined,
			config: field('config') || undefined,
		};
	if (formType === 'INFISICAL')
		return {
			siteUrl: field('siteUrl') || 'https://app.infisical.com',
			clientId: field('clientId'),
			clientSecret: field('clientSecret'),
			projectId: field('projectId'),
			environmentSlug: field('environmentSlug'),
			secretPath: field('secretPath') || '/',
		};
	if (formType === 'AWS' || formType === 'AWS_PARAMETER_STORE')
		return {
			region: field('region'),
			accessKeyId: field('accessKeyId'),
			secretAccessKey: field('secretAccessKey'),
			endpoint: field('endpoint') || undefined,
			...(formType === 'AWS_PARAMETER_STORE' && field('parameterPath')
				? {parameterPath: field('parameterPath')}
				: {}),
		};
	if (formType === 'AZURE')
		return {
			vaultUri: field('vaultUri'),
			tenantId: field('tenantId'),
			clientId: field('clientId'),
			clientSecret: field('clientSecret'),
		};
	return {
		region: field('region') || 'fr-par',
		projectId: field('projectId'),
		secretKey: field('secretKey'),
		apiUrl: field('apiUrl') || 'https://api.scaleway.com',
	};
}

export function vaultDefaultApiUrl(
	formType: VaultProviderType,
	url: string,
) {
	return (
		url.trim() ||
		(formType === 'DOPPLER'
			? 'https://api.doppler.com'
			: 'http://localhost:8200')
	);
}

export function parseVaultAssignments(json: string) {
	try {
		const value = JSON.parse(json);
		return Array.isArray(value) ? value : [];
	} catch {
		return [];
	}
}

export function requireVaultAssignments(json: string) {
	try {
		const value = JSON.parse(json);
		if (!Array.isArray(value)) throw new Error();
		return value;
	} catch {
		throw new Error(
			'Assignments must be a JSON array, for example [{"project_id":1,"environment_ids":[2]}]',
		);
	}
}
