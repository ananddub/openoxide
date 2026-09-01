import {useEffect, useState} from 'react';
import {toast} from 'sonner';
import {
	useDatabaseSwarmSettings,
	useDatabaseUpdateSwarmSettings,
	parseDatabaseKind,
} from '#/api/live';
import {formatApiError} from '#/api/utils';
import type {SwarmKey} from './forms';

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
	const query = useDatabaseSwarmSettings(
		kind,
		BigInt(database?.id || 0),
	);
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
					next[key] = key === 'labels_swarm' && !Array.isArray(parsed)
						? Object.entries(parsed).map(([labelKey, labelValue]) => ({key: labelKey, value: labelValue}))
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
				if (key !== 'stop_grace_period_swarm') serialize(body, key, values[key]);
			});
			await save.mutateAsync({
				kind,
				id: database?.id,
				values: {
					...body,
					stop_grace_period_swarm: grace
						? Number(grace)
						: null,
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
		setValues((old: any) => ({...old, [active]: Array.isArray(old[active]) ? [] : {...(defaults[active] || {})}}));
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

function serialize(body: any, key: string, value: any) {
	if (key === 'stop_grace_period_swarm') return;
	if (key === 'network_swarm') {
		const networks = (Array.isArray(value) ? value : [])
			.filter(network => network.Target?.trim())
			.map(network => ({
				Target: network.Target.trim(),
				Aliases: typeof network.Aliases === 'string' ? csv(network.Aliases) : network.Aliases,
				DriverOpts: Object.fromEntries(
					(network.DriverOptsEntries || [])
						.filter((option: any) => option.key?.trim())
						.map((option: any) => [option.key.trim(), option.value || '']),
				),
			}));
		body[key] = networks.length ? JSON.stringify(networks) : null;
		return;
	}
	if (key === 'labels_swarm') {
		const labels = Array.isArray(value) ? value.filter(item => item.key?.trim()) : [];
		body[key] = labels.length
			? JSON.stringify(Object.fromEntries(labels.map(item => [item.key.trim(), item.value || ''])))
			: null;
		return;
	}
	if (key === 'endpoint_spec_swarm') {
		const endpoint = {...value};
		endpoint.Ports = (endpoint.Ports || []).filter((port: any) => Number(port.TargetPort) > 0 && Number(port.PublishedPort) > 0);
		body[key] = Object.keys(endpoint).length ? JSON.stringify(endpoint) : null;
		return;
	}
	value = {...value};
	if (key === 'mode_swarm') {
		body[key] = JSON.stringify(
			value.type === 'Global'
				? {Global: {}}
				: {Replicated: {Replicas: Number(value.Replicas || 1)}},
		);
		return;
	}
	if (key === 'health_check_swarm' && value.Test) {
		value.Test = Array.isArray(value.Test)
			? value.Test
			: value.Test.startsWith('CMD')
				? [value.Test]
				: ['CMD-SHELL', value.Test];
	}
	if (key === 'placement_swarm') {
		if (typeof value.Constraints === 'string')
			value.Constraints = csv(value.Constraints);
		if (typeof value.Preferences === 'string')
			value.Preferences = csv(value.Preferences).map(
				(SpreadDescriptor: string) => ({SpreadDescriptor}),
			);
		if (typeof value.Platforms === 'string')
			value.Platforms = csv(value.Platforms).map((item: string) => {
				const [Architecture, OS] = item.split(':');
				return {Architecture, OS};
			});
	}
	if (key !== 'labels_swarm' && key !== 'endpoint_spec_swarm') {
		delete value.key;
		delete value.value;
		body[key] = Object.keys(value).length ? JSON.stringify(value) : null;
	}
}
function networkFormValue(network: any) {
	return {
		Target: network.Target || '',
		Aliases: (network.Aliases || []).join(', '),
		DriverOptsEntries: Object.entries(network.DriverOpts || {}).map(
			([key, value]) => ({key, value}),
		),
	};
}
function csv(value: string) {
	return value
		.split(',')
		.map(item => item.trim())
		.filter(Boolean);
}
