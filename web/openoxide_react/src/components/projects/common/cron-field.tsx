import {useState} from 'react';
import {Input} from '#/components/ui/input';
import {Label} from '#/components/ui/label';
import {cn} from '#/api/utils';
import {describeCron} from '#/utils/cron';

/** Common recurring-job cadences offered while typing the cron field. */
export const CRON_PRESETS = [
	{label: 'Every 5 minutes', value: '*/5 * * * *'},
	{label: 'Every 15 minutes', value: '*/15 * * * *'},
	{label: 'Hourly', value: '0 * * * *'},
	{label: 'Daily at midnight', value: '0 0 * * *'},
	{label: 'Weekly on Monday', value: '0 0 * * 1'},
	{label: 'Monthly on the 1st', value: '0 0 1 * *'},
];

/**
 * Cron schedule picker used by every service schedule/backup form. One field:
 * type or focus to get frequency suggestions, or just type your own custom
 * 5-part expression — no separate preset row above.
 */
export function CronField({
	value,
	onChange,
	label = 'Cron Schedule',
	required = true,
	placeholder = 'Start typing for suggestions…',
}: {
	value: string;
	onChange: (v: string) => void;
	label?: string;
	required?: boolean;
	placeholder?: string;
}) {
	const [open, setOpen] = useState(false);
	const q = value.trim().toLowerCase();
	const human = describeCron(value);
	const isPreset = CRON_PRESETS.some(p => p.value === value);
	const matches = CRON_PRESETS.filter(
		p =>
			!q ||
			p.label.toLowerCase().includes(q) ||
			p.value.toLowerCase().includes(q),
	);

	return (
		<div className="flex flex-col gap-1.5">
			<Label className="text-xs font-semibold">
				{label} {required && '*'}
			</Label>

			<div className="relative">
				<Input
					value={value}
					onChange={e => {
						onChange(e.target.value);
						setOpen(true);
					}}
					onFocus={() => setOpen(true)}
					onBlur={() => setOpen(false)}
					placeholder={placeholder}
					required={required}
					className="h-9 font-mono text-xs"
				/>

				{open && matches.length > 0 && (
					<div className="absolute z-50 mt-1 max-h-56 w-full overflow-auto rounded-lg border bg-card p-1 shadow-md">
						{matches.map(p => (
							<button
								key={p.value}
								type="button"
								onMouseDown={e => {
									e.preventDefault();
									onChange(p.value);
									setOpen(false);
								}}
								className={cn(
									'flex w-full cursor-pointer items-center justify-between gap-3 rounded-md px-2.5 py-1.5 text-start text-xs transition-colors hover:bg-accent/40',
									p.value === value && 'bg-primary/10 text-primary',
								)}>
								<span className="font-medium">{p.label}</span>
								<span className="font-mono text-[11px] text-muted-foreground">
									{p.value}
								</span>
							</button>
						))}
					</div>
				)}
			</div>

			<span className="text-[11px] text-muted-foreground">
				{human
					? `Runs: ${human}.`
					: isPreset
						? 'Pick a suggestion above, or type your own.'
						: '5-part cron — minute hour day month weekday (e.g. 0 2 * * * = 02:00 every day).'}
			</span>
		</div>
	);
}
