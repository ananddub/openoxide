import {Input} from '#/components/ui/input';
import {Label} from '#/components/ui/label';
import {Switch} from '#/components/ui/switch';
import {Select, SelectContent, SelectItem, SelectTrigger, SelectValue} from '#/components/ui/select';
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
	onChange: <K extends keyof DomainFormState>(key: K, value: DomainFormState[K]) => void;
};

const fieldClass = 'mt-1.5 font-mono text-xs';

export function DomainFormFields({state, services, onChange}: Props) {
	return (
		<>
			<div><Label>Host *</Label><Input className={fieldClass} value={state.host} onChange={event => onChange('host', event.target.value)} placeholder="app.example.com" /></div>
			{services.length > 0 && <div><Label>Compose service *</Label><Select value={state.serviceName} onValueChange={value => value && onChange('serviceName', value)}><SelectTrigger className="mt-1.5 text-xs"><SelectValue placeholder="Select service" /></SelectTrigger><SelectContent>{services.map(service => <SelectItem key={service} value={service}>{service}</SelectItem>)}</SelectContent></Select></div>}
			<div><Label>Container port *</Label><Input className={fieldClass} type="number" min="1" max="65535" value={state.port} onChange={event => onChange('port', event.target.value)} /></div>
			<div><Label>Public path</Label><Input className={fieldClass} value={state.path} onChange={event => onChange('path', event.target.value)} placeholder="/api" /></div>
			<div><Label>Internal path</Label><Input className={fieldClass} value={state.internalPath} onChange={event => onChange('internalPath', event.target.value)} placeholder="/" /></div>
			<ToggleField label="HTTPS / SSL" description="Secure traffic through Traefik" checked={state.https} onChange={value => onChange('https', value)} />
			{state.https && <div><Label>Certificate type</Label><Select value={state.certificateType} onValueChange={value => onChange('certificateType', value as DomainFormData['certificate_type'])}><SelectTrigger className="mt-1.5 text-xs"><SelectValue /></SelectTrigger><SelectContent><SelectItem value="LETSENCRYPT">Let's Encrypt (automatic)</SelectItem><SelectItem value="NONE">None (manual/default certificate)</SelectItem><SelectItem value="CUSTOM">Custom resolver</SelectItem></SelectContent></Select><p className="mt-1 text-[11px] text-muted-foreground">None keeps HTTPS enabled without a Traefik certificate resolver.</p></div>}
			{state.https && state.certificateType === 'CUSTOM' && <div><Label>Custom certificate resolver *</Label><Input className={fieldClass} value={state.customResolver} onChange={event => onChange('customResolver', event.target.value)} placeholder="myresolver" /></div>}
			<div><Label>Custom entrypoint</Label><Input className={fieldClass} value={state.customEntrypoint} onChange={event => onChange('customEntrypoint', event.target.value)} placeholder="websecure" /></div>
			<div><Label>Middlewares</Label><Input className="mt-1.5 text-xs" value={state.middlewares} onChange={event => onChange('middlewares', event.target.value)} placeholder="auth@file, rate-limit@file" /></div>
			<ToggleField label="Strip public path" description="Remove the public prefix before forwarding" checked={state.stripPath} onChange={value => onChange('stripPath', value)} />
		</>
	);
}

function ToggleField({label, description, checked, onChange}: {label: string; description: string; checked: boolean; onChange: (value: boolean) => void}) {
	return <div className="flex items-center justify-between rounded-lg border p-3"><div><Label>{label}</Label><p className="text-[11px] text-muted-foreground">{description}</p></div><Switch checked={checked} onCheckedChange={onChange} /></div>;
}
