import React, {useEffect, useState} from 'react';
import {Globe} from 'lucide-react';
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
	credentialKind,
	dnsProviderOptions,
	type DnsProviderItem,
	type DnsProviderType,
} from './dns-provider-catalog';
import {useDnsProviderFormActions} from './use-dns-provider-form-actions';

/**
 * Create / edit DNS provider modal, including the credentials test call.
 * Owns all form state; `provider` seeds the form when editing.
 */
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
	const [formToken, setFormToken] = useState('');
	const [formAccessKey, setFormAccessKey] = useState('');

	useEffect(() => {
		if (!open) return;
		setFormName(provider?.name ?? '');
		setFormType(
			((provider?.provider_type || 'CLOUDFLARE').toUpperCase() ??
				'CLOUDFLARE') as DnsProviderType,
		);
		setFormToken('');
		try {
			const creds = JSON.parse(provider?.credentials_json || '{}');
			setFormAccessKey(creds.accessKeyId || creds.apiKey || '');
		} catch {
			setFormAccessKey('');
		}
	}, [open, provider]);

	const {save, test, isSubmitting, isTesting} = useDnsProviderFormActions(
		() => ({
			provider,
			name: formName,
			type: formType,
			accessKey: formAccessKey,
			token: formToken,
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
					<SheetTitle className="flex items-center gap-2">
						<Globe className="size-4 text-primary" />
						{provider ? 'Update DNS Provider' : 'Add DNS Provider'}
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
						<div className="space-y-1">
							<FieldLabel>Provider Name</FieldLabel>
							<Input
								placeholder="e.g. Cloudflare Production"
								value={formName}
								onChange={e => setFormName(e.target.value)}
								className="h-10 bg-muted/20 text-xs"
							/>
						</div>

						<ProviderTypeSelect
							value={formType}
							onChange={v => setFormType(v as DnsProviderType)}
							disabled={!!provider}
							options={dnsProviderOptions}
						/>

						{credentialKind(formType) !== 'token' && (
							<div className="space-y-1">
								<FieldLabel>
									{formType === 'GODADDY' ? 'API Key' : 'AWS Access Key ID'}
								</FieldLabel>
								<Input
									value={formAccessKey}
									onChange={e => setFormAccessKey(e.target.value)}
									className="h-10 bg-muted/20 font-mono text-xs"
								/>
							</div>
						)}
						<div className="space-y-1">
							<FieldLabel>
								{formType === 'GODADDY'
									? 'API Secret'
									: 'API Token / Secret Key'}
							</FieldLabel>
							<Input
								type="password"
								placeholder={
									provider
										? 'Leave blank to keep existing token'
										: 'API Token with Zone:DNS Edit permissions'
								}
								value={formToken}
								onChange={e => setFormToken(e.target.value)}
								className="h-10 bg-muted/20 font-mono text-xs"
							/>
						</div>
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
