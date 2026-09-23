import type {SwarmKey} from './forms';
import {FieldGrid} from '#/components/shared/field';
import {SwarmSelectField} from './select-field';
import {Num} from './field';
import {PlacementRows} from './placement-rows';
import {EndpointPorts} from './endpoint-ports';
import {LabelsList} from './labels-list';
import {NetworkList} from './network-list';

export function SwarmPlacementForm({
	section,
	value,
	set,
}: {
	section: SwarmKey;
	value: any;
	set: (key: string, value: any) => void;
}) {
	if (section === 'placement_swarm')
		return (
			<div className="flex max-w-2xl flex-col gap-4">
				<PlacementRows
					label="Constraints"
					values={value.Constraints}
					placeholder="node.role==manager"
					set={next => set('Constraints', next)}
				/>
				<PlacementRows
					label="Preferences"
					values={value.Preferences}
					placeholder="node.labels.zone"
					set={next =>
						set(
							'Preferences',
							next.map((item: string) => ({SpreadDescriptor: item})),
						)
					}
				/>
				<PlacementRows
					label="Platforms"
					values={value.Platforms}
					placeholder="amd64:linux"
					set={next =>
						set(
							'Platforms',
							next.map((item: string) => {
								const [Architecture, OS] = item.split(':');
								return {Architecture, OS};
							}),
						)
					}
				/>
				<Num
					label="Max Replicas Per Node"
					value={value.MaxReplicas}
					set={v => set('MaxReplicas', v)}
				/>
			</div>
		);
	if (section === 'network_swarm')
		return <NetworkList value={value} set={set} />;
	if (section === 'labels_swarm')
		return <LabelsList value={value} set={set} />;
	if (section === 'endpoint_spec_swarm')
		return (
			<div className="flex max-w-2xl flex-col gap-4">
				<SwarmSelectField
					label="Endpoint Mode"
					value={value.Mode || 'vip'}
					onChange={next => set('Mode', next)}
					options={[
						{value: 'vip', label: 'VIP (Virtual IP)'},
						{value: 'dnsrr', label: 'DNS Round Robin'},
					]}
				/>
				<EndpointPorts
					value={value.Ports}
					set={next => set('Ports', next)}
				/>
			</div>
		);
	return (
		<div className="flex max-w-2xl flex-col gap-4">
			<FieldGrid>
				<Num
					label="nofile soft"
					value={value.nofile_soft}
					set={v => set('nofile_soft', v)}
				/>
				<Num
					label="nofile hard"
					value={value.nofile_hard}
					set={v => set('nofile_hard', v)}
				/>
			</FieldGrid>
		</div>
	);
}
