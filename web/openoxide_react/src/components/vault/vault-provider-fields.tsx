import {Input} from '#/components/ui/input';
import {WizardField} from '#/components/shared/sheet-wizard';
import {type VaultProviderType} from './vault-provider-catalog';

// [field name, label, placeholder] triples per provider type.
export const VAULT_FIELD_DEFINITIONS: Record<
	string,
	Array<[string, string, string]>
> = {
	INFISICAL: [
		['siteUrl', 'Site URL', 'https://app.infisical.com'],
		['clientId', 'Client ID', 'Universal Auth client ID'],
		['clientSecret', 'Client Secret', 'Universal Auth client secret'],
		['projectId', 'Project ID', 'Infisical project ID'],
		['environmentSlug', 'Environment', 'prod'],
		['secretPath', 'Secret Path', '/'],
	],
	AWS: [
		['region', 'Region', 'us-east-1'],
		['accessKeyId', 'Access Key ID', 'AKIA...'],
		['secretAccessKey', 'Secret Access Key', 'Secret access key'],
		['endpoint', 'Endpoint (optional)', 'https://...'],
	],
	AWS_PARAMETER_STORE: [
		['region', 'Region', 'us-east-1'],
		['accessKeyId', 'Access Key ID', 'AKIA...'],
		['secretAccessKey', 'Secret Access Key', 'Secret access key'],
		[
			'parameterPath',
			'Path Prefix (optional)',
			'/myapp — parameter discovery root',
		],
		['endpoint', 'Endpoint (optional)', 'https://...'],
	],
	DOPPLER: [
		['project', 'Project (optional)', 'Doppler project'],
		['config', 'Config (optional)', 'Doppler config'],
	],
	AZURE: [
		['vaultUri', 'Vault URI', 'https://my-vault.vault.azure.net'],
		['tenantId', 'Tenant ID', 'Azure tenant ID'],
		['clientId', 'Client ID', 'Application client ID'],
		['clientSecret', 'Client Secret', 'Application client secret'],
	],
	SCALEWAY: [
		['region', 'Region', 'fr-par'],
		['projectId', 'Project ID', 'Scaleway project ID'],
		['secretKey', 'Secret Key', 'Scaleway secret key'],
		['apiUrl', 'API URL (optional)', 'https://api.scaleway.com'],
	],
};

export function isSecretField(name: string) {
	return name.toLowerCase().includes('secret') || name === 'secretKey';
}

/** KV mount / namespace inputs plus reference-format hint (HashiCorp). */
export function HashicorpFields({
	mount,
	namespace,
	onMountChange,
	onNamespaceChange,
}: {
	mount: string;
	namespace: string;
	onMountChange: (v: string) => void;
	onNamespaceChange: (v: string) => void;
}) {
	return (
		<>
			<div className="grid grid-cols-2 gap-3">
				<WizardField label="KV Mount">
					<Input
						placeholder="secret"
						value={mount}
						onChange={e => onMountChange(e.target.value)}
						className="font-mono"
					/>
				</WizardField>
				<WizardField label="Namespace" optional>
					<Input
						placeholder="admin"
						value={namespace}
						onChange={e => onNamespaceChange(e.target.value)}
						className="font-mono"
					/>
				</WizardField>
			</div>
			<div className="rounded-lg bg-muted/30 p-2.5 text-[11px] text-muted-foreground">
				Reference format:{' '}
				<code className="font-mono font-semibold text-primary">
					{'${{vault.<name>.path/to/secret:FIELD}}'}
				</code>
			</div>
		</>
	);
}

export function getVaultTokenLabel(formType: VaultProviderType) {
	return formType === 'DOPPLER'
		? 'Service Token'
		: formType === 'HASHICORP'
			? 'Vault Token'
			: 'Authentication Secret';
}

/** Definition-driven credential inputs for non-HashiCorp providers. */
export function VaultTypeFields({
	formType,
	field,
	setField,
}: {
	formType: string;
	field: (name: string) => string;
	setField: (name: string, value: string) => void;
}) {
	const definitions = VAULT_FIELD_DEFINITIONS[formType] ?? [];
	return (
		<div className="grid gap-3">
			{definitions.map(
				([name, label, placeholder]: [string, string, string]) => {
					const optional = label.endsWith(' (optional)');
					return (
						<WizardField
							key={name}
							label={
								optional
									? label.slice(0, -11)
									: label
							}
							optional={optional}>
							<Input
								type={
									isSecretField(name)
										? 'password'
										: 'text'
								}
								placeholder={placeholder}
								value={field(name)}
								onChange={e =>
									setField(name, e.target.value)
								}
								className="font-mono"
							/>
						</WizardField>
					);
				},
			)}
		</div>
	);
}
