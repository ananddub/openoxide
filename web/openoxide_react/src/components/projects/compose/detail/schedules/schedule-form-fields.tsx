import {Input} from '#/components/ui/input';
import {Label} from '#/components/ui/label';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import {CronField} from '#/components/projects/common/cron-field';

interface ScheduleFormFieldsProps {
	servicesList: string[];
	name: string;
	onName: (v: string) => void;
	serviceName: string;
	onServiceName: (v: string) => void;
	command: string;
	onCommand: (v: string) => void;
	cronExpr: string;
	onCronExpr: (v: string) => void;
}

/** Input controls for the create/edit compose schedule form. */
export function ScheduleFormFields(p: ScheduleFormFieldsProps) {
	return (
		<>
			<div className="flex flex-col gap-1.5">
				<Label className="text-xs font-semibold">Schedule Name *</Label>
				<Input
					value={p.name}
					onChange={e => p.onName(e.target.value)}
					placeholder="e.g., Hourly Cache Cleanup"
					className="h-9 text-xs"
				/>
			</div>

			<div className="flex w-full flex-col gap-1.5">
				<Label className="text-xs font-semibold">Target Service *</Label>
				<Select
					value={p.serviceName}
					onValueChange={val => val && p.onServiceName(val)}>
					<SelectTrigger className="!h-9 w-full text-xs">
						<SelectValue placeholder="Select service" />
					</SelectTrigger>
					<SelectContent className="border- bg-card">
						{p.servicesList.map(srv => (
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
					value={p.command}
					onChange={e => p.onCommand(e.target.value)}
					placeholder="e.g., php artisan schedule:run"
					className="h-9 font-mono text-xs"
				/>
			</div>

			<CronField
				value={p.cronExpr}
				onChange={p.onCronExpr}
				label="Cron Schedule"
			/>
		</>
	);
}
