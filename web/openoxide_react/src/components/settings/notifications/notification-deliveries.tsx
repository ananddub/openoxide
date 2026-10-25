import {AlertCircle, Check, Clock, History, Search, X} from 'lucide-react';
import type {ComponentType} from 'react';
import type {NotificationDeliveryPayload} from '#/api/live';
import {Button} from '#/components/ui/button';
import {timeAgo} from '#/components/activity/activity-model';
import {
	ListCard,
	ListEmpty,
	ListRow,
	ListSkeleton,
	RowTile,
} from '#/components/settings/list-card';
import {triggerEventLabel} from './notification-triggers';

/** One alert = all delivery attempts sharing a `correlation_id`, collapsed to
 *  its latest attempt. The backend records every retry as its own row, so
 *  without this grouping a single alert with 3 attempts shows up 3 times. */
export type DeliveryAlert = {
	/** Latest attempt's id — the panel's read-receipt key for the alert. */
	id: number;
	notification_id: number;
	correlation_id: string;
	trigger_name: string;
	title: string;
	body: string;
	/** Status of the latest attempt (the alert's final known outcome). */
	status: string;
	attempt: number;
	/** How many attempts the alert has so far. */
	attempts: number;
	error: string | null;
	/** When the alert first fired (oldest attempt). */
	started_at: number;
	finished_at: number | null;
};

/** Collapse raw delivery attempts (newest-first) into one alert per
 *  `correlation_id`, keeping insertion order (newest alert first). */
export function groupDeliveries(
	rows: NotificationDeliveryPayload[],
): DeliveryAlert[] {
	const byCorrelation = new Map<string, DeliveryAlert>();
	for (const d of rows) {
		const existing = byCorrelation.get(d.correlation_id);
		if (!existing) {
			byCorrelation.set(d.correlation_id, {
				id: d.id,
				notification_id: d.notification_id,
				correlation_id: d.correlation_id,
				trigger_name: d.trigger_name,
				title: d.title,
				body: d.body,
				status: d.status,
				attempt: d.attempt,
				attempts: 1,
				error: d.error,
				started_at: d.created_at,
				finished_at: d.finished_at,
			});
		} else {
			// Rows arrive newest-first, so every later hit is an older attempt.
			existing.attempts += 1;
			existing.started_at = d.created_at;
		}
	}
	return [...byCorrelation.values()];
}

export type DeliveryStatusId = 'all' | 'DELIVERED' | 'FAILED' | 'PENDING';

export const DELIVERY_STATUSES: {id: DeliveryStatusId; label: string}[] = [
	{id: 'all', label: 'All Alerts'},
	{id: 'DELIVERED', label: 'Delivered'},
	{id: 'FAILED', label: 'Not delivered'},
	{id: 'PENDING', label: 'Pending'},
];

const STATUS_META: Record<
	string,
	{label: string; tile: string; icon: ComponentType<{className?: string}>}
> = {
	DELIVERED: {
		label: 'Delivered',
		tile: 'bg-emerald-500/10 text-emerald-400',
		icon: Check,
	},
	FAILED: {
		label: 'Not delivered',
		tile: 'bg-rose-500/10 text-rose-400',
		icon: X,
	},
	PENDING: {
		label: 'Pending',
		tile: 'bg-amber-500/10 text-amber-400',
		icon: Clock,
	},
};

const UNKNOWN = {
	label: 'Unknown',
	tile: 'bg-muted text-muted-foreground',
	icon: AlertCircle,
};

/** The delivery-attempt list — what actually went out, and what failed. */
export function NotificationDeliveriesList({
	rows,
	isLoading,
	filtered,
	channelName,
	onClear,
}: {
	rows: DeliveryAlert[];
	isLoading: boolean;
	/** Filters are on, so an empty list means "no match", not "never sent". */
	filtered: boolean;
	channelName: (id: number) => string;
	onClear: () => void;
}) {
	if (isLoading) return <ListSkeleton />;

	if (rows.length === 0)
		return filtered ? (
			<ListEmpty
				icon={Search}
				title="No alerts match"
				description="Clear the search and status filter to see every delivered alert again."
				action={
					<Button
						variant="outline"
						onClick={onClear}
						className="h-9 gap-1.5">
						Clear filters
					</Button>
				}
			/>
		) : (
			<ListEmpty
				icon={History}
				title="Nothing delivered yet"
				description="Every alert this organization sends — and whether the provider accepted it — shows up here."
			/>
		);

	return (
		<ListCard>
			{rows.map(d => {
				const meta = STATUS_META[d.status] ?? UNKNOWN;
				const Icon = meta.icon;
				const subtitle = `${channelName(d.notification_id)} · ${triggerEventLabel(d.trigger_name)} · ${timeAgo(d.started_at)}`;
				return (
					<ListRow
						key={d.correlation_id}
						icon={
							<RowTile className={meta.tile}>
								<Icon className="size-4" />
							</RowTile>
						}
						title={d.title || triggerEventLabel(d.trigger_name)}
						subtitle={
							d.status === 'FAILED' && d.error
								? `${subtitle} · ${d.error}`
								: subtitle
						}
						badges={
							<>
								{d.attempts > 1 && (
									<span className="rounded-md bg-foreground/[0.05] px-1.5 py-0.5 text-[10px] font-medium text-muted-foreground">
										{d.attempts} attempts
									</span>
								)}
								<span className="rounded-md bg-foreground/[0.05] px-1.5 py-0.5 text-[10px] font-medium text-muted-foreground">
									{meta.label}
								</span>
							</>
						}
					/>
				);
			})}
		</ListCard>
	);
}
