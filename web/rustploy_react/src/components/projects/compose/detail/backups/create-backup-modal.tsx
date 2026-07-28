import {useState} from 'react';
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
	onCreate: (data: {
		name: string;
		serviceName: string;
		volumeName: string;
		cronExpr: string;
		prefix: string;
		turnOff: boolean;
	}) => Promise<void>;
}

export function CreateBackupModal({isOpen, onClose, servicesList, onCreate}: CreateBackupModalProps) {
	const [name, setName] = useState('');
	const [serviceName, setServiceName] = useState(servicesList[0] || 'app');
	const [volumeName, setVolumeName] = useState('');
	const [cronExpr, setCronExpr] = useState('0 0 * * *');
	const [prefix, setPrefix] = useState('volume-backups/');
	const [turnOff, setTurnOff] = useState(false);
	const [creating, setCreating] = useState(false);

	if (!isOpen) return null;

	const handleSubmit = async (e: React.FormEvent) => {
		e.preventDefault();
		if (!name.trim() || !volumeName.trim()) {
			toast.error('Rule Name and Volume Name are required');
			return;
		}

		setCreating(true);
		try {
			await onCreate({name, serviceName, volumeName, cronExpr, prefix, turnOff});
			onClose();
		} finally {
			setCreating(false);
		}
	};

	return (
		<div className="fixed inset-0 z-50 bg-black/60 backdrop-blur-sm flex items-center justify-center p-4">
			<div className="bg-card border border-border rounded-xl shadow-2xl w-full max-w-lg overflow-hidden animate-in fade-in duration-150">
				<div className="p-4 border-b border-border flex items-center justify-between bg-muted/30">
					<div className="flex items-center gap-2">
						<Database className="w-4 h-4 text-primary" />
						<h3 className="text-sm font-bold text-foreground">Create Compose Volume Backup Rule</h3>
					</div>
					<Button variant="ghost" size="icon" onClick={onClose} className="h-7 w-7 text-muted-foreground">
						<X className="w-4 h-4" />
					</Button>
				</div>

				<form onSubmit={handleSubmit} className="p-5 flex flex-col gap-4">
					<div className="flex flex-col gap-1.5">
						<Label className="text-xs font-semibold">Rule Name *</Label>
						<Input
							value={name}
							onChange={e => setName(e.target.value)}
							placeholder="e.g., Daily DB Volume Snapshots"
							className="h-9 text-xs"
						/>
					</div>

					<div className="grid grid-cols-2 gap-3">
						<div className="flex flex-col gap-1.5">
							<Label className="text-xs font-semibold">Target Service *</Label>
							<Select value={serviceName} onValueChange={val => val && setServiceName(val)}>
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
							<Label className="text-xs font-semibold">Volume Name / Path *</Label>
							<Input
								value={volumeName}
								onChange={e => setVolumeName(e.target.value)}
								placeholder="e.g., postgres_data"
								className="h-9 text-xs font-mono"
							/>
						</div>
					</div>

					<div className="grid grid-cols-2 gap-3">
						<div className="flex flex-col gap-1.5">
							<Label className="text-xs font-semibold">Cron Schedule *</Label>
							<Input
								value={cronExpr}
								onChange={e => setCronExpr(e.target.value)}
								placeholder="0 0 * * *"
								className="h-9 text-xs font-mono"
							/>
						</div>

						<div className="flex flex-col gap-1.5">
							<Label className="text-xs font-semibold">S3 Prefix</Label>
							<Input
								value={prefix}
								onChange={e => setPrefix(e.target.value)}
								placeholder="volume-backups/"
								className="h-9 text-xs font-mono"
							/>
						</div>
					</div>

					<div className="flex items-center justify-between border border-border/60 rounded-lg p-3 bg-muted/20">
						<div>
							<Label className="text-xs font-semibold">Pause Container on Backup</Label>
							<p className="text-[11px] text-muted-foreground">Stop container briefly to ensure consistent data snapshot</p>
						</div>
						<Switch checked={turnOff} onCheckedChange={setTurnOff} />
					</div>

					<div className="pt-2 flex items-center justify-end gap-2 border-t border-border">
						<Button type="button" variant="outline" onClick={onClose} className="h-8 text-xs font-semibold">
							Cancel
						</Button>
						<Button type="submit" disabled={creating} className="h-8 text-xs font-semibold">
							{creating ? <RefreshCw className="w-3.5 h-3.5 animate-spin mr-1" /> : null}
							Create Backup Rule
						</Button>
					</div>
				</form>
			</div>
		</div>
	);
}
