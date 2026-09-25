import {useCallback, useEffect, useMemo, useState} from 'react';
import {
	useAuditExport,
	useAuditLogItems,
	type AuditLogPayload,
} from '#/api/live/audit';
import {Sheet, SheetContent} from '#/components/ui/sheet';
import {
	type CategoryId,
	type PeriodKey,
	sentenceFor,
	periodCutoff,
	toSec,
} from './audit-format';
import {AuditCategoryTabs} from './audit-page-chrome';
import {AuditFilterBar} from './audit-filter-bar';
import {AuditFeed} from './audit-feed';
import {
	ListRegion,
	SectionHeader,
	Toolbar,
} from '#/components/settings/list-card';
import {Button} from '#/components/ui/button';
import {Download, Loader2} from 'lucide-react';
import {AuditDetails} from './audit-details';

/**
 * Audit log as a proper table: sticky Time / Actor / Action / Resource
 * column heads, day groups as in-table sub-header rows, events scrolling
 * inside one bordered card — the same grammar as the deployments and
 * requests tables. Category tabs span the top; search + actor + period
 * filters, CSV export, and a click-to-open details sheet (who / what /
 * when / metadata).
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
	const [details, setDetails] = useState<AuditLogPayload | null>(null);

	// Debounce keystrokes into the filter value.
	const [debounced, setDebounced] = useState(search);
	useEffect(() => {
		const t = setTimeout(() => setDebounced(search), 250);
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
			if (toSec(e.created_at) < cutoff) return false;
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
		<div className="flex h-full w-full animate-in flex-col gap-5 duration-200 fade-in">
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
						onSelect={c => setCategory(c)}
					/>
				}>
				<AuditFilterBar
					search={search}
					onSearch={setSearch}
					actor={actor}
					onActor={v => setActor(v)}
					period={period}
					onPeriod={v => setPeriod(v)}
					actors={actors}
				/>
			</Toolbar>

			{/* History table — fixed header row, events scroll inside the card */}
			<ListRegion>
				<AuditFeed
					isLoading={isLoading}
					total={total}
					rows={filtered}
					filtersActive={filtersActive}
					onClear={clearFilters}
					onSelect={setDetails}
				/>
			</ListRegion>

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
