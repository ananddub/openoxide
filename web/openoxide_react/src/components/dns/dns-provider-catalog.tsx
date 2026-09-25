import {Globe} from 'lucide-react';
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
	| 'OVH'
	| 'DESEC'
	| 'BUNNY'
	| 'CONSTELLIX'
	| 'DNSMADEEASY'
	| 'EXOSCALE'
	| 'NIFCLOUD'
	| 'WEBSUPPORT'
	| 'SPACESHIP'
	| 'DNSIMPLE'
	| 'GANDIV5'
	| 'NAMEDOTCOM'
	| 'NAMESILO'
	| 'DUCKDNS'
	| 'FREEMYIP'
	| 'IPV64'
	| 'DDNSS'
	| 'DYNU'
	| 'PLESK'
	| 'CPANEL'
	| 'GOOGLECLOUDDNS'
	| 'IONOS'
	| 'HOSTINGDE'
	| 'NETCUP'
	| 'NETLIFY'
	| 'HUAWEICLOUD'
	| 'BAIDUCLOUD'
	| 'AZUREDNS'
	| 'IBMCLOUD'
	| 'HURRICANE'
	| 'HOSTINGER'
	| 'AUTODNS'
	| 'LIGHTSAIL'
	| 'EDGEDNS'
	| 'SCALEWAY'
	| 'GCORE'
	| 'VERCEL'
	| 'VULTR'
	| 'BLUECATV2'
	| 'EASYDNS'
	| 'JOKER'
	| 'MYTHICBEASTS'
	| 'NAMECHEAP'
	| 'TRANSIP'
	| 'NS1'
	| 'LUADNS'
	| 'CLOUDNS'
	| 'GLESYS'
	| 'DREAMHOST'
	| 'ALIDNS'
	| 'TENCENTCLOUD'
	| 'VOLCENGINE'
	| 'YANDEXCLOUD'
	| 'DOMENESHOP'
	| 'SAFEDNS'
	| 'SIMPLYCOM'
	| 'ARVANCLOUD'
	| 'INWX'
	| 'ULTRADNS'
	| 'INFOBLOX'
	| 'ORACLECLOUD'
	| 'RFC2136';

const smallIcon = 'size-4 shrink-0';

/** Optional fallback for providers that cannot enumerate zones themselves. */
const ZONE_FIELD: DnsProviderField = {
	key: 'zone',
	label: 'Zone (optional)',
	hint: 'Only needed if the provider cannot list your zones — set it to the domain you manage.',
};

type BridgeProvider = {
	label: string;
	fields: DnsProviderField[];
};

