import {useState} from 'react';
import {Calendar, RefreshCw, X} from 'lucide-react';
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
				<div className="p-4 border-b border-border flex items-center justify-between bg-muted/30">
					<div className="flex items-center gap-2">
						<Calendar className="w-4 h-4 text-primary" />
						<h3 className="text-sm font-bold text-foreground">Create Compose Schedule Task</h3>
					</div>
					<Button variant="ghost" size="icon" onClick={onClose} className="h-7 w-7 text-muted-foreground">
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
							<Label className="text-xs font-semibold">Target Service Container *</Label>
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
							<Label className="text-xs font-semibold">Cron Schedule *</Label>
							<Input
								value={cronExpr}
								onChange={e => setCronExpr(e.target.value)}
								placeholder="0 * * * *"
								className="h-9 text-xs font-mono"
							/>
						</div>
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

					<div className="pt-2 flex items-center justify-end gap-2 border-t border-border">
						<Button type="button" variant="outline" onClick={onClose} className="h-8 text-xs font-semibold">
							Cancel
						</Button>
						<Button type="submit" disabled={creating} className="h-8 text-xs font-semibold">
							{creating ? <RefreshCw className="w-3.5 h-3.5 animate-spin mr-1" /> : null}
							Create Schedule
						</Button>
					</div>
				</form>
			</div>
		</div>
	);
}
