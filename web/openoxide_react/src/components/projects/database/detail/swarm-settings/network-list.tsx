import {useState} from 'react';
import {ChevronRight, Plus, X} from 'lucide-react';
import {Input} from '#/components/ui/input';
import {Button} from '#/components/ui/button';
import {cn} from '#/api/utils';
import {DataTable} from './list-shell';

type DriverOpt = {key: string; value: string};
type Network = {
	Target: string;
	Aliases: any;
	DriverOptsEntries: DriverOpt[];
};

/** Editable network list. Each network is one clean row (target + aliases);
 * the rarely-used driver options live behind an "Options" disclosure so the
 * table never looks like it has two competing rows. */
export function NetworkList({
	value,
	set,
}: {
	value: any;
	set: (key: string, value: any) => void;
}) {
	const networks: Network[] = Array.isArray(value)
		? value
		: value?.Target
			? [value]
			: [];
	const [open, setOpen] = useState<Record<number, boolean>>({});
	const isOpen = (index: number, count: number) =>
		open[index] ?? count > 0;
	const aliasText = (network: Network) =>
		Array.isArray(network.Aliases)
			? network.Aliases.join(', ')
			: network.Aliases || '';
	return (
		<DataTable
			title="Networks"
			description="Attach this service to overlay networks."
			addLabel="Add network"
			empty="No networks attached yet."
			rows={networks}
			columns={[
				{
					header: 'Network target',
					cell: (row, update) => (
						<Input
							value={row.Target || ''}
							placeholder="openoxide-network"
							className="h-9 min-w-[120px]"
							onChange={e => update({Target: e.target.value})}
						/>
					),
				},
				{
					header: 'Aliases',
					cell: (row, update) => (
						<Input
							value={aliasText(row)}
							placeholder="api, db"
							className="h-9 min-w-[120px]"
							onChange={e => update({Aliases: e.target.value})}
						/>
					),
				},
				{
					header: 'Driver options',
					headClass: 'w-1',
					cellClass: 'w-1',
					cell: (row, _update, index) => {
						const count = (row.DriverOptsEntries || []).length;
						const expanded = isOpen(index, count);
						return (
							<button
								type="button"
								onClick={() =>
									setOpen(prev => ({
										...prev,
										[index]: !expanded,
									}))
								}
								aria-expanded={expanded}
								className="flex items-center gap-1 rounded-md px-1.5 py-1 text-xs whitespace-nowrap text-muted-foreground transition-colors hover:bg-muted hover:text-foreground">
								<ChevronRight
									className={cn(
										'size-3.5 transition-transform',
										expanded && 'rotate-90',
									)}
								/>
								{count > 0 ? `${count}` : 'None'}
							</button>
						);
					},
				},
			]}
			renderDetail={(row, update, index) => {
				const entries = row.DriverOptsEntries || [];
				if (!isOpen(index, entries.length)) return null;
				return (
					<DriverOpts
						target={row.Target}
						entries={entries}
						onChange={next => update({DriverOptsEntries: next})}
					/>
				);
			}}
			onAdd={() =>
				set('networks', [
					...networks,
					{Target: '', Aliases: '', DriverOptsEntries: []},
				])
			}
			onUpdate={(index, patch) =>
				set(
					'networks',
					networks.map((item, i) =>
						i === index ? {...item, ...patch} : item,
					),
				)
			}
			onRemove={index =>
				set(
					'networks',
					networks.filter((_, i) => i !== index),
				)
			}
		/>
	);
}

/** Nested driver-option editor, visually marked as a child of one network. */
function DriverOpts({
	target,
	entries,
	onChange,
}: {
	target: string;
	entries: DriverOpt[];
	onChange: (next: DriverOpt[]) => void;
}) {
	const patch = (index: number, item: Partial<DriverOpt>) =>
		onChange(
			entries.map((entry, i) =>
				i === index ? {...entry, ...item} : entry,
			),
		);
	return (
		<div className="ml-1 border-l-2 border-border pl-3">
			<div className="flex items-center justify-between gap-3">
				<span className="truncate text-xs text-muted-foreground">
					Driver options
					{target ? (
						<span className="text-foreground"> · {target}</span>
					) : null}
				</span>
				<Button
					type="button"
					variant="ghost"
					size="sm"
					onClick={() => onChange([...entries, {key: '', value: ''}])}
					className="h-7 shrink-0 gap-1 px-2 text-xs">
					<Plus className="size-3.5" /> Add option
				</Button>
			</div>
			{entries.length === 0 ? (
				<p className="mt-1 text-[13px] text-muted-foreground">
					No driver options for this network.
				</p>
			) : (
				<div className="mt-2 flex flex-col gap-2">
					{entries.map((option, index) => (
						<div key={index} className="flex items-center gap-2">
							<Input
								value={option.key}
								placeholder="driver option key"
								className="h-8 min-w-0 flex-1"
								onChange={e => patch(index, {key: e.target.value})}
							/>
							<Input
								value={option.value}
								placeholder="value"
								className="h-8 min-w-0 flex-1"
								onChange={e => patch(index, {value: e.target.value})}
							/>
							<button
								type="button"
								aria-label="Remove driver option"
								onClick={() =>
									onChange(entries.filter((_, i) => i !== index))
								}
								className="flex size-7 shrink-0 items-center justify-center rounded-md text-muted-foreground transition-colors hover:bg-muted hover:text-foreground">
								<X className="size-4" />
							</button>
						</div>
					))}
				</div>
			)}
		</div>
	);
}
