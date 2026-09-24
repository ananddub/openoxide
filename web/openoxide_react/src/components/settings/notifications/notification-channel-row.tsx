import {
	Bell,
	FlaskConical,
	Loader2,
	MoreHorizontal,
	Pencil,
	Plus,
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
import {timeAgo} from '#/components/activity/activity-model';
import {
	ListEmpty,
	ListRow,
	RowTile,
	ViewCard,
} from '#/components/settings/list-card';
import {providerMeta} from './notification-providers';
import {triggerSummary} from './notification-triggers';

type ChannelProps = {
	channel: NotificationPayload;
	/** The channel's newest delivery attempt, if it has ever fired. */
	last?: {status: string; at: number; error: string | null};
	testing: boolean;
	onTest: (n: NotificationPayload) => void;
	onEdit: (n: NotificationPayload) => void;
	onDelete: (n: NotificationPayload) => void;
};

function ChannelMenu({
	channel,
	testing,
	onTest,
	onEdit,
	onDelete,
}: ChannelProps) {
	return (
		<DropdownMenu>
			<DropdownMenuTrigger
				render={
					<Button
						variant="ghost"
						size="icon"
						aria-label="Channel actions"
						className="size-7 rounded-lg text-muted-foreground hover:text-foreground focus-visible:ring-0"
					/>
				}>
				<MoreHorizontal className="size-4" />
			</DropdownMenuTrigger>
			<DropdownMenuContent align="end" className="w-44">
				<DropdownMenuItem
					disabled={testing}
					onClick={() => onTest(channel)}
					className="cursor-pointer gap-2 text-xs">
					{testing ? (
						<Loader2 className="size-3.5 animate-spin" />
					) : (
						<FlaskConical className="size-3.5" />
					)}
					{testing ? 'Sending…' : 'Send test'}
				</DropdownMenuItem>
				<DropdownMenuItem
					onClick={() => onEdit(channel)}
					className="cursor-pointer gap-2 text-xs">
					<Pencil className="size-3.5" /> Edit
				</DropdownMenuItem>
				<DropdownMenuItem
					onClick={() => onDelete(channel)}
					className="cursor-pointer gap-2 text-xs text-destructive focus:text-destructive">
					<Trash2 className="size-3.5" /> Delete
				</DropdownMenuItem>
			</DropdownMenuContent>
		</DropdownMenu>
	);
}

function ChannelChips({
	channel,
	failed,
}: {
	channel: NotificationPayload;
	failed: boolean;
}) {
	const meta = providerMeta(channel.notification_type);
	return (
		<>
			<span className="shrink-0 rounded-md bg-foreground/[0.05] px-1.5 py-0.5 text-[10px] font-medium text-muted-foreground">
				{meta.label}
			</span>
			{failed && (
				<span className="shrink-0 rounded-md bg-rose-500/10 px-1.5 py-0.5 text-[10px] font-medium text-rose-400">
					Last delivery failed
				</span>
			)}
		</>
	);
}

/** One notification channel row with its Test/Edit/Delete kebab. */
export function NotificationChannelRow(props: ChannelProps) {
	const {channel, last, testing} = props;
	const meta = providerMeta(channel.notification_type);
	const Icon = meta.icon;
	const failed = last?.status === 'FAILED';

	return (
		<ListRow
			icon={
				<RowTile>
					<Icon className="size-4" />
				</RowTile>
			}
			title={channel.name}
			subtitle={
				last
					? `${triggerSummary(channel)} · last ${last.status.toLowerCase()} ${timeAgo(last.at)}`
					: triggerSummary(channel)
			}
			badges={<ChannelChips channel={channel} failed={!!failed} />}>
			<ChannelMenu
				channel={channel}
				testing={testing}
				onTest={props.onTest}
				onEdit={props.onEdit}
				onDelete={props.onDelete}
			/>
		</ListRow>
	);
}

/** Grid counterpart of {@link NotificationChannelRow}. */
export function NotificationChannelCard(props: ChannelProps) {
	const {channel, last, testing} = props;
	const meta = providerMeta(channel.notification_type);
	const Icon = meta.icon;
	const failed = last?.status === 'FAILED';

	return (
		<ViewCard>
			<div className="flex min-w-0 items-start gap-3">
				<RowTile>
					<Icon className="size-4" />
				</RowTile>
				<div className="min-w-0 flex-1">
					<span className="block truncate text-sm font-medium text-foreground">
						{channel.name}
					</span>
					<p className="mt-0.5 truncate text-[11px] text-muted-foreground">
						{meta.label}
					</p>
				</div>
			</div>
			<div className="flex items-end justify-between gap-2">
				<div className="flex min-w-0 flex-wrap items-center gap-1.5">
					<ChannelChips channel={channel} failed={!!failed} />
				</div>
				<div className="shrink-0 opacity-0 transition-opacity group-focus-within:opacity-100 group-hover:opacity-100">
					<ChannelMenu
						channel={channel}
						testing={testing}
						onTest={props.onTest}
						onEdit={props.onEdit}
						onDelete={props.onDelete}
					/>
				</div>
			</div>
		</ViewCard>
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
				<Button onClick={onCreate} className="h-9 gap-1.5">
					<Plus className="size-3.5" />
					New channel
				</Button>
			}
		/>
	);
}
