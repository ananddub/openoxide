import {useEffect, useState} from 'react';
import {Globe, Loader2, ShieldCheck, ShieldX} from 'lucide-react';
import {Input} from '#/components/ui/input';
import {Switch} from '#/components/ui/switch';
import {Button} from '#/components/ui/button';
import {useValidateDomainDns} from '#/api/live';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import {WizardField} from '#/components/shared/sheet-wizard';
import type {DomainFormData} from './domain-dialog';

export type DomainFormState = {
	host: string;
	path: string;
	internalPath: string;
	port: string;
	serviceName: string;
	https: boolean;
	certificateType: DomainFormData['certificate_type'];
	customResolver: string;
	customEntrypoint: string;
	stripPath: boolean;
	middlewares: string;
};

type Props = {
	state: DomainFormState;
	services: string[];
	onChange: <K extends keyof DomainFormState>(
		key: K,
		value: DomainFormState[K],
	) => void;
};

const mono = 'bg-background font-mono text-sm';

/** Checkpoint 1 — the host and the container it points at. */
export function DomainTargetFields({state, services, onChange}: Props) {
	return (
		<div className="flex flex-col gap-5">
			<WizardField
				label="Host"
				required
				hint="The public address people will open in a browser.">
				<Input
					className={mono}
					value={state.host}
					onChange={event => onChange('host', event.target.value)}
					placeholder="app.example.com"
					autoFocus
				/>
				<DnsValidationRow host={state.host} />
			</WizardField>

			{services.length > 0 && (
				<WizardField
					label="Compose service"
					required
					hint="Which container in the stack receives this traffic.">
					<Select
						value={state.serviceName}
						onValueChange={value =>
							value && onChange('serviceName', value)
						}>
						<SelectTrigger className="text-sm">
							<SelectValue placeholder="Select service" />
						</SelectTrigger>
						<SelectContent>
							{services.map(service => (
								<SelectItem key={service} value={service}>
									{service}
								</SelectItem>
							))}
						</SelectContent>
					</Select>
				</WizardField>
			)}

			<WizardField
				label="Container port"
				required
				hint="The port the service listens on inside its container.">
				<Input
					className={mono}
					type="number"
					min="1"
					max="65535"
					value={state.port}
					onChange={event => onChange('port', event.target.value)}
				/>
			</WizardField>
		</div>
	);
}

/** Checkpoint 2 — how the public URL maps onto the container. */
export function DomainRoutingFields({state, onChange}: Props) {
	return (
		<div className="flex flex-col gap-5">
			<WizardField
				label="Public path"
				optional
				hint="URL prefix this route claims — leave / for the whole host.">
				<Input
					className={mono}
					value={state.path}
					onChange={event => onChange('path', event.target.value)}
					placeholder="/api"
				/>
			</WizardField>

			<WizardField
				label="Internal path"
				optional
				hint="What the container is asked for instead of the public path.">
				<Input
					className={mono}
					value={state.internalPath}
					onChange={event => onChange('internalPath', event.target.value)}
					placeholder="/"
				/>
			</WizardField>

			<SwitchRow
				label="Strip public path"
				description="Remove the public prefix before forwarding — needs a public path other than /."
				checked={state.stripPath}
				onChange={value => onChange('stripPath', value)}
			/>
		</div>
	);
}

