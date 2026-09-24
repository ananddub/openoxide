import {useNavigate} from '@tanstack/react-router';
import {Bell, Check, Clock, Settings2, X} from 'lucide-react';
import {
	useActiveOrgId,
	useNotificationDeliveries,
	useNotificationListByOrganization,
	type NotificationDeliveryPayload,
} from '#/api/live';
import {cn} from '#/api/utils';
import {timeAgo} from '#/components/activity/activity-model';
import {
	DropdownMenu,
	DropdownMenuContent,
	DropdownMenuItem,
	DropdownMenuLabel,
	DropdownMenuSeparator,
	DropdownMenuTrigger,
} from '#/components/ui/dropdown';
import {triggerEventLabel} from '#/components/settings/notifications/notification-triggers';

const STATUS_DOT: Record<string, string> = {
	DELIVERED: 'bg-emerald-500',
	FAILED: 'bg-rose-500',
	PENDING: 'bg-amber-500',
};

const ROWS = 8;

/** Alerts rail entry — the organization's recent delivery attempts, read straight from the same live query the Settings page uses. Rows are quiet text (no per-row action); the one control is the jump to the full history. */
export function SidebarNotifications({collapsed}: {collapsed: boolean}) {
	const navigate = useNavigate();
	const orgId = useActiveOrgId();
	const {data: deliveries = []} = useNotificationDeliveries(orgId);
	const {data: channels = []} = useNotificationListByOrganization(orgId);

	const recent = deliveries.slice(0, ROWS);
	const needsAttention = deliveries.some(
		d => d.status === 'FAILED' || d.status === 'PENDING',
	);

	const channelName = (delivery: NotificationDeliveryPayload) =>
		channels.find(c => c.id === Number(delivery.notification_id))?.name ||
		'Channel removed';

	return (
		<DropdownMenu>
			<DropdownMenuTrigger
				render={
					<button
						type="button"
						aria-label="Notifications"
						title="Notifications"
						className="relative flex size-6 shrink-0 items-center justify-center rounded-md text-muted-foreground transition-colors hover:bg-sidebar-accent hover:text-foreground">
						<Bell className="size-4" strokeWidth={1.7} />
						{needsAttention && (
							<span className="absolute top-0.5 right-0.5 size-1.5 rounded-full bg-rose-500 ring-2 ring-sidebar" />
						)}
					</button>
				}
			/>
			<DropdownMenuContent
				side="top"
				align={collapsed ? 'start' : 'end'}
				className="w-80 rounded-lg">
				<DropdownMenuLabel className="flex items-center justify-between px-1.5">
					Notifications
					<span className="font-mono text-[11px] opacity-70">
						{deliveries.length}
					</span>
				</DropdownMenuLabel>
				<DropdownMenuSeparator />

				{recent.length === 0 ? (
					<p className="px-1.5 py-3 text-xs text-muted-foreground">
						No alerts delivered yet. Channels you configure in Settings
						report here once they fire.
					</p>
				) : (
					<div className="flex flex-col">
						{recent.map(delivery => (
							<div
								key={delivery.id}
								className="flex cursor-default items-start gap-2 rounded-md px-1.5 py-1.5">
								<span
									className={cn(
										'mt-1.5 size-1.5 shrink-0 rounded-full',
										STATUS_DOT[delivery.status] ?? 'bg-muted-foreground',
									)}
								/>
								<div className="min-w-0 flex-1">
									<p className="truncate text-[13px] leading-tight font-medium text-foreground">
										{delivery.title ||
											triggerEventLabel(delivery.trigger_name)}
									</p>
									<p className="mt-0.5 truncate text-[11px] leading-tight text-muted-foreground">
										{channelName(delivery)} ·{' '}
										{timeAgo(delivery.created_at)}
									</p>
								</div>
								{delivery.status === 'FAILED' ? (
									<X className="mt-0.5 size-3.5 shrink-0 text-rose-500/80" />
								) : delivery.status === 'PENDING' ? (
									<Clock className="mt-0.5 size-3.5 shrink-0 text-amber-500/80" />
								) : (
									<Check className="mt-0.5 size-3.5 shrink-0 text-emerald-500/80" />
								)}
							</div>
						))}
					</div>
				)}

				<DropdownMenuSeparator />
				<DropdownMenuItem
					className="text-[13px]"
					onClick={() =>
						void navigate({
							to: '/settings',
							search: {tab: 'notifications'} as never,
						})
					}>
					<Settings2 className="size-4" /> Notification settings
				</DropdownMenuItem>
			</DropdownMenuContent>
		</DropdownMenu>
	);
}
