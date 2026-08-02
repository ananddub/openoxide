import {useState, useEffect} from 'react';
import {createPortal} from 'react-dom';
import {Database, RefreshCw, X} from 'lucide-react';
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

interface CreateBackupModalProps {
	isOpen: boolean;
	onClose: () => void;
	servicesList: string[];
	defaultServiceName?: string;
	defaultVolumeName?: string;
	hideServiceAndVolumeSelect?: boolean;
	editingBackup?: any;
	onCreate: (data: {
		name: string;
		serviceName: string;
		volumeName: string;
		cronExpr: string;
		prefix: string;
		turnOff: boolean;
	}) => Promise<void>;
}

export function CreateBackupModal({
	isOpen,
	onClose,
	servicesList,
	defaultServiceName,
	defaultVolumeName,
	hideServiceAndVolumeSelect,
	editingBackup,
	onCreate,
}: CreateBackupModalProps) {
	const [name, setName] = useState('');
	const [serviceName, setServiceName] = useState(defaultServiceName || servicesList[0] || 'app');
	const [volumeName, setVolumeName] = useState(defaultVolumeName || 'data');
	const [cronExpr, setCronExpr] = useState('0 2 * * *');
	const [prefix, setPrefix] = useState('');
	const [turnOff, setTurnOff] = useState(false);
	const [saving, setSaving] = useState(false);

	useEffect(() => {
		if (editingBackup) {
			setName(editingBackup.name || '');
			setServiceName(editingBackup.service_name || defaultServiceName || servicesList[0] || 'app');
			setVolumeName(editingBackup.volume_name || defaultVolumeName || 'data');
			setCronExpr(editingBackup.cron_expression || '0 2 * * *');
			setPrefix(editingBackup.backup_prefix || '');
			setTurnOff(editingBackup.stop_container_during_backup === 1);
		} else {
			if (defaultServiceName) setServiceName(defaultServiceName);
			if (defaultVolumeName) setVolumeName(defaultVolumeName);
		}
	}, [editingBackup, defaultServiceName, defaultVolumeName, servicesList]);

	if (!isOpen || typeof document === 'undefined') return null;

	const handleSubmit = async (e: React.FormEvent) => {
		e.preventDefault();
		if (!name.trim()) {
			toast.error('Backup name is required');
			return;
		}

		setSaving(true);
		try {
			await onCreate({
				name: name.trim(),
				serviceName,
				volumeName: volumeName.trim(),
				cronExpr: cronExpr.trim(),
				prefix: prefix.trim(),
				turnOff,
			});
			onClose();
		} finally {
			setSaving(false);
		}
	};

	const modalJSX = (
		<div className="fixed inset-0 z-[999999] bg-black/60 backdrop-blur-sm flex items-center justify-center p-4">
			<div className="bg-card border border-border rounded-xl shadow-2xl w-full max-w-2xl overflow-hidden animate-in fade-in duration-150">
				<div className="p-4 border-b border-border flex items-center justify-between bg-muted/30">
					<div className="flex items-center gap-2">
						<Database className="w-4 h-4 text-primary" />
						<h3 className="text-sm font-bold text-foreground">
							{editingBackup ? 'Edit Volume Backup Rule' : 'Create Compose Volume Backup'}
						</h3>
					</div>
					<Button variant="ghost" size="icon" onClick={onClose} className="h-7 w-7 text-muted-foreground">
						<X className="w-4 h-4" />
					</Button>
				</div>

				<form onSubmit={handleSubmit} className="p-5 flex flex-col gap-4">
					<div className="flex flex-col gap-1.5">
						<Label className="text-xs font-semibold">Backup Rule Name *</Label>
						<Input
							value={name}
							onChange={e => setName(e.target.value)}
							placeholder="Daily Postgres Data Backup"
							className="h-9 text-xs"
						/>
					</div>

					{!hideServiceAndVolumeSelect && (
						<div className="grid grid-cols-1 sm:grid-cols-2 gap-4 w-full">
							<div className="flex flex-col gap-1.5 w-full">
								<Label className="text-xs font-semibold">Service *</Label>
								<Select value={serviceName} onValueChange={v => v && setServiceName(v)}>
									<SelectTrigger className="h-9 text-xs w-full font-semibold bg-muted/20 border-border">
										<SelectValue placeholder="Select service" />
									</SelectTrigger>
									<SelectContent className="bg-card border-border">
										{servicesList.map(srv => (
											<SelectItem key={srv} value={srv} className="text-xs font-semibold">{srv}</SelectItem>
										))}
									</SelectContent>
								</Select>
							</div>

							<div className="flex flex-col gap-1.5 w-full">
								<Label className="text-xs font-semibold">Volume Name *</Label>
								<Input
									value={volumeName}
									onChange={e => setVolumeName(e.target.value)}
									placeholder="postgres_data"
									className="h-9 text-xs font-mono w-full"
								/>
							</div>
						</div>
					)}

					<div className="flex flex-col gap-1.5">
						<Label className="text-xs font-semibold">Cron Schedule (UTC) *</Label>
						<Input
							value={cronExpr}
							onChange={e => setCronExpr(e.target.value)}
							placeholder="0 2 * * * (Daily at 2 AM)"
							className="h-9 text-xs font-mono"
						/>
					</div>

					<div className="flex flex-col gap-1.5">
						<Label className="text-xs font-semibold">Backup File Prefix</Label>
						<Input
							value={prefix}
							onChange={e => setPrefix(e.target.value)}
							placeholder="db_backup"
							className="h-9 text-xs font-mono"
						/>
					</div>

					<div className="flex items-center justify-between border border-border/60 rounded-lg p-3 bg-muted/20">
						<div>
							<Label className="text-xs font-semibold">Stop Container During Backup</Label>
							<p className="text-[11px] text-muted-foreground">Ensures zero database corruption during snapshot</p>
						</div>
						<Switch checked={turnOff} onCheckedChange={setTurnOff} />
					</div>

					<div className="pt-2 flex items-center justify-end border-t border-border">
						<Button type="submit" disabled={saving} className="w-full sm:w-auto h-9 px-6 font-bold text-xs bg-primary hover:bg-primary/90 text-primary-foreground shadow-md">
							{saving ? <RefreshCw className="w-3.5 h-3.5 animate-spin mr-1" /> : null}
							{editingBackup ? 'Update Backup Rule' : 'Schedule Backup'}
						</Button>
					</div>
				</form>
			</div>
		</div>
	);

	return createPortal(modalJSX, document.body);
}
