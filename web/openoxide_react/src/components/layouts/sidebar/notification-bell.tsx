import {useMemo, useState} from 'react';
import {useNavigate} from '@tanstack/react-router';
import {
	Bell,
	Check,
	CheckCheck,
	Clock,
	ListChecks,
	Settings2,
	X,
} from 'lucide-react';
import {
	useActiveOrgId,
	useNotificationDeliveries,
	useNotificationListByOrganization,
} from '#/api/live';
import {cn} from '#/api/utils';
import {timeAgo} from '#/components/activity/activity-model';
import {
	DropdownMenu,
	DropdownMenuContent,
	DropdownMenuLabel,
	DropdownMenuSeparator,
	DropdownMenuTrigger,
} from '#/components/ui/dropdown';
import {groupDeliveries} from '#/components/settings/notifications/notification-deliveries';
import {triggerEventLabel} from '#/components/settings/notifications/notification-triggers';

const STATUS_DOT: Record<string, string> = {
	DELIVERED: 'bg-emerald-500',
	FAILED: 'bg-rose-500',
	PENDING: 'bg-amber-500',
};

const ROWS = 8;
const READ_CAP = 500;
const readKey = (orgId: bigint) => `openoxide:notifications:read:${orgId}`;

/** Delivery ids the user has acknowledged, persisted per organization in localStorage — the backend has no read receipts, so the panel owns this state. */
function useReadDeliveries(orgId: bigint) {
	const [readIds, setReadIds] = useState<Set<number>>(() => {
		try {
			const raw = localStorage.getItem(readKey(orgId));
			return new Set<number>(raw ? (JSON.parse(raw) as number[]) : []);
		} catch {
			return new Set<number>();
		}
	});
	const mark = (ids: number[]) =>
		setReadIds(prev => {
			const next = new Set(prev);
			for (const id of ids) next.add(id);
			const trimmed = [...next].slice(-READ_CAP);
			try {
				localStorage.setItem(readKey(orgId), JSON.stringify(trimmed));
			} catch {
				/* private mode — read state just won't persist */
			}
			return new Set(trimmed);
		});
	return {
		readIds,
		markRead: (id: number) => mark([id]),
		markAllRead: mark,
	};
}

