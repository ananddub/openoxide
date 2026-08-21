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
import {useAppStore} from '#/stores/app-store';

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
		destinationId: number;
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
	const [serviceName, setServiceName] = useState(
		defaultServiceName || servicesList[0] || 'app',
	);
	const [volumeName, setVolumeName] = useState(
		defaultVolumeName || 'data',
	);
	const [cronExpr, setCronExpr] = useState('0 2 * * *');
	const [prefix, setPrefix] = useState('');
	const [turnOff, setTurnOff] = useState(false);
	const destinations = useAppStore(state => state.destinations || []);
	const [destinationId, setDestinationId] = useState('');
	const [saving, setSaving] = useState(false);

	useEffect(() => {
		if (editingBackup) {
			setName(editingBackup.name || '');
			setServiceName(
				editingBackup.service_name ||
					defaultServiceName ||
					servicesList[0] ||
					'app',
			);
			setVolumeName(
				editingBackup.volume_name || defaultVolumeName || 'data',
			);
			setCronExpr(editingBackup.cron_expression || '0 2 * * *');
			setPrefix(editingBackup.backup_prefix || '');
			setTurnOff(editingBackup.stop_container_during_backup === 1);
		} else {
			if (defaultServiceName) setServiceName(defaultServiceName);
			if (defaultVolumeName) setVolumeName(defaultVolumeName);
		}
	}, [editingBackup, defaultServiceName, defaultVolumeName, servicesList]);

	useEffect(() => {
		if (!destinationId && destinations.length > 0) {
			setDestinationId(String(destinations[0].id));
		}
	}, [destinationId, destinations]);

	if (!isOpen || typeof document === 'undefined') return null;

	const handleSubmit = async (e: React.FormEvent) => {
		e.preventDefault();
		if (!name.trim() || !volumeName.trim() || !destinationId) {
			toast.error('Backup name, volume, and S3 destination are required');
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
				destinationId: Number(destinationId),
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
						<Database className="h-4 w-4 text-primary" />
						<h3 className="text-sm font-bold text-foreground">
							{editingBackup
								? 'Edit Volume Backup Rule'
								: 'Create Compose Volume Backup'}
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
						<Label className="text-xs font-semibold">S3 Destination *</Label>
						<Select value={destinationId} onValueChange={value => value && setDestinationId(value)}>
							<SelectTrigger className="h-9 w-full text-xs">
								<SelectValue placeholder="Select S3 destination" />
							</SelectTrigger>
							<SelectContent>
								{destinations.map((destination: any) => (
									<SelectItem key={destination.id} value={String(destination.id)}>
										{destination.name} ({destination.bucket})
									</SelectItem>
								))}
							</SelectContent>
						</Select>
					</div>

					<div className="flex flex-col gap-1.5">
						<Label className="text-xs font-semibold">
							Backup Rule Name *
						</Label>
						<Input
							value={name}
							onChange={e => setName(e.target.value)}
							placeholder="Daily Postgres Data Backup"
							className="h-9 text-xs"
						/>
					</div>

					{!hideServiceAndVolumeSelect && (
						<div className="grid w-full grid-cols-1 gap-4 sm:grid-cols-2">
							<div className="flex w-full flex-col gap-1.5">
								<Label className="text-xs font-semibold">Service *</Label>
								<Select
									value={serviceName}
									onValueChange={v => v && setServiceName(v)}>
									<SelectTrigger className="h-9 w-full border-border bg-muted/20 text-xs font-semibold">
										<SelectValue placeholder="Select service" />
									</SelectTrigger>
									<SelectContent className="border-border bg-card">
										{servicesList.map(srv => (
											<SelectItem
												key={srv}
												value={srv}
												className="text-xs font-semibold">
												{srv}
											</SelectItem>
										))}
									</SelectContent>
								</Select>
							</div>

							<div className="flex w-full flex-col gap-1.5">
								<Label className="text-xs font-semibold">
									Volume Name *
								</Label>
								<Input
									value={volumeName}
									onChange={e => setVolumeName(e.target.value)}
									placeholder="postgres_data"
									className="h-9 w-full font-mono text-xs"
								/>
							</div>
						</div>
					)}

					<div className="flex flex-col gap-1.5">
						<Label className="text-xs font-semibold">
							Cron Schedule (UTC) *
						</Label>
						<Input
							value={cronExpr}
							onChange={e => setCronExpr(e.target.value)}
							placeholder="0 2 * * * (Daily at 2 AM)"
							className="h-9 font-mono text-xs"
						/>
					</div>

					<div className="flex flex-col gap-1.5">
						<Label className="text-xs font-semibold">
							Backup File Prefix
						</Label>
						<Input
							value={prefix}
							onChange={e => setPrefix(e.target.value)}
							placeholder="db_backup"
							className="h-9 font-mono text-xs"
						/>
					</div>

					<div className="flex items-center justify-between rounded-lg border border-border/60 bg-muted/20 p-3">
						<div>
							<Label className="text-xs font-semibold">
								Stop Container During Backup
							</Label>
							<p className="text-[11px] text-muted-foreground">
								Ensures zero database corruption during snapshot
							</p>
						</div>
						<Switch checked={turnOff} onCheckedChange={setTurnOff} />
					</div>

					<div className="flex items-center justify-end border-t border-border pt-2">
						<Button
							type="submit"
							disabled={saving}
							className="h-9 w-full bg-primary px-6 text-xs font-bold text-primary-foreground shadow-md hover:bg-primary/90 sm:w-auto">
							{saving ? (
								<RefreshCw className="mr-1 h-3.5 w-3.5 animate-spin" />
							) : null}
							{editingBackup ? 'Update Backup Rule' : 'Schedule Backup'}
						</Button>
					</div>
				</form>
			</div>
		</div>
	);

	return createPortal(modalJSX, document.body);
}
