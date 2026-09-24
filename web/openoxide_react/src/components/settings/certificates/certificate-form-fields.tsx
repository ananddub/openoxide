import {Input} from '#/components/ui/input';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import {Switch} from '#/components/ui/switch';
import {Textarea} from '#/components/ui/textarea';
import {WizardField} from '#/components/shared/sheet-wizard';
import type {CertificateForm, RemoteServer} from './certificate-types';

type Props = {
	editing: boolean;
	form: CertificateForm;
	servers: RemoteServer[];
	onChange: <K extends keyof CertificateForm>(
		key: K,
		value: CertificateForm[K],
	) => void;
};

const slug = (value: string) =>
	value
		.toLowerCase()
		.trim()
		.replace(/[^a-z0-9]+/g, '-')
		.replace(/^-|-$/g, '');

/** Scroll body of the certificate sheet — every field in the house dialect. */
export function CertificateFormFields({
	editing,
	form,
	servers,
	onChange,
}: Props) {
	return (
		<div className="flex min-h-0 flex-1 flex-col gap-5 overflow-y-auto px-6 py-6">
			<WizardField label="Name" required>
				<Input
					value={form.name}
					onChange={event => {
						onChange('name', event.target.value);
						if (!editing)
							onChange('certificate_path', slug(event.target.value));
					}}
					placeholder="Production wildcard"
				/>
			</WizardField>

			<WizardField
				label="Directory"
				required
				hint="Folder under the panel's certificate store. Derived from the name.">
				<Input
					value={form.certificate_path}
					onChange={event =>
						onChange('certificate_path', slug(event.target.value))
					}
					placeholder="production-wildcard"
					disabled={editing}
					className="font-mono"
				/>
			</WizardField>

			<WizardField label="Target server">
				<Select
					value={form.server_id}
					onValueChange={value => value && onChange('server_id', value)}
					disabled={editing}>
					<SelectTrigger>
						<SelectValue placeholder="Select target server" />
					</SelectTrigger>
					<SelectContent className="bg-card">
						<SelectItem value="local">Openoxide (local)</SelectItem>
						{servers
							.filter(server => server.server_type !== 'BUILD')
							.map(server => (
								<SelectItem key={server.id} value={String(server.id)}>
									{server.name} ({server.ip_address})
								</SelectItem>
							))}
					</SelectContent>
				</Select>
			</WizardField>

			<WizardField label="Certificate chain" required>
				<Textarea
					className="min-h-40 resize-y rounded-lg font-mono text-xs"
					value={form.certificate_data}
					onChange={event =>
						onChange('certificate_data', event.target.value)
					}
					placeholder="-----BEGIN CERTIFICATE-----"
				/>
			</WizardField>

			<WizardField
				label="Private key"
				required={!editing}
				hint={
					editing
						? 'Leave blank to keep the current private key.'
						: 'Must match the certificate chain above.'
				}>
				<Textarea
					className="min-h-40 resize-y rounded-lg font-mono text-xs"
					value={form.private_key}
					onChange={event => onChange('private_key', event.target.value)}
					placeholder="-----BEGIN PRIVATE KEY-----"
				/>
			</WizardField>

			<div className="flex items-center justify-between gap-4 rounded-xl border border-border/60 px-4 py-3.5">
				<div className="min-w-0">
					<p className="text-[13px] font-medium text-foreground">
						Track manual renewals
					</p>
					<p className="mt-0.5 text-[13px] text-muted-foreground">
						Store replacement history for this certificate.
					</p>
				</div>
				<Switch
					checked={form.auto_renew}
					onCheckedChange={checked => onChange('auto_renew', checked)}
				/>
			</div>
		</div>
	);
}