/** Checkpoint 3 — TLS, certificate resolver and Traefik extras. */
export function DomainTlsFields({state, onChange}: Props) {
	return (
		<div className="flex flex-col gap-5">
			<SwitchRow
				label="HTTPS / SSL"
				description="Terminate TLS at the proxy and redirect secure traffic."
				checked={state.https}
				onChange={value => onChange('https', value)}
			/>

			{state.https && (
				<WizardField
					label="Certificate"
					hint="“None” keeps HTTPS enabled without a Traefik certificate resolver.">
					<Select
						value={state.certificateType}
						onValueChange={value =>
							onChange(
								'certificateType',
								value as DomainFormData['certificate_type'],
							)
						}>
						<SelectTrigger className="text-sm">
							<SelectValue />
						</SelectTrigger>
						<SelectContent>
							<SelectItem value="LETSENCRYPT">
								Let's Encrypt (automatic)
							</SelectItem>
							<SelectItem value="NONE">
								None (manual/default certificate)
							</SelectItem>
							<SelectItem value="CUSTOM">Custom resolver</SelectItem>
						</SelectContent>
					</Select>
				</WizardField>
			)}

			{state.https && state.certificateType === 'CUSTOM' && (
				<WizardField
					label="Custom certificate resolver"
					required
					hint="The name of the resolver declared on the proxy.">
					<Input
						className={mono}
						value={state.customResolver}
						onChange={event =>
							onChange('customResolver', event.target.value)
						}
						placeholder="myresolver"
					/>
				</WizardField>
			)}

			<WizardField
				label="Custom entrypoint"
				optional
				hint="Override the proxy entrypoint this route attaches to.">
				<Input
					className={mono}
					value={state.customEntrypoint}
					onChange={event =>
						onChange('customEntrypoint', event.target.value)
					}
					placeholder="websecure"
				/>
			</WizardField>

			<WizardField
				label="Middlewares"
				optional
				hint="Comma-separated — they run in the order you list them.">
				<Input
					className={mono}
					value={state.middlewares}
					onChange={event => onChange('middlewares', event.target.value)}
					placeholder="auth@file, rate-limit@file"
				/>
			</WizardField>
		</div>
	);
}

function SwitchRow({
	label,
	description,
	checked,
	onChange,
}: {
	label: string;
	description: string;
	checked: boolean;
	onChange: (value: boolean) => void;
}) {
	return (
		<label className="flex cursor-pointer items-center justify-between gap-4 rounded-xl border px-4 py-3.5 transition-colors hover:bg-muted/30">
			<span className="min-w-0">
				<span className="block text-sm font-medium text-foreground">
					{label}
				</span>
				<span className="block text-[13px] leading-relaxed text-muted-foreground">
					{description}
				</span>
			</span>
			<Switch checked={checked} onCheckedChange={onChange} />
		</label>
	);
}

/** "Validate DNS" check against the host being typed. */
function DnsValidationRow({host}: {host: string}) {
	const validation = useValidateDomainDns();
	const [validatedHost, setValidatedHost] = useState('');
	const trimmed = host.trim();
	const result = validatedHost === trimmed ? validation.data : undefined;
	// Auto-validate (style) once the host looks complete and typing settled.
	useEffect(() => {
		if (!trimmed || (validatedHost === trimmed && !validation.isError))
			return;
		if (trimmed !== 'localhost' && !trimmed.includes('.')) return;
		const timer = setTimeout(() => {
			setValidatedHost(trimmed);
			validation.mutate({host: trimmed});
		}, 900);
		return () => clearTimeout(timer);
		// eslint-disable-next-line react-hooks/exhaustive-deps
	}, [trimmed]);
	return (
		<div className="mt-2 flex flex-col items-start gap-2">
			<Button
				type="button"
				variant="outline"
				size="sm"
				className="h-8 gap-1.5 text-xs"
				disabled={!trimmed || validation.isPending}
				onClick={() => {
					setValidatedHost(trimmed);
					validation.mutate({host: trimmed});
				}}>
				{validation.isPending ? (
					<Loader2 className="size-3.5 animate-spin" />
				) : (
					<Globe className="size-3.5" />
				)}
				Validate DNS
			</Button>
			{validation.isError && (
				<p className="text-[13px] text-amber-600">
					{validation.error.message || 'DNS check failed'}
				</p>
			)}
			{result?.isValid && (
				<p className="flex flex-wrap items-center gap-1.5 text-[13px] font-medium text-green-600">
					<ShieldCheck className="size-3.5" />
					DNS Valid
					<span className="font-mono text-[11px] text-muted-foreground">
						{result.resolvedIps.join(', ')}
					</span>
				</p>
			)}
			{result && !result.isValid && (
				<p className="flex flex-wrap items-center gap-1.5 text-[13px] leading-relaxed text-amber-600">
					<ShieldX className="size-3.5" />
					{result.error ??
						`Resolves to ${result.resolvedIps.join(', ') || 'nothing'} — expected ${result.expectedIps.join(', ')}`}
				</p>
			)}
		</div>
	);
}
