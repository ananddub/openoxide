import {Input} from '#/components/ui/input';
import {Label} from '#/components/ui/label';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';

export const CRON_PRESETS = [
	{label: 'Every 5m', value: '*/5 * * * *'},
	{label: 'Every 15m', value: '*/15 * * * *'},
	{label: 'Hourly', value: '0 * * * *'},
	{label: 'Daily', value: '0 0 * * *'},
];

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

export function CronExpressionField({
	cronExpr,
	onCronExpr,
}: {
	cronExpr: string;
	onCronExpr: (v: string) => void;
}) {
	return (
		<div className="flex flex-col gap-2">
			<Label className="text-xs font-semibold">Cron Expression *</Label>
			<Input
				value={cronExpr}
				onChange={e => onCronExpr(e.target.value)}
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
						onClick={() => onCronExpr(p.value)}
						className={`rounded  px-2 py-0.5 text-[10px] transition-colors ${
							cronExpr === p.value
								? ' bg-primary/10 font-bold text-primary'
								: ' bg-muted/40 text-muted-foreground hover:bg-muted'
						}`}>
						{p.label}
					</button>
				))}
			</div>
		</div>
	);
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

			<CronExpressionField
				cronExpr={p.cronExpr}
				onCronExpr={p.onCronExpr}
			/>
		</>
	);
}
