import React, {useEffect, useState} from 'react';
import {Loader2, RefreshCw, Save} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {
	Sheet,
	SheetContent,
	SheetDescription,
	SheetHeader,
	SheetTitle,
} from '#/components/ui/sheet';
import {
	OptionRow,
	StepBody,
	StepCheckpoints,
	WizardField,
	type WizardStep,
} from '#/components/shared/sheet-wizard';
import {
	buildVaultConfig,
	getVaultProviderLabel,
	vaultProviderOptions,
	VaultProviderIcon,
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

const STEPS: WizardStep[] = [
	{id: 'provider', label: 'Provider'},
	{id: 'connection', label: 'Connection'},
	{id: 'projects', label: 'Projects'},
	{id: 'review', label: 'Review'},
];

const STEP_DESC: Record<string, string> = {
	provider: 'Name this provider and pick where your secrets live.',
	connection: 'Point at the server and paste its credentials.',
	projects: 'Scope it to specific projects, or leave it account-wide.',
	review: 'Confirm the details and test the connection.',
};

const PROVIDER_HINTS: Record<string, string> = {
	HASHICORP: 'Self-hosted Vault or OpenBao',
	DOPPLER: 'Doppler service token',
	INFISICAL: 'Universal Auth / self-hosted',
	AWS: 'Access key + secret',
	AWS_PARAMETER_STORE: 'Systems Manager parameters',
	AZURE: 'Azure Key Vault',
	SCALEWAY: 'Scaleway Secret Manager',
};

/** Create / edit vault provider — a four-checkpoint wizard: pick provider, enter credentials, scope to projects, then review + test. */
export function VaultProviderFormDialog({
	open,
	onOpenChange,
	provider,
}: {
	open: boolean;
	onOpenChange: (open: boolean) => void;
	provider: VaultProviderItem | null;
}) {
	const [stepIdx, setStepIdx] = useState(0);
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
		setStepIdx(0);
		setFormName(provider?.name ?? '');
		setFormType(
			(provider
				? provider.provider_type.toUpperCase()
				: 'HASHICORP') as VaultProviderType,
		);
		setFormUrl(provider?.api_url || '');
		setFormToken('');
		setFormNamespace(provider?.namespace || '');
		let config: Record<string, string> = {};
		try {
			config = JSON.parse(provider?.config_json || '{}');
		} catch {
			config = {};
		}
		setFormFields(config);
		setFormMount(config.mount || 'secret');
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

	const step = STEPS[stepIdx].id;
	const last = stepIdx === STEPS.length - 1;
	const canContinue = step === 'provider' ? !!formName.trim() : true;

	const handleSubmit = async (e: React.FormEvent) => {
		e.preventDefault();
		if (await save()) onOpenChange(false);
	};

	const goNext = () => setStepIdx(i => Math.min(i + 1, STEPS.length - 1));
	const goBack = () => setStepIdx(i => Math.max(i - 1, 0));

	const assignmentCount = (() => {
		try {
			const v = JSON.parse(formAssignments);
			return Array.isArray(v) ? v.length : 0;
		} catch {
			return 0;
		}
	})();

	return (
		<Sheet open={open} onOpenChange={onOpenChange}>
			<SheetContent>
				<SheetHeader>
					<SheetTitle>
						{provider ? 'Update Vault Provider' : 'New Vault Provider'}
					</SheetTitle>
					<SheetDescription>{STEP_DESC[step]}</SheetDescription>
					<StepCheckpoints steps={STEPS} idx={stepIdx} />
				</SheetHeader>

				<form
					onSubmit={handleSubmit}
					className="flex min-h-0 flex-1 flex-col">
					<div className="flex min-h-0 flex-1 flex-col overflow-y-auto px-6 pt-6 pb-6">
						<StepBody key={step}>
							{step === 'provider' && (
								<div className="flex flex-col gap-5">
									<WizardField label="Name" required>
										<Input
											placeholder="prod-vault"
											value={formName}
											onChange={e => setFormName(e.target.value)}
											className="bg-background"
										/>
									</WizardField>

									{provider ? (
										<OptionRow
											selected
											onClick={() => {}}
											icon={
												<VaultProviderIcon
													type={formType}
													className="size-5 shrink-0"
												/>
											}
											title={getVaultProviderLabel(formType)}
											subtitle="Provider type can’t be changed after creation."
										/>
									) : (
										<div className="flex flex-col gap-2">
											{vaultProviderOptions.map(o => (
												<OptionRow
													key={o.value}
													selected={formType === o.value}
													onClick={() =>
														setFormType(o.value as VaultProviderType)
													}
													icon={
														<VaultProviderIcon
															type={o.value}
															className="size-5 shrink-0"
														/>
													}
													title={o.label}
													subtitle={PROVIDER_HINTS[o.value]}
												/>
											))}
										</div>
									)}
								</div>
							)}

							{step === 'connection' && (
								<div className="flex flex-col gap-5">
									{formType === 'HASHICORP' && (
										<WizardField
											label="Vault URL"
											hint="Defaults to http://localhost:8200 if left blank.">
											<Input
												placeholder="https://vault.example.com:8200"
												value={formUrl}
												onChange={e => setFormUrl(e.target.value)}
												className="bg-background font-mono"
											/>
										</WizardField>
									)}

									<WizardField
										label={getVaultTokenLabel(formType)}
										required
										hint={
											provider
												? 'Leave blank to keep the existing token.'
												: 'Stored encrypted; never sent back to the browser.'
										}>
										<Input
											type="password"
											placeholder={
												provider
													? 'Leave blank to keep existing token'
													: 'Token'
											}
											value={formToken}
											onChange={e => setFormToken(e.target.value)}
											className="bg-background font-mono"
										/>
									</WizardField>

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
								</div>
							)}

							{step === 'projects' && (
								<VaultProviderAssignments
									value={formAssignments}
									onChange={setFormAssignments}
								/>
							)}

							{step === 'review' && (
								<div className="flex flex-col gap-5">
									<div className="flex items-center gap-3.5 rounded-xl border border-border/60 bg-muted/20 px-4 py-3.5">
										<span className="flex size-10 shrink-0 items-center justify-center rounded-lg border border-border/60 bg-background">
											<VaultProviderIcon
												type={formType}
												className="size-5 shrink-0"
											/>
										</span>
										<div className="min-w-0 flex-1">
											<p className="truncate text-sm font-medium text-foreground">
												{formName || 'Untitled provider'}
											</p>
											<p className="mt-0.5 truncate text-[13px] text-muted-foreground">
												{getVaultProviderLabel(formType)}
												{formUrl ? ` · ${formUrl}` : ''}
											</p>
										</div>
									</div>

									<div className="flex flex-col gap-1.5">
										<span className="text-[13px] font-medium text-foreground">
											Access
										</span>
										<p className="text-[13px] text-muted-foreground">
											{assignmentCount > 0
												? `Scoped to ${assignmentCount} project${
														assignmentCount > 1 ? 's' : ''
													}.`
												: 'Available to all projects.'}
										</p>
									</div>

									<div className="rounded-xl border border-border/60 bg-muted/20 px-4 py-3.5">
										<p className="mb-1.5 text-[13px] font-medium text-foreground">
											Reference your secrets with
										</p>
										<code className="font-mono text-[12px] break-all text-primary">
											{`\${{vault.${formName || '<name>'}.<secret>}}`}
										</code>
									</div>
								</div>
							)}
						</StepBody>
					</div>

					<div className="flex items-center justify-end gap-3 border-t border-border px-6 py-5">
						{stepIdx > 0 && (
							<Button
								type="button"
								variant="outline"
								className="mr-auto"
								onClick={goBack}>
								Back
							</Button>
						)}

						{last ? (
							<div className="flex items-center gap-3">
								<Button
									type="button"
									variant="outline"
									onClick={() => void test()}
									disabled={isTesting}
									className="h-10 gap-1.5 px-4">
									{isTesting ? (
										<Loader2 className="size-3.5 animate-spin" />
									) : (
										<RefreshCw className="size-3.5 text-emerald-500" />
									)}
									Test
								</Button>
								<Button
									type="submit"
									disabled={isSubmitting || !formName.trim()}
									className="h-10 gap-1.5 px-5">
									{isSubmitting ? (
										<Loader2 className="size-3.5 animate-spin" />
									) : (
										<Save className="size-3.5" />
									)}
									{provider ? 'Save changes' : 'Add provider'}
								</Button>
							</div>
						) : (
							<Button
								type="button"
								disabled={!canContinue}
								onClick={goNext}>
								Continue
							</Button>
						)}
					</div>
				</form>
			</SheetContent>
		</Sheet>
	);
}
