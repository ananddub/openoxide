import {Input} from '#/components/ui/input';
import {Button} from '#/components/ui/button';
import {Plus, Trash2} from 'lucide-react';
import type {SwarmKey} from './forms';
import {SwarmSelectField} from './select-field';
export function SwarmRuntimeForm({
	section,
	value,
	set,
}: {
	section: SwarmKey;
	value: any;
	set: (key: string, value: any) => void;
}) {
	if (section === 'mode_swarm')
		return (
			<div className="flex max-w-2xl flex-col gap-4">
				<SwarmSelectField
					label="Service Mode"
					value={value.type || 'Replicated'}
					onChange={next => set('type', next)}
					options={[
						{value: 'Replicated', label: 'Replicated'},
						{value: 'Global', label: 'Global'},
					]}
				/>
				{value.type !== 'Global' && (
					<Num
						label="Replicas"
						value={value.Replicas}
						set={v => set('Replicas', v)}
					/>
				)}
			</div>
		);
	if (section === 'restart_policy_swarm')
		return (
			<div className="flex max-w-2xl flex-col gap-4">
				<SwarmSelectField
					label="Condition"
					value={value.Condition || 'on-failure'}
					onChange={next => set('Condition', next)}
					options={[
						{value: 'none', label: 'None'},
						{value: 'on-failure', label: 'On Failure'},
						{value: 'any', label: 'Any'},
					]}
				/>
				<Num
					label="Delay (nanoseconds)"
					value={value.Delay}
					set={v => set('Delay', v)}
				/>
				<Num
					label="Max Attempts"
					value={value.MaxAttempts}
					set={v => set('MaxAttempts', v)}
				/>
				<Num
					label="Window (nanoseconds)"
					value={value.Window}
					set={v => set('Window', v)}
				/>
			</div>
		);
	if (section === 'health_check_swarm')
		return (
			<div className="flex max-w-2xl flex-col gap-4">
				<HealthCommands value={value.Test} set={next => set('Test', next)} />
				<div className="flex flex-col gap-4">
				<Num
					label="Interval (nanoseconds)"
					value={value.Interval}
					set={v => set('Interval', v)}
				/>
				<Num
					label="Timeout (nanoseconds)"
					value={value.Timeout}
					set={v => set('Timeout', v)}
				/>
				<Num
					label="Start Period (nanoseconds)"
					value={value.StartPeriod}
					set={v => set('StartPeriod', v)}
				/>
				<Num
					label="Retries"
					value={value.Retries}
					set={v => set('Retries', v)}
				/>
				</div>
			</div>
		);
	return (
		<div className="flex max-w-2xl flex-col gap-4">
			<Num
				label="Parallelism"
				value={value.Parallelism}
				set={v => set('Parallelism', v)}
			/>
			<Num
				label="Delay (nanoseconds)"
				value={value.Delay}
				set={v => set('Delay', v)}
			/>
			<SwarmSelectField
				label="Failure Action"
				value={value.FailureAction || 'pause'}
				onChange={next => set('FailureAction', next)}
				options={[
					{value: 'pause', label: 'Pause'},
					{value: 'continue', label: 'Continue'},
					{value: 'rollback', label: 'Rollback'},
				]}
			/>
			<SwarmSelectField
				label="Order"
				value={value.Order || 'stop-first'}
				onChange={next => set('Order', next)}
				options={[
					{value: 'stop-first', label: 'Stop First'},
					{value: 'start-first', label: 'Start First'},
				]}
			/>
			<Num
				label="Monitor (nanoseconds)"
				value={value.Monitor}
				set={v => set('Monitor', v)}
			/>
			<Num
				label="Max Failure Ratio"
				value={value.MaxFailureRatio}
				set={v => set('MaxFailureRatio', v)}
			/>
		</div>
	);
}
function HealthCommands({value, set}: {value: any; set: (value: string[]) => void}) {
	const commands = Array.isArray(value) && value.length ? value : ['CMD-SHELL', ''];
	return <div className="space-y-2"><div><h4 className="text-sm font-medium">Test Commands</h4><p className="text-xs text-muted-foreground">Command to run for the health check.</p></div>{commands.map((command, index) => <div key={index} className="flex gap-2"><Input value={command} onChange={e => set(commands.map((item, i) => i === index ? e.target.value : item))} placeholder={index === 0 ? 'CMD-SHELL' : 'curl -f http://localhost/health'} />{commands.length > 1 && <Button type="button" variant="destructive" size="sm" onClick={() => set(commands.filter((_, i) => i !== index))}>Remove</Button>}</div>)}<Button type="button" variant="outline" size="sm" className="w-fit" onClick={() => set([...commands, ''])}><Plus className="mr-1 size-3" />Add command</Button></div>;
}
function Num({
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
function Field({
	label,
	children,
}: {
	label: string;
	children: React.ReactNode;
}) {
	return (
		<label className="flex flex-col gap-1.5 text-sm font-medium">
			{label}
			{description(label) && <span className="text-xs font-normal text-muted-foreground">{description(label)}</span>}
			{children}
		</label>
	);
}
function description(label: string) {
	const values: Record<string, string> = {
		'Interval (nanoseconds)': 'Time between consecutive health checks.',
		'Timeout (nanoseconds)': 'Maximum time allowed for one health check.',
		'Start Period (nanoseconds)': 'Grace period before health checks begin.',
		'Retries': 'Consecutive failures required to mark the service unhealthy.',
		'Delay (nanoseconds)': 'Time to wait before restarting or updating a task.',
		'Max Attempts': 'Maximum restart attempts before giving up.',
		'Window (nanoseconds)': 'Time window used to evaluate restart failures.',
		'Parallelism': 'Number of tasks updated at the same time.',
		'Monitor (nanoseconds)': 'Time to monitor an update for failures.',
		'Max Failure Ratio': 'Allowed failure ratio during an update.',
		'Replicas': 'Number of running tasks for replicated mode.',
	};
	return values[label] || '';
}
