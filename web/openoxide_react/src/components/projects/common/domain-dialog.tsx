import {useEffect, useState} from 'react';
import {Loader2} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {
	Sheet,
	SheetContent,
	SheetDescription,
	SheetFooter,
	SheetHeader,
	SheetTitle,
} from '#/components/ui/sheet';
import {toast} from 'sonner';
import {
	StepTimeline,
	type WizardStep,
} from '#/components/shared/sheet-wizard';
import {
	DomainRoutingFields,
	DomainTargetFields,
	DomainTlsFields,
	type DomainFormState,
} from './domain-form-fields';

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

const STEPS: WizardStep[] = [
	{id: 'target', label: 'Target'},
	{id: 'routing', label: 'Routing'},
	{id: 'tls', label: 'Security'},
];

const STEP_DESC: Record<string, string> = {
	target: 'The host people open, and the container behind it.',
	routing: 'How that URL maps onto the path the service expects.',
	tls: 'HTTPS, certificates and any Traefik extras.',
};

const EMPTY: DomainFormState = {
	host: '',
	path: '/',
	internalPath: '/',
	port: '3000',
	serviceName: '',
	https: true,
	certificateType: 'LETSENCRYPT',
	customResolver: '',
	customEntrypoint: '',
	stripPath: false,
	middlewares: '',
};

interface DomainDialogProps {
	isOpen: boolean;
	onClose: () => void;
	editingDomain?: any | null;
	services?: string[];
	onSave: (data: DomainFormData) => Promise<void>;
}

