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

export type DeliveryStatusId = 'all' | 'DELIVERED' | 'FAILED' | 'PENDING';

export const DELIVERY_STATUSES: {id: DeliveryStatusId; label: string}[] = [
	{id: 'all', label: 'All Alerts'},
	{id: 'DELIVERED', label: 'Delivered'},
	{id: 'FAILED', label: 'Failed'},
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
		label: 'Failed',
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
	rows: NotificationDeliveryPayload[];
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
				return (
					<ListRow
						key={d.id}
						icon={
							<RowTile className={meta.tile}>
								<Icon className="size-4" />
							</RowTile>
						}
						title={d.title || triggerEventLabel(d.trigger_name)}
						subtitle={
							d.status === 'FAILED' && d.error
								? `${channelName(d.notification_id)} · ${d.error}`
								: `${channelName(d.notification_id)} · ${triggerEventLabel(d.trigger_name)} · ${timeAgo(d.created_at)}`
						}
						badges={
							<span className="rounded-md bg-foreground/[0.05] px-1.5 py-0.5 text-[10px] font-medium text-muted-foreground">
								{meta.label}
							</span>
						}
					/>
				);
			})}
		</ListCard>
	);
}
