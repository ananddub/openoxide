import {useCallback, useEffect, useMemo, useState} from 'react';
import {
	useAuditExport,
	useAuditLogItems,
	type AuditLogPayload,
} from '#/api/live/audit';
import {Sheet, SheetContent} from '#/components/ui/sheet';
import {
	PER_PAGE,
	type CategoryId,
	type PeriodKey,
	sentenceFor,
	periodCutoff,
	dayKey,
	dayLabel,
} from './audit-format';
import {AuditCategoryTabs} from './audit-page-chrome';
import {AuditFilterBar} from './audit-filter-bar';
import {AuditFeed} from './audit-feed';
import {SectionHeader, Toolbar} from '#/components/settings/list-card';
import {Button} from '#/components/ui/button';
import {Download, Loader2} from 'lucide-react';
import {AuditDetails} from './audit-details';

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
			if (
				category !== 'all' &&
				sentenceFor(e.action).category !== category
			)
				return false;
			if (q) {
				const hay =
					`${e.user_email} ${e.action} ${e.resource_type} ${e.resource_name ?? ''} ${e.metadata ?? ''}`.toLowerCase();
				if (!hay.includes(q)) return false;
			}
			return true;
		});
	}, [events, debounced, actor, category, period]);

	const total = filtered.length;
	const pages = Math.max(1, Math.ceil(total / PER_PAGE));
	const pageRows = filtered.slice((page - 1) * PER_PAGE, page * PER_PAGE);

	const days = useMemo(() => {
		const groups: {key: string; label: string; rows: AuditLogPayload[]}[] =
			[];
		for (const row of pageRows) {
			const key = dayKey(row.created_at);
			const last = groups[groups.length - 1];
			if (last && last.key === key) last.rows.push(row);
			else
				groups.push({key, label: dayLabel(row.created_at), rows: [row]});
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
		category !== 'all' ||
		!!actor ||
		period !== 'all' ||
		!!debounced.trim();

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
		<div className="flex w-full animate-in flex-col gap-5 duration-200 fade-in">
			<SectionHeader
				title="Audit Log"
				description="Every action taken in this panel — members, permissions, servers and deployments."
				action={
					<Button
						onClick={() => void downloadCsv()}
						disabled={exportMutation.isPending}
						variant="outline"
						className="h-9 gap-1.5 rounded-lg px-4">
						{exportMutation.isPending ? (
							<Loader2 className="size-3.5 animate-spin" />
						) : (
							<Download className="size-3.5" />
						)}
						Export CSV
					</Button>
				}
			/>

			<Toolbar
				action={
					<AuditCategoryTabs
						category={category}
						counts={counts}
						onSelect={c => {
							setCategory(c);
							setPage(1);
						}}
					/>
				}>
				<AuditFilterBar
					search={search}
					onSearch={setSearch}
					actor={actor}
					onActor={v => {
						setActor(v);
						setPage(1);
					}}
					period={period}
					onPeriod={v => {
						setPeriod(v);
						setPage(1);
					}}
					actors={actors}
				/>
			</Toolbar>

			<AuditFeed
				isLoading={isLoading}
				total={total}
				days={days}
				filtersActive={filtersActive}
				onClear={clearFilters}
				page={page}
				pages={pages}
				onPage={setPage}
				onSelect={setDetails}
			/>

			{/* Details sheet */}
			<Sheet
				open={details !== null}
				onOpenChange={open => !open && setDetails(null)}>
				<SheetContent className="gap-0 p-0">
					{details && (
						<div className="flex min-h-0 flex-1 flex-col gap-5 overflow-y-auto px-6 py-6">
							<AuditDetails event={details} />
						</div>
					)}
				</SheetContent>
			</Sheet>
		</div>
	);
}
