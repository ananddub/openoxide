import {useState} from 'react';
import {
	Dialog,
	DialogContent,
	DialogHeader,
	DialogTitle,
	DialogFooter,
} from '#/components/ui/dialog';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {Label} from '#/components/ui/label';
import {Switch} from '#/components/ui/switch';
import {useAppStore} from '#/stores/app-store';
import {Select, SelectContent, SelectItem, SelectTrigger, SelectValue} from '#/components/ui/select';
import {toast} from 'sonner';
import {$api} from '#/api/query';
import {formatApiError} from '#/api/utils';

interface CreateVolumeBackupModalProps {
	open: boolean;
	onOpenChange: (open: boolean) => void;
	app: any;
	onSuccess: () => void;
}

export function CreateVolumeBackupModal({
	open,
	onOpenChange,
	app,
	onSuccess,
}: CreateVolumeBackupModalProps) {
	const [name, setName] = useState('');
	const [volumeName, setVolumeName] = useState('');
	const [cronExpr, setCronExpr] = useState('0 0 * * *');
	const [prefix, setPrefix] = useState('volume-backups/');
	const destinations = useAppStore(state => state.destinations || []);
	const [destinationId, setDestinationId] = useState('');
	const [turnOff, setTurnOff] = useState(false);
	const [isSubmitting, setIsSubmitting] = useState(false);

	const createMutation = $api.useMutation('post', '/backups/volume');

	const handleSubmit = async (e: React.FormEvent) => {
		e.preventDefault();
		if (!name.trim() || !volumeName.trim() || !destinationId) {
			toast.error('Name, volume, and S3 destination are required');
			return;
		}

		setIsSubmitting(true);
		try {
			await createMutation.mutateAsync({
				body: {
					name: name.trim(),
					volume_name: volumeName.trim(),
					cron_expression: cronExpr.trim(),
					prefix: prefix.trim(),
					destination_id: Number(destinationId),
					organization_id: app?.organization_id || 1,
					application_id: app?.id,
					app_name: app?.app_name || app?.name || 'app',
					service_type: 'standalone',
					turn_off: turnOff ? 1 : 0,
				},
			});
			toast.success('Volume backup configuration created successfully');
			onSuccess();
			onOpenChange(false);
			setName('');
			setVolumeName('');
		} catch (err: any) {
			toast.error(formatApiError(err));
		} finally {
			setIsSubmitting(false);
		}
	};

	return (
		<Dialog open={open} onOpenChange={onOpenChange}>
			<DialogContent className="sm:max-w-[480px]">
				<DialogHeader>
					<DialogTitle>Configure Volume Backup</DialogTitle>
				</DialogHeader>
				<form onSubmit={handleSubmit} className="space-y-4 py-2">
					<div className="space-y-1.5">
						<Label>S3 Destination *</Label>
						<Select value={destinationId} onValueChange={value => value && setDestinationId(value)}>
							<SelectTrigger><SelectValue placeholder="Select S3 destination" /></SelectTrigger>
							<SelectContent>
								{destinations.map((destination: any) => (
									<SelectItem key={destination.id} value={String(destination.id)}>
										{destination.name} ({destination.bucket})
									</SelectItem>
								))}
							</SelectContent>
						</Select>
					</div>

					<div className="space-y-1.5">
						<Label htmlFor="vol-name">Backup Rule Name</Label>
						<Input
							id="vol-name"
							placeholder="e.g. daily-app-data-backup"
							value={name}
							onChange={e => setName(e.target.value)}
							required
						/>
					</div>

					<div className="space-y-1.5">
						<Label htmlFor="vol-path">Volume Name / Mount Path</Label>
						<Input
							id="vol-path"
							placeholder="e.g. app_data or /var/lib/data"
							value={volumeName}
							onChange={e => setVolumeName(e.target.value)}
							required
						/>
					</div>

					<div className="space-y-1.5">
						<Label htmlFor="vol-cron" className="text-xs font-semibold">
							Cron Schedule *
						</Label>
						<Input
							id="vol-cron"
							placeholder="0 0 * * *"
							value={cronExpr}
							onChange={e => setCronExpr(e.target.value)}
							required
							className="h-9 font-mono text-xs"
						/>
					</div>

					<div className="space-y-1.5">
						<Label htmlFor="vol-prefix" className="text-xs font-semibold">
							S3 Prefix Path
						</Label>
						<Input
							id="vol-prefix"
							placeholder="volume-backups/"
							value={prefix}
							onChange={e => setPrefix(e.target.value)}
							className="h-9 font-mono text-xs"
						/>
					</div>

					<div className="flex items-center justify-between rounded-lg border border-border bg-muted/20 p-3">
						<div className="space-y-0.5">
							<Label
								htmlFor="turn-off-switch"
								className="cursor-pointer text-sm font-semibold">
								Pause App During Snapshot
							</Label>
							<p className="text-[11px] text-muted-foreground">
								Temporarily pause container to ensure safe consistent tar
								snapshot
							</p>
						</div>
						<Switch
							id="turn-off-switch"
							checked={turnOff}
							onCheckedChange={setTurnOff}
						/>
					</div>

					<DialogFooter className="pt-3">
						<Button
							type="submit"
							disabled={isSubmitting}
							className="h-9 w-full bg-primary px-6 text-xs font-bold text-primary-foreground shadow-md hover:bg-primary/90 sm:w-auto">
							{isSubmitting ? 'Creating...' : 'Create Backup Rule'}
						</Button>
					</DialogFooter>
				</form>
			</DialogContent>
		</Dialog>
	);
}
