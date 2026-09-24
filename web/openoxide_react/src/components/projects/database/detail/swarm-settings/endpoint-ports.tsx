import {Input} from '#/components/ui/input';
import {DataTable} from './list-shell';
import {InlineSelect} from './inline-controls';

const numberCell =
	'h-9 min-w-[72px] text-center tabular-nums [appearance:textfield] [&::-webkit-inner-spin-button]:appearance-none [&::-webkit-outer-spin-button]:appearance-none';

const toNumber = (raw: string) =>
	raw === '' ? undefined : Number.isNaN(Number(raw)) ? raw : Number(raw);

type Port = {
	Protocol: string;
	PublishMode: string;
	TargetPort: any;
	PublishedPort: any;
};

/** Editable published-port list rendered as a four-column table. */
export function EndpointPorts({
	value,
	set,
}: {
	value: any;
	set: (value: any[]) => void;
}) {
	const ports: Port[] = Array.isArray(value) ? value : [];
	return (
		<DataTable
			title="Published ports"
			description="Expose service ports through the Swarm endpoint."
			addLabel="Add port"
			empty="No published ports yet."
			rows={ports}
			columns={[
				{
					header: 'Protocol',
					cell: (row, update) => (
						<InlineSelect
							value={row.Protocol || 'tcp'}
							onChange={next => update({Protocol: next})}
							options={[
								{value: 'tcp', label: 'TCP'},
								{value: 'udp', label: 'UDP'},
								{value: 'sctp', label: 'SCTP'},
							]}
						/>
					),
				},
				{
					header: 'Mode',
					cell: (row, update) => (
						<InlineSelect
							value={row.PublishMode || 'ingress'}
							onChange={next => update({PublishMode: next})}
							options={[
								{value: 'ingress', label: 'Ingress'},
								{value: 'host', label: 'Host'},
							]}
						/>
					),
				},
				{
					header: 'Target',
					cell: (row, update) => (
						<Input
							type="number"
							inputMode="numeric"
							value={row.TargetPort ?? ''}
							placeholder="0"
							className={numberCell}
							onChange={e =>
								update({TargetPort: toNumber(e.target.value)})
							}
						/>
					),
				},
				{
					header: 'Published',
					cell: (row, update) => (
						<Input
							type="number"
							inputMode="numeric"
							value={row.PublishedPort ?? ''}
							placeholder="0"
							className={numberCell}
							onChange={e =>
								update({PublishedPort: toNumber(e.target.value)})
							}
						/>
					),
				},
			]}
			onAdd={() =>
				set([
					...ports,
					{
						Protocol: 'tcp',
						TargetPort: 0,
						PublishedPort: 0,
						PublishMode: 'ingress',
					},
				])
			}
			onUpdate={(index, patch) =>
				set(
					ports.map((item, i) =>
						i === index ? {...item, ...patch} : item,
					),
				)
			}
			onRemove={index => set(ports.filter((_, i) => i !== index))}
		/>
	);
}
