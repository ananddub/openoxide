import {Input} from '#/components/ui/input';
import {Button} from '#/components/ui/button';
import {Plus, Trash2} from 'lucide-react';
import type {SwarmKey} from './forms';
import {SwarmSelectField} from './select-field';
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
				<PlacementRows label="Constraints" values={value.Constraints} placeholder="node.role==manager" set={next => set('Constraints', next)} />
				<PlacementRows label="Preferences" values={value.Preferences} placeholder="node.labels.zone" set={next => set('Preferences', next.map((item: string) => ({SpreadDescriptor: item})))} />
				<PlacementRows label="Platforms" values={value.Platforms} placeholder="amd64:linux" set={next => set('Platforms', next.map((item: string) => { const [Architecture, OS] = item.split(':'); return {Architecture, OS}; }))} />
				<Num
					label="Max Replicas Per Node"
					value={value.MaxReplicas}
					set={v => set('MaxReplicas', v)}
				/>
			</div>
		);
	if (section === 'network_swarm')
		return (
			<NetworkList value={value} set={set} />
		);
	if (section === 'labels_swarm')
		return (
			<LabelsList value={value} set={set} />
		);
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
				<EndpointPorts value={value.Ports} set={next => set('Ports', next)} />
			</div>
		);
	return (
		<div className="flex max-w-2xl flex-col gap-4">
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
		</div>
	);
}

function EndpointPorts({value, set}: {value: any; set: (value: any[]) => void}) {
	const ports = Array.isArray(value) ? value : [];
	const update = (index: number, patch: any) => set(ports.map((port: any, i: number) => i === index ? {...port, ...patch} : port));
	return <div className="space-y-2"><div><h4 className="text-sm font-medium">Published Ports</h4><p className="text-xs text-muted-foreground">Expose service ports through the Swarm endpoint.</p></div>{ports.map((port: any, index: number) => <div key={index} className="flex flex-col gap-3 rounded-md  p-3"><Field label="Protocol"><SwarmSelectField label="" value={port.Protocol || 'tcp'} onChange={next => update(index, {Protocol: next})} options={[{value: 'tcp', label: 'TCP'}, {value: 'udp', label: 'UDP'}, {value: 'sctp', label: 'SCTP'}]} /></Field><Num label="Target" value={port.TargetPort} set={v => update(index, {TargetPort: v})} /><Num label="Published" value={port.PublishedPort} set={v => update(index, {PublishedPort: v})} /><Field label="Publish mode"><SwarmSelectField label="" value={port.PublishMode || 'ingress'} onChange={next => update(index, {PublishMode: next})} options={[{value: 'ingress', label: 'Ingress'}, {value: 'host', label: 'Host'}]} /></Field><Button type="button" variant="destructive" size="sm" className="w-fit" onClick={() => set(ports.filter((_: any, i: number) => i !== index))}>Remove</Button></div>)}<Button type="button" variant="outline" size="sm" className="w-fit" onClick={() => set([...ports, {Protocol: 'tcp', TargetPort: 0, PublishedPort: 0, PublishMode: 'ingress'}])}><Plus className="mr-1 size-3" />Add port</Button></div>;
}

function PlacementRows({label, values, placeholder, set}: {label: string; values: any; placeholder: string; set: (values: string[]) => void}) {
	const rows = Array.isArray(values) ? values.map((item: any) => typeof item === 'string' ? item : item.SpreadDescriptor || `${item.Architecture || ''}:${item.OS || ''}`) : values ? String(values).split(',') : [''];
	return <div className="space-y-2"><div><h4 className="text-sm font-medium">{label}</h4></div>{rows.map((row: string, index: number) => <div key={index} className="flex gap-2"><Input value={row} placeholder={placeholder} onChange={e => set(rows.map((item, i) => i === index ? e.target.value : item))} />{rows.length > 1 && <Button type="button" variant="destructive" size="sm" onClick={() => set(rows.filter((_, i) => i !== index))}>Remove</Button>}</div>)}<Button type="button" variant="outline" size="sm" className="w-fit" onClick={() => set([...rows, ''])}><Plus className="mr-1 size-3" />Add {label.toLowerCase()}</Button></div>;
}

