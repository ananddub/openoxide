import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {Label} from '#/components/ui/label';
import {
	Dialog,
	DialogContent,
	DialogDescription,
	DialogHeader,
	DialogTitle,
} from '#/components/ui/dialog';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import {Clock, Terminal} from 'lucide-react';
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

export function ScheduleDialog(props: ScheduleDialogProps) {
	const {isOpen, onClose} = props;
	const form = useScheduleDialog(props);

	return (
		<Dialog open={isOpen} onOpenChange={open => !open && onClose()}>
			<DialogContent className="border- max-h-[90vh] overflow-y-auto rounded-xl bg-card p-6.5 shadow-lg sm:max-w-2xl">
				<DialogHeader className="flex flex-row items-center gap-3 space-y-0 border-b  pb-4">
					<div className="flex size-10 shrink-0 items-center justify-center rounded-xl bg-primary/10 text-primary">
						<Clock className="size-5" />
					</div>
					<div>
						<DialogTitle className="text-base font-bold text-foreground">
							{form.editingSchedule
								? 'Edit Schedule Task'
								: 'Create Schedule Task'}
						</DialogTitle>
						<DialogDescription className="mt-0.5 text-xs text-muted-foreground">
							Configure automated cron jobs and command executions.
						</DialogDescription>
					</div>
				</DialogHeader>

				<form onSubmit={form.handleSubmit} className="space-y-4 pt-2">
					<div className="w-full space-y-1.5">
						<Label className="text-xs font-semibold">Name *</Label>
						<Input
							placeholder="e.g. Daily Database Backup"
							value={form.name}
							onChange={e => form.setName(e.target.value)}
							required
							className="h-9 w-full text-xs"
						/>
					</div>

					<div className="w-full space-y-1.5">
						<Label className="text-xs font-semibold">Description</Label>
						<Input
							placeholder="Optional task details..."
							value={form.description}
							onChange={e => form.setDescription(e.target.value)}
							className="h-9 w-full text-xs"
						/>
					</div>

					<ScheduleTargetFields form={form} />

					<div className="w-full space-y-1.5">
						<div className="flex items-center justify-between">
							<Label className="text-xs font-semibold">
								Cron Expression *
							</Label>
							<div className="flex items-center gap-1">
								{CRON_PRESETS.map(preset => (
									<Button
										key={preset.value}
										type="button"
										variant={
											form.cronExpression === preset.value
												? 'default'
												: 'ghost'
										}
										size="xs"
										onClick={() => form.setCronExpression(preset.value)}
										className="font-mono text-[10px]">
										{preset.label}
									</Button>
								))}
							</div>
						</div>
						<Input
							placeholder="*/5 * * * *"
							value={form.cronExpression}
							onChange={e => form.setCronExpression(e.target.value)}
							required
							className="h-9 w-full font-mono text-xs"
						/>
					</div>

					<div className="w-full space-y-1.5">
						<Label className="text-xs font-semibold">Shell Type</Label>
						<Select
							value={form.shellType}
							onValueChange={val => form.setShellType(val ?? 'bash')}>
							<SelectTrigger className="!h-9 w-full text-xs">
								<SelectValue />
							</SelectTrigger>
							<SelectContent className="border- bg-card">
								<SelectItem value="bash" className="text-xs">
									BASH
								</SelectItem>
								<SelectItem value="sh" className="text-xs">
									SH
								</SelectItem>
							</SelectContent>
						</Select>
					</div>

					<div className="w-full space-y-1.5">
						<Label className="flex items-center gap-1.5 text-xs font-semibold">
							<Terminal className="size-3.5 text-muted-foreground" />
							Command *
						</Label>
						<textarea
							placeholder="e.g. docker exec my_app npm run cleanup"
							value={form.command}
							onChange={e => form.setCommand(e.target.value)}
							required
							rows={3}
							className="flex w-full resize-none rounded-lg bg-transparent px-3 py-2 font-mono text-xs leading-relaxed text-foreground outline-none placeholder:text-muted-foreground focus-visible:border-ring focus-visible:ring-2 focus-visible:ring-ring/50 dark:bg-input/30"
						/>
					</div>

					<div className="flex justify-end border-t  pt-3">
						<Button
							type="submit"
							disabled={form.isSubmitting}
							className="h-9 w-full bg-primary px-5 text-xs font-semibold text-primary-foreground shadow-sm hover:bg-primary/95 sm:w-auto">
							{form.isSubmitting
								? 'Saving...'
								: form.editingSchedule
									? 'Save Changes'
									: 'Create Schedule'}
						</Button>
					</div>
				</form>
			</DialogContent>
		</Dialog>
	);
}
