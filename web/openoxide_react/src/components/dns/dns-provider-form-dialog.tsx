import React, {useEffect, useState} from 'react';
import {Input} from '#/components/ui/input';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import {
	Sheet,
	SheetContent,
	SheetDescription,
	SheetHeader,
	SheetTitle,
} from '#/components/ui/sheet';
import {ProviderTypeSelect} from '#/components/shared/provider-type-select';
import {WizardField} from '#/components/shared/sheet-wizard';
import {TestSaveFooter} from '#/components/shared/test-save-footer';
import {
	dnsProviderFields,
	dnsProviderOptions,
	type DnsProviderItem,
	type DnsProviderType,
} from './dns-provider-catalog';
import {useDnsProviderFormActions} from './use-dns-provider-form-actions';

/** Create / edit DNS provider modal, including the credentials test call. Owns all form state; `provider` seeds the form when editing. */
export function DnsProviderFormDialog({
	open,
	onOpenChange,
	provider,
}: {
	open: boolean;
	onOpenChange: (open: boolean) => void;
	provider: DnsProviderItem | null;
}) {
	const [formName, setFormName] = useState('');
	const [formType, setFormType] = useState<DnsProviderType>('CLOUDFLARE');
	const [values, setValues] = useState<Record<string, string>>({});

	// Seeds every field of `type` from stored credentials (edit) or defaults.
	const seedValues = (type: DnsProviderType) => {
		const seeded: Record<string, string> = {};
		let stored: Record<string, unknown> = {};
		try {
			stored = JSON.parse(provider?.credentials_json || '{}');
		} catch {
			stored = {};
		}
		for (const field of dnsProviderFields(type)) {
			// Stored masked secrets round-trip as-is: sending them back tells the
			// backend to keep the existing value.
			seeded[field.key] =
				typeof stored[field.key] === 'string'
					? (stored[field.key] as string)
					: (field.defaultValue ?? '');
		}
		return seeded;
	};

	useEffect(() => {
		if (!open) return;
		setFormName(provider?.name ?? '');
		const type =
			((
				provider?.provider_type || 'CLOUDFLARE'
			).toUpperCase() as DnsProviderType) ?? 'CLOUDFLARE';
		setFormType(type);
		setValues(seedValues(type));
		// eslint-disable-next-line react-hooks/exhaustive-deps
	}, [open, provider]);

	const changeType = (type: DnsProviderType) => {
		setFormType(type);
		// Switching provider must seed the new field set, otherwise defaults
		// (e.g. the OVH endpoint) never enter `values` and the save guard fails.
		setValues(prev => ({...seedValues(type), ...prev}));
	};

	const setValue = (key: string, value: string) =>
		setValues(prev => ({...prev, [key]: value}));

	const {save, test, isSubmitting, isTesting} = useDnsProviderFormActions(
		() => ({
			provider,
			name: formName,
			type: formType,
			values,
		}),
	);

	const handleSaveProvider = async (e: React.FormEvent) => {
		e.preventDefault();
		if (await save()) onOpenChange(false);
	};

	return (
		<Sheet open={open} onOpenChange={onOpenChange}>
			<SheetContent>
				<SheetHeader>
					<SheetTitle>
						{provider ? 'Update DNS Provider' : 'New DNS Provider'}
					</SheetTitle>
					<SheetDescription>
						Configure API credentials for automated DNS-01 Let's Encrypt
						Wildcard SSL.
					</SheetDescription>
				</SheetHeader>

				<form
					onSubmit={handleSaveProvider}
					className="flex min-h-0 flex-1 flex-col">
					<div className="flex min-h-0 flex-1 flex-col space-y-4 overflow-y-auto px-6 py-5">
						<WizardField label="Provider Name">
							<Input
								placeholder="e.g. Cloudflare Production"
								value={formName}
								onChange={e => setFormName(e.target.value)}
							/>
						</WizardField>

						<ProviderTypeSelect
							value={formType}
							onChange={v => changeType(v as DnsProviderType)}
							disabled={!!provider}
							options={dnsProviderOptions}
						/>

						{dnsProviderFields(formType).map(field => (
							<WizardField
								key={field.key}
								label={field.label}
								hint={field.hint}>
								{field.select ? (
									<Select
										value={values[field.key] || field.select[0].value}
										onValueChange={v => setValue(field.key, v ?? '')}>
										<SelectTrigger>
											<SelectValue>
												{
													(
														field.select.find(
															o => o.value === values[field.key],
														) ?? field.select[0]
													).label
												}
											</SelectValue>
										</SelectTrigger>
										<SelectContent className="w-[var(--anchor-width)]">
											{field.select.map(option => (
												<SelectItem
													key={option.value}
													value={option.value}>
													{option.label}
												</SelectItem>
											))}
										</SelectContent>
									</Select>
								) : (
									<Input
										type={field.secret ? 'password' : 'text'}
										placeholder={
											field.secret && provider
												? 'Leave unchanged to keep existing value'
												: undefined
										}
										value={values[field.key] ?? ''}
										onChange={e => setValue(field.key, e.target.value)}
										className={field.secret ? 'font-mono' : undefined}
									/>
								)}
							</WizardField>
						))}
					</div>

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
