import {createFileRoute, Navigate} from '@tanstack/react-router';
import {useState} from 'react';
import {Plus} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {
	AlertDialog,
	AlertDialogAction,
	AlertDialogCancel,
	AlertDialogContent,
	AlertDialogDescription,
	AlertDialogFooter,
	AlertDialogHeader,
	AlertDialogTitle,
} from '#/components/ui/alert-dialog';
import {
	useActiveOrgId,
	useNotificationDelete,
	useNotificationListByOrganization,
	useNotificationTest,
	type NotificationPayload,
} from '#/api/live';
import {formatApiError} from '#/api/utils';
import {toast} from 'sonner';
import {ListCard} from '#/components/settings/list-card';
import {
	NotificationChannelRow,
	NotificationChannelsEmpty,
} from '#/components/settings/notifications/notification-channel-row';
import {NotificationChannelDialog} from '#/components/settings/notifications/notification-channel-dialog';
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

/**
 * Notification channels: list-card rows with provider tile, name, trigger
 * summary and a Test / Edit / Delete kebab. Create is a dialog: provider +
 * its config fields + which events fire the channel.
 */
export function NotificationsPage() {
	const orgId = useActiveOrgId();
	const {data: channels = [], isLoading} =
		useNotificationListByOrganization(orgId);
	const deleteMutation = useNotificationDelete();
	const testMutation = useNotificationTest();

	const [draft, setDraft] = useState<DraftState | null>(null);
	const [editing, setEditing] = useState<NotificationPayload | null>(null);
	const [deleting, setDeleting] = useState<NotificationPayload | null>(
		null,
	);

	const openCreate = () => {
		setEditing(null);
		setDraft(emptyDraft());
	};

	const openEdit = (n: NotificationPayload) => {
		setEditing(n);
		setDraft(draftFromChannel(n));
	};

	const test = async (n: NotificationPayload) => {
		try {
			await testMutation.mutateAsync(n.id);
			toast.success(`Test sent to ${n.name}`);
		} catch (err) {
			toast.error(formatApiError(err));
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
		<div className="flex w-full animate-in flex-col gap-6 duration-200 fade-in">
			<div className="flex flex-wrap items-start justify-between gap-4 sm:items-center">
				<div className="flex flex-col gap-0.5">
					<h2 className="text-sm font-medium text-foreground">
						Notification channels
					</h2>
					<p className="text-xs text-muted-foreground">
						Discord, Slack, Telegram, email, and webhook alerts for
						deployments, backups, and server events.
					</p>
				</div>
				<Button onClick={openCreate} className="gap-1.5 px-4 text-sm">
					<Plus className="size-4" />
					Add channel
				</Button>
			</div>

			<ListCard
				title={`${channels.length} ${channels.length === 1 ? 'channel' : 'channels'}`}
				description="Each channel fires on the events you select.">
				{isLoading ? (
					<div className="space-y-2 p-5">
						{[...Array(2)].map((_, i) => (
							<div
								key={i}
								className="h-14 animate-pulse rounded-xl bg-muted/30"
							/>
						))}
					</div>
				) : channels.length === 0 ? (
					<NotificationChannelsEmpty onCreate={openCreate} />
				) : (
					<div className="divide-y divide-border/40">
						{channels.map(n => (
							<NotificationChannelRow
								key={n.id}
								channel={n}
								onTest={n => void test(n)}
								onEdit={openEdit}
								onDelete={setDeleting}
							/>
						))}
					</div>
				)}
			</ListCard>

			{/* Create / edit dialog */}
			<NotificationChannelDialog
				draft={draft}
				setDraft={setDraft}
				editing={editing}
				orgId={orgId}
				onClose={() => setDraft(null)}
			/>

			{/* Delete confirmation */}
			<AlertDialog
				open={deleting !== null}
				onOpenChange={open => !open && setDeleting(null)}>
				<AlertDialogContent>
					<AlertDialogHeader>
						<AlertDialogTitle>Delete channel</AlertDialogTitle>
						<AlertDialogDescription>
							Alerts will stop reaching{' '}
							<span className="font-medium text-foreground">
								{deleting?.name}
							</span>
							. This cannot be undone.
						</AlertDialogDescription>
					</AlertDialogHeader>
					<AlertDialogFooter>
						<AlertDialogCancel>Cancel</AlertDialogCancel>
						<AlertDialogAction
							onClick={() => void remove()}
							className="bg-destructive text-white hover:bg-destructive/90">
							Delete
						</AlertDialogAction>
					</AlertDialogFooter>
				</AlertDialogContent>
			</AlertDialog>
		</div>
	);
}
