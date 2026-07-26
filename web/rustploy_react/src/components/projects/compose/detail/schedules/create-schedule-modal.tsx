import {useState} from 'react';
import {RefreshCw, X, Clock} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {Label} from '#/components/ui/label';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import {toast} from 'sonner';

interface CreateScheduleModalProps {
	isOpen: boolean;
	onClose: () => void;
	servicesList: string[];
	onCreate: (data: {
		name: string;
		serviceName: string;
		command: string;
		cronExpr: string;
	}) => Promise<void>;
}

const CRON_PRESETS = [
	{label: 'Every 5m', value: '*/5 * * * *'},
	{label: 'Every 15m', value: '*/15 * * * *'},
	{label: 'Hourly', value: '0 * * * *'},
	{label: 'Daily', value: '0 0 * * *'},
];

export function CreateScheduleModal({isOpen, onClose, servicesList, onCreate}: CreateScheduleModalProps) {
	const [name, setName] = useState('');
	const [serviceName, setServiceName] = useState(servicesList[0] || 'app');
	const [command, setCommand] = useState('');
	const [cronExpr, setCronExpr] = useState('0 * * * *');
	const [creating, setCreating] = useState(false);

	if (!isOpen) return null;

	const handleSubmit = async (e: React.FormEvent) => {
		e.preventDefault();
		if (!name.trim() || !command.trim()) {
			toast.error('Schedule Name and Command are required');
			return;
		}

		setCreating(true);
		try {
			await onCreate({name, serviceName, command, cronExpr});
			onClose();
		} finally {
			setCreating(false);
		}
	};

	return (
		<div className="fixed inset-0 z-50 bg-black/60 backdrop-blur-sm flex items-center justify-center p-4">
			<div className="bg-card border border-border rounded-xl shadow-2xl w-full max-w-lg overflow-hidden animate-in fade-in duration-150">
				<div className="p-4 border-b border-border/40 flex items-center justify-between bg-muted/20">
					<div className="flex items-center gap-2.5">
						<div className="size-8 rounded-lg bg-primary/10 flex items-center justify-center text-primary border border-primary/20">
							<Clock className="w-4 h-4" />
						</div>
						<div>
							<h3 className="text-sm font-bold text-foreground">Create Compose Schedule Task</h3>
							<p className="text-[11px] text-muted-foreground">Automate commands on container services</p>
						</div>
					</div>
					<Button variant="ghost" size="icon" onClick={onClose} className="h-7 w-7 text-muted-foreground hover:text-foreground">
						<X className="w-4 h-4" />
					</Button>
				</div>

				<form onSubmit={handleSubmit} className="p-5 flex flex-col gap-4">
					<div className="flex flex-col gap-1.5">
						<Label className="text-xs font-semibold">Schedule Name *</Label>
						<Input
							value={name}
							onChange={e => setName(e.target.value)}
							placeholder="e.g., Hourly Cache Cleanup"
							className="h-9 text-xs"
						/>
					</div>

					<div className="grid grid-cols-2 gap-3">
						<div className="flex flex-col gap-1.5">
							<Label className="text-xs font-semibold">Target Service *</Label>
							<Select value={serviceName} onValueChange={setServiceName}>
								<SelectTrigger className="!h-9 text-xs">
									<SelectValue placeholder="Select service" />
								</SelectTrigger>
								<SelectContent className="bg-card border-border">
									{servicesList.map((srv) => (
										<SelectItem key={srv} value={srv} className="text-xs">
											{srv}
										</SelectItem>
									))}
								</SelectContent>
							</Select>
						</div>

						<div className="flex flex-col gap-1.5">
							<div className="flex items-center justify-between">
								<Label className="text-xs font-semibold">Cron Schedule *</Label>
							</div>
							<Input
								value={cronExpr}
								onChange={e => setCronExpr(e.target.value)}
								placeholder="0 * * * *"
								className="h-9 text-xs font-mono"
							/>
						</div>
					</div>

					{/* Quick Cron Presets */}
					<div className="flex items-center gap-1.5">
						<span className="text-[11px] text-muted-foreground font-medium">Quick Presets:</span>
						{CRON_PRESETS.map(preset => (
							<button
								key={preset.value}
								type="button"
								onClick={() => setCronExpr(preset.value)}
								className={`text-[10px] px-2 py-0.5 rounded font-mono transition-colors border ${
									cronExpr === preset.value
										? 'bg-primary text-primary-foreground border-primary'
										: 'bg-muted/40 text-muted-foreground border-border/40 hover:text-foreground hover:bg-muted'
								}`}
							>
								{preset.label}
							</button>
						))}
					</div>

					<div className="flex flex-col gap-1.5">
						<Label className="text-xs font-semibold">Command to Execute *</Label>
						<Input
							value={command}
							onChange={e => setCommand(e.target.value)}
							placeholder="e.g., npm run cron:clean"
							className="h-9 text-xs font-mono"
						/>
					</div>

					<div className="pt-3 flex items-center justify-end border-t border-border/30">
						<Button type="submit" disabled={creating} className="h-9 text-xs font-semibold bg-primary text-primary-foreground hover:bg-primary/95 px-5 shadow-sm w-full sm:w-auto">
							{creating ? <RefreshCw className="w-3.5 h-3.5 animate-spin mr-1" /> : null}
							Create Schedule
						</Button>
					</div>
				</form>
			</div>
		</div>
	);
}
