import {
	createFileRoute,
	Navigate,
	useSearch,
} from '@tanstack/react-router';
import {useEffect, useMemo, useRef, useState} from 'react';
import {BellRing, History, Plus, Search} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import {
	useActiveOrgId,
	useNotificationDelete,
	useNotificationDeliveries,
	useNotificationListByOrganization,
	useNotificationTest,
	type NotificationPayload,
} from '#/api/live';
import {cn, formatApiError} from '#/api/utils';
import {toast} from 'sonner';
import {
	GridSkeleton,
	ListCard,
	ListEmpty,
	ListRegion,
	ListSkeleton,
	SectionHeader,
	Toolbar,
	ToolbarSearch,
	ViewGrid,
	ViewToggle,
	useListView,
} from '#/components/settings/list-card';
import {ConfirmDeleteDialog} from '#/components/settings/confirm-delete-dialog';
import {
	NotificationChannelCard,
	NotificationChannelRow,
	NotificationChannelsEmpty,
} from '#/components/settings/notifications/notification-channel-row';
import {NotificationChannelSheet} from '#/components/settings/notifications/notification-channel-sheet';
import {
	DELIVERY_STATUSES,
	NotificationDeliveriesList,
	groupDeliveries,
	type DeliveryStatusId,
} from '#/components/settings/notifications/notification-deliveries';
import {
	PROVIDERS,
	providerMeta,
} from '#/components/settings/notifications/notification-providers';
import {
	emptyDraft,
	draftFromChannel,
	type DraftState,
} from '#/components/settings/notifications/notification-triggers';

export const Route = createFileRoute('/_app/settings/notifications')({
	component: () => (
		<Navigate to="/settings" search={{tab: 'notifications'}} replace />
	),
});

type ViewId = 'channels' | 'deliveries';

