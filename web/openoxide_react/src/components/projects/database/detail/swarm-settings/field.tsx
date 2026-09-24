import {useState} from 'react';
import {Minus, Plus} from 'lucide-react';
import {Input} from '#/components/ui/input';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import {Field as FormField} from '#/components/shared/field';

const DESCRIPTIONS: Record<string, string> = {
	'Network target': 'Network to attach this service to.',
	'Aliases (comma separated)':
		'Additional names available through service discovery.',
	'Label key': 'Metadata key used to identify or filter the service.',
	'Label value': 'Value associated with the service label.',
	'Endpoint Mode':
		'VIP uses a virtual IP; DNSRR resolves each task directly.',
	Protocol: 'Transport protocol used by the published port.',
	'Test Commands': 'Command to run for the health check.',
	'Interval (nanoseconds)': 'Time between consecutive health checks.',
	'Timeout (nanoseconds)': 'Maximum time allowed for one health check.',
	'Start Period (nanoseconds)': 'Grace period before health checks begin.',
	Retries: 'Consecutive failures required to mark the service unhealthy.',
	'Delay (nanoseconds)':
		'Time to wait before restarting or updating a task.',
	'Max Attempts': 'Maximum restart attempts before giving up.',
	'Window (nanoseconds)': 'Duration the scheduler considers for restarts.',
	'Monitor (nanoseconds)':
		'How long to wait to judge a task before moving on.',
	'Stop Grace Period (nanoseconds)':
		'Time to wait before forcefully stopping a task.',
	Parallelism: 'Number of tasks updated in parallel at once.',
	'Max Failure Ratio':
		'Fraction of failed tasks that aborts the whole update.',
	Replicas: 'Number of task copies the service should run.',
	'Max Replicas Per Node':
		'Limit on how many replicas may run on a single node.',
};

/** Time units, all expressed as a multiple of one nanosecond. */
const UNITS = [
	{value: 'ns', label: 'Nanoseconds', short: 'ns', factor: 1},
	{value: 'us', label: 'Microseconds', short: 'µs', factor: 1_000},
	{value: 'ms', label: 'Milliseconds', short: 'ms', factor: 1_000_000},
	{value: 's', label: 'Seconds', short: 's', factor: 1_000_000_000},
	{value: 'm', label: 'Minutes', short: 'min', factor: 60_000_000_000},
	{value: 'h', label: 'Hours', short: 'hr', factor: 3_600_000_000_000},
];

/** Pick the largest unit that renders the value as a whole number ≥ 1. */
function deriveUnit(ns: number): string {
	for (let i = UNITS.length - 1; i >= 0; i--) {
		const factor = UNITS[i].factor;
		if (ns > 0 && ns % factor === 0 && ns / factor >= 1)
			return UNITS[i].value;
	}
	return 'ns';
}

/** Trim float noise so 0.5s stays "0.5", not "0.5000000000001". */
function clean(n: number): string {
	if (Number.isInteger(n)) return String(n);
	return String(parseFloat(n.toFixed(6)));
}

const spinless =
	'[appearance:textfield] [&::-webkit-inner-spin-button]:appearance-none [&::-webkit-outer-spin-button]:appearance-none';

export function Field({
	label,
	children,
}: {
	label: string;
	children: React.ReactNode;
}) {
	return (
		<FormField label={label} hint={DESCRIPTIONS[label]}>
			{children}
		</FormField>
	);
}

export function Num({
	label,
	value,
	set,
	step = 1,
	min = 0,
}: {
	label: string;
	value: any;
	set: (value: number | undefined) => void;
	step?: number;
	min?: number;
}) {
	const current =
		value == null || value === '' ? undefined : Number(value);
	const base = Number.isFinite(current) ? (current as number) : min;
	const bump = (delta: number) =>
		set(parseFloat((base + delta).toFixed(6)));
	return (
		<FormField label={label} hint={DESCRIPTIONS[label]}>
			<div className="flex w-full items-center gap-1 rounded-lg border border-border/60 p-1">
				<button
					type="button"
					onClick={() => bump(-step)}
					disabled={base <= min}
					aria-label={`Decrease ${label}`}
					className="flex size-8 shrink-0 items-center justify-center rounded-md text-muted-foreground transition-colors hover:bg-muted hover:text-foreground disabled:pointer-events-none disabled:opacity-40">
					<Minus className="size-4" />
				</button>
				<Input
					type="number"
					inputMode="decimal"
					value={value ?? ''}
					min={min}
					step={step}
					onChange={event =>
						set(
							event.target.value === ''
								? undefined
								: Number(event.target.value),
						)
					}
					placeholder="0"
					className={`h-8 min-w-0 flex-1 border-0 bg-transparent p-0 text-center text-sm font-medium tabular-nums focus-visible:ring-0 ${spinless}`}
				/>
				<button
					type="button"
					onClick={() => bump(step)}
					aria-label={`Increase ${label}`}
					className="flex size-8 shrink-0 items-center justify-center rounded-md text-muted-foreground transition-colors hover:bg-muted hover:text-foreground disabled:pointer-events-none disabled:opacity-40">
					<Plus className="size-4" />
				</button>
			</div>
		</FormField>
	);
}

/** A duration stored in nanoseconds, edited as a value + friendly unit. */
export function Duration({
	label,
	value,
	set,
}: {
	label: string;
	value: any;
	set: (value: number | undefined) => void;
}) {
	const cleanLabel = label.replace(/\s*\(nanoseconds\)/i, '');
	const hint = DESCRIPTIONS[label];
	const ns = value == null || value === '' ? null : Number(value);
	const [unit, setUnit] = useState(() =>
		ns && ns > 0 ? deriveUnit(ns) : 'ms',
	);
	const factor = UNITS.find(u => u.value === unit)!.factor;
	const display = ns == null ? '' : clean(ns / factor);

	const onAmount = (raw: string) => {
		if (raw.trim() === '') {
			set(undefined);
			return;
		}
		const n = Number(raw);
		if (Number.isNaN(n)) return;
		set(Math.round(n * factor));
	};
	const onUnit = (next: string) => {
		const nf = UNITS.find(u => u.value === next)!.factor;
		if (ns != null) set(Math.round((ns / factor) * nf));
		setUnit(next);
	};

	return (
		<FormField label={cleanLabel} hint={hint}>
			<div className="flex items-center gap-2">
				<Input
					type="number"
					inputMode="decimal"
					value={display}
					onChange={event => onAmount(event.target.value)}
					placeholder="0"
					className={`flex-1 ${spinless}`}
				/>
				<Select value={unit} onValueChange={next => next && onUnit(next)}>
					<SelectTrigger className="w-[110px] shrink-0">
						<SelectValue />
					</SelectTrigger>
					<SelectContent>
						{UNITS.map(u => (
							<SelectItem key={u.value} value={u.value}>
								{u.short}
							</SelectItem>
						))}
					</SelectContent>
				</Select>
			</div>
		</FormField>
	);
}
