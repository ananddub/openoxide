import {useState, useEffect} from 'react';
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
			setDomain(editingDomain.domain || '');
			setServiceName(editingDomain.service_name || servicesList[0] || 'app');
			setContainerPort(String(editingDomain.container_port || 3000));
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

	if (!isOpen) return null;

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
				containerPort: Number(containerPort) || 3000,
				https,
				path,
			});
			onClose();
		} finally {
			setSaving(false);
		}
	};

	return (
		<div className="fixed inset-0 z-50 bg-black/60 backdrop-blur-sm flex items-center justify-center p-4">
			<div className="bg-card border border-border rounded-xl shadow-2xl w-full max-w-lg overflow-hidden animate-in fade-in duration-150">
				<div className="p-4 border-b border-border flex items-center justify-between bg-muted/30">
					<div className="flex items-center gap-2">
						<Globe className="w-4 h-4 text-primary" />
						<h3 className="text-sm font-bold text-foreground">
							{editingDomain ? 'Edit Compose Domain Route' : 'Add Compose Domain Route'}
						</h3>
					</div>
					<Button variant="ghost" size="icon" onClick={onClose} className="h-7 w-7 text-muted-foreground">
						<X className="w-4 h-4" />
					</Button>
				</div>

				<form onSubmit={handleSubmit} className="p-5 flex flex-col gap-4">
					<div className="flex flex-col gap-1.5">
						<Label className="text-xs font-semibold">Domain Name *</Label>
						<Input
							value={domain}
							onChange={e => setDomain(e.target.value)}
							placeholder="api.yourdomain.com"
							className="h-9 text-xs font-mono"
						/>
					</div>

					<div className="grid grid-cols-2 gap-3">
						<div className="flex flex-col gap-1.5">
							<Label className="text-xs font-semibold">Target Compose Service *</Label>
							<Select value={serviceName} onValueChange={setServiceName}>
								<SelectTrigger className="h-9 text-xs">
									<SelectValue placeholder="Select service" />
								</SelectTrigger>
								<SelectContent>
									{servicesList.map((srv) => (
										<SelectItem key={srv} value={srv} className="text-xs">
											{srv}
										</SelectItem>
									))}
								</SelectContent>
							</Select>
						</div>

						<div className="flex flex-col gap-1.5">
							<Label className="text-xs font-semibold">Container Port *</Label>
							<Input
								type="number"
								value={containerPort}
								onChange={e => setContainerPort(e.target.value)}
								placeholder="3000"
								className="h-9 text-xs font-mono"
							/>
						</div>
					</div>

					<div className="flex flex-col gap-1.5">
						<Label className="text-xs font-semibold">URL Path</Label>
						<Input
							value={path}
							onChange={e => setPath(e.target.value)}
							placeholder="/"
							className="h-9 text-xs font-mono"
						/>
					</div>

					<div className="flex items-center justify-between border border-border/60 rounded-lg p-3 bg-muted/20">
						<div>
							<Label className="text-xs font-semibold">Enable Automatic HTTPS / SSL</Label>
							<p className="text-[11px] text-muted-foreground">Issue Let's Encrypt SSL certificate via Traefik</p>
						</div>
						<Switch checked={https} onCheckedChange={setHttps} />
					</div>

					<div className="pt-2 flex items-center justify-end gap-2 border-t border-border">
						<Button type="button" variant="outline" onClick={onClose} className="h-8 text-xs font-semibold">
							Cancel
						</Button>
						<Button type="submit" disabled={saving} className="h-8 text-xs font-semibold">
							{saving ? <RefreshCw className="w-3.5 h-3.5 animate-spin mr-1" /> : null}
							{editingDomain ? 'Update Domain' : 'Add Domain'}
						</Button>
					</div>
				</form>
			</div>
		</div>
	);
}
