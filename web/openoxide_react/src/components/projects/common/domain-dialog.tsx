import {useEffect, useState} from 'react';
import {createPortal} from 'react-dom';
import {Globe, RefreshCw, X} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {toast} from 'sonner';
import {DomainFormFields, type DomainFormState} from './domain-form-fields';

export type DomainFormData = {
	host: string;
	path: string;
	internal_path: string;
	strip_path: boolean;
	port: number;
	service_name?: string;
	https: boolean;
	certificate_type: 'LETSENCRYPT' | 'NONE' | 'CUSTOM';
	custom_cert_resolver?: string;
	custom_entrypoint?: string;
	middlewares: string;
};

interface DomainDialogProps {
	isOpen: boolean;
	onClose: () => void;
	editingDomain?: any | null;
	services?: string[];
	onSave: (data: DomainFormData) => Promise<void>;
}

export function DomainDialog({isOpen, onClose, editingDomain, services = [], onSave}: DomainDialogProps) {
	const [state, setState] = useState<DomainFormState>({host: '', path: '/', internalPath: '/', port: '3000', serviceName: '', https: true, certificateType: 'LETSENCRYPT', customResolver: '', customEntrypoint: '', stripPath: false, middlewares: ''});
	const [saving, setSaving] = useState(false);
	const editingId = editingDomain?.id ?? null;
	const defaultService = services[0] || '';

	useEffect(() => {
		if (!isOpen) return;
		let middlewares = '';
		try { const parsed = JSON.parse(editingDomain?.middlewares || '[]'); middlewares = Array.isArray(parsed) ? parsed.join(', ') : ''; } catch { middlewares = editingDomain?.middlewares || ''; }
		const cert = String(editingDomain?.certificate_type || 'LETSENCRYPT').toUpperCase();
		setState({host: editingDomain?.host || '', path: editingDomain?.path || '/', internalPath: editingDomain?.internal_path || '/', port: String(editingDomain?.port ?? 3000), serviceName: editingDomain?.service_name || defaultService, https: editingDomain?.https !== false, certificateType: cert === 'CUSTOM' || cert === 'NONE' ? cert : 'LETSENCRYPT', customResolver: editingDomain?.custom_cert_resolver || '', customEntrypoint: editingDomain?.custom_entrypoint || '', stripPath: Boolean(editingDomain?.strip_path), middlewares});
	// Initialize only when opening or switching the edited domain. Parent list
	// refreshes must never overwrite values while the user is typing.
	// eslint-disable-next-line react-hooks/exhaustive-deps
	}, [isOpen, editingId]);

	if (!isOpen || typeof document === 'undefined') return null;

	const submit = async (event: React.FormEvent) => {
		event.preventDefault();
		const normalizedPath = state.path.trim() || '/';
		const normalizedInternal = state.internalPath.trim() || '/';
		const numericPort = Number(state.port);
		if (!state.host.trim()) return toast.error('Host is required');
		if (!normalizedPath.startsWith('/') || !normalizedInternal.startsWith('/')) return toast.error('Paths must start with /');
		if (!Number.isInteger(numericPort) || numericPort < 1 || numericPort > 65535) return toast.error('Port must be between 1 and 65535');
		if (state.stripPath && normalizedPath === '/') return toast.error('Strip Path requires a non-root path');
		if (state.certificateType === 'CUSTOM' && !state.customResolver.trim()) return toast.error('Custom certificate resolver is required');
		setSaving(true);
		try {
			await onSave({
				host: state.host.trim(), path: normalizedPath, internal_path: normalizedInternal,
				strip_path: state.stripPath, port: numericPort, service_name: state.serviceName || undefined,
				https: state.https, certificate_type: state.https ? state.certificateType : 'NONE',
				custom_cert_resolver: state.certificateType === 'CUSTOM' ? state.customResolver.trim() : undefined,
				custom_entrypoint: state.customEntrypoint.trim() || undefined,
				middlewares: JSON.stringify(state.middlewares.split(',').map(value => value.trim()).filter(Boolean)),
			});
			onClose();
		} catch (error: any) { toast.error(error?.message || 'Unable to save domain'); }
		finally { setSaving(false); }
	};

	return createPortal(
		<div className="pointer-events-auto fixed inset-0 z-[1000000] flex items-center justify-center overflow-y-auto bg-black/60 p-4 backdrop-blur-sm">
			<div className="pointer-events-auto relative z-[1000001] my-8 w-full max-w-2xl rounded-xl bg-card shadow-2xl">
				<div className="flex items-center justify-between border-b border- bg-muted/30 p-4"><div className="flex items-center gap-2"><Globe className="size-4 text-primary" /><h3 className="text-sm font-bold">{editingDomain ? 'Edit Domain Route' : 'Add Domain Route'}</h3></div><Button variant="ghost" size="icon" onClick={onClose} className="size-7"><X className="size-4" /></Button></div>
				<form onSubmit={submit} className="flex flex-col gap-4 p-5">
					<DomainFormFields state={state} services={services} onChange={(key, value) => setState(current => ({...current, [key]: value}))} />
					<div className="flex justify-end border-t pt-4"><Button type="submit" disabled={saving}>{saving && <RefreshCw className="mr-2 size-3.5 animate-spin" />}{editingDomain ? 'Update Domain' : 'Add Domain'}</Button></div>
				</form>
			</div>
		</div>, document.body);
}
