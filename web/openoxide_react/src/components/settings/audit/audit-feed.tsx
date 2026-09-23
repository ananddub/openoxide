import {History, User as UserIcon} from 'lucide-react';
import type {AuditLogPayload} from '#/api/live/audit';
import {cn} from '#/api/utils';
import {Button} from '#/components/ui/button';
import {
	ListEmpty,
	ListSkeleton,
	RowTile,
	SectionLabel,
} from '#/components/settings/list-card';
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

/** Day-grouped sentence cards with loading/empty states and pagination. */
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
	if (isLoading) return <ListSkeleton rows={4} />;

	if (total === 0) {
		return (
			<ListEmpty
				icon={History}
				title={
					filtersActive ? 'No matching events' : 'Nothing recorded yet'
				}
				description={
					filtersActive
						? 'Try a different search text, actor or time period.'
						: 'Actions taken in this panel — members, permissions, servers, deployments — appear here as a history.'
				}
				action={
					filtersActive && (
						<Button
							variant="ghost"
							onClick={onClear}
							className="mt-4 h-9 text-xs font-semibold text-primary">
							Clear filters
						</Button>
					)
				}
			/>
		);
	}

	return (
		<>
			<div className="flex flex-col gap-4">
				{days.map(day => (
					<div
						key={day.key}
						className="w-full overflow-hidden rounded-xl border bg-card">
						<div className="border-b px-4 py-2">
							<SectionLabel>{day.label}</SectionLabel>
						</div>
						<div className="divide-y">
							{day.rows.map(e => {
								const info = sentenceFor(e.action);
								return (
									<button
										key={e.id}
										type="button"
										onClick={() => onSelect(e)}
										className="group flex w-full cursor-pointer items-center gap-3.5 px-4 py-3 text-start transition-colors hover:bg-accent/30 focus:bg-accent/30 focus:outline-none">
										<span className="relative shrink-0">
											<RowTile>
												<UserIcon className="size-4 text-muted-foreground" />
											</RowTile>
											<span
												className={cn(
													'absolute -end-0.5 -bottom-0.5 size-2.5 rounded-full ring-2 ring-card',
													TONE_DOT[info.tone],
												)}
											/>
										</span>
										<span className="min-w-0 flex-1">
											<span className="block truncate text-sm text-foreground">
												<span className="font-medium">
													{e.user_email || 'System'}
												</span>{' '}
												{info.verb}
												<span className="font-medium">
													{' '}
													{resourceLabel(e)}
												</span>
											</span>
											<span className="mt-0.5 flex items-center gap-1.5 text-xs text-muted-foreground">
												<span>{clockTime(e.created_at)}</span>
												<span className="rounded-md bg-foreground/[0.05] px-1.5 py-0.5 text-[10px]">
													{e.user_role || 'member'}
												</span>
											</span>
										</span>
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

			{total > PER_PAGE && (
				<div className="flex items-center justify-between text-xs text-muted-foreground">
					<span>
						Showing {(page - 1) * PER_PAGE + 1}–
						{Math.min(page * PER_PAGE, total)} of {total}
					</span>
					<div className="flex items-center gap-2">
						<Button
							variant="outline"
							size="sm"
							disabled={page === 1}
							onClick={() => onPage(Math.max(1, page - 1))}
							className="h-8">
							Previous
						</Button>
						<Button
							variant="outline"
							size="sm"
							disabled={page >= pages}
							onClick={() => onPage(Math.min(pages, page + 1))}
							className="h-8">
							Next
						</Button>
					</div>
				</div>
			)}
		</>
	);
}