/** Alerts rail entry — the organization's recent delivery attempts with a local unread badge, per-row mark-as-read (rows expand to show the delivered message) and a jump to the full history. */
export function SidebarNotifications({collapsed}: {collapsed: boolean}) {
	const navigate = useNavigate();
	const orgId = useActiveOrgId();
	const {data: deliveries = []} = useNotificationDeliveries(orgId);
	const {data: channels = []} = useNotificationListByOrganization(orgId);
	const {readIds, markRead, markAllRead} = useReadDeliveries(orgId);
	const [expanded, setExpanded] = useState<string | number | null>(null);

	// One entry per alert — raw rows would repeat an alert once per retry.
	const alerts = useMemo(() => groupDeliveries(deliveries), [deliveries]);
	const recent = alerts.slice(0, ROWS);
	const unread = useMemo(
		() => alerts.filter(a => !readIds.has(a.id)),
		[alerts, readIds],
	);
	const unreadCount = unread.length;

	const channelName = (id: number) =>
		channels.find(c => c.id === id)?.name || 'Channel removed';

	const openSettings = (view?: 'deliveries') =>
		void navigate({
			to: '/settings',
			search: {tab: 'notifications', view} as never,
		});

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
						{unreadCount > 0 && (
							<span className="absolute -top-1 -right-1.5 flex h-3.5 min-w-3.5 items-center justify-center rounded-full bg-rose-500 px-0.5 font-mono text-[9px] font-bold text-white tabular-nums ring-2 ring-sidebar">
								{unreadCount > 9 ? '9+' : unreadCount}
							</span>
						)}
					</button>
				}
			/>
			<DropdownMenuContent
				side="top"
				align={collapsed ? 'start' : 'end'}
				className="w-80 rounded-lg">
				<DropdownMenuLabel className="flex items-center justify-between gap-2 px-1.5">
					<span className="flex items-center gap-2">
						Notifications
						{unreadCount > 0 && (
							<span className="rounded-full bg-rose-500/10 px-1.5 py-px font-mono text-[10px] font-semibold text-rose-500 tabular-nums">
								{unreadCount} new
							</span>
						)}
					</span>
					{unreadCount > 0 && (
						<button
							type="button"
							onClick={() => markAllRead(alerts.map(a => a.id))}
							className="flex cursor-pointer items-center gap-1 text-[11px] font-medium text-primary hover:underline">
							<CheckCheck className="size-3.5" />
							Mark all read
						</button>
					)}
				</DropdownMenuLabel>
				<DropdownMenuSeparator />

				{recent.length === 0 ? (
					<p className="px-1.5 py-3 text-xs text-muted-foreground">
						No alerts delivered yet. Channels you configure in Settings
						report here once they fire.
					</p>
				) : (
					<div className="flex max-h-80 flex-col gap-px overflow-y-auto">
						{recent.map(alert => {
							const isRead = readIds.has(alert.id);
							const isOpen = expanded === alert.id;
							return (
								<button
									key={alert.correlation_id}
									type="button"
									onClick={() => {
										if (!isRead) markRead(alert.id);
										setExpanded(isOpen ? null : alert.id);
									}}
									className={cn(
										'flex w-full cursor-pointer items-start gap-2 rounded-md px-1.5 py-1.5 text-left transition-colors hover:bg-accent/60',
										isOpen && 'bg-accent/60',
									)}>
									<span
										className={cn(
											'mt-1.5 size-1.5 shrink-0 rounded-full',
											isRead
												? 'bg-muted-foreground/25'
												: (STATUS_DOT[alert.status] ??
														'bg-muted-foreground'),
										)}
									/>
									<span className="min-w-0 flex-1">
										<span className="flex items-center gap-1.5">
											<span
												className={cn(
													'truncate text-[13px] leading-tight text-foreground',
													isRead ? 'font-normal' : 'font-semibold',
												)}>
												{alert.title ||
													triggerEventLabel(alert.trigger_name)}
											</span>
											{alert.status === 'FAILED' ? (
												<X className="ml-auto size-3.5 shrink-0 text-rose-500/80" />
											) : alert.status === 'PENDING' ? (
												<Clock className="ml-auto size-3.5 shrink-0 text-amber-500/80" />
											) : (
												<Check className="ml-auto size-3.5 shrink-0 text-emerald-500/80" />
											)}
										</span>
										<span className="mt-0.5 block truncate text-[11px] leading-tight text-muted-foreground">
											{channelName(alert.notification_id)} ·{' '}
											{timeAgo(alert.started_at)}
										</span>
										{isOpen && (
											<span className="mt-1.5 flex flex-col gap-1 rounded-md bg-muted/40 px-2 py-1.5">
												<span className="text-[11px] leading-snug break-words text-foreground/80">
													{alert.body || '(no message body)'}
												</span>
												{alert.error && (
													<span className="text-[11px] leading-snug break-words text-rose-500">
														{alert.error}
													</span>
												)}
												<span className="font-mono text-[10px] text-muted-foreground">
													{alert.status} · attempt {alert.attempt}
													{alert.attempts > 1
														? ` of ${alert.attempts}`
														: ''}
													·{' '}
													{timeAgo(alert.finished_at ?? alert.started_at)}
												</span>
											</span>
										)}
									</span>
								</button>
							);
						})}
					</div>
				)}

				<DropdownMenuSeparator />
				<button
					type="button"
					onClick={() => openSettings('deliveries')}
					className="flex w-full cursor-pointer items-center gap-2 rounded-md px-1.5 py-1.5 text-[13px] font-medium text-foreground transition-colors hover:bg-accent">
					<ListChecks className="size-4 text-muted-foreground" />
					View all deliveries
				</button>
				<button
					type="button"
					onClick={() => openSettings()}
					className="flex w-full cursor-pointer items-center gap-2 rounded-md px-1.5 py-1.5 text-[13px] font-medium text-foreground transition-colors hover:bg-accent">
					<Settings2 className="size-4 text-muted-foreground" />
					Notification settings
				</button>
			</DropdownMenuContent>
		</DropdownMenu>
	);
}
