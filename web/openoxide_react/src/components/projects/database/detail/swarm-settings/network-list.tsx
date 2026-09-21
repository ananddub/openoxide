import {Input} from '#/components/ui/input';
import {Button} from '#/components/ui/button';
import {Plus, Trash2} from 'lucide-react';
import {Field} from './field';

/** Editable network attachment list (target, aliases, driver options). */
export function NetworkList({
	value,
	set,
}: {
	value: any;
	set: (key: string, value: any) => void;
}) {
	const networks = Array.isArray(value)
		? value
		: value?.Target
			? [value]
			: [];
	const update = (index: number, patch: any) =>
		set(
			'networks',
			networks.map((item: any, i: number) =>
				i === index ? {...item, ...patch} : item,
			),
		);
	return (
		<div className="flex max-w-2xl flex-col gap-3">
			{networks.map((network: any, index: number) => (
				<div key={index} className="space-y-3 rounded-lg  p-3">
					<div className="flex flex-col gap-3">
						<Field label="Network target">
							<Input
								value={network.Target || ''}
								onChange={e => update(index, {Target: e.target.value})}
								placeholder="openoxide-network"
							/>
						</Field>
						<Button
							type="button"
							variant="ghost"
							size="icon"
							title="Remove network"
							onClick={() =>
								set(
									'networks',
									networks.filter((_: any, i: number) => i !== index),
								)
							}>
							<Trash2 className="size-4" />
						</Button>
					</div>
					<Field label="Aliases (comma separated)">
						<Input
							value={
								(network.Aliases || []).join?.(', ') ||
								network.Aliases ||
								''
							}
							onChange={e => update(index, {Aliases: e.target.value})}
						/>
					</Field>
					<DriverOpts
						network={network}
						onChange={patch => update(index, patch)}
					/>
				</div>
			))}
			<Button
				type="button"
				variant="outline"
				size="sm"
				className="w-fit"
				onClick={() =>
					set('networks', [
						...networks,
						{Target: '', Aliases: '', DriverOptsEntries: []},
					])
				}>
				<Plus className="mr-1 size-3" />
				Add network
			</Button>
		</div>
	);
}

function DriverOpts({
	network,
	onChange,
}: {
	network: any;
	onChange: (patch: any) => void;
}) {
	const entries = network.DriverOptsEntries || [];
	const patchEntries = (next: any[]) =>
		onChange({DriverOptsEntries: next});
	return (
		<div className="space-y-2">
			<span className="text-xs font-medium">Driver options</span>
			{entries.map((option: any, optionIndex: number) => (
				<div
					key={optionIndex}
					className="flex flex-col gap-2 rounded-md  p-2">
					<Input
						value={option.key}
						placeholder="driver option key"
						onChange={e =>
							patchEntries(
								entries.map((item: any, i: number) =>
									i === optionIndex
										? {...item, key: e.target.value}
										: item,
								),
							)
						}
					/>
					<Input
						value={option.value}
						placeholder="value"
						onChange={e =>
							patchEntries(
								entries.map((item: any, i: number) =>
									i === optionIndex
										? {...item, value: e.target.value}
										: item,
								),
							)
						}
					/>
					<Button
						type="button"
						variant="ghost"
						size="sm"
						className="w-fit"
						title="Remove driver option"
						onClick={() =>
							patchEntries(
								entries.filter((_: any, i: number) => i !== optionIndex),
							)
						}>
						<Trash2 className="mr-1 size-4" />
						Remove option
					</Button>
				</div>
			))}
			<Button
				type="button"
				variant="outline"
				size="sm"
				onClick={() => patchEntries([...entries, {key: '', value: ''}])}>
				<Plus className="mr-1 size-3" />
				Add option
			</Button>
		</div>
	);
}