/** Notifications — the SSH Keys / S3 grammar: one toolbar row (search · filter · view toggle · New) above a single bordered list card. The two views are the organization's channels and the alerts the backend actually delivered for them. Create/edit is a sheet, delete a quiet confirm dialog. */
export function NotificationsPage() {
	const orgId = useActiveOrgId();
	const {data: channels = [], isLoading} =
		useNotificationListByOrganization(orgId);
	const {data: deliveries = [], isLoading: deliveriesLoading} =
		useNotificationDeliveries(orgId);
	const deleteMutation = useNotificationDelete();
	const testMutation = useNotificationTest();

	const [view, setView] = useState<ViewId>('channels');
	// Deep link from the bell's "View all deliveries": ?tab=notifications&view=deliveries
	const {view: viewParam} = useSearch({from: '/_app/settings'});
	useEffect(() => {
		if (viewParam) setView(viewParam);
	}, [viewParam]);
	const [layout, setLayout] = useListView('settings-notifications-view');
	const [search, setSearch] = useState('');
	const [providerFilter, setProviderFilter] = useState('All');
	const [status, setStatus] = useState<DeliveryStatusId>('all');
	const [testingId, setTestingId] = useState<number | null>(null);

	const [draft, setDraft] = useState<DraftState | null>(null);
	const [editing, setEditing] = useState<NotificationPayload | null>(null);
	const [deleting, setDeleting] = useState<NotificationPayload | null>(
		null,
	);

	const searchRef = useRef<HTMLInputElement>(null);

	// Same `/` shortcut as the Projects / S3 toolbars.
	useEffect(() => {
		const onKey = (e: KeyboardEvent) => {
			if (e.key !== '/' || e.metaKey || e.ctrlKey || e.altKey) return;
			const el = document.activeElement as HTMLElement | null;
			if (
				el &&
				(el.tagName === 'INPUT' ||
					el.tagName === 'TEXTAREA' ||
					el.tagName === 'SELECT' ||
					el.isContentEditable)
			) {
				return;
			}
			e.preventDefault();
			searchRef.current?.focus();
		};
		window.addEventListener('keydown', onKey);
		return () => window.removeEventListener('keydown', onKey);
	}, []);

	const channelName = (id: number) =>
		channels.find(c => c.id === id)?.name ?? `Channel #${id}`;

	/** Only the providers this organization actually uses get a filter row. */
	const usedProviders = useMemo(() => {
		const used = new Set(channels.map(c => c.notification_type));
		return PROVIDERS.filter(p => used.has(p.id));
	}, [channels]);

	const query = search.trim().toLowerCase();
	const filtering =
		query !== '' ||
		(view === 'channels' ? providerFilter !== 'All' : status !== 'all');

	const visibleChannels = useMemo(
		() =>
			channels.filter(c => {
				if (
					providerFilter !== 'All' &&
					c.notification_type !== providerFilter
				)
					return false;
				if (!query) return true;
				return `${c.name} ${providerMeta(c.notification_type).label}`
					.toLowerCase()
					.includes(query);
			}),
		[channels, providerFilter, query],
	);

	const visibleAlerts = useMemo(
		() =>
			groupDeliveries(deliveries).filter(a => {
				if (status !== 'all' && a.status !== status) return false;
				if (!query) return true;
				return `${a.title} ${a.body} ${channelName(a.notification_id)} ${a.error ?? ''}`
					.toLowerCase()
					.includes(query);
			}),
		// eslint-disable-next-line react-hooks/exhaustive-deps
		[deliveries, status, query, channels],
	);

	/** Newest attempt per channel — the row's last-known delivery state. */
	const lastDelivery = useMemo(() => {
		const byChannel = new Map<
			number,
			{status: string; at: number; error: string | null}
		>();
		// The list arrives newest-first, so the first hit is the latest.
		for (const d of deliveries) {
			if (!byChannel.has(d.notification_id))
				byChannel.set(d.notification_id, {
					status: d.status,
					at: d.created_at,
					error: d.error,
				});
		}
		return byChannel;
	}, [deliveries]);

	const clearFilters = () => {
		setSearch('');
		setProviderFilter('All');
		setStatus('all');
	};

	const openCreate = () => {
		setEditing(null);
		setDraft(emptyDraft());
	};

	const openEdit = (n: NotificationPayload) => {
		setEditing(n);
		setDraft(draftFromChannel(n));
	};

	const test = async (n: NotificationPayload) => {
		setTestingId(n.id);
		try {
			const res = await testMutation.mutateAsync(n.id);
			// The RPC resolves even when the dispatch failed — success carries the verdict.
			if (res.success) {
				toast.success(`Test sent to ${n.name}`);
			} else {
				toast.error(res.message || `Test to ${n.name} failed`);
			}
		} catch (err) {
			toast.error(formatApiError(err));
		} finally {
			setTestingId(null);
		}
	};

	const remove = async () => {
		if (!deleting) return;
		try {
			await deleteMutation.mutateAsync(deleting.id);
			toast.success('Channel deleted');
		} catch (err) {
			toast.error(formatApiError(err));
		} finally {
			setDeleting(null);
		}
	};

	return (
		<div
			className={
				view === 'channels'
					? 'flex h-full w-full animate-in flex-col gap-5 duration-200 fade-in'
					: 'flex w-full animate-in flex-col gap-5 duration-200 fade-in'
			}>
			<SectionHeader
				title="Notifications"
				description="Channels that receive this organization's alerts, and the deliveries the backend actually sent."
				action={
					<Button onClick={openCreate} className="h-9 gap-1.5">
						<Plus className="size-3.5" />
						New channel
					</Button>
				}
			/>

			{/* Search · filter · channels / alerts · grid-list */}
			<Toolbar
				action={
					<>
						{view === 'channels' ? (
							<Select
								value={providerFilter}
								onValueChange={v => setProviderFilter(v ?? 'All')}>
								<SelectTrigger className="h-9 w-auto rounded-lg bg-card">
									<SelectValue>
										{providerFilter === 'All'
											? 'All Providers'
											: providerMeta(providerFilter).label}
									</SelectValue>
								</SelectTrigger>
								<SelectContent className="bg-card">
									<SelectItem value="All">All Providers</SelectItem>
									{usedProviders.map(p => (
										<SelectItem key={p.id} value={p.id}>
											{p.label}
										</SelectItem>
									))}
								</SelectContent>
							</Select>
						) : (
							<Select
								value={status}
								onValueChange={v =>
									setStatus((v as DeliveryStatusId) ?? 'all')
								}>
								<SelectTrigger className="h-9 w-auto rounded-lg bg-card">
									<SelectValue>
										{DELIVERY_STATUSES.find(s => s.id === status)?.label}
									</SelectValue>
								</SelectTrigger>
								<SelectContent className="bg-card">
									{DELIVERY_STATUSES.map(s => (
										<SelectItem key={s.id} value={s.id}>
											{s.label}
										</SelectItem>
									))}
								</SelectContent>
							</Select>
						)}

						{/* Channels / delivered-alerts toggle */}
						<div className="flex items-center rounded-lg border bg-card p-1">
							<button
								type="button"
								aria-label="Channels"
								title="Channels"
								onClick={() => setView('channels')}
								className={cn(
									'flex size-7 cursor-pointer items-center justify-center rounded-md text-muted-foreground transition-colors hover:text-foreground',
									view === 'channels' && 'bg-accent text-foreground',
								)}>
								<BellRing className="size-3.5" />
							</button>
							<button
								type="button"
								aria-label="Delivered alerts"
								title="Delivered alerts"
								onClick={() => setView('deliveries')}
								className={cn(
									'flex size-7 cursor-pointer items-center justify-center rounded-md text-muted-foreground transition-colors hover:text-foreground',
									view === 'deliveries' && 'bg-accent text-foreground',
								)}>
								<History className="size-3.5" />
							</button>
						</div>

						{view === 'channels' && (
							<ViewToggle view={layout} onChange={setLayout} />
						)}
					</>
				}>
				<ToolbarSearch
					inputRef={searchRef}
					value={search}
					onChange={setSearch}
					placeholder={
						view === 'channels'
							? 'Search channels by name or provider…'
							: 'Search alerts by title, body or error…'
					}
					className="min-w-[200px] flex-1 sm:max-w-none"
				/>
			</Toolbar>

			{/* Channels: fixed card area with inner scroll. Deliveries (alert history): scrolls with the page, title included. */}
			{view === 'channels' ? (
				<ListRegion>
					{isLoading ? (
						layout === 'grid' ? (
							<GridSkeleton />
						) : (
							<ListSkeleton />
						)
					) : channels.length === 0 ? (
						<NotificationChannelsEmpty onCreate={openCreate} />
					) : visibleChannels.length === 0 && filtering ? (
						<ListEmpty
							icon={Search}
							title="No channels match"
							description="Clear the search and provider filter to see every channel again."
							action={
								<Button
									variant="outline"
									onClick={clearFilters}
									className="h-9 gap-1.5">
									Clear filters
								</Button>
							}
						/>
					) : layout === 'grid' ? (
						<ViewGrid>
							{visibleChannels.map(n => (
								<NotificationChannelCard
									key={n.id}
									channel={n}
									last={lastDelivery.get(n.id)}
									testing={testingId === n.id}
									onTest={n => void test(n)}
									onEdit={openEdit}
									onDelete={setDeleting}
								/>
							))}
						</ViewGrid>
					) : (
						<ListCard>
							{visibleChannels.map(n => (
								<NotificationChannelRow
									key={n.id}
									channel={n}
									last={lastDelivery.get(n.id)}
									testing={testingId === n.id}
									onTest={n => void test(n)}
									onEdit={openEdit}
									onDelete={setDeleting}
								/>
							))}
						</ListCard>
					)}
				</ListRegion>
			) : (
				<NotificationDeliveriesList
					rows={visibleAlerts}
					isLoading={deliveriesLoading}
					channelName={channelName}
					filtered={filtering}
					onClear={clearFilters}
				/>
			)}

			{/* Create / edit sheet */}
			<NotificationChannelSheet
				draft={draft}
				setDraft={setDraft}
				editing={editing}
				orgId={orgId}
				onClose={() => setDraft(null)}
			/>

			{/* Delete confirmation */}
			<ConfirmDeleteDialog
				open={deleting !== null}
				onOpenChange={o => !o && setDeleting(null)}
				title="Delete channel"
				description={
					<span>
						Alerts will stop reaching{' '}
						<span className="font-medium text-foreground">
							{deleting?.name}
						</span>
						. This cannot be undone.
					</span>
				}
				confirmLabel="Delete channel"
				busy={deleteMutation.isPending}
				onConfirm={() => void remove()}
			/>
		</div>
	);
}
