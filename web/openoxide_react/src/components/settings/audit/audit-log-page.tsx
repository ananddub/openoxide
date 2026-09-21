import {useCallback, useEffect, useMemo, useState} from 'react';
import {
	Activity,
	Download,
	Loader2,
	Search,
	ShieldCheck,
	User as UserIcon,
	X,
} from 'lucide-react';
import {
	useAuditExport,
	useAuditLogItems,
	type AuditLogPayload,
} from '#/api/live/audit';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import {
	Dialog,
	DialogContent,
	DialogDescription,
	DialogTitle,
} from '#/components/ui/dialog';
import {Button} from '#/components/ui/button';
import {cn} from '#/api/utils';

/**
 * Audit log as OpenShip renders it: a readable history, not a table.
 * Category tabs span the top; the feed is day-grouped sentence rows
 * ("test@gmail.com deleted permission group #5") with a tone dot, clock
 * time, and relative age. Search + actor + period filters, CSV export,
 * and a click-to-open details sheet (who / what / when / metadata).
 *
 * The same component is the Settings → Audit tab content; the deployments
 * Activity tab keeps its lighter merged-feed variant.
 */

type EntryTone = 'success' | 'danger' | 'info' | 'warning' | 'neutral';

const TONE_DOT: Record<EntryTone, string> = {
	success: 'bg-emerald-500',
	danger: 'bg-rose-500',
	info: 'bg-sky-500',
	warning: 'bg-amber-500',
	neutral: 'bg-muted-foreground/40',
};

type CategoryId = 'all' | 'team' | 'access' | 'system';

type Sentence = {verb: string; tone: EntryTone; category: CategoryId};

const AUDIT_SENTENCES: Record<string, Sentence> = {
	INVITE_CREATED: {verb: 'invited', tone: 'info', category: 'team'},
	INVITE_ACCEPTED: {verb: 'accepted an invite to', tone: 'success', category: 'team'},
	INVITE_CANCELLED: {verb: 'cancelled an invite for', tone: 'neutral', category: 'team'},
	MEMBER_ROLE_UPDATED: {verb: 'changed the role of', tone: 'info', category: 'team'},
	MEMBER_REMOVED: {verb: 'removed', tone: 'danger', category: 'team'},
	PERMISSION_GROUP_CREATED: {verb: 'created', tone: 'success', category: 'access'},
	PERMISSION_GROUP_UPDATED: {verb: 'updated', tone: 'info', category: 'access'},
	PERMISSION_GROUP_DELETED: {verb: 'deleted', tone: 'danger', category: 'access'},
	PERMISSION_GROUP_ASSIGNED: {verb: 'assigned a permission group to', tone: 'info', category: 'access'},
	USER_POLICIES_REPLACED: {verb: 'replaced policies on', tone: 'warning', category: 'access'},
};

const CATEGORIES: {id: CategoryId; label: string; icon: typeof Activity}[] = [
	{id: 'all', label: 'All', icon: Activity},
	{id: 'team', label: 'Team', icon: UserIcon},
	{id: 'access', label: 'Access', icon: ShieldCheck},
];

function sentenceFor(action: string): Sentence {
	return (
		AUDIT_SENTENCES[action] ?? {
			verb: action.toLowerCase().replace(/_/g, ' '),
			tone: 'neutral',
			category: 'system',
		}
	);
}

/** Human label for an audit resource, from metadata/name/id. */
function resourceLabel(e: AuditLogPayload): string {
	if (e.resource_name) return e.resource_name;
	const meta = parseMeta(e);
	const email = typeof meta.email === 'string' ? meta.email : undefined;
	const kind = (e.resource_type || '').replace(/_/g, ' ');
	if (email) return email;
	if (e.resource_id) return `${kind} #${e.resource_id}`;
	return kind;
}

function parseMeta(e: AuditLogPayload): Record<string, unknown> {
	try {
		return e.metadata ? JSON.parse(e.metadata) : {};
	} catch {
		return {};
	}
}

function timeAgo(sec: number): string {
	const diff = Math.floor(Date.now() / 1000 - sec);
	if (diff < 0) return 'just now';
	if (diff < 60) return `${diff}s ago`;
	const min = Math.floor(diff / 60);
	if (min < 60) return `${min}m ago`;
	const hr = Math.floor(min / 60);
	if (hr < 24) return `${hr}h ago`;
	return `${Math.floor(hr / 24)}d ago`;
}