/** Add / edit a domain route as a checkpoint wizard. */
export function DomainDialog({
	isOpen,
	onClose,
	editingDomain,
	services = [],
	onSave,
}: DomainDialogProps) {
	const [state, setState] = useState<DomainFormState>(EMPTY);
	const [stepIdx, setStepIdx] = useState(0);
	const [saving, setSaving] = useState(false);
	const editing = !!editingDomain;
	const editingId = editingDomain?.id ?? null;
	const defaultService = services[0] || '';

	useEffect(() => {
		if (!isOpen) return;
		setStepIdx(0);
		let middlewares = '';
		try {
			const parsed = JSON.parse(editingDomain?.middlewares || '[]');
			middlewares = Array.isArray(parsed) ? parsed.join(', ') : '';
		} catch {
			middlewares = editingDomain?.middlewares || '';
		}
		const cert = String(
			editingDomain?.certificate_type || 'LETSENCRYPT',
		).toUpperCase();
		setState({
			host: editingDomain?.host || '',
			path: editingDomain?.path || '/',
			internalPath: editingDomain?.internal_path || '/',
			port: String(editingDomain?.port ?? 3000),
			serviceName: editingDomain?.service_name || defaultService,
			https: editingDomain?.https !== false,
			certificateType:
				cert === 'CUSTOM' || cert === 'NONE' ? cert : 'LETSENCRYPT',
			customResolver: editingDomain?.custom_cert_resolver || '',
			customEntrypoint: editingDomain?.custom_entrypoint || '',
			stripPath: Boolean(editingDomain?.strip_path),
			middlewares,
		});
		// Initialize only when opening or switching the edited domain. Parent list refreshes must never overwrite values while the user is typing. eslint-disable-next-line react-hooks/exhaustive-deps
	}, [isOpen, editingId]);

	const patch = <K extends keyof DomainFormState>(
		key: K,
		value: DomainFormState[K],
	) => setState(current => ({...current, [key]: value}));

	const path = state.path.trim() || '/';
	const internalPath = state.internalPath.trim() || '/';
	const port = Number(state.port);
	const step = STEPS[stepIdx].id;
	const last = stepIdx === STEPS.length - 1;

	const targetReady =
		!!state.host.trim() &&
		Number.isInteger(port) &&
		port >= 1 &&
		port <= 65535 &&
		(services.length === 0 || !!state.serviceName);
	const routingReady =
		path.startsWith('/') &&
		internalPath.startsWith('/') &&
		(!state.stripPath || path !== '/');
	const tlsReady =
		state.certificateType !== 'CUSTOM' || !!state.customResolver.trim();
	const formReady = targetReady && routingReady && tlsReady;
	const stepReady =
		step === 'target'
			? targetReady
			: step === 'routing'
				? routingReady
				: tlsReady;

	const submit = async () => {
		setSaving(true);
		try {
			await onSave({
				host: state.host.trim(),
				path,
				internal_path: internalPath,
				strip_path: state.stripPath,
				port,
				service_name: state.serviceName || undefined,
				https: state.https,
				certificate_type: state.https ? state.certificateType : 'NONE',
				custom_cert_resolver:
					state.certificateType === 'CUSTOM'
						? state.customResolver.trim()
						: undefined,
				custom_entrypoint: state.customEntrypoint.trim() || undefined,
				middlewares: JSON.stringify(
					state.middlewares
						.split(',')
						.map(value => value.trim())
						.filter(Boolean),
				),
			});
			onClose();
		} catch (error: any) {
			toast.error(error?.message || 'Unable to save domain');
		} finally {
			setSaving(false);
		}
	};

	const next = () => setStepIdx(i => Math.min(i + 1, STEPS.length - 1));

	return (
		<Sheet open={isOpen} onOpenChange={open => !open && onClose()}>
			<SheetContent>
				<SheetHeader>
					<SheetTitle className="pr-8">
						{editingDomain ? 'Edit Domain Route' : 'Add Domain Route'}
					</SheetTitle>
					<SheetDescription>
						{editing
							? 'Everything on one page — change only what you need.'
							: STEP_DESC[step]}
					</SheetDescription>
				</SheetHeader>

				<div className="flex min-h-0 flex-1 flex-col overflow-y-auto px-6 py-6">
					{editing ? (
						<div className="flex flex-col gap-6">
							<DomainTargetFields
								state={state}
								services={services}
								onChange={patch}
							/>
							<DomainRoutingFields
								state={state}
								services={services}
								onChange={patch}
							/>
							<DomainTlsFields
								state={state}
								services={services}
								onChange={patch}
							/>
						</div>
					) : (
						<StepTimeline steps={STEPS} idx={stepIdx}>
							<div key={step}>
								{step === 'target' && (
									<DomainTargetFields
										state={state}
										services={services}
										onChange={patch}
									/>
								)}
								{step === 'routing' && (
									<DomainRoutingFields
										state={state}
										services={services}
										onChange={patch}
									/>
								)}
								{step === 'tls' && (
									<DomainTlsFields
										state={state}
										services={services}
										onChange={patch}
									/>
								)}
							</div>
						</StepTimeline>
					)}
				</div>

				{editing ? (
					<SheetFooter className="justify-between">
						<p className="self-center text-xs text-muted-foreground">
							{state.host.trim() || 'host'}
							{path !== '/' ? path : ''}
						</p>
						<Button
							onClick={() => void submit()}
							disabled={saving || !formReady}
							className="h-10 gap-1.5 px-5">
							{saving && <Loader2 className="size-3.5 animate-spin" />}
							Update Domain
						</Button>
					</SheetFooter>
				) : last ? (
					<SheetFooter className="justify-between">
						<Button
							variant="outline"
							onClick={() => setStepIdx(i => i - 1)}>
							Back
						</Button>
						<Button
							onClick={() => void submit()}
							disabled={saving || !stepReady}
							className="h-10 gap-1.5 px-5">
							{saving && <Loader2 className="size-3.5 animate-spin" />}
							{editingDomain ? 'Update Domain' : 'Add Domain'}
						</Button>
					</SheetFooter>
				) : (
					<SheetFooter>
						{stepIdx > 0 && (
							<Button
								variant="outline"
								onClick={() => setStepIdx(i => i - 1)}>
								Back
							</Button>
						)}
						<Button disabled={!stepReady} onClick={next}>
							Continue
						</Button>
					</SheetFooter>
				)}
			</SheetContent>
		</Sheet>
	);
}
