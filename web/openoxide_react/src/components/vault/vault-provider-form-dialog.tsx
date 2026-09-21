import React, {useEffect, useState} from 'react';
import {KeyRound} from 'lucide-react';
import {Input} from '#/components/ui/input';
import {
	Sheet,
	SheetContent,
	SheetDescription,
	SheetHeader,
	SheetTitle,
} from '#/components/ui/sheet';
import {FieldLabel} from '#/components/shared/field-label';
import {ProviderTypeSelect} from '#/components/shared/provider-type-select';
import {TestSaveFooter} from '#/components/shared/test-save-footer';
import {
	buildVaultConfig,
	vaultProviderOptions,
	type VaultProviderItem,
	type VaultProviderType,
} from './vault-provider-catalog';
import {
	HashicorpFields,
	VaultTypeFields,
	getVaultTokenLabel,
} from './vault-provider-fields';
import {VaultProviderAssignments} from './vault-provider-assignments';
import {useVaultProviderFormActions} from './use-vault-provider-form-actions';

/**
 * Create / edit vault provider modal: credentials per provider type plus
 * project assignments, with a live connection test through the backend.
 */
export function VaultProviderFormDialog({
	open,
	onOpenChange,
	provider,
}: {
	open: boolean;
	onOpenChange: (open: boolean) => void;
	provider: VaultProviderItem | null;
}) {
	const [formName, setFormName] = useState('');
	const [formType, setFormType] = useState<VaultProviderType>('HASHICORP');
	const [formUrl, setFormUrl] = useState('');
	const [formMount, setFormMount] = useState('secret');
	const [formToken, setFormToken] = useState('');
	const [formNamespace, setFormNamespace] = useState('');
	const [formFields, setFormFields] = useState<Record<string, string>>({});
	const [formAssignments, setFormAssignments] = useState('[]');

	useEffect(() => {
		if (!open) return;
		setFormName(provider?.name ?? '');
		setFormType(
			(provider
				? provider.provider_type.toUpperCase()
				: 'HASHICORP') as VaultProviderType,
		);
		setFormUrl(provider?.api_url || '');
		setFormToken('');
		setFormNamespace(provider?.namespace || '');
		setFormMount('secret');
		try {
			setFormFields(JSON.parse(provider?.config_json || '{}'));
		} catch {
			setFormFields({});
		}
		setFormAssignments(
			JSON.stringify(provider?.assignments || [], null, 2),
		);
	}, [open, provider]);

	const field = (name: string) => formFields[name] || '';
	const setField = (name: string, value: string) =>
		setFormFields(prev => ({...prev, [name]: value}));

	const {save, test, isSubmitting, isTesting} =
		useVaultProviderFormActions(() => ({
			provider,
			name: formName,
			type: formType,
			url: formUrl,
			token: formToken,
			namespace: formNamespace,
			configJson: () =>
				JSON.stringify(buildVaultConfig(formType, formMount, field)),
			assignmentsJson: formAssignments,
		}));

	const handleSaveProvider = async (e: React.FormEvent) => {
		e.preventDefault();
		if (await save()) onOpenChange(false);
	};

	return (
		<Sheet open={open} onOpenChange={onOpenChange}>
			<SheetContent>
				<SheetHeader>
					<SheetTitle className="flex items-center gap-2">
						<KeyRound className="size-4 text-primary" />
						{provider ? 'Update Vault Provider' : 'Add Vault Provider'}
					</SheetTitle>
					<SheetDescription>
						Reference secrets in your environment variables with{' '}
						<code className="font-mono text-primary">
							{'${{vault.<name>.<secret>}}'}
						</code>
						. Secrets are fetched at deploy time.
					</SheetDescription>
				</SheetHeader>

				<form
					onSubmit={handleSaveProvider}
					className="flex flex-1 flex-col space-y-4 overflow-y-auto px-6 py-5">
					<div className="space-y-1">
						<FieldLabel>Name</FieldLabel>
						<Input
							placeholder="prod-vault"
							value={formName}
							onChange={e => setFormName(e.target.value)}
							className="h-10 bg-muted/20 text-xs"
						/>
					</div>

					<ProviderTypeSelect
						label="Provider"
						value={formType}
						onChange={v => setFormType(v as VaultProviderType)}
						disabled={!!provider}
						options={vaultProviderOptions}
					/>

					{formType === 'HASHICORP' && (
						<div className="space-y-1">
							<FieldLabel>Vault URL</FieldLabel>
							<Input
								placeholder="https://vault.example.com:8200"
								value={formUrl}
								onChange={e => setFormUrl(e.target.value)}
								className="h-10 bg-muted/20 text-xs"
							/>
						</div>
					)}

					<div className="space-y-1">
						<FieldLabel>{getVaultTokenLabel(formType)}</FieldLabel>
						<Input
							type="password"
							placeholder={
								provider ? 'Leave blank to keep existing token' : 'Token'
							}
							value={formToken}
							onChange={e => setFormToken(e.target.value)}
							className="h-10 bg-muted/20 font-mono text-xs"
						/>
					</div>

					{formType === 'HASHICORP' ? (
						<HashicorpFields
							mount={formMount}
							namespace={formNamespace}
							onMountChange={setFormMount}
							onNamespaceChange={setFormNamespace}
						/>
					) : (
						<VaultTypeFields
							formType={formType}
							field={field}
							setField={setField}
						/>
					)}

					<VaultProviderAssignments
						value={formAssignments}
						onChange={setFormAssignments}
					/>

					<TestSaveFooter
						isTesting={isTesting}
						isSubmitting={isSubmitting}
						editing={!!provider}
						onTest={test}
					/>
				</form>
			</SheetContent>
		</Sheet>
	);
}
