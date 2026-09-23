import {useState, useEffect} from 'react';
import {Database, RefreshCw} from 'lucide-react';
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
import {useDestinationList} from '#/api/live';
import {BackupFormFields} from './backup-form-fields';

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
	const [cronExpr, setCronExpr] = useState('');
	const [prefix, setPrefix] = useState('');
	const [turnOff, setTurnOff] = useState(false);
	// Live destinations query; the table-watch stream (destinations table)
	// keeps it fresh.
	const destinations = useDestinationList().data ?? [];
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

	if (typeof document === 'undefined') return null;

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

	return (
		<Sheet open={isOpen} onOpenChange={open => !open && onClose()}>
			<SheetContent>
				<SheetHeader>
					<SheetTitle className="flex items-center gap-2">
						<Database className="size-4 text-primary" />
						{editingBackup
							? 'Edit Volume Backup Rule'
							: 'Create Compose Volume Backup'}
					</SheetTitle>
					<SheetDescription>
						Schedule recurring volume backups to an S3 destination.
					</SheetDescription>
				</SheetHeader>

				<form
					id="create-backup-form"
					onSubmit={handleSubmit}
					className="flex flex-1 flex-col gap-4 overflow-y-auto px-6 py-5">
					<BackupFormFields
						destinations={destinations}
						servicesList={servicesList}
						destinationId={destinationId}
						onDestinationId={setDestinationId}
						name={name}
						onName={setName}
						serviceName={serviceName}
						onServiceName={setServiceName}
						volumeName={volumeName}
						onVolumeName={setVolumeName}
						cronExpr={cronExpr}
						onCronExpr={setCronExpr}
						prefix={prefix}
						onPrefix={setPrefix}
						turnOff={turnOff}
						onTurnOff={setTurnOff}
						hideServiceAndVolumeSelect={hideServiceAndVolumeSelect}
					/>
				</form>

				<SheetFooter>
					<Button type="button" variant="outline" onClick={onClose}>
						Cancel
					</Button>
					<Button
						type="submit"
						form="create-backup-form"
						disabled={saving}
						className="gap-1.5">
						{saving ? (
							<RefreshCw className="size-3.5 animate-spin" />
						) : null}
						{editingBackup ? 'Update Backup Rule' : 'Schedule Backup'}
					</Button>
				</SheetFooter>
			</SheetContent>
		</Sheet>
	);
}
