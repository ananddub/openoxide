import {Input} from '#/components/ui/input';

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
};

export function Field({
	label,
	children,
}: {
	label: string;
	children: React.ReactNode;
}) {
	return (
		<label className="flex flex-col gap-1.5 text-sm font-medium">
			{label}
			{DESCRIPTIONS[label] && (
				<span className="text-xs font-normal text-muted-foreground">
					{DESCRIPTIONS[label]}
				</span>
			)}
			{children}
		</label>
	);
}

export function Num({
	label,
	value,
	set,
}: {
	label: string;
	value: any;
	set: (value: number) => void;
}) {
	return (
		<Field label={label}>
			<Input
				type="number"
				value={value ?? ''}
				onChange={e => set(Number(e.target.value))}
			/>
		</Field>
	);
}
