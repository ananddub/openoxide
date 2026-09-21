import {
	Bot,
	Pencil,
	Plus,
	Trash2,
	Cpu,
	CheckCircle2,
	XCircle,
	Link2,
} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {
	ListCard,
	ListRow,
	ListEmpty,
	RowTile,
} from '#/components/settings/list-card';
import type {AiSetting} from './ai-types';

type Props = {
	items: AiSetting[];
	loading: boolean;
	onCreate: () => void;
	onEdit: (item: AiSetting) => void;
	onDelete: (id: number) => void;
};

/** OpenShip's list-card rows: provider tile, name + model/URL, status chip, quiet actions. */
export function AiSettingsList({
	items,
	loading,
	onCreate,
	onEdit,
	onDelete,
}: Props) {
	return (
		<ListCard
			title={`${items.length} ${items.length === 1 ? 'model' : 'models'} configured`}
			description="Providers used for log diagnosis and stack generation.">
			{loading ? (
				<div className="space-y-2 p-5">
					{[...Array(2)].map((_, i) => (
						<div
							key={i}
							className="h-14 animate-pulse rounded-xl bg-muted/30"
						/>
					))}
				</div>
			) : items.length === 0 ? (
				<ListEmpty
					icon={Bot}
					title="No AI configurations yet"
					description="Connect an OpenAI, Anthropic, Gemini, or local Ollama provider for smart log diagnosis and stack generation."
					action={
						<Button size="sm" onClick={onCreate} className="gap-1.5">
							<Plus className="size-3.5" /> Configure provider
						</Button>
					}
				/>
			) : (
				<div className="divide-y divide-border/40">
					{items.map(item => (
						<ListRow
							key={item.id}
							icon={
								<RowTile className="bg-primary/10 text-primary">
									<Bot className="size-4" />
								</RowTile>
							}
							title={item.name}
							badges={
								<>
									<span className="rounded-md bg-muted/50 px-1.5 py-0.5 text-[10px] font-medium tracking-wide text-muted-foreground uppercase">
										{item.provider || 'Custom'}
									</span>
									{item.is_enabled ? (
										<span className="flex items-center gap-1 rounded-full bg-emerald-500/10 px-2 py-0.5 text-[10px] font-medium text-emerald-500">
											<CheckCircle2 className="size-3" /> Enabled
										</span>
									) : (
										<span className="flex items-center gap-1 rounded-full bg-muted px-2 py-0.5 text-[10px] font-medium text-muted-foreground">
											<XCircle className="size-3" /> Disabled
										</span>
									)}
								</>
							}
							subtitle={
								<span className="flex items-center gap-3 truncate">
									<span className="flex items-center gap-1 font-mono text-[11px]">
										<Cpu className="size-3 opacity-70" />
										{item.model}
									</span>
									<span className="flex items-center gap-1 truncate">
										<Link2 className="size-3 shrink-0 opacity-60" />
										{item.api_url}
									</span>
								</span>
							}>
							<Button
								variant="ghost"
								size="icon"
								title="Edit"
								onClick={() => onEdit(item)}
								className="size-7 rounded-lg text-muted-foreground hover:text-foreground">
								<Pencil className="size-3.5" />
							</Button>
							<Button
								variant="ghost"
								size="icon"
								title="Delete"
								onClick={() => onDelete(item.id)}
								className="size-7 rounded-lg text-muted-foreground hover:bg-destructive/10 hover:text-destructive">
								<Trash2 className="size-3.5" />
							</Button>
						</ListRow>
					))}
				</div>
			)}
		</ListCard>
	);
}
