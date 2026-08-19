import {
	Bot,
	Loader2,
	Pencil,
	Plus,
	Trash2,
	Cpu,
	CheckCircle2,
	XCircle,
	Link2,
} from 'lucide-react';
import {Badge} from '#/components/ui/badge';
import {Button} from '#/components/ui/button';
import type {AiSetting} from './ai-types';

type Props = {
	items: AiSetting[];
	loading: boolean;
	onCreate: () => void;
	onEdit: (item: AiSetting) => void;
	onDelete: (id: number) => void;
};

export function AiSettingsList({
	items,
	loading,
	onCreate,
	onEdit,
	onDelete,
}: Props) {
	if (loading) {
		return (
			<div className="flex flex-col items-center justify-center gap-3 py-16 text-muted-foreground">
				<Loader2 className="size-6 animate-spin text-primary" />
				<span className="text-xs font-medium">
					Loading AI configurations...
				</span>
			</div>
		);
	}

	if (items.length === 0) {
		return (
			<div className="flex flex-col items-center justify-center gap-3 rounded-2xl border border-dashed border-border/80 bg-muted/5 p-6 py-16 text-center">
				<div className="flex size-12 items-center justify-center rounded-2xl border border-border/60 bg-muted/60 text-muted-foreground">
					<Bot className="size-6 opacity-60" />
				</div>
				<div>
					<p className="text-sm font-semibold text-foreground">
						No AI configurations yet
					</p>
					<p className="mt-0.5 max-w-sm text-xs text-muted-foreground">
						Connect an OpenAI, Anthropic, Gemini, or local Ollama provider
						for smart log diagnosis and stack generation.
					</p>
				</div>
				<Button
					size="sm"
					onClick={onCreate}
					className="mt-2 gap-1.5 rounded-lg bg-primary text-xs font-semibold text-primary-foreground">
					<Plus className="size-3.5" /> Configure AI Provider
				</Button>
			</div>
		);
	}

	return (
		<div className="flex flex-col gap-3">
			{items.map(item => (
				<div
					key={item.id}
					className="flex flex-col justify-between gap-4 rounded-2xl border border-border/80 bg-card p-4 transition-all hover:border-primary/40 hover:shadow-xs sm:flex-row sm:items-center">
					<div className="flex min-w-0 items-center gap-3.5">
						<div className="flex size-11 shrink-0 items-center justify-center rounded-xl border border-primary/20 bg-primary/10 text-primary">
							<Bot className="size-5" />
						</div>

						<div className="flex min-w-0 flex-col gap-1">
							<div className="flex flex-wrap items-center gap-2">
								<span className="truncate text-sm font-bold text-foreground">
									{item.name}
								</span>
								<Badge
									variant="outline"
									className="text-[10px] font-semibold tracking-wider uppercase">
									{item.provider || 'Custom'}
								</Badge>
								{item.is_enabled ? (
									<span className="flex items-center gap-1 rounded-full border border-emerald-500/20 bg-emerald-500/10 px-2 py-0.5 text-[10px] font-bold text-emerald-500">
										<CheckCircle2 className="size-3" /> Enabled
									</span>
								) : (
									<span className="flex items-center gap-1 rounded-full border border-border bg-muted px-2 py-0.5 text-[10px] font-bold text-muted-foreground">
										<XCircle className="size-3" /> Disabled
									</span>
								)}
							</div>

							<div className="flex items-center gap-3 truncate text-xs text-muted-foreground">
								<span className="flex items-center gap-1 rounded bg-muted/40 px-1.5 py-0.5 font-mono text-[11px] font-medium text-foreground/80">
									<Cpu className="size-3 opacity-70" />
									{item.model}
								</span>
								<span className="flex items-center gap-1 truncate text-muted-foreground/80">
									<Link2 className="size-3 shrink-0 opacity-60" />
									{item.api_url}
								</span>
							</div>
						</div>
					</div>

					<div className="flex shrink-0 items-center gap-1.5 self-end sm:self-center">
						<Button
							variant="ghost"
							size="icon"
							title="Edit AI Configuration"
							onClick={() => onEdit(item)}
							className="size-8 rounded-lg text-muted-foreground hover:text-foreground">
							<Pencil className="size-3.5" />
						</Button>
						<Button
							variant="ghost"
							size="icon"
							title="Delete AI Configuration"
							onClick={() => onDelete(item.id)}
							className="size-8 rounded-lg text-muted-foreground hover:bg-destructive/10 hover:text-destructive">
							<Trash2 className="size-3.5" />
						</Button>
					</div>
				</div>
			))}
		</div>
	);
}
