import {
	CloudflareIcon,
	AwsIcon,
	HetznerIcon,
	DigitalOceanIcon,
	GoDaddyIcon,
	PorkbunIcon,
	InfomaniakIcon,
	OvhIcon,
} from '#/components/icons/provider-icons';
import type {ProviderOption} from '#/components/shared/provider-type-select';

export interface DnsProviderItem {
	id: number;
	name: string;
	provider_type: string;
	credentials_json: string;
	organization_id: number;
	created_at: number;
	updated_at: number;
}

export type DnsProviderType =
	| 'CLOUDFLARE'
	| 'ROUTE53'
	| 'HETZNER'
	| 'DIGITALOCEAN'
	| 'GODADDY'
	| 'PORKBUN'
	| 'INFOMANIAK'
	| 'OVH';

const smallIcon = 'size-4 shrink-0';

export const dnsProviderOptions: ProviderOption[] = [
	{
		value: 'CLOUDFLARE',
		label: 'Cloudflare DNS',
		icon: <CloudflareIcon className={`${smallIcon} text-amber-500`} />,
	},
	{
		value: 'ROUTE53',
		label: 'Amazon Route53',
		icon: <AwsIcon className={smallIcon} />,
	},
	{
		value: 'HETZNER',
		label: 'Hetzner DNS',
		icon: <HetznerIcon className={smallIcon} />,
	},
	{
		value: 'DIGITALOCEAN',
		label: 'DigitalOcean DNS',
		icon: <DigitalOceanIcon className={smallIcon} />,
	},
	{
		value: 'GODADDY',
		label: 'GoDaddy DNS',
		icon: <GoDaddyIcon className={smallIcon} />,
	},
	{
		value: 'PORKBUN',
		label: 'Porkbun DNS',
		icon: <PorkbunIcon className={smallIcon} />,
	},
	{
		value: 'INFOMANIAK',
		label: 'Infomaniak DNS',
		icon: <InfomaniakIcon className={`${smallIcon} text-red-500`} />,
	},
	{
		value: 'OVH',
		label: 'OVHcloud DNS',
		icon: <OvhIcon className={smallIcon} />,
	},
];

export function DnsProviderIcon({
	type,
	className = 'size-7 shrink-0',
}: {
	type: string;
	className?: string;
}) {
	switch (type.toUpperCase()) {
		case 'CLOUDFLARE':
			return <CloudflareIcon className={className} />;
		case 'ROUTE53':
			return <AwsIcon className={className} />;
		case 'HETZNER':
			return <HetznerIcon className={className} />;
		case 'DIGITALOCEAN':
			return <DigitalOceanIcon className={className} />;
		case 'GODADDY':
			return <GoDaddyIcon className={className} />;
		case 'PORKBUN':
			return <PorkbunIcon className={className} />;
		case 'INFOMANIAK':
			return <InfomaniakIcon className={className} />;
		case 'OVH':
			return <OvhIcon className={className} />;
		default:
			return <CloudflareIcon className={className} />;
	}
}

export function getDnsProviderLabel(type: string) {
	return (
		dnsProviderOptions.find(o => o.value === type.toUpperCase())?.label ??
		type
	);
}

export const ovhEndpointOptions = [
	{value: 'ovh-eu', label: 'OVHcloud Europe'},
	{value: 'ovh-ca', label: 'OVHcloud Canada'},
	{value: 'ovh-us', label: 'OVHcloud US'},
	{value: 'kimsufi-eu', label: 'Kimsufi Europe'},
	{value: 'kimsufi-ca', label: 'Kimsufi Canada'},
	{value: 'soyoustart-eu', label: 'So you Start Europe'},
	{value: 'soyoustart-ca', label: 'So you Start Canada'},
];

export interface DnsProviderField {
	key: string;
	label: string;
	secret?: boolean;
	hint?: string;
	select?: {value: string; label: string}[];
	defaultValue?: string;
}

/** Credential field layout per provider; mirrors what the backend validates. */
export function dnsProviderFields(type: string): DnsProviderField[] {
	switch (type.toUpperCase()) {
		case 'ROUTE53':
			return [
				{key: 'accessKeyId', label: 'Access Key ID'},
				{
					key: 'secretAccessKey',
					label: 'Secret Access Key',
					secret: true,
					hint: 'Use an IAM user/role scoped to route53:ListHostedZones, route53:ListResourceRecordSets and route53:ChangeResourceRecordSets — avoid root account credentials.',
				},
			];
		case 'GODADDY':
			return [
				{key: 'apiKey', label: 'API Key'},
				{
					key: 'apiSecret',
					label: 'API Secret',
					secret: true,
					hint: 'Generate a production key/secret pair at developer.godaddy.com — test keys do not work against the public API.',
				},
			];
		case 'PORKBUN':
			return [
				{key: 'apiKey', label: 'API Key'},
				{
					key: 'secretApiKey',
					label: 'Secret API Key',
					secret: true,
					hint: 'Create API keys at porkbun.com/account/api and make sure API access is enabled for the domains you want managed.',
				},
			];
		case 'INFOMANIAK':
			return [
				{
					key: 'apiToken',
					label: 'API Token',
					secret: true,
					hint: 'Create a token at manager.infomaniak.com with the domain:read, dns:read and dns:write scopes.',
				},
			];
		case 'OVH':
			return [
				{
					key: 'endpoint',
					label: 'API Endpoint',
					select: ovhEndpointOptions,
					defaultValue: 'ovh-eu',
				},
				{key: 'applicationKey', label: 'Application Key'},
				{
					key: 'applicationSecret',
					label: 'Application Secret',
					secret: true,
				},
				{
					key: 'consumerKey',
					label: 'Consumer Key',
					secret: true,
					hint: 'Create the three keys at once on api.ovh.com/createToken, with exactly these five rights: GET /domain/zone, GET /domain/zone/*, POST /domain/zone/*, PUT /domain/zone/*, DELETE /domain/zone/*. The first one lists your zones and has to be granted on its own: OVH matches rights per exact path, so /domain/zone/* does not cover it.',
				},
			];
		case 'HETZNER':
			return [
				{
					key: 'apiToken',
					label: 'API Token',
					secret: true,
					hint: 'Create a token in the Hetzner Cloud console under DNS → API tokens.',
				},
			];
		case 'DIGITALOCEAN':
			return [
				{
					key: 'apiToken',
					label: 'API Token',
					secret: true,
					hint: 'Create a personal access token with Write permission under DigitalOcean → API.',
				},
			];
		default:
			return [
				{
					key: 'apiToken',
					label: 'API Token',
					secret: true,
					hint: 'Create a token scoped to Zone → DNS → Edit for the zones you want managed. Avoid the Global API Key.',
				},
			];
	}
}

export function buildDnsCredentials(
	type: string,
	values: Record<string, string>,
): string {
	const config: Record<string, string> = {};
	for (const field of dnsProviderFields(type)) {
		config[field.key] = (values[field.key] ?? '').trim();
	}
	return JSON.stringify(config);
}
