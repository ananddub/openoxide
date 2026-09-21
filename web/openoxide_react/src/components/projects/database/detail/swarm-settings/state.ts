import {useEffect, useState} from 'react';
import {toast} from 'sonner';
import {
	useDatabaseSwarmSettings,
	useDatabaseUpdateSwarmSettings,
	parseDatabaseKind,
} from '#/api/live';
import {formatApiError} from '#/api/utils';
import type {SwarmKey} from './forms';
import {serialize, networkFormValue} from './swarm-serialization';

export const swarmSections: ReadonlyArray<[SwarmKey, string]> = [
	['health_check_swarm', 'Health Check'],
	['restart_policy_swarm', 'Restart Policy'],
	['placement_swarm', 'Placement'],
	['update_config_swarm', 'Update Config'],
	['rollback_config_swarm', 'Rollback Config'],
	['mode_swarm', 'Mode'],
	['network_swarm', 'Network'],
	['labels_swarm', 'Labels'],
	['stop_grace_period_swarm', 'Stop Grace Period'],
	['endpoint_spec_swarm', 'Endpoint Spec'],
];
const defaults: any = {
	mode_swarm: {type: 'Replicated', Replicas: 1},
	restart_policy_swarm: {Condition: 'on-failure'},
	update_config_swarm: {FailureAction: 'rollback', Order: 'stop-first'},
	rollback_config_swarm: {FailureAction: 'pause', Order: 'stop-first'},
	endpoint_spec_swarm: {Mode: 'vip'},
};

export function useDatabaseSwarmSettingsState(
	database: any,
	onUpdated: () => void,
) {
	const kind = parseDatabaseKind(database?.kind || 'postgres');
	const query = useDatabaseSwarmSettings(kind, BigInt(database?.id || 0));
	const save = useDatabaseUpdateSwarmSettings();
	const [active, setActive] = useState<SwarmKey>('mode_swarm');
	const [values, setValues] = useState<any>({...defaults});
	const [grace, setGrace] = useState('');
	useEffect(() => {
		const data: any = query.data || {};
		const next: any = {...defaults};
		swarmSections.forEach(([key]) => {
			if (data[key])
				try {
					const parsed = JSON.parse(data[key]);
					next[key] =
						key === 'labels_swarm' && !Array.isArray(parsed)
							? Object.entries(parsed).map(([labelKey, labelValue]) => ({
									key: labelKey,
									value: labelValue,
								}))
							: Array.isArray(parsed)
								? key === 'network_swarm'
									? parsed.map(networkFormValue)
									: parsed
								: {...next[key], ...parsed};
				} catch {}
		});
		setValues(next);
		setGrace(
			data.stop_grace_period_swarm == null
				? ''
				: String(data.stop_grace_period_swarm),
		);
	}, [query.data]);
	const set = (key: string, value: any) =>
		setValues((old: any) => ({
			...old,
			[active]: ['networks', 'labels'].includes(key)
				? value
				: {...old[active], [key]: value},
		}));
	const saveAll = async () => {
		try {
			const body: any = {};
			swarmSections.forEach(([key]) => {
				if (key !== 'stop_grace_period_swarm')
					serialize(body, key, values[key]);
			});
			await save.mutateAsync({
				kind,
				id: database?.id,
				values: {
					...body,
					stop_grace_period_swarm: grace ? Number(grace) : null,
				},
			} as Parameters<typeof save.mutateAsync>[0]);
			toast.success('Swarm settings updated');
			onUpdated();
			return true;
		} catch (error) {
			toast.error(formatApiError(error));
			return false;
		}
	};
	const clearActive = () => {
		if (active === 'stop_grace_period_swarm') {
			setGrace('');
			return;
		}
		setValues((old: any) => ({
			...old,
			[active]: Array.isArray(old[active]) ? [] : {...defaults[active]},
		}));
	};
	return {
		active,
		setActive,
		values,
		set,
		grace,
		setGrace,
		saveAll,
		clearActive,
		saving: save.isPending,
	};
}