/** Providers routed through the dns-update bridge; keys must match the backend credential parser. */
const bridgeProviders: Record<string, BridgeProvider> = {
	DESEC: {
		label: 'deSEC DNS',
		fields: [
			{
				key: 'apiToken',
				label: 'API Token',
				secret: true,
				hint: 'Create a token at desec.io with the domains:read and records:write scopes.',
			},
		],
	},
	BUNNY: {
		label: 'Bunny DNS',
		fields: [
			{
				key: 'apiKey',
				label: 'API Key',
				secret: true,
				hint: 'Find your API key in the Bunny control panel under Account → API & JS Key.',
			},
		],
	},
	CONSTELLIX: {
		label: 'Constellix DNS',
		fields: [
			{key: 'apiKey', label: 'API Key'},
			{
				key: 'secretKey',
				label: 'Secret Key',
				secret: true,
				hint: 'Create an API key pair at constellix.com with DNS read/write access.',
			},
		],
	},
	DNSMADEEASY: {
		label: 'DNS Made Easy',
		fields: [
			{key: 'apiKey', label: 'API Key'},
			{
				key: 'apiSecret',
				label: 'API Secret',
				secret: true,
				hint: 'Use a sandbox or production key pair from dmadeeasy.com; the panel talks to the production API.',
			},
		],
	},
	EXOSCALE: {
		label: 'Exoscale DNS',
		fields: [
			{key: 'apiKey', label: 'API Key'},
			{
				key: 'apiSecret',
				label: 'API Secret',
				secret: true,
				hint: 'Create API keys in the Exoscale account settings with DNS management access.',
			},
		],
	},
	NIFCLOUD: {
		label: 'Nifcloud DNS',
		fields: [
			{key: 'accessKey', label: 'Access Key'},
			{
				key: 'secretKey',
				label: 'Secret Key',
				secret: true,
				hint: 'Generate a security key pair in the Nifcloud static DNS control panel.',
			},
		],
	},
	WEBSUPPORT: {
		label: 'WebSupport DNS',
		fields: [
			{key: 'apiKey', label: 'API Key'},
			{
				key: 'apiSecret',
				label: 'API Secret',
				secret: true,
				hint: 'Create API access in the WebSupport admin interface under DNS → API access.',
			},
		],
	},
	SPACESHIP: {
		label: 'Spaceship DNS',
		fields: [
			{key: 'apiKey', label: 'API Key'},
			{
				key: 'apiSecret',
				label: 'API Secret',
				secret: true,
				hint: 'Generate API credentials at spaceship.dev; they differ from your panel login.',
			},
		],
	},
	DNSIMPLE: {
		label: 'DNSimple',
		fields: [
			{
				key: 'apiToken',
				label: 'API Token',
				secret: true,
				hint: 'A personal or account OAuth token from dnsimple.com with DNS access.',
			},
			{
				key: 'accountId',
				label: 'Account ID',
				hint: 'The numeric account or user id shown in your DNSimple profile URL.',
			},
		],
	},
	GANDIV5: {
		label: 'Gandi.net (v5)',
		fields: [
			{
				key: 'apiToken',
				label: 'API Token',
				secret: true,
				hint: 'Create a personal access token at Gandi with the zone:read and zone:update scopes.',
			},
		],
	},
	NAMEDOTCOM: {
		label: 'Name.com DNS',
		fields: [
			{key: 'username', label: 'Username'},
			{
				key: 'apiToken',
				label: 'API Token',
				secret: true,
				hint: 'Enable API access in your Name.com account settings.',
			},
		],
	},
	NAMESILO: {
		label: 'NameSilo DNS',
		fields: [
			{
				key: 'apiKey',
				label: 'API Key',
				secret: true,
				hint: 'Enable the API and generate a key at namesilo.com account settings.',
			},
		],
	},
	DUCKDNS: {
		label: 'DuckDNS',
		fields: [
			{
				key: 'token',
				label: 'Account Token',
				secret: true,
				hint: 'The token shown on your DuckDNS control page. DuckDNS only manages <sub>.duckdns.org — set the Zone field to your full domain.',
			},
			ZONE_FIELD,
		],
	},
	FREEMYIP: {
		label: 'FreeMyIP',
		fields: [
			{
				key: 'token',
				label: 'API Token',
				secret: true,
				hint: 'Set Zone to your .freemyip.com hostname and the token from the zones page.',
			},
			ZONE_FIELD,
		],
	},
	IPV64: {
		label: 'IPv64',
		fields: [
			{
				key: 'apiKey',
				label: 'API Auth Key',
				secret: true,
				hint: 'Account owner keys can manage all subdomains; create one from the IPv64 API tab.',
			},
		],
	},
	DDNSS: {
		label: 'DDNSS',
		fields: [
			{
				key: 'key',
				label: 'Auth Key',
				secret: true,
				hint: 'Use a zone auth key for the domain you manage; set Zone accordingly.',
			},
			ZONE_FIELD,
		],
	},
	DYNU: {
		label: 'Dynu',
		fields: [
			{
				key: 'apiKey',
				label: 'API Key (API_ACCESS_TOKEN)',
				secret: true,
				hint: 'Enable API access in your Dynu control panel and use the API access token.',
			},
		],
	},
	PLESK: {
		label: 'Plesk DNS',
		fields: [
			{
				key: 'baseUrl',
				label: 'Panel URL',
				hint: 'Full https:// URL of the Plesk server, e.g. https://plesk.example.com:8443.',
			},
			{
				key: 'apiKey',
				label: 'API Token',
				secret: true,
				hint: 'Create a read/write API key in Plesk under Extensions → API Client.',
			},
		],
	},
	CPANEL: {
		label: 'cPanel DNS',
		fields: [
			{
				key: 'baseUrl',
				label: 'Panel URL',
				hint: 'https:// URL of the cPanel/WHM server, including port if custom.',
			},
			{key: 'username', label: 'cPanel Username'},
			{
				key: 'apiToken',
				label: 'API Token',
				secret: true,
				hint: 'Generate a session token in cPanel under Security → API Tokens.',
			},
		],
	},
	GOOGLECLOUDDNS: {
		label: 'Google Cloud DNS',
		fields: [
			{
				key: 'serviceAccountJson',
				label: 'Service Account JSON',
				secret: true,
				hint: 'The full JSON key of a service account with roles/dns.admin (or dns.editor).',
			},
			{
				key: 'projectId',
				label: 'Project ID',
				hint: 'The GCP project that owns the managed zones.',
			},
			{
				key: 'managedZone',
				label: 'Managed Zone (optional)',
				hint: 'Restrict to a single managed zone name; leave empty to list all zones in the project.',
			},
		],
	},
	IONOS: {
		label: 'IONOS DNS',
		fields: [
			{
				key: 'apiKey',
				label: 'API Key',
				secret: true,
				hint: 'Create a key at api.ionos.com with the DNS service activated.',
			},
		],
	},
	HOSTINGDE: {
		label: 'hosting.de DNS',
		fields: [
			{
				key: 'apiKey',
				label: 'API Token',
				secret: true,
				hint: 'Use the DNS API token from your hosting.de customer portal.',
			},
		],
	},
	NETCUP: {
		label: 'Netcup DNS',
		fields: [
			{key: 'customerNumber', label: 'Customer Number'},
			{key: 'apiKey', label: 'API Key'},
			{
				key: 'apiPassword',
				label: 'API Password',
				secret: true,
				hint: 'Enable the API in the Netcup member area and generate a key/password pair.',
			},
		],
	},
	NETLIFY: {
		label: 'Netlify DNS',
		fields: [
			{
				key: 'accessToken',
				label: 'Access Token',
				secret: true,
				hint: 'A personal access token from app.netlify.com with DNS access.',
			},
		],
	},
	HUAWEICLOUD: {
		label: 'Huawei Cloud DNS',
		fields: [
			{key: 'accessKey', label: 'Access Key ID'},
			{
				key: 'secretKey',
				label: 'Secret Access Key',
				secret: true,
				hint: 'IAM credentials with DNS Administrator permissions.',
			},
			{
				key: 'region',
				label: 'Region (optional)',
				hint: 'Defaults to cn-north-4; use your project region, e.g. eu-west-0.',
			},
		],
	},
	BAIDUCLOUD: {
		label: 'Baidu Cloud DNS',
		fields: [
			{key: 'accessKey', label: 'Access Key'},
			{
				key: 'secretKey',
				label: 'Secret Key',
				secret: true,
				hint: 'Security credentials from the Baidu Cloud console.',
			},
		],
	},
	AZUREDNS: {
		label: 'Azure DNS',
		fields: [
			{
				key: 'tenantId',
				label: 'Tenant ID',
				hint: 'The Entra ID directory that hosts the app registration.',
			},
			{
				key: 'clientId',
				label: 'Client ID',
				hint: 'App registration client (application) ID.',
			},
			{
				key: 'clientSecret',
				label: 'Client Secret',
				secret: true,
				hint: 'Give the app registration the DNS Zone Contributor role on the resource group.',
			},
			{key: 'subscriptionId', label: 'Subscription ID'},
			{
				key: 'resourceGroup',
				label: 'Resource Group',
				hint: 'The resource group containing the Azure DNS zones.',
			},
		],
	},
	IBMCLOUD: {
		label: 'IBM Cloud DNS',
		fields: [
			{key: 'username', label: 'Username'},
			{
				key: 'apiKey',
				label: 'API Key',
				secret: true,
				hint: 'An IBM Cloud or SoftLayer API key with DNS services access.',
			},
		],
	},
	HURRICANE: {
		label: 'Hurricane Electric DNS',
		fields: [
			{
				key: 'zone',
				label: 'Zone',
				hint: 'HE requires one token per domain — pair this zone with the token below.',
			},
			{
				key: 'token',
				label: 'Domain Token',
				secret: true,
				hint: 'Create a token per domain at dns.he.net (with DNS API enabled). HE does not list zones, so the Zone field is required.',
			},
		],
	},
	HOSTINGER: {
		label: 'Hostinger DNS',
		fields: [
			{
				key: 'apiToken',
				label: 'API Token',
				secret: true,
				hint: 'Generate a token in Hostinger hPanel under Advanced → API tokens.',
			},
		],
	},
	AUTODNS: {
		label: 'autoDNS',
		fields: [
			{key: 'username', label: 'Username'},
			{key: 'password', label: 'Password', secret: true},
			{
				key: 'context',
				label: 'Context ID (optional)',
				hint: 'The numeric autoDNS context that contains your zones.',
			},
		],
	},
	LIGHTSAIL: {
		label: 'Amazon Lightsail DNS',
		fields: [
			{key: 'accessKeyId', label: 'Access Key ID'},
			{
				key: 'secretAccessKey',
				label: 'Secret Access Key',
				secret: true,
				hint: 'IAM credentials scoped to lightsail:* on your DNS-only distributions.',
			},
			{
				key: 'region',
				label: 'Region (optional)',
				hint: 'e.g. us-east-1 — Lightsail is regional.',
			},
		],
	},
	EDGEDNS: {
		label: 'Akamai EdgeDNS',
		fields: [
			{
				key: 'host',
				label: 'EdgeDNS Host',
				hint: 'e.g. dns.akamaiapis.net — from your EdgeDNS configuration.',
			},
			{key: 'clientToken', label: 'Client Token'},
			{key: 'clientSecret', label: 'Client Secret', secret: true},
			{key: 'accessToken', label: 'Access Token', secret: true},
			{
				key: 'accountSwitchKey',
				label: 'Account Switch Key (optional)',
				hint: 'Use when your credentials span multiple Akamai accounts.',
			},
		],
	},
	SCALEWAY: {
		label: 'Scaleway DNS',
		fields: [
			{
				key: 'apiToken',
				label: 'API Token',
				secret: true,
				hint: 'A Scaleway IAM API key with DNS permissions.',
			},
		],
	},
	GCORE: {
		label: 'Gcore DNS',
		fields: [
			{
				key: 'apiToken',
				label: 'API Token',
				secret: true,
				hint: 'Create a token in the Gcore customer portal under API tokens.',
			},
		],
	},
	VERCEL: {
		label: 'Vercel DNS',
		fields: [
			{
				key: 'authToken',
				label: 'Auth Token',
				secret: true,
				hint: 'An account or team token from vercel.com/account/tokens.',
			},
			{
				key: 'teamId',
				label: 'Team ID (optional)',
				hint: 'Required when the domain belongs to a team rather than your personal account.',
			},
		],
	},
	VULTR: {
		label: 'Vultr DNS',
		fields: [
			{
				key: 'apiKey',
				label: 'API Key',
				secret: true,
				hint: 'Create an API key in the Vultr customer portal.',
			},
		],
	},
	BLUECATV2: {
		label: 'BlueCat IPAM',
		fields: [
			{
				key: 'serverUrl',
				label: 'Server URL',
				hint: 'Base URL of the BlueCat Configuration API, e.g. https://bluecat.example.com.',
			},
			{key: 'username', label: 'Username'},
			{key: 'password', label: 'Password', secret: true},
			{
				key: 'configName',
				label: 'Configuration Name',
				hint: 'The BlueCat configuration containing your DNS views and zones.',
			},
			{
				key: 'viewName',
				label: 'View Name (optional)',
				hint: 'Defaults to "default".',
			},
		],
	},
	EASYDNS: {
		label: 'easyDNS',
		fields: [
			{
				key: 'token',
				label: 'API Token',
				secret: true,
				hint: 'The username part of your easyDNS API token, in TOKEN:KEY form.',
			},
			{
				key: 'key',
				label: 'API Key',
				secret: true,
				hint: 'The key part of your easyDNS API credentials.',
			},
		],
	},
	JOKER: {
		label: 'Joker.com DNS',
		fields: [
			{
				key: 'apiKey',
				label: 'API Key (optional)',
				secret: true,
				hint: 'Preferred — a domain or account API key. If empty, provide username and password below.',
			},
			{key: 'username', label: 'Username (optional)'},
			{key: 'password', label: 'Password (optional)', secret: true},
		],
	},
	MYTHICBEASTS: {
		label: 'Mythic Beasts DNS',
		fields: [
			{key: 'username', label: 'Username'},
			{
				key: 'password',
				label: 'Password',
				secret: true,
				hint: 'Use an app-specific password, not your panel login.',
			},
		],
	},
	NAMECHEAP: {
		label: 'Namecheap DNS',
		fields: [
			{
				key: 'apiUser',
				label: 'API User',
				hint: 'Usually your Namecheap username.',
			},
			{
				key: 'apiKey',
				label: 'API Key',
				secret: true,
				hint: 'Enable the Namecheap API in your account; you need ≥20 domain purchases or 50 referrals for some endpoints.',
			},
			{
				key: 'clientIp',
				label: 'Client IP',
				hint: 'A public IP whitelisted in your Namecheap API settings.',
			},
			{
				key: 'username',
				label: 'DDNS User (optional)',
				hint: 'Only needed when the domain uses Dynamic DNS.',
			},
		],
	},
	TRANSIP: {
		label: 'TransIP DNS',
		fields: [
			{key: 'login', label: 'Account Login'},
			{
				key: 'privateKeyPem',
				label: 'Private Key (PEM)',
				secret: true,
				hint: 'The contents of the RSA private key generated for your TransIP API token.',
			},
			{
				key: 'globalKey',
				label: 'Global Key',
				hint: '"true" if the token was created with global access instead of a label.',
			},
		],
	},
	NS1: {
		label: 'NS1 DNS',
		fields: [
			{
				key: 'apiKey',
				label: 'API Key',
				secret: true,
				hint: 'A data-service API key from the NS1 portal with zone read/write.',
			},
		],
	},
	LUADNS: {
		label: 'LuaDNS',
		fields: [
			{key: 'apiUsername', label: 'API Username'},
			{
				key: 'apiToken',
				label: 'API Token',
				secret: true,
				hint: 'Enable API access in your LuaDNS account settings.',
			},
		],
	},
	CLOUDNS: {
		label: 'Cloud DNS (cloudns.net)',
		fields: [
			{
				key: 'authId',
				label: 'Auth ID (optional)',
				hint: 'Main auth id — or use a sub-auth id below.',
			},
			{key: 'subAuthId', label: 'Sub-Auth ID (optional)'},
			{
				key: 'authPassword',
				label: 'Auth Password',
				secret: true,
				hint: 'The auth password matching the auth or sub-auth id.',
			},
		],
	},
	GLESYS: {
		label: 'Glesys DNS',
		fields: [
			{key: 'apiUser', label: 'API User'},
			{
				key: 'apiKey',
				label: 'API Key',
				secret: true,
				hint: 'Find credentials in the Glesys customer portal under API.',
			},
		],
	},
	DREAMHOST: {
		label: 'DreamHost DNS',
		fields: [
			{
				key: 'apiKey',
				label: 'API Key',
				secret: true,
				hint: 'Generate a key with the dns-read-write permission in the DreamHost panel.',
			},
		],
	},
	ALIDNS: {
		label: 'Aliyun DNS',
		fields: [
			{key: 'accessKey', label: 'Access Key ID'},
			{
				key: 'secretKey',
				label: 'Access Key Secret',
				secret: true,
				hint: 'RAM user credentials with AliyunDNSFullAccess.',
			},
			{key: 'region', label: 'Region (optional)'},
		],
	},
	TENCENTCLOUD: {
		label: 'Tencent Cloud DNSPod',
		fields: [
			{key: 'secretId', label: 'Secret ID'},
			{
				key: 'secretKey',
				label: 'Secret Key',
				secret: true,
				hint: 'CAM API credentials with DNSPod permissions.',
			},
			{key: 'region', label: 'Region (optional)'},
		],
	},
	VOLCENGINE: {
		label: 'Volcengine DNS',
		fields: [
			{key: 'accessKey', label: 'Access Key'},
			{key: 'secretKey', label: 'Secret Key', secret: true},
			{key: 'region', label: 'Region (optional)'},
		],
	},
	YANDEXCLOUD: {
		label: 'Yandex Cloud DNS',
		fields: [
			{
				key: 'iamTokenB64',
				label: 'IAM Token (base64)',
				secret: true,
				hint: 'A base64-encoded IAM token for a service account with dns.dnsAdmin.',
			},
			{
				key: 'folderId',
				label: 'Folder ID',
				hint: 'The Yandex Cloud folder containing the DNS zones.',
			},
		],
	},
	DOMENESHOP: {
		label: 'Domeneshop DNS',
		fields: [
			{key: 'apiToken', label: 'API Token'},
			{
				key: 'apiSecret',
				label: 'API Secret',
				secret: true,
				hint: 'Create API credentials at domene.shop with DNS write access.',
			},
		],
	},
	SAFEDNS: {
		label: 'SafeDNS',
		fields: [
			{
				key: 'apiToken',
				label: 'API Token',
				secret: true,
				hint: 'Generate a token at safedns.skauto.net.',
			},
		],
	},
	SIMPLYCOM: {
		label: 'simply.com DNS',
		fields: [
			{key: 'accountName', label: 'Account Name'},
			{
				key: 'apiKey',
				label: 'API Key',
				secret: true,
				hint: 'An API key created in the simply.com control panel.',
			},
		],
	},
	ARVANCLOUD: {
		label: 'ArvanCloud DNS',
		fields: [
			{
				key: 'apiKey',
				label: 'API Key',
				secret: true,
				hint: 'Create a key in the ArvanCloud panel with DNS record permissions.',
			},
		],
	},
	INWX: {
		label: 'InterNetX (inwx) DNS',
		fields: [
			{key: 'username', label: 'Username'},
			{key: 'password', label: 'Password', secret: true},
			{
				key: 'sandbox',
				label: 'Sandbox',
				hint: '"true" to use the OT&E test environment instead of production.',
			},
		],
	},
	ULTRADNS: {
		label: 'UltraDNS',
		fields: [
			{key: 'username', label: 'Username'},
			{key: 'password', label: 'Password', secret: true},
			{
				key: 'endpoint',
				label: 'API Endpoint (optional)',
				hint: 'Defaults to the UltraDNS REST v3 public endpoint.',
			},
		],
	},
	INFOBLOX: {
		label: 'Infoblox DNS',
		fields: [
			{
				key: 'host',
				label: 'Grid Manager Host',
				hint: 'Hostname or IP of the Infoblox Grid Manager, e.g. ib.example.com.',
			},
			{key: 'username', label: 'Username'},
			{key: 'password', label: 'Password', secret: true},
			{
				key: 'wapiVersion',
				label: 'WAPI Version (optional)',
				hint: 'e.g. 2.10 — defaults to the latest supported version.',
			},
			{key: 'dnsView', label: 'DNS View (optional)'},
		],
	},
	ORACLECLOUD: {
		label: 'Oracle Cloud DNS',
		fields: [
			{key: 'tenancyOcid', label: 'Tenancy OCID'},
			{key: 'userOcid', label: 'User OCID'},
			{
				key: 'fingerprint',
				label: 'API Key Fingerprint',
				hint: 'SHA-256 fingerprint of the uploaded API public key.',
			},
			{
				key: 'privateKeyPem',
				label: 'Private Key (PEM)',
				secret: true,
				hint: 'Contents of the corresponding .pem private API key.',
			},
			{key: 'region', label: 'Region', hint: 'e.g. eu-frankfurt-1.'},
			{
				key: 'compartmentOcid',
				label: 'Compartment OCID',
				hint: 'The compartment that owns the DNS zones.',
			},
		],
	},
	RFC2136: {
		label: 'RFC 2136 (BIND, etc.)',
		fields: [
			{
				key: 'server',
				label: 'Server Host',
				hint: 'The authoritative DNS server accepting dynamic updates.',
			},
			{key: 'port', label: 'Port (optional)', hint: 'Defaults to 53.'},
			{key: 'keyName', label: 'TSIG Key Name'},
			{
				key: 'key',
				label: 'TSIG Key (base64)',
				secret: true,
				hint: 'The base64 secret from your tsig-keygen / nsupdate key file.',
			},
			{
				key: 'algorithm',
				label: 'TSIG Algorithm (optional)',
				hint: 'hmac-sha256 by default; also supports hmac-md5, hmac-sha1, hmac-sha224, hmac-sha384, hmac-sha512.',
			},
			ZONE_FIELD,
		],
	},
};

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
	...Object.entries(bridgeProviders).map(([value, p]) => ({
		value,
		label: p.label,
	})),
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
			return <Globe className={`${className} text-muted-foreground`} />;
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
		default: {
			const bridge = bridgeProviders[type.toUpperCase()];
			if (bridge) {
				return bridge.fields.some(f => f.key === 'zone')
					? bridge.fields
					: [...bridge.fields, ZONE_FIELD];
			}
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
