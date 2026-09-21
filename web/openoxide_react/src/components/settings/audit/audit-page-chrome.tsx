import {Download, Loader2} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {cn} from '#/api/utils';
import {CATEGORIES, type CategoryId} from './audit-format';

/** Page title + Export CSV button. */
export function AuditHeader({
	exporting,
	onExport,
}: {
	exporting: boolean;
	onExport: () => void;
}) {
	return (
		<div className="flex flex-wrap items-end justify-between gap-4">
			<div className="flex flex-col gap-1">
				<h2 className="text-sm font-medium text-foreground">Audit Log</h2>
				<p className="text-xs text-muted-foreground">
					A readable history of who did what in this organization.
				</p>
			</div>
			<Button
				variant="outline"
				size="sm"
				onClick={onExport}
				disabled={exporting}
				className="h-8 gap-1.5 text-xs">
				{exporting ? (
					<Loader2 className="size-3.5 animate-spin" />
				) : (
					<Download className="size-3.5" />
				)}
				Export CSV
			</Button>
		</div>
	);
}

/** All / Team / Access tabs with per-category counts. */
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
		<div className="flex items-center gap-1 border-b ">
			{CATEGORIES.map(c => (
				<button
					key={c.id}
					type="button"
					onClick={() => onSelect(c.id)}
					className={cn(
						'flex items-center gap-2 border-b-2 px-3 py-2 text-sm font-medium transition-colors',
						category === c.id
							? 'border-foreground text-foreground'
							: ' text-muted-foreground hover:text-foreground',
					)}>
					<c.icon className="size-4" strokeWidth={1.7} />
					{c.label}
					<span className="rounded-md bg-muted/60 px-1.5 py-0.5 font-mono text-[10px] text-muted-foreground">
						{counts[c.id]}
					</span>
				</button>
			))}
		</div>
	);
}
