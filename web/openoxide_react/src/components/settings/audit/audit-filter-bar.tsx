import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import {ToolbarSearch} from '#/components/settings/list-card';
import {PERIODS, type PeriodKey} from './audit-format';

/**
 * Audit filters — a fragment of toolbar controls (search, actor, period);
 * the toolbar itself owns the layout. Each control resets itself, so no
 * separate clear action here.
 */
export function AuditFilterBar({
	search,
	onSearch,
	actor,
	onActor,
	period,
	onPeriod,
	actors,
}: {
	search: string;
	onSearch: (v: string) => void;
	actor: string;
	onActor: (v: string) => void;
	period: PeriodKey;
	onPeriod: (v: PeriodKey) => void;
	actors: [string, number][];
}) {
	return (
		<>
			<ToolbarSearch
				value={search}
				onChange={onSearch}
				placeholder="Search events"
				className="min-w-[200px] flex-1 sm:max-w-none"
			/>
			<Select
				value={actor}
				onValueChange={v => {
					onActor(v ?? '');
				}}>
				<SelectTrigger className="h-9 w-44 rounded-lg bg-card">
					<SelectValue className="text-foreground">
						{actor || 'Anyone'}
					</SelectValue>
				</SelectTrigger>
				<SelectContent className="bg-card">
					<SelectItem value="">Anyone</SelectItem>
					{actors.map(([who, n]) => (
						<SelectItem key={who} value={who}>
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
				<SelectTrigger className="h-9 w-36 rounded-lg bg-card">
					<SelectValue className="text-foreground">
						{PERIODS.find(p => p.value === period)?.label ?? 'All time'}
					</SelectValue>
				</SelectTrigger>
				<SelectContent className="bg-card">
					{PERIODS.map(p => (
						<SelectItem key={p.value} value={p.value}>
							{p.label}
						</SelectItem>
					))}
				</SelectContent>
			</Select>
		</>
	);
}
