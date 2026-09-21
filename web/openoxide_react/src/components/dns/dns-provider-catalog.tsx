import {Globe} from 'lucide-react';
import {CloudflareIcon, AwsIcon} from '#/components/icons/provider-icons';
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
	| 'GODADDY';

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
		icon: <Globe className={`${smallIcon} text-orange-500`} />,
	},
	{
		value: 'DIGITALOCEAN',
		label: 'DigitalOcean DNS',
		icon: <Globe className={`${smallIcon} text-sky-500`} />,
	},
	{
		value: 'GODADDY',
		label: 'GoDaddy DNS',
		icon: <Globe className={`${smallIcon} text-green-600`} />,
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
		default:
			return <Globe className={`${className} text-primary`} />;
	}
}

export function getDnsProviderLabel(type: string) {
	return (
		dnsProviderOptions.find(o => o.value === type.toUpperCase())?.label ??
		type
	);
}

// Credential field layout per provider: single token, AWS pair, or
// key/secret pair. `token` is always the secret half.
export function credentialKind(type: string): 'token' | 'aws' | 'keypair' {
	return type === 'ROUTE53'
		? 'aws'
		: type === 'GODADDY'
			? 'keypair'
			: 'token';
}

export function buildDnsCredentials(
	type: string,
	accessKey: string,
	token: string,
): string {
	switch (credentialKind(type)) {
		case 'aws':
			return JSON.stringify({
				accessKeyId: accessKey.trim(),
				secretAccessKey: token.trim(),
			});
		case 'keypair':
			return JSON.stringify({
				apiKey: accessKey.trim(),
				apiSecret: token.trim(),
			});
		default:
			return JSON.stringify({apiToken: token.trim()});
	}
}
