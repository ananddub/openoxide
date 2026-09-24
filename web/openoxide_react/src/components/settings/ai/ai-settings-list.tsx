import {
	CheckCircle2,
	Cpu,
	Link2,
	Pencil,
	Trash2,
	XCircle,
} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {
	ListCard,
	ListRow,
	type ListView,
	RowTile,
	ViewCard,
	ViewGrid,
} from '#/components/settings/list-card';
import {AiProviderIcon} from './ai-provider-icon';
import type {AiSetting} from './ai-types';

type Props = {
	items: AiSetting[];
	view: ListView;
	onEdit: (item: AiSetting) => void;
	onDelete: (item: AiSetting) => void;
};

function ProviderTile({item}: {item: AiSetting}) {
	return (
		<RowTile>
			<AiProviderIcon
				name={item.provider || item.name}
				className="size-4"
			/>
		</RowTile>
	);
}

function StatusChip({enabled}: {enabled: boolean}) {
	return enabled ? (
		<span className="flex shrink-0 items-center gap-1 rounded-md bg-emerald-500/10 px-1.5 py-0.5 text-[10px] font-medium text-emerald-500">
			<CheckCircle2 className="size-3" /> Enabled
		</span>
	) : (
		<span className="flex shrink-0 items-center gap-1 rounded-md bg-foreground/[0.05] px-1.5 py-0.5 text-[10px] font-medium text-muted-foreground">
			<XCircle className="size-3" /> Disabled
		</span>
	);
}

function ProviderChip({item}: {item: AiSetting}) {
	return (
		<span className="shrink-0 rounded-md bg-foreground/[0.05] px-1.5 py-0.5 text-[10px] font-medium text-muted-foreground">
			{item.provider || 'Custom'}
		</span>
	);
}

function ModelLine({item}: {item: AiSetting}) {
	return (
		<span className="flex items-center gap-1 font-mono text-[11px]">
			<Cpu className="size-3 opacity-70" />
			{item.model}
		</span>
	);
}

function UrlLine({item}: {item: AiSetting}) {
	return (
		<span className="flex min-w-0 items-center gap-1 truncate">
			<Link2 className="size-3 shrink-0 opacity-60" />
			{item.api_url}
		</span>
	);
}

function Actions({
	item,
	onEdit,
	onDelete,
}: {
	item: AiSetting;
	onEdit: (item: AiSetting) => void;
	onDelete: (item: AiSetting) => void;
}) {
	return (
		<>
			<Button
				variant="ghost"
				size="icon"
				title="Edit provider"
				onClick={() => onEdit(item)}
				className="size-7 rounded-lg text-muted-foreground hover:text-foreground">
				<Pencil className="size-3.5" />
			</Button>
			<Button
				variant="ghost"
				size="icon"
				title="Delete provider"
				onClick={() => onDelete(item)}
				className="size-7 rounded-lg text-muted-foreground hover:bg-destructive/10 hover:text-destructive">
				<Trash2 className="size-3.5" />
			</Button>
		</>
	);
}

/** Provider list — grid cards or one bordered list card, same grammar. */
export function AiSettingsList({items, view, onEdit, onDelete}: Props) {
	if (view === 'grid') {
		return (
			<ViewGrid>
				{items.map(item => (
					<ViewCard key={item.id}>
						<div className="flex min-w-0 items-start gap-3">
							<ProviderTile item={item} />
							<div className="min-w-0 flex-1">
								<span className="block truncate text-sm font-medium text-foreground">
									{item.name}
								</span>
								<p className="mt-0.5 truncate text-[11px] text-muted-foreground">
									<UrlLine item={item} />
								</p>
							</div>
						</div>
						<div className="flex items-end justify-between gap-2">
							<div className="flex min-w-0 flex-wrap items-center gap-1.5">
								<ProviderChip item={item} />
								<StatusChip enabled={!!item.is_enabled} />
							</div>
							<div className="flex shrink-0 items-center gap-1 opacity-0 transition-opacity group-focus-within:opacity-100 group-hover:opacity-100">
								<Actions item={item} onEdit={onEdit} onDelete={onDelete} />
							</div>
						</div>
					</ViewCard>
				))}
			</ViewGrid>
		);
	}

	return (
		<ListCard>
			{items.map(item => (
				<ListRow
					key={item.id}
					icon={<ProviderTile item={item} />}
					title={item.name}
					badges={
						<>
							<ProviderChip item={item} />
							<StatusChip enabled={!!item.is_enabled} />
						</>
					}
					subtitle={
						<span className="flex items-center gap-3 truncate">
							<ModelLine item={item} />
							<UrlLine item={item} />
						</span>
					}>
					<Actions item={item} onEdit={onEdit} onDelete={onDelete} />
				</ListRow>
			))}
		</ListCard>
	);
}
