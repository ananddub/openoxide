import {Badge} from '#/components/ui/badge';
import {TemplateLogo} from './template-logo';
import type {TemplateMeta} from '#/api/live';

/** Compact list row for the template registry. */
export function TemplateListItem({
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
			className="flex w-full cursor-pointer items-center gap-3 px-4 py-3 text-left transition-colors duration-150 hover:bg-accent/40">
			<TemplateLogo template={template} className="size-8 shrink-0" />
			<div className="w-40 shrink-0 truncate text-sm font-semibold text-foreground">
				{template.name}
			</div>
			<div className="min-w-0 grow truncate text-xs text-muted-foreground/80">
				{template.description}
			</div>
			{template.tags?.[0] && (
				<Badge
					variant="secondary"
					className="hidden shrink-0 font-mono text-[10px] font-normal sm:inline-flex">
					{template.tags[0]}
				</Badge>
			)}
			<Badge
				variant="outline"
				className="hidden shrink-0 font-mono text-[10px] font-normal md:inline-flex">
				{template.version}
			</Badge>
		</button>
	);
}
