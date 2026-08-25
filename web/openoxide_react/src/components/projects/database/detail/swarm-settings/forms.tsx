import {SwarmPlacementForm} from './placement';
import {SwarmRuntimeForm} from './runtime';
export type SwarmKey =
	| 'health_check_swarm'
	| 'restart_policy_swarm'
	| 'placement_swarm'
	| 'update_config_swarm'
	| 'rollback_config_swarm'
	| 'mode_swarm'
	| 'network_swarm'
	| 'labels_swarm'
	| 'endpoint_spec_swarm'
	| 'ulimits_swarm'
	| 'stop_grace_period_swarm';
export function SwarmSettingsForm({
	section,
	value,
	set,
}: {
	section: SwarmKey;
	value: any;
	set: (key: string, value: any) => void;
}) {
	return [
		'mode_swarm',
		'restart_policy_swarm',
		'update_config_swarm',
		'rollback_config_swarm',
		'health_check_swarm',
	].includes(section) ? (
		<SwarmRuntimeForm section={section} value={value} set={set} />
	) : (
		<SwarmPlacementForm section={section} value={value} set={set} />
	);
}
