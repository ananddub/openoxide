import type {SwarmKey} from './forms';
import {SwarmSelectField} from './select-field';
import {Num} from './field';
import {HealthCommands} from './health-commands';

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
				<HealthCommands
					value={value.Test}
					set={next => set('Test', next)}
				/>
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
