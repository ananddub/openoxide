import {cn} from '#/api/utils';
import {CATEGORIES, type CategoryId} from './audit-format';

/**
 * Category picker — the segmented `rounded-lg border bg-card p-1` group
 * the resource toolbars use for view toggles, with a quiet count per
 * segment.
 */
export function AuditCategoryTabs({
	category,
	counts,
	onSelect,
}: {
	category: CategoryId;
	counts: Record<CategoryId, number>;
	onSelect: (c: CategoryId) => void;
}) {
	return (
		<div className="flex h-9 max-w-full shrink-0 items-center gap-0.5 overflow-x-auto rounded-lg border bg-card p-0.5">
			{CATEGORIES.map(c => (
				<button
					key={c.id}
					type="button"
					onClick={() => onSelect(c.id)}
					className={cn(
						'flex shrink-0 self-stretch cursor-pointer items-center gap-1.5 rounded-md px-2.5 text-[13px] font-medium text-muted-foreground transition-colors hover:text-foreground',
						category === c.id && 'bg-accent text-foreground',
					)}>
					<c.icon className="size-3.5" strokeWidth={1.7} />
					{c.label}
					<span className="font-mono text-[11px] opacity-70">
						{counts[c.id]}
					</span>
				</button>
			))}
		</div>
	);
}