function clockTime(sec: number): string {
	return new Date(sec * 1000).toLocaleTimeString([], {
		hour: '2-digit',
		minute: '2-digit',
	});
}

function absoluteTime(sec: number): string {
	return new Date(sec * 1000).toLocaleString();
}

function dayKey(sec: number): string {
	const d = new Date(sec * 1000);
	return `${d.getFullYear()}-${d.getMonth()}-${d.getDate()}`;
}

function dayLabel(sec: number): string {
	const now = new Date();
	const startOf = (x: Date) =>
		new Date(x.getFullYear(), x.getMonth(), x.getDate()).getTime();
	const d = new Date(sec * 1000);
	const diffDays = Math.round((startOf(now) - startOf(d)) / 86400000);
	if (diffDays <= 0) return 'Today';
	if (diffDays === 1) return 'Yesterday';
	return d.toLocaleDateString(undefined, {
		weekday: 'short',
		month: 'short',
		day: 'numeric',
		year: d.getFullYear() === now.getFullYear() ? undefined : 'numeric',
	});
}

const PER_PAGE = 25;

type PeriodKey = 'all' | 'today' | '7d' | '30d' | '90d';

const PERIODS: {value: PeriodKey; label: string}[] = [
	{value: 'all', label: 'All time'},
	{value: 'today', label: 'Today'},
	{value: '7d', label: 'Last 7 days'},
	{value: '30d', label: 'Last 30 days'},
	{value: '90d', label: 'Last 90 days'},
];

function periodCutoff(period: PeriodKey): number {
	if (period === 'all') return 0;
	const now = Date.now() / 1000;
	if (period === 'today') {
		const d = new Date();
		d.setHours(0, 0, 0, 0);
		return d.getTime() / 1000;
	}
	const days = period === '7d' ? 7 : period === '30d' ? 30 : 90;
	return now - days * 86400;
}

