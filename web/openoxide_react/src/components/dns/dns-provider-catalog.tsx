import {
	AwsIcon,
	CloudflareIcon,
	InfomaniakIcon,
	OvhIcon,
	PorkbunIcon,
} from '#/components/icons/provider-icons';
import {DnsProviderMark} from '#/components/icons/dns-brand-marks';
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

/**
 * The supported set is `dns::DnsProviderKind` on the backend and the
 * `DnsProviderType` proto enum — a value missing here has no client either.
 */
export type DnsProviderType =
	| 'CLOUDFLARE'
	| 'ROUTE53'
	| 'HOSTINGER'
	| 'PORKBUN'
	| 'INFOMANIAK'
	| 'OVH'
	| 'SPACESHIP';

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
		value: 'HOSTINGER',
		label: 'Hostinger DNS',
		icon: <DnsProviderMark value="HOSTINGER" className={smallIcon} />,
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
	{
		value: 'SPACESHIP',
		label: 'Spaceship DNS',
		icon: <DnsProviderMark value="SPACESHIP" className={smallIcon} />,
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
		case 'PORKBUN':
			return <PorkbunIcon className={className} />;
		case 'INFOMANIAK':
			return <InfomaniakIcon className={`${className} text-red-500`} />;
		case 'OVH':
			return <OvhIcon className={className} />;
		default:
			return <DnsProviderMark value={type} className={className} />;
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

/**
 * Credential fields per provider. The keys and `secret` flags mirror
 * `DnsProviderKind::credential_fields`, which is what the server validates and masks, so a
 * field can never be collected here and ignored there.
 */
const dnsProviderFieldsByType: Record<
	DnsProviderType,
	DnsProviderField[]
> = {
	CLOUDFLARE: [
		{
			key: 'apiToken',
			label: 'API Token',
			secret: true,
			hint: 'Create a token scoped to Zone → DNS → Edit for the zones you want managed. Avoid the Global API Key.',
		},
	],
	ROUTE53: [
		{key: 'accessKeyId', label: 'Access Key ID'},
		{
			key: 'secretAccessKey',
			label: 'Secret Access Key',
			secret: true,
			hint: 'Use an IAM user/role scoped to route53:ListHostedZones, route53:ListResourceRecordSets and route53:ChangeResourceRecordSets — avoid root account credentials.',
		},
	],
	HOSTINGER: [
		{
			key: 'apiToken',
			label: 'API Token',
			secret: true,
			hint: 'Generate a token in Hostinger hPanel under Advanced → API tokens.',
		},
	],
	PORKBUN: [
		{key: 'apiKey', label: 'API Key'},
		{
			key: 'secretApiKey',
			label: 'Secret API Key',
			secret: true,
			hint: 'Create API keys at porkbun.com/account/api and make sure API access is enabled for the domains you want managed.',
		},
	],
	INFOMANIAK: [
		{
			key: 'apiToken',
			label: 'API Token',
			secret: true,
			hint: 'Create a token at manager.infomaniak.com with the domain:read, dns:read and dns:write scopes.',
		},
	],
	OVH: [
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
	],
	SPACESHIP: [
		{key: 'apiKey', label: 'API Key'},
		{
			key: 'apiSecret',
			label: 'API Secret',
			secret: true,
			hint: 'Generate API credentials at spaceship.dev; they differ from your panel login.',
		},
	],
};

export function dnsProviderFields(type: string): DnsProviderField[] {
	return (
		dnsProviderFieldsByType[type.toUpperCase() as DnsProviderType] ?? []
	);
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
