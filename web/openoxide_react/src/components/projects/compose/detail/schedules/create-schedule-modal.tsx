import {useState, useEffect} from 'react';
import {Clock, RefreshCw} from 'lucide-react';
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
import {ScheduleFormFields} from './schedule-form-fields';

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
	const [cronExpr, setCronExpr] = useState('');
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
			setCronExpr('');
		}
	}, [editingSchedule, servicesList]);

	if (typeof document === 'undefined') return null;

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
		<Sheet open={isOpen} onOpenChange={open => !open && onClose()}>
			<SheetContent>
				<SheetHeader>
					<SheetTitle className="flex items-center gap-2">
						<Clock className="size-4 text-primary" />
						{editingSchedule
							? 'Edit Schedule / Cron Task'
							: 'Create Compose Schedule Task'}
					</SheetTitle>
					<SheetDescription>
						Automate commands on container services.
					</SheetDescription>
				</SheetHeader>

				<form
					id="create-schedule-form"
					onSubmit={handleSubmit}
					className="flex flex-1 flex-col gap-4 overflow-y-auto px-6 py-5">
					<ScheduleFormFields
						servicesList={servicesList}
						name={name}
						onName={setName}
						serviceName={serviceName}
						onServiceName={setServiceName}
						command={command}
						onCommand={setCommand}
						cronExpr={cronExpr}
						onCronExpr={setCronExpr}
					/>
				</form>

				<SheetFooter>
					<Button type="button" variant="outline" onClick={onClose}>
						Cancel
					</Button>
					<Button
						type="submit"
						form="create-schedule-form"
						disabled={creating}
						className="gap-1.5">
						{creating ? (
							<RefreshCw className="size-3.5 animate-spin" />
						) : null}
						{editingSchedule ? 'Update Schedule' : 'Create Schedule'}
					</Button>
				</SheetFooter>
			</SheetContent>
		</Sheet>
	);
}
