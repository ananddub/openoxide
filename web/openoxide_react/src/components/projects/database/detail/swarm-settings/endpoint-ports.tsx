import {Button} from '#/components/ui/button';
import {Plus} from 'lucide-react';
import {SwarmSelectField} from './select-field';
import {Field, Num} from './field';

/** Editable published-port list for the swarm endpoint spec form. */
export function EndpointPorts({
	value,
	set,
}: {
	value: any;
	set: (value: any[]) => void;
}) {
	const ports = Array.isArray(value) ? value : [];
	const update = (index: number, patch: any) =>
		set(
			ports.map((port: any, i: number) =>
				i === index ? {...port, ...patch} : port,
			),
		);
	return (
		<div className="space-y-2">
			<div>
				<h4 className="text-sm font-medium">Published Ports</h4>
				<p className="text-xs text-muted-foreground">
					Expose service ports through the Swarm endpoint.
				</p>
			</div>
			{ports.map((port: any, index: number) => (
				<div key={index} className="flex flex-col gap-3 rounded-md  p-3">
					<Field label="Protocol">
						<SwarmSelectField
							label=""
							value={port.Protocol || 'tcp'}
							onChange={next => update(index, {Protocol: next})}
							options={[
								{value: 'tcp', label: 'TCP'},
								{value: 'udp', label: 'UDP'},
								{value: 'sctp', label: 'SCTP'},
							]}
						/>
					</Field>
					<Num
						label="Target"
						value={port.TargetPort}
						set={v => update(index, {TargetPort: v})}
					/>
					<Num
						label="Published"
						value={port.PublishedPort}
						set={v => update(index, {PublishedPort: v})}
					/>
					<Field label="Publish mode">
						<SwarmSelectField
							label=""
							value={port.PublishMode || 'ingress'}
							onChange={next => update(index, {PublishMode: next})}
							options={[
								{value: 'ingress', label: 'Ingress'},
								{value: 'host', label: 'Host'},
							]}
						/>
					</Field>
					<Button
						type="button"
						variant="destructive"
						size="sm"
						className="w-fit"
						onClick={() =>
							set(ports.filter((_: any, i: number) => i !== index))
						}>
						Remove
					</Button>
				</div>
			))}
			<Button
				type="button"
				variant="outline"
				size="sm"
				className="w-fit"
				onClick={() =>
					set([
						...ports,
						{
							Protocol: 'tcp',
							TargetPort: 0,
							PublishedPort: 0,
							PublishMode: 'ingress',
						},
					])
				}>
				<Plus className="mr-1 size-3" />
				Add port
			</Button>
		</div>
	);
}
