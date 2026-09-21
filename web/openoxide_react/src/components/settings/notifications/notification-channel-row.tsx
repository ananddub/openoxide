import {
	Bell,
	Pencil,
	Plus,
	FlaskConical,
	MoreHorizontal,
	Trash2,
} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {
	DropdownMenu,
	DropdownMenuContent,
	DropdownMenuItem,
	DropdownMenuTrigger,
} from '#/components/ui/dropdown';
import type {NotificationPayload} from '#/api/live';
import {
	ListEmpty,
	ListRow,
	RowTile,
} from '#/components/settings/list-card';
import {providerMeta} from './notification-providers';
import {triggerSummary} from './notification-triggers';

/** One notification channel row with its Test/Edit/Delete kebab. */
export function NotificationChannelRow({
	channel,
	onTest,
	onEdit,
	onDelete,
}: {
	channel: NotificationPayload;
	onTest: (n: NotificationPayload) => void;
	onEdit: (n: NotificationPayload) => void;
	onDelete: (n: NotificationPayload) => void;
}) {
	const meta = providerMeta(channel.notification_type);
	const Icon = meta.icon;
	return (
		<ListRow
			icon={
				<RowTile className={meta.tile}>
					<Icon className="size-4" />
				</RowTile>
			}
			title={channel.name}
			subtitle={triggerSummary(channel)}
			badges={
				<span className="rounded-md bg-muted/50 px-1.5 py-0.5 text-[10px] font-medium text-muted-foreground">
					{meta.label}
				</span>
			}>
			<DropdownMenu>
				<DropdownMenuTrigger
					render={
						<Button
							variant="ghost"
							size="icon"
							className="size-7 rounded-lg text-muted-foreground hover:text-foreground"
						/>
					}>
					<MoreHorizontal className="size-4" />
				</DropdownMenuTrigger>
				<DropdownMenuContent align="end" className="w-40">
					<DropdownMenuItem
						onClick={() => onTest(channel)}
						className="gap-2 text-xs">
						<FlaskConical className="size-3.5" /> Send test
					</DropdownMenuItem>
					<DropdownMenuItem
						onClick={() => onEdit(channel)}
						className="gap-2 text-xs">
						<Pencil className="size-3.5" /> Edit
					</DropdownMenuItem>
					<DropdownMenuItem
						onClick={() => onDelete(channel)}
						className="gap-2 text-xs text-destructive focus:text-destructive">
						<Trash2 className="size-3.5" /> Delete
					</DropdownMenuItem>
				</DropdownMenuContent>
			</DropdownMenu>
		</ListRow>
	);
}

/** Empty-state shown when the organization has no channels yet. */
export function NotificationChannelsEmpty({
	onCreate,
}: {
	onCreate: () => void;
}) {
	return (
		<ListEmpty
			icon={Bell}
			title="No channels yet"
			description="Add a Discord, Slack, Telegram, or email channel to get deployment and server alerts."
			action={
				<Button size="sm" onClick={onCreate} className="gap-1.5">
					<Plus className="size-3.5" /> Add channel
				</Button>
			}
		/>
	);
}
