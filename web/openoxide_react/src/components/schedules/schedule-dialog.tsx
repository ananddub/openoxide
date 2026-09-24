import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {Textarea} from '#/components/ui/textarea';
import {
	Sheet,
	SheetContent,
	SheetDescription,
	SheetFooter,
	SheetHeader,
	SheetTitle,
} from '#/components/ui/sheet';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import {WizardField} from '#/components/shared/sheet-wizard';
import {
	useScheduleDialog,
	type ScheduleDialogProps,
} from './use-schedule-dialog';
import {ScheduleTargetFields} from './schedule-target-fields';

const CRON_PRESETS = [
	{label: 'Every 5m', value: '*/5 * * * *'},
	{label: 'Every 15m', value: '*/15 * * * *'},
	{label: 'Hourly', value: '0 * * * *'},
	{label: 'Daily', value: '0 0 * * *'},
	{label: 'Weekly', value: '0 0 * * 0'},
];

/** Create / edit schedule — the house right sheet. */
export function ScheduleDialog(props: ScheduleDialogProps) {
	const {isOpen, onClose} = props;
	const form = useScheduleDialog(props);

	return (
		<Sheet
			open={isOpen}
			onOpenChange={open => {
				if (!open) onClose();
			}}>
			<SheetContent className="p-0">
				<SheetHeader>
					<SheetTitle>
						{form.editingSchedule
							? 'Edit Schedule Task'
							: 'New Schedule Task'}
					</SheetTitle>
					<SheetDescription>
						Run a command on a server or service on a cron schedule.
					</SheetDescription>
				</SheetHeader>

				<form
					onSubmit={form.handleSubmit}
					className="flex min-h-0 flex-1 flex-col">
					<div className="flex min-h-0 flex-1 flex-col gap-5 overflow-y-auto px-6 py-6">
						<WizardField label="Name" required>
							<Input
								placeholder="Daily Database Backup"
								value={form.name}
								onChange={e => form.setName(e.target.value)}
								required
								autoFocus
							/>
						</WizardField>

						<WizardField label="Description" optional>
							<Input
								placeholder="What this task does"
								value={form.description}
								onChange={e => form.setDescription(e.target.value)}
							/>
						</WizardField>

						<ScheduleTargetFields form={form} />

						<WizardField label="Cron expression" required>
							<Input
								placeholder="*/5 * * * *"
								value={form.cronExpression}
								onChange={e => form.setCronExpression(e.target.value)}
								required
								className="font-mono"
							/>
							<div className="flex flex-wrap gap-1 pt-1">
								{CRON_PRESETS.map(preset => (
									<Button
										key={preset.value}
										type="button"
										variant={
											form.cronExpression === preset.value
												? 'default'
												: 'outline'
										}
										size="xs"
										onClick={() => form.setCronExpression(preset.value)}
										className="font-mono">
										{preset.label}
									</Button>
								))}
							</div>
						</WizardField>

						<WizardField label="Shell">
							<Select
								value={form.shellType}
								onValueChange={val => val && form.setShellType(val)}>
								<SelectTrigger>
									<SelectValue />
								</SelectTrigger>
								<SelectContent className="bg-card">
									<SelectItem value="bash">BASH</SelectItem>
									<SelectItem value="sh">SH</SelectItem>
								</SelectContent>
							</Select>
						</WizardField>

						<WizardField label="Command" required>
							<Textarea
								placeholder="docker exec my_app npm run cleanup"
								value={form.command}
								onChange={e => form.setCommand(e.target.value)}
								required
								rows={4}
								className="min-h-32 resize-y rounded-lg font-mono text-xs"
							/>
						</WizardField>
					</div>

					<SheetFooter>
						<Button
							type="submit"
							disabled={form.isSubmitting}
							className="h-10 gap-1.5 px-5">
							{form.editingSchedule ? 'Save Changes' : 'Save'}
						</Button>
					</SheetFooter>
				</form>
			</SheetContent>
		</Sheet>
	);
}
