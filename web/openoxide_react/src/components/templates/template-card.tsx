import {Badge} from '#/components/ui/badge';
import {TemplateLogo} from './template-logo';
import type {TemplateMeta} from '#/api/live';

/** Gallery card: logo, name + version, clamped description, tag chips. */
export function TemplateCard({
	template,
	onOpen,
}: {
	template: TemplateMeta;
	onOpen: (template: TemplateMeta) => void;
}) {
	return (
		<button
			type="button"
			onClick={() => onOpen(template)}
			className="flex cursor-pointer flex-col items-start gap-3 rounded-xl border bg-card p-4 text-left transition-colors duration-150 hover:border-foreground/25 hover:bg-accent/40">
			<div className="flex w-full items-center gap-3">
				<TemplateLogo template={template} className="size-9 shrink-0" />
				<div className="min-w-0 grow">
					<div className="truncate text-sm font-semibold text-foreground">
						{template.name}
					</div>
					<div className="truncate font-mono text-[11px] text-muted-foreground/70">
						{template.version}
					</div>
				</div>
			</div>
			<p className="line-clamp-2 w-full text-xs leading-relaxed text-muted-foreground/80">
				{template.description}
			</p>
			{template.tags?.length > 0 && (
				<div className="flex flex-wrap gap-1">
					{template.tags.slice(0, 3).map(tag => (
						<Badge
							key={tag}
							variant="secondary"
							className="font-mono text-[10px] font-normal">
							{tag}
						</Badge>
					))}
				</div>
			)}
		</button>
	);
}
