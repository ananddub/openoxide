import {User as UserIcon} from 'lucide-react';
import type {AuditLogPayload} from '#/api/live/audit';
import {cn} from '#/api/utils';
import {
	TONE_DOT,
	PER_PAGE,
	sentenceFor,
	resourceLabel,
	clockTime,
	absoluteTime,
	timeAgo,
} from './audit-format';

type DayGroup = {key: string; label: string; rows: AuditLogPayload[]};

/** Day-grouped sentence rows, loading/empty states, and pagination. */
export function AuditFeed({
	isLoading,
	total,
	days,
	filtersActive,
	onClear,
	page,
	pages,
	onPage,
	onSelect,
}: {
	isLoading: boolean;
	total: number;
	days: DayGroup[];
	filtersActive: boolean;
	onClear: () => void;
	page: number;
	pages: number;
	onPage: (p: number) => void;
	onSelect: (e: AuditLogPayload) => void;
}) {
	return (
		<>
			{/* Feed */}
			{isLoading ? (
				<div className="space-y-3">
					{[...Array(4)].map((_, i) => (
						<div
							key={i}
							className="h-16 animate-pulse rounded-xl bg-muted/40"
						/>
					))}
				</div>
			) : total === 0 ? (
				<div className="rounded-xl bg-card py-16 text-center">
					<p className="text-sm text-muted-foreground">
						{filtersActive
							? 'No events match the current filters.'
							: 'Nothing recorded yet — actions in the panel will appear here.'}
					</p>
					{filtersActive && (
						<button
							type="button"
							onClick={onClear}
							className="mt-3 text-xs font-medium text-primary hover:underline">
							Clear filters
						</button>
					)}
				</div>
			) : (
				<div className="space-y-4">
					{days.map(day => (
						<div
							key={day.key}
							className="overflow-hidden rounded-xl bg-card">
							<div className="border-b  px-5 py-2.5">
								<span className="text-xs font-semibold tracking-wide text-muted-foreground uppercase">
									{day.label}
								</span>
							</div>
							<div className="divide-y divide-border/40">
								{day.rows.map(e => {
									const info = sentenceFor(e.action);
									return (
										<button
											key={e.id}
											type="button"
											onClick={() => onSelect(e)}
											className="flex w-full items-center gap-4 px-5 py-3.5 text-start transition-colors hover:bg-muted/30 focus:bg-muted/40 focus:outline-none">
											<span className="relative flex size-8 shrink-0 items-center justify-center rounded-full bg-muted/60">
												<UserIcon className="size-3.5 text-muted-foreground" />
												<span
													className={cn(
														'absolute -end-0.5 -bottom-0.5 size-2.5 rounded-full ring-2 ring-card',
														TONE_DOT[info.tone],
													)}
												/>
											</span>
											<div className="min-w-0 flex-1">
												<p className="truncate text-sm text-foreground">
													<span className="font-medium">
														{e.user_email || 'System'}
													</span>{' '}
													{info.verb}
													<span className="font-medium">
														{' '}
														{resourceLabel(e)}
													</span>
												</p>
												<p className="mt-0.5 flex items-center gap-1.5 text-xs text-muted-foreground">
													<span>{clockTime(e.created_at)}</span>
													<span className="rounded-md bg-muted/60 px-1.5 py-0.5 text-[10px]">
														{e.user_role || 'member'}
													</span>
												</p>
											</div>
											<time
												className="shrink-0 text-xs text-muted-foreground"
												title={absoluteTime(e.created_at)}>
												{timeAgo(e.created_at)}
											</time>
										</button>
									);
								})}
							</div>
						</div>
					))}
				</div>
			)}

			{/* Pagination */}
			{total > PER_PAGE && (
				<div className="flex items-center justify-between text-sm text-muted-foreground">
					<span>
						Showing {(page - 1) * PER_PAGE + 1}–
						{Math.min(page * PER_PAGE, total)} of {total}
					</span>
					<div className="flex gap-2">
						<button
							type="button"
							onClick={() => onPage(Math.max(1, page - 1))}
							disabled={page === 1}
							className="rounded-lg bg-muted/40 px-3 py-1.5 transition-colors hover:bg-muted/40 disabled:opacity-50">
							Previous
						</button>
						<button
							type="button"
							onClick={() => onPage(Math.min(pages, page + 1))}
							disabled={page >= pages}
							className="rounded-lg bg-muted/40 px-3 py-1.5 transition-colors hover:bg-muted/40 disabled:opacity-50">
							Next
						</button>
					</div>
				</div>
			)}
		</>
	);
}