function LabelsList({value, set}: {value: any; set: (key: string, value: any) => void}) {
	const labels = Array.isArray(value) ? value : value?.key ? [{key: value.key, value: value.value || ''}] : [];
	const update = (index: number, patch: any) => set('labels', labels.map((item: any, i: number) => i === index ? {...item, ...patch} : item));
	return (
		<div className="flex max-w-2xl flex-col gap-3">
			<div><h4 className="text-sm font-medium">Labels</h4><p className="text-xs text-muted-foreground">Add key-value labels to your service.</p></div>
			{labels.map((label: any, index: number) => <div key={index} className="flex flex-col gap-3 rounded-md  p-3"><Field label="Label key"><Input value={label.key || ''} placeholder="com.example.app.name" onChange={e => update(index, {key: e.target.value})} /></Field><Field label="Label value"><Input value={label.value || ''} placeholder="my-app" onChange={e => update(index, {value: e.target.value})} /></Field><Button type="button" variant="destructive" size="sm" className="w-fit" onClick={() => set('labels', labels.filter((_: any, i: number) => i !== index))}>Remove</Button></div>)}
			<Button type="button" variant="outline" size="sm" className="w-fit" onClick={() => set('labels', [...labels, {key: '', value: ''}])}><Plus className="mr-1 size-3" />Add label</Button>
		</div>
	);
}

function NetworkList({value, set}: {value: any; set: (key: string, value: any) => void}) {
	const networks = Array.isArray(value) ? value : value?.Target ? [value] : [];
	const update = (index: number, patch: any) =>
		set('networks', networks.map((item: any, i: number) => i === index ? {...item, ...patch} : item));
	return (
		<div className="flex max-w-2xl flex-col gap-3">
			{networks.map((network: any, index: number) => (
				<div key={index} className="space-y-3 rounded-lg  p-3">
					<div className="flex flex-col gap-3">
						<Field label="Network target">
							<Input value={network.Target || ''} onChange={e => update(index, {Target: e.target.value})} placeholder="openoxide-network" />
						</Field>
						<Button type="button" variant="ghost" size="icon" title="Remove network" onClick={() => set('networks', networks.filter((_: any, i: number) => i !== index))}><Trash2 className="size-4" /></Button>
					</div>
					<Field label="Aliases (comma separated)"><Input value={(network.Aliases || []).join?.(', ') || network.Aliases || ''} onChange={e => update(index, {Aliases: e.target.value})} /></Field>
					<div className="space-y-2">
						<span className="text-xs font-medium">Driver options</span>
						{(network.DriverOptsEntries || []).map((option: any, optionIndex: number) => <div key={optionIndex} className="flex flex-col gap-2 rounded-md  p-2"><Input value={option.key} placeholder="driver option key" onChange={e => update(index, {DriverOptsEntries: network.DriverOptsEntries.map((item: any, i: number) => i === optionIndex ? {...item, key: e.target.value} : item)})} /><Input value={option.value} placeholder="value" onChange={e => update(index, {DriverOptsEntries: network.DriverOptsEntries.map((item: any, i: number) => i === optionIndex ? {...item, value: e.target.value} : item)})} /><Button type="button" variant="ghost" size="sm" className="w-fit" title="Remove driver option" onClick={() => update(index, {DriverOptsEntries: network.DriverOptsEntries.filter((_: any, i: number) => i !== optionIndex)})}><Trash2 className="mr-1 size-4" />Remove option</Button></div>)}
						<Button type="button" variant="outline" size="sm" onClick={() => update(index, {DriverOptsEntries: [...(network.DriverOptsEntries || []), {key: '', value: ''}]})}><Plus className="mr-1 size-3" />Add option</Button>
					</div>
				</div>
			))}
			<Button type="button" variant="outline" size="sm" className="w-fit" onClick={() => set('networks', [...networks, {Target: '', Aliases: '', DriverOptsEntries: []}])}><Plus className="mr-1 size-3" />Add network</Button>
		</div>
	);
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
		'Network target': 'Network to attach this service to.',
		'Aliases (comma separated)': 'Additional names available through service discovery.',
		'Label key': 'Metadata key used to identify or filter the service.',
		'Label value': 'Value associated with the service label.',
		'Endpoint Mode': 'VIP uses a virtual IP; DNSRR resolves each task directly.',
		'Protocol': 'Transport protocol used by the published port.',
		'Target': 'Port exposed by the container.',
		'Published': 'Port exposed on the Swarm endpoint.',
		'Publish mode': 'Ingress routes through the routing mesh; host binds locally.',
		'nofile soft': 'Soft per-process open-file limit.',
		'nofile hard': 'Hard per-process open-file limit.',
	};
	return values[label] || '';
}
