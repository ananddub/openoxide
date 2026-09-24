import {Fragment, type ReactNode} from 'react';
import {Plus, Trash2} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {cn} from '#/api/utils';

export type Column<T> = {
	header: string;
	cell: (
		row: T,
		update: (patch: Partial<T>) => void,
		index: number,
	) => ReactNode;
	headClass?: string;
	cellClass?: string;
};

/** A real, editable table: one header row of column names, then one row per
 * record with inline controls in each cell and a single delete icon at the
 * end. Used by the swarm key/value style editors (labels, ports, networks)
 * so the fields read as columns instead of repeating labels on every row. */
export function DataTable<T extends Record<string, any>>({
	title,
	description,
	addLabel,
	empty,
	columns,
	rows,
	onAdd,
	onUpdate,
	onRemove,
	renderDetail,
}: {
	title: string;
	description?: string;
	addLabel: string;
	empty: string;
	columns: Column<T>[];
	rows: T[];
	onAdd: () => void;
	onUpdate: (index: number, patch: Partial<T>) => void;
	onRemove: (index: number) => void;
	renderDetail?: (
		row: T,
		update: (patch: Partial<T>) => void,
		index: number,
	) => ReactNode;
}) {
	return (
		<div className="flex flex-col gap-3">
			<div className="flex items-start justify-between gap-3">
				<div className="min-w-0">
					<h4 className="text-sm font-medium text-foreground">{title}</h4>
					{description && (
						<p className="text-[13px] leading-snug text-muted-foreground">
							{description}
						</p>
					)}
				</div>
				<Button
					type="button"
					variant="outline"
					onClick={onAdd}
					className="h-8 shrink-0 gap-1.5 px-3 text-xs">
					<Plus className="size-3.5" /> {addLabel}
				</Button>
			</div>
			{rows.length === 0 ? (
				<div className="rounded-xl border border-dashed border-border bg-muted/20 px-4 py-8 text-center text-sm text-muted-foreground">
					{empty}
				</div>
			) : (
				<div className="overflow-hidden rounded-xl border border-border">
					<div className="overflow-x-auto">
						<table className="w-full border-collapse text-sm">
							<thead>
								<tr className="bg-muted/40">
									{columns.map((col, i) => (
										<th
											key={i}
											className={cn(
												'border-b border-border px-2.5 py-2 text-left text-[11px] font-semibold tracking-wide text-muted-foreground uppercase',
												col.headClass,
											)}>
											{col.header}
										</th>
									))}
									<th className="w-9 border-b border-border px-2 py-2" />
								</tr>
							</thead>
							<tbody>
								{rows.map((row, index) => {
									const detail = renderDetail?.(
										row,
										patch => onUpdate(index, patch),
										index,
									);
									return (
										<Fragment key={index}>
											<tr className="border-b border-border/70 align-middle last:border-0">
												{columns.map((col, i) => (
													<td
														key={i}
														className={cn('px-2.5 py-2', col.cellClass)}>
														{col.cell(
															row,
															patch => onUpdate(index, patch),
															index,
														)}
													</td>
												))}
												<td className="px-2 py-2 text-right">
													<Button
														type="button"
														variant="ghost"
														size="icon"
														onClick={() => onRemove(index)}
														aria-label="Remove"
														className="size-8 text-muted-foreground hover:bg-destructive/10 hover:text-destructive">
														<Trash2 className="size-4" />
													</Button>
												</td>
											</tr>
											{detail && (
												<tr className="border-b border-border/70 bg-muted/20 last:border-0">
													<td
														colSpan={columns.length + 1}
														className="px-3 pt-1 pb-3">
														{detail}
													</td>
												</tr>
											)}
										</Fragment>
									);
								})}
							</tbody>
						</table>
					</div>
				</div>
			)}
		</div>
	);
}
