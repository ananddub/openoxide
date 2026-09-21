import {Plus, X} from 'lucide-react';
import {cn} from '#/api/utils';
import type {ShellEntry} from './types';

/** Shell tab strip: per-tab status dot + label + close, and a "+ New Shell"
 * button capped at 3 shells. */
export function ShellTabStrip({
	shells,
	activeId,
	onSelect,
	onClose,
	onAdd,
}: {
	shells: ShellEntry[];
	activeId: string;
	onSelect: (id: string) => void;
	onClose: (id: string) => void;
	onAdd: () => void;
}) {
	return (
		<div className="flex items-center gap-1.5 border-b border-border/50 bg-muted/20 px-2.5 py-2">
			<div className="flex min-w-0 flex-1 items-center gap-1 overflow-x-auto">
				{shells.map(entry => {
					const isActive = entry.id === activeId;
					return (
						<div
							key={entry.id}
							className={cn(
								'group inline-flex shrink-0 items-center gap-1.5 rounded-lg py-1.5 ps-2.5 pe-1.5 text-[12px] transition-colors',
								isActive
									? 'bg-card text-foreground shadow-sm ring-1 ring-border/60'
									: 'text-muted-foreground hover:bg-foreground/[0.05] hover:text-foreground',
							)}>
							<button
								type="button"
								onClick={() => onSelect(entry.id)}
								className="inline-flex cursor-pointer items-center gap-1.5 font-medium">
								<span
									className={cn(
										'size-1.5 rounded-full transition-colors',
										entry.status === 'connected'
											? 'bg-emerald-500'
											: entry.status === 'connecting'
												? 'bg-amber-500'
												: entry.status === 'error'
													? 'bg-rose-500'
													: 'bg-muted-foreground/40',
									)}
								/>
								{entry.label}
							</button>
							<button
								type="button"
								onClick={e => {
									e.stopPropagation();
									onClose(entry.id);
								}}
								aria-label={`Close ${entry.label}`}
								className={cn(
									'cursor-pointer rounded-md p-0.5 text-muted-foreground transition-all hover:bg-foreground/10 hover:text-foreground',
									isActive
										? 'opacity-70 hover:opacity-100'
										: 'opacity-0 group-hover:opacity-70',
								)}>
								<X className="size-3" />
							</button>
						</div>
					);
				})}
			</div>
			<button
				type="button"
				onClick={onAdd}
				disabled={shells.length >= 3}
				title={
					shells.length >= 3
						? 'Maximum of 3 shells per terminal'
						: 'Open a new shell'
				}
				className={cn(
					'inline-flex shrink-0 cursor-pointer items-center gap-1.5 rounded-lg px-2.5 py-1.5 text-[12px] font-medium transition-colors',
					shells.length >= 3
						? 'cursor-not-allowed text-muted-foreground/40'
						: 'text-muted-foreground hover:bg-foreground/[0.05] hover:text-foreground',
				)}>
				<Plus className="size-3.5" />
				New Shell
			</button>
		</div>
	);
}
