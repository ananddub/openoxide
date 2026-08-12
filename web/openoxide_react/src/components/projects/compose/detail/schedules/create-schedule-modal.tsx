import {useState, useEffect} from 'react';
import {createPortal} from 'react-dom';
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
	editingSchedule?: any;
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

export function CreateScheduleModal({isOpen, onClose, servicesList, editingSchedule, onCreate}: CreateScheduleModalProps) {
	const [name, setName] = useState('');
	const [serviceName, setServiceName] = useState(servicesList[0] || 'app');
	const [command, setCommand] = useState('');
	const [cronExpr, setCronExpr] = useState('0 * * * *');
	const [creating, setCreating] = useState(false);

	useEffect(() => {
		if (editingSchedule) {
			setName(editingSchedule.name || '');
			setServiceName(editingSchedule.service_name || editingSchedule.target || editingSchedule.app_name || servicesList[0] || 'app');
			setCommand(editingSchedule.command || '');
			setCronExpr(editingSchedule.cron_expression || editingSchedule.cronExpr || '0 * * * *');
		} else {
			setName('');
			setServiceName(servicesList[0] || 'app');
			setCommand('');
			setCronExpr('0 * * * *');
		}
	}, [editingSchedule, servicesList]);

	if (!isOpen || typeof document === 'undefined') return null;

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

	const modalJSX = (
		<div className="fixed inset-0 z-[999999] bg-black/60 backdrop-blur-sm flex items-center justify-center p-4">
			<div className="bg-card border border-border rounded-xl shadow-2xl w-full max-w-3xl overflow-hidden animate-in fade-in duration-150">
				<div className="p-4 border-b border-border/40 flex items-center justify-between bg-muted/20">
					<div className="flex items-center gap-2.5">
						<div className="size-8 rounded-lg bg-primary/10 flex items-center justify-center text-primary border border-primary/20">
							<Clock className="w-4 h-4" />
						</div>
						<div>
							<h3 className="text-sm font-bold text-foreground">
								{editingSchedule ? 'Edit Schedule / Cron Task' : 'Create Compose Schedule Task'}
							</h3>
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

					<div className="flex flex-col gap-1.5 w-full">
						<Label className="text-xs font-semibold">Target Service *</Label>
						<Select value={serviceName} onValueChange={val => val && setServiceName(val)}>
							<SelectTrigger className="!h-9 text-xs w-full">
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
						<Label className="text-xs font-semibold">Command to Run *</Label>
						<Input
							value={command}
							onChange={e => setCommand(e.target.value)}
							placeholder="e.g., php artisan schedule:run"
							className="h-9 text-xs font-mono"
						/>
					</div>

					<div className="flex flex-col gap-2">
						<Label className="text-xs font-semibold">Cron Expression *</Label>
						<Input
							value={cronExpr}
							onChange={e => setCronExpr(e.target.value)}
							placeholder="0 * * * *"
							className="h-9 text-xs font-mono"
						/>
						<div className="flex items-center gap-1.5 pt-1">
							<span className="text-[11px] text-muted-foreground mr-1">Presets:</span>
							{CRON_PRESETS.map(p => (
								<button
									key={p.value}
									type="button"
									onClick={() => setCronExpr(p.value)}
									className={`text-[10px] px-2 py-0.5 rounded border transition-colors ${
										cronExpr === p.value
											? 'bg-primary/10 border-primary/30 text-primary font-bold'
											: 'bg-muted/40 border-border/40 text-muted-foreground hover:bg-muted'
									}`}
								>
									{p.label}
								</button>
							))}
						</div>
					</div>

					<div className="pt-2 flex items-center justify-end border-t border-border/40">
						<Button
							type="submit"
							disabled={creating}
							className="h-9 px-5 font-bold text-xs bg-primary hover:bg-primary/90 text-primary-foreground shadow-md"
						>
							{creating ? <RefreshCw className="w-3.5 h-3.5 animate-spin mr-1.5" /> : null}
							{editingSchedule ? 'Update Schedule' : 'Create Schedule'}
						</Button>
					</div>
				</form>
			</div>
		</div>
	);

	return createPortal(modalJSX, document.body);
}