export function AuditLogPage() {
	const {data: events = [], isLoading} = useAuditLogItems({limit: 500});
	const exportMutation = useAuditExport();

	const [search, setSearch] = useState('');
	const [category, setCategory] = useState<CategoryId>('all');
	const [actor, setActor] = useState('');
	const [period, setPeriod] = useState<PeriodKey>('all');
	const [page, setPage] = useState(1);
	const [details, setDetails] = useState<AuditLogPayload | null>(null);

	// Debounce keystrokes into the filter value.
	const [debounced, setDebounced] = useState(search);
	useEffect(() => {
		const t = setTimeout(() => {
			setDebounced(search);
			setPage(1);
		}, 250);
		return () => clearTimeout(t);
	}, [search]);

	const actors = useMemo(() => {
		const seen = new Map<string, number>();
		for (const e of events) {
			const who = e.user_email || 'System';
			seen.set(who, (seen.get(who) ?? 0) + 1);
		}
		return [...seen.entries()].sort((a, b) => b[1] - a[1]);
	}, [events]);

	const filtered = useMemo(() => {
		const cutoff = periodCutoff(period);
		const q = debounced.trim().toLowerCase();
		return events.filter(e => {
			if (e.created_at < cutoff) return false;
			if (actor && (e.user_email || 'System') !== actor) return false;
			if (category !== 'all' && sentenceFor(e.action).category !== category)
				return false;
			if (q) {
				const hay = `${e.user_email} ${e.action} ${e.resource_type} ${e.resource_name ?? ''} ${e.metadata ?? ''}`.toLowerCase();
				if (!hay.includes(q)) return false;
			}
			return true;
		});
	}, [events, debounced, actor, category, period]);

	const total = filtered.length;
	const pages = Math.max(1, Math.ceil(total / PER_PAGE));
	const pageRows = filtered.slice((page - 1) * PER_PAGE, page * PER_PAGE);

	const days = useMemo(() => {
		const groups: {key: string; label: string; rows: AuditLogPayload[]}[] = [];
		for (const row of pageRows) {
			const key = dayKey(row.created_at);
			const last = groups[groups.length - 1];
			if (last && last.key === key) last.rows.push(row);
			else groups.push({key, label: dayLabel(row.created_at), rows: [row]});
		}
		return groups;
	}, [pageRows]);

	const counts = useMemo(() => {
		const map: Record<CategoryId, number> = {
			all: events.length,
			team: 0,
			access: 0,
			system: 0,
		};
		for (const e of events) map[sentenceFor(e.action).category]++;
		return map;
	}, [events]);

	const filtersActive =
		category !== 'all' || !!actor || period !== 'all' || !!debounced.trim();

	const clearFilters = useCallback(() => {
		setSearch('');
		setCategory('all');
		setActor('');
		setPeriod('all');
		setPage(1);
	}, []);

	const downloadCsv = useCallback(async () => {
		const csv = await exportMutation.mutateAsync(undefined);
		const blob = new Blob([csv], {type: 'text/csv'});
		const url = URL.createObjectURL(blob);
		const a = document.createElement('a');
		a.href = url;
		a.download = `audit-log-${new Date().toISOString().slice(0, 10)}.csv`;
		a.click();
		URL.revokeObjectURL(url);
	}, [exportMutation]);

	return (
		<div className="flex w-full animate-in flex-col gap-6 duration-200 fade-in">
			{/* Header */}
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
					onClick={() => void downloadCsv()}
					disabled={exportMutation.isPending}
					className="h-8 gap-1.5 text-xs">
					{exportMutation.isPending ? (
						<Loader2 className="size-3.5 animate-spin" />
					) : (
						<Download className="size-3.5" />
					)}
					Export CSV
				</Button>
			</div>

			{/* Category tabs — full width, above the feed */}
			<div className="flex items-center gap-1 border-b ">
				{CATEGORIES.map(c => (
					<button
						key={c.id}
						type="button"
						onClick={() => {
							setCategory(c.id);
							setPage(1);
						}}
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

			{/* Filter bar */}
			<div className="flex flex-wrap items-center gap-3">
				<div className="relative min-w-[220px] flex-1">
					<Search className="absolute start-3 top-1/2 size-4 -translate-y-1/2 text-muted-foreground" />
					<input
						type="search"
						value={search}
						onChange={e => setSearch(e.target.value)}
						placeholder="Search events, actors, resources..."
						className="w-full rounded-xl bg-muted/30 py-2 pe-3 ps-10 text-sm text-foreground transition-all placeholder:text-muted-foreground/50 focus:outline-none focus:ring-2 focus:ring-primary/20"
					/>
				</div>
				<Select
					value={actor}
					onValueChange={v => {
						setActor(v ?? '');
						setPage(1);
					}}>
					<SelectTrigger className="!h-9 w-44 rounded-xl  bg-muted/30 text-xs">
						<SelectValue className="text-foreground">
							{actor || 'Anyone'}
						</SelectValue>
					</SelectTrigger>
					<SelectContent>
						<SelectItem value="" className="text-xs">
							Anyone
						</SelectItem>
						{actors.map(([who, n]) => (
							<SelectItem key={who} value={who} className="text-xs">
								{who} ({n})
							</SelectItem>
						))}
					</SelectContent>
				</Select>
				<Select
					value={period}
					onValueChange={v => {
						setPeriod((v as PeriodKey) ?? 'all');
						setPage(1);
					}}>
					<SelectTrigger className="!h-9 w-40 rounded-xl  bg-muted/30 text-xs">
						<SelectValue className="text-foreground">
							{PERIODS.find(p => p.value === period)?.label ?? 'All time'}
						</SelectValue>
					</SelectTrigger>
					<SelectContent>
						{PERIODS.map(p => (
							<SelectItem key={p.value} value={p.value} className="text-xs">
								{p.label}
							</SelectItem>
						))}
					</SelectContent>
				</Select>
				{filtersActive && (
					<button
						type="button"
						onClick={clearFilters}
						className="flex items-center gap-1 text-xs font-medium text-primary hover:underline">
						<X className="size-3" />
						Clear filters
					</button>
				)}
			</div>

			{/* Feed */}
			{isLoading ? (
				<div className="space-y-3">
					{[...Array(4)].map((_, i) => (
						<div
							key={i}
							className="h-16 animate-pulse rounded-2xl bg-muted/40"
						/>
					))}
				</div>
			) : total === 0 ? (
				<div className="rounded-2xl bg-card py-16 text-center">
					<p className="text-sm text-muted-foreground">
						{filtersActive
							? 'No events match the current filters.'
							: 'Nothing recorded yet — actions in the panel will appear here.'}
					</p>
					{filtersActive && (
						<button
							type="button"
							onClick={clearFilters}
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
							className="overflow-hidden rounded-2xl bg-card">
							<div className="border-b  px-5 py-2.5">
								<span className="text-xs font-semibold uppercase tracking-wide text-muted-foreground">
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
											onClick={() => setDetails(e)}
											className="flex w-full items-center gap-4 px-5 py-3.5 text-start transition-colors hover:bg-muted/30 focus:bg-muted/40 focus:outline-none">
											<span className="relative flex size-8 shrink-0 items-center justify-center rounded-full bg-muted/60">
												<UserIcon className="size-3.5 text-muted-foreground" />
												<span
													className={cn(
														'absolute -bottom-0.5 -end-0.5 size-2.5 rounded-full ring-2 ring-card',
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
							onClick={() => setPage(p => Math.max(1, p - 1))}
							disabled={page === 1}
							className="rounded-lg bg-muted/40 px-3 py-1.5 transition-colors hover:bg-muted/40 disabled:opacity-50">
							Previous
						</button>
						<button
							type="button"
							onClick={() => setPage(p => Math.min(pages, p + 1))}
							disabled={page >= pages}
							className="rounded-lg bg-muted/40 px-3 py-1.5 transition-colors hover:bg-muted/40 disabled:opacity-50">
							Next
						</button>
					</div>
				</div>
			)}

			{/* Details sheet */}
			<Dialog open={details !== null} onOpenChange={open => !open && setDetails(null)}>
				<DialogContent className="max-h-[85vh] overflow-y-auto sm:max-w-[720px]">
					{details && <AuditDetails event={details} />}
				</DialogContent>
			</Dialog>
		</div>
	);
}

function AuditDetails({event}: {event: AuditLogPayload}) {
	const info = sentenceFor(event.action);
	const meta = parseMeta(event);
	return (
		<div className="flex flex-col gap-5">
			<div>
				<div className="flex items-center gap-2">
					<span
						className={cn('size-2 shrink-0 rounded-full', TONE_DOT[info.tone])}
					/>
					<DialogTitle className="text-lg font-semibold text-foreground">
						{event.action.replace(/_/g, ' ').toLowerCase()}
					</DialogTitle>
				</div>
				<p className="mt-1.5 text-sm text-foreground/80">
					<span className="font-medium">{event.user_email || 'System'}</span>{' '}
					{info.verb}
					<span className="font-medium"> {resourceLabel(event)}</span>
				</p>
				<DialogDescription className="mt-0.5 text-xs text-muted-foreground">
					{timeAgo(event.created_at)} · {absoluteTime(event.created_at)}
				</DialogDescription>
			</div>

			<section className="rounded-xl bg-muted/20 p-4">
				<DetailRow label="Who" value={event.user_email || 'System'} />
				<DetailRow label="Role" value={event.user_role || '—'} />
				<DetailRow label="Action" value={event.action} mono />
				<DetailRow
					label="Resource"
					value={`${event.resource_type}${event.resource_id ? ` #${event.resource_id}` : ''}`}
				/>
			</section>

			{Object.keys(meta).length > 0 && (
				<section className="overflow-hidden rounded-xl ">
					<div className="border-b  px-4 py-2.5">
						<span className="text-xs font-semibold uppercase tracking-wide text-muted-foreground">
							Metadata
						</span>
					</div>
					<pre className="max-h-96 overflow-auto p-4 font-mono text-xs leading-relaxed break-words whitespace-pre-wrap">
						{JSON.stringify(meta, null, 2)}
					</pre>
				</section>
			)}
		</div>
	);
}

function DetailRow({
	label,
	value,
	mono,
}: {
	label: string;
	value: string;
	mono?: boolean;
}) {
	return (
		<div className="flex items-start gap-3 py-1.5">
			<span className="w-24 shrink-0 text-xs text-muted-foreground">{label}</span>
			<span
				className={cn(
					'flex-1 text-sm break-all text-foreground',
					mono && 'font-mono text-xs',
				)}>
				{value}
			</span>
		</div>
	);
}
