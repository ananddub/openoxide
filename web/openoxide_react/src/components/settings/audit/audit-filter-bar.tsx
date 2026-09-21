import {Search, X} from 'lucide-react';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import {PERIODS, type PeriodKey} from './audit-format';

/** Search + actor + period filters for the audit feed. */
export function AuditFilterBar({
	search,
	onSearch,
	actor,
	onActor,
	period,
	onPeriod,
	actors,
	filtersActive,
	onClear,
}: {
	search: string;
	onSearch: (v: string) => void;
	actor: string;
	onActor: (v: string) => void;
	period: PeriodKey;
	onPeriod: (v: PeriodKey) => void;
	actors: [string, number][];
	filtersActive: boolean;
	onClear: () => void;
}) {
	return (
		<div className="flex flex-wrap items-center gap-3">
			<div className="relative min-w-[220px] flex-1">
				<Search className="absolute start-3 top-1/2 size-4 -translate-y-1/2 text-muted-foreground" />
				<input
					type="search"
					value={search}
					onChange={e => onSearch(e.target.value)}
					placeholder="Search events, actors, resources..."
					className="w-full rounded-xl bg-muted/30 py-2 ps-10 pe-3 text-sm text-foreground transition-all placeholder:text-muted-foreground/50 focus:ring-2 focus:ring-primary/20 focus:outline-none"
				/>
			</div>
			<Select
				value={actor}
				onValueChange={v => {
					onActor(v ?? '');
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
					onPeriod((v as PeriodKey) ?? 'all');
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
					onClick={onClear}
					className="flex items-center gap-1 text-xs font-medium text-primary hover:underline">
					<X className="size-3" />
					Clear filters
				</button>
			)}
		</div>
	);
}
