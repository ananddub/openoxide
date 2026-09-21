/** Comma-separated string → trimmed non-empty tokens. */
export function csv(value: string): string[] {
	return value
		.split(',')
		.map(item => item.trim())
		.filter(Boolean);
}

/** Normalizes a raw network object into form-editable field values. */
export function networkFormValue(network: any) {
	return {
		Target: network.Target || '',
		Aliases: (network.Aliases || []).join(', '),
		DriverOptsEntries: Object.entries(network.DriverOpts || {}).map(
			([key, value]) => ({key, value}),
		),
	};
}

/** Serializes one swarm section's editable value into the RPC `body`. */
export function serialize(body: any, key: string, value: any) {
	if (key === 'stop_grace_period_swarm') return;
	if (key === 'network_swarm') {
		body[key] = serializeNetworks(value);
		return;
	}
	if (key === 'labels_swarm') {
		body[key] = serializeLabels(value);
		return;
	}
	if (key === 'endpoint_spec_swarm') {
		body[key] = serializeEndpoint(value);
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
	if (key === 'placement_swarm') normalizePlacement(value);
	if (key !== 'labels_swarm' && key !== 'endpoint_spec_swarm') {
		delete value.key;
		delete value.value;
		body[key] = Object.keys(value).length ? JSON.stringify(value) : null;
	}
}

function serializeNetworks(value: any): string | null {
	const networks = (Array.isArray(value) ? value : [])
		.filter((network: any) => network.Target?.trim())
		.map((network: any) => ({
			Target: network.Target.trim(),
			Aliases:
				typeof network.Aliases === 'string'
					? csv(network.Aliases)
					: network.Aliases,
			DriverOpts: Object.fromEntries(
				(network.DriverOptsEntries || [])
					.filter((option: any) => option.key?.trim())
					.map((option: any) => [option.key.trim(), option.value || '']),
			),
		}));
	return networks.length ? JSON.stringify(networks) : null;
}

function serializeLabels(value: any): string | null {
	const labels = Array.isArray(value)
		? value.filter((item: any) => item.key?.trim())
		: [];
	return labels.length
		? JSON.stringify(
				Object.fromEntries(
					labels.map((item: any) => [item.key.trim(), item.value || '']),
				),
			)
		: null;
}

function serializeEndpoint(value: any): string | null {
	const endpoint = {...value};
	endpoint.Ports = (endpoint.Ports || []).filter(
		(port: any) =>
			Number(port.TargetPort) > 0 && Number(port.PublishedPort) > 0,
	);
	return Object.keys(endpoint).length ? JSON.stringify(endpoint) : null;
}

function normalizePlacement(value: any) {
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
