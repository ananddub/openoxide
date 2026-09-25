import {useMemo} from 'react';
import {History, User as UserIcon} from 'lucide-react';
import type {AuditLogPayload} from '#/api/live/audit';
import {usePermissionGroupMembers} from '#/api/live';
import {getAvatarInitials, isSolidColorAvatar} from '#/lib/avatar-utils';
import {cn} from '#/api/utils';
import {Button} from '#/components/ui/button';
import {ListEmpty, ListSkeleton} from '#/components/settings/list-card';
import {
	Table,
	TableBody,
	TableCell,
	TableHead,
	TableHeader,
	TableRow,
} from '#/components/ui/table';
import {
	TONE_DOT,
	sentenceFor,
	resourceLabel,
	clockTime,
	absoluteTime,
	toSec,
} from './audit-format';

const HEAD_CLS =
	'px-4 py-1.5 text-[11px] font-semibold tracking-wider text-muted-foreground uppercase';

function dateLabel(sec: number): string {
	const s = toSec(sec);
	if (!s) return '—';
	return new Date(s * 1000).toLocaleDateString([], {
		day: 'numeric',
		month: 'short',
	});
}

/** One bordered card table — sticky column heads, flat newest-first rows scrolling inside the card. Same grammar as the deployments/requests tables. */
export function AuditFeed({
	isLoading,
	total,
	rows,
	filtersActive,
	onClear,
	onSelect,
}: {
	isLoading: boolean;
	total: number;
	rows: AuditLogPayload[];
	filtersActive: boolean;
	onClear: () => void;
	onSelect: (e: AuditLogPayload) => void;
}) {
	// user_id → org member, so each row can show the actor's real avatar.
	const {data: membersData} = usePermissionGroupMembers();
	const usersById = useMemo(() => {
		const map = new Map<
			number,
			{email?: string; name?: string; avatar?: string | null}
		>();
		const list = Array.isArray(membersData) ? membersData : [];
		for (const m of list as any[]) {
			map.set(Number(m.user_id), m);
		}
		return map;
	}, [membersData]);

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
		<div className="flex h-full min-h-0 w-full flex-col overflow-hidden rounded-xl border bg-card">
			{/* Table's own container div is the scroller — sticky heads pin for real */}
			<div className="min-h-0 flex-1 [&>div]:h-full [&>div]:overflow-y-auto">
				<Table>
					<TableHeader className="sticky top-0 z-10 bg-muted-solid">
						<TableRow className="border-b hover:bg-transparent">
							<TableHead className={cn(HEAD_CLS, 'w-24')}>
								Time
							</TableHead>
							<TableHead className={HEAD_CLS}>
								Actor
							</TableHead>
							<TableHead className={HEAD_CLS}>
								Action
							</TableHead>
							<TableHead
								className={cn(HEAD_CLS, 'hidden sm:table-cell')}>
								Resource
							</TableHead>
						</TableRow>
					</TableHeader>
					<TableBody>
						{rows.map(e => {
							const info = sentenceFor(e.action);
							const member = usersById.get(
								Number(e.user_id ?? 0),
							);
							const email =
								member?.email || String(e.user_email || '');
							const displayName =
								member?.name?.trim() ||
								(email ? email.split('@')[0] : '');
							const avatar = member?.avatar || '';
							return (
								<TableRow
									key={e.id}
									onClick={() => onSelect(e)}
									className="group cursor-pointer border-b border-border transition-colors hover:bg-accent/30">
									<TableCell className="w-24 px-4 py-2 font-mono text-xs whitespace-nowrap text-muted-foreground tabular-nums">
										<time title={absoluteTime(e.created_at)}>
											<span className="block text-[10px] text-muted-foreground/70">
												{dateLabel(e.created_at)}
											</span>
											{clockTime(e.created_at)}
										</time>
									</TableCell>
									<TableCell className="max-w-0 px-4 py-2">
										<div className="flex min-w-0 items-center gap-2.5">
											<span
												className={cn(
													'flex size-6 shrink-0 items-center justify-center overflow-hidden rounded-full bg-foreground/[0.06] text-[9px] font-semibold text-foreground uppercase',
												)}>
												{isSolidColorAvatar(
													avatar,
												) ? (
													<span
														className="size-full ring-1 ring-border/50 ring-inset"
														style={{
															backgroundColor:
																avatar,
														}}
													/>
												) : avatar ? (
													<img
														src={avatar}
														alt={displayName}
														className="size-full bg-muted object-cover"
													/>
												) : displayName ? (
													getAvatarInitials(
														displayName,
													)
												) : (
													<UserIcon className="size-3.5 text-muted-foreground" />
												)}
											</span>
											<span className="truncate text-[13px] font-medium text-foreground">
												{email || 'System'}
											</span>
											<span className="shrink-0 rounded-md bg-foreground/[0.05] px-1.5 py-0.5 text-[10px] text-muted-foreground">
												{e.user_role || 'member'}
											</span>
										</div>
									</TableCell>
									<TableCell className="px-4 py-2">
										<span className="flex items-center gap-2 text-[13px] text-foreground">
											<span
												className={cn(
													'size-2 shrink-0 rounded-full',
													TONE_DOT[info.tone],
												)}
											/>
											{info.verb}
										</span>
									</TableCell>
									<TableCell className="hidden max-w-0 px-4 py-2 sm:table-cell">
										<span className="block truncate text-[13px] text-muted-foreground">
											{resourceLabel(e)}
										</span>
									</TableCell>
								</TableRow>
							);
						})}
					</TableBody>
				</Table>
			</div>
		</div>
	);
}
