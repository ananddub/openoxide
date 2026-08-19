import {useState, useEffect} from 'react';
import {createPortal} from 'react-dom';
import {Globe, RefreshCw, X} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {Label} from '#/components/ui/label';
import {Switch} from '#/components/ui/switch';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import {toast} from 'sonner';

interface ComposeDomainModalProps {
	isOpen: boolean;
	onClose: () => void;
	editingDomain: any | null;
	servicesList: string[];
	onSave: (data: {
		domain: string;
		serviceName: string;
		containerPort: number;
		https: boolean;
		path: string;
	}) => Promise<void>;
}

export function ComposeDomainModal({
	isOpen,
	onClose,
	editingDomain,
	servicesList,
	onSave,
}: ComposeDomainModalProps) {
	const [domain, setDomain] = useState('');
	const [serviceName, setServiceName] = useState(servicesList[0] || 'app');
	const [containerPort, setContainerPort] = useState('3000');
	const [https, setHttps] = useState(true);
	const [path, setPath] = useState('/');
	const [saving, setSaving] = useState(false);

	useEffect(() => {
		if (editingDomain) {
			setDomain(editingDomain.host || editingDomain.domain || '');
			setServiceName(
				editingDomain.service_name || servicesList[0] || 'app',
			);
			setContainerPort(
				String(editingDomain.port ?? editingDomain.container_port ?? 3000),
			);
			setHttps(editingDomain.https !== false);
			setPath(editingDomain.path || '/');
		} else {
			setDomain('');
			setServiceName(servicesList[0] || 'app');
			setContainerPort('3000');
			setHttps(true);
			setPath('/');
		}
	}, [editingDomain, servicesList]);

	if (!isOpen || typeof document === 'undefined') return null;

	const handleSubmit = async (e: React.FormEvent) => {
		e.preventDefault();
		if (!domain.trim()) {
			toast.error('Domain host name is required');
			return;
		}

		setSaving(true);
		try {
			await onSave({
				domain: domain.trim(),
				serviceName,
				containerPort: parseInt(containerPort, 10) || 80,
				https,
				path,
			});
			onClose();
		} finally {
			setSaving(false);
		}
	};

	const modalJSX = (
		<div className="fixed inset-0 z-[999999] flex items-center justify-center bg-black/60 p-4 backdrop-blur-sm">
			<div className="w-full max-w-2xl animate-in overflow-hidden rounded-xl border border-border bg-card shadow-2xl duration-150 fade-in">
				<div className="flex items-center justify-between border-b border-border bg-muted/30 p-4">
					<div className="flex items-center gap-2">
						<Globe className="h-4 w-4 text-primary" />
						<h3 className="text-sm font-bold text-foreground">
							{editingDomain
								? 'Edit Compose Domain Route'
								: 'Add Compose Domain Route'}
						</h3>
					</div>
					<Button
						variant="ghost"
						size="icon"
						onClick={onClose}
						className="h-7 w-7 text-muted-foreground">
						<X className="h-4 w-4" />
					</Button>
				</div>

				<form onSubmit={handleSubmit} className="flex flex-col gap-4 p-5">
					<div className="flex flex-col gap-1.5">
						<Label className="text-xs font-semibold">Domain Name *</Label>
						<Input
							value={domain}
							onChange={e => setDomain(e.target.value)}
							placeholder="api.yourdomain.com"
							className="h-9 font-mono text-xs"
						/>
					</div>

					<div className="flex w-full flex-col gap-1.5">
						<div className="flex items-center justify-between">
							<Label className="text-xs font-semibold">
								Target Compose Service *
							</Label>
						</div>
						<div className="flex gap-2">
							<Input
								value={serviceName}
								onChange={e => setServiceName(e.target.value)}
								placeholder="app"
								className="h-9 flex-1 font-mono text-xs"
							/>
							<Select
								value={
									servicesList.includes(serviceName) ? serviceName : ''
								}
								onValueChange={val => val && setServiceName(val)}>
								<SelectTrigger className="h-9 w-[130px] text-xs">
									<SelectValue placeholder="Presets" />
								</SelectTrigger>
								<SelectContent>
									{Array.from(
										new Set(
											[...servicesList, serviceName].filter(Boolean),
										),
									).map(srv => (
										<SelectItem
											key={srv}
											value={srv}
											className="font-mono text-xs">
											{srv}
										</SelectItem>
									))}
								</SelectContent>
							</Select>
						</div>
					</div>

					<div className="flex w-full flex-col gap-1.5">
						<Label className="text-xs font-semibold">
							Container Port *
						</Label>
						<Input
							type="number"
							value={containerPort}
							onChange={e => setContainerPort(e.target.value)}
							placeholder="3000"
							className="h-9 w-full font-mono text-xs"
						/>
					</div>

					<div className="flex flex-col gap-1.5">
						<Label className="text-xs font-semibold">URL Path</Label>
						<Input
							value={path}
							onChange={e => setPath(e.target.value)}
							placeholder="/"
							className="h-9 font-mono text-xs"
						/>
					</div>

					<div className="flex items-center justify-between rounded-lg border border-border/60 bg-muted/20 p-3">
						<div>
							<Label className="text-xs font-semibold">
								Enable Automatic HTTPS / SSL
							</Label>
							<p className="text-[11px] text-muted-foreground">
								Issue Let's Encrypt SSL certificate via Traefik
							</p>
						</div>
						<Switch checked={https} onCheckedChange={setHttps} />
					</div>

					<div className="flex items-center justify-end border-t border-border pt-2">
						<Button
							type="submit"
							disabled={saving}
							className="h-9 w-full bg-primary px-6 text-xs font-bold text-primary-foreground shadow-md hover:bg-primary/90 sm:w-auto">
							{saving ? (
								<RefreshCw className="mr-1 h-3.5 w-3.5 animate-spin" />
							) : null}
							{editingDomain ? 'Update Domain' : 'Add Domain'}
						</Button>
					</div>
				</form>
			</div>
		</div>
	);

	return createPortal(modalJSX, document.body);
}
