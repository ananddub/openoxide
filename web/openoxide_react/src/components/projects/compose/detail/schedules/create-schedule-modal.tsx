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

export function CreateScheduleModal({
	isOpen,
	onClose,
	servicesList,
	editingSchedule,
	onCreate,
}: CreateScheduleModalProps) {
	const [name, setName] = useState('');
	const [serviceName, setServiceName] = useState(servicesList[0] || 'app');
	const [command, setCommand] = useState('');
	const [cronExpr, setCronExpr] = useState('0 * * * *');
	const [creating, setCreating] = useState(false);

	useEffect(() => {
		if (editingSchedule) {
			setName(editingSchedule.name || '');
			setServiceName(
				editingSchedule.service_name ||
					editingSchedule.target ||
					editingSchedule.app_name ||
					servicesList[0] ||
					'app',
			);
			setCommand(editingSchedule.command || '');
			setCronExpr(
				editingSchedule.cron_expression ||
					editingSchedule.cronExpr ||
					'0 * * * *',
			);
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
		<div className="fixed inset-0 z-[999999] flex items-center justify-center bg-black/60 p-4 backdrop-blur-sm">
			<div className="w-full max-w-3xl animate-in overflow-hidden rounded-xl border border-border bg-card shadow-2xl duration-150 fade-in">
				<div className="flex items-center justify-between border-b border-border/40 bg-muted/20 p-4">
					<div className="flex items-center gap-2.5">
						<div className="flex size-8 items-center justify-center rounded-lg border border-primary/20 bg-primary/10 text-primary">
							<Clock className="h-4 w-4" />
						</div>
						<div>
							<h3 className="text-sm font-bold text-foreground">
								{editingSchedule
									? 'Edit Schedule / Cron Task'
									: 'Create Compose Schedule Task'}
							</h3>
							<p className="text-[11px] text-muted-foreground">
								Automate commands on container services
							</p>
						</div>
					</div>
					<Button
						variant="ghost"
						size="icon"
						onClick={onClose}
						className="h-7 w-7 text-muted-foreground hover:text-foreground">
						<X className="h-4 w-4" />
					</Button>
				</div>

				<form onSubmit={handleSubmit} className="flex flex-col gap-4 p-5">
					<div className="flex flex-col gap-1.5">
						<Label className="text-xs font-semibold">
							Schedule Name *
						</Label>
						<Input
							value={name}
							onChange={e => setName(e.target.value)}
							placeholder="e.g., Hourly Cache Cleanup"
							className="h-9 text-xs"
						/>
					</div>

					<div className="flex w-full flex-col gap-1.5">
						<Label className="text-xs font-semibold">
							Target Service *
						</Label>
						<Select
							value={serviceName}
							onValueChange={val => val && setServiceName(val)}>
							<SelectTrigger className="!h-9 w-full text-xs">
								<SelectValue placeholder="Select service" />
							</SelectTrigger>
							<SelectContent className="border-border bg-card">
								{servicesList.map(srv => (
									<SelectItem key={srv} value={srv} className="text-xs">
										{srv}
									</SelectItem>
								))}
							</SelectContent>
						</Select>
					</div>

					<div className="flex flex-col gap-1.5">
						<Label className="text-xs font-semibold">
							Command to Run *
						</Label>
						<Input
							value={command}
							onChange={e => setCommand(e.target.value)}
							placeholder="e.g., php artisan schedule:run"
							className="h-9 font-mono text-xs"
						/>
					</div>

					<div className="flex flex-col gap-2">
						<Label className="text-xs font-semibold">
							Cron Expression *
						</Label>
						<Input
							value={cronExpr}
							onChange={e => setCronExpr(e.target.value)}
							placeholder="0 * * * *"
							className="h-9 font-mono text-xs"
						/>
						<div className="flex items-center gap-1.5 pt-1">
							<span className="mr-1 text-[11px] text-muted-foreground">
								Presets:
							</span>
							{CRON_PRESETS.map(p => (
								<button
									key={p.value}
									type="button"
									onClick={() => setCronExpr(p.value)}
									className={`rounded border px-2 py-0.5 text-[10px] transition-colors ${
										cronExpr === p.value
											? 'border-primary/30 bg-primary/10 font-bold text-primary'
											: 'border-border/40 bg-muted/40 text-muted-foreground hover:bg-muted'
									}`}>
									{p.label}
								</button>
							))}
						</div>
					</div>

					<div className="flex items-center justify-end border-t border-border/40 pt-2">
						<Button
							type="submit"
							disabled={creating}
							className="h-9 bg-primary px-5 text-xs font-bold text-primary-foreground shadow-md hover:bg-primary/90">
							{creating ? (
								<RefreshCw className="mr-1.5 h-3.5 w-3.5 animate-spin" />
							) : null}
							{editingSchedule ? 'Update Schedule' : 'Create Schedule'}
						</Button>
					</div>
				</form>
			</div>
		</div>
	);

	return createPortal(modalJSX, document.body);
}
