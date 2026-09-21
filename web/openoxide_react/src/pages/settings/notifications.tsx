import {createFileRoute, Navigate} from '@tanstack/react-router';
import { useMemo, useState } from 'react';
import {
	Bell,
	GitBranch,
	Mail,
	MessageSquare,
	Pencil,
	Plus,
	Send,
	ShieldAlert,
	Trash2,
	Voicemail,
	Wrench,
	Zap,
} from 'lucide-react';
import { Button } from '#/components/ui/button';
import { Input } from '#/components/ui/input';
import { Label } from '#/components/ui/label';
import { Switch } from '#/components/ui/switch';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import {
	Dialog,
	DialogContent,
	DialogDescription,
	DialogFooter,
	DialogHeader,
	DialogTitle,
} from '#/components/ui/dialog';
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
	DropdownMenu,
	DropdownMenuContent,
	DropdownMenuItem,
	DropdownMenuTrigger,
} from '#/components/ui/dropdown';
import { MoreHorizontal, FlaskConical } from 'lucide-react';
import {
	useActiveOrgId,
	useNotificationCreate,
	useNotificationDelete,
	useNotificationListByOrganization,
	useNotificationTest,
	useNotificationUpdate,
	type NotificationPayload,
} from '#/api/live';
import { formatApiError } from '#/api/utils';
import { toast } from 'sonner';
import { ListCard, ListRow, ListEmpty, RowTile } from '#/components/settings/list-card';

export const Route = createFileRoute('/_app/settings/notifications')({
	component: () => (
		<Navigate to="/settings" search={{tab: 'notifications'}} replace />
	),
});

/**
 * Notification channels, real data now (was a static stub): OpenShip's
 * list-card rows — provider tile, name, trigger summary, kebab with
 * Test / Edit / Delete. Create is a dialog: provider + its config fields
 * + which events fire the channel.
 */

type ProviderId =
	| 'DISCORD'
	| 'SLACK'
	| 'TELEGRAM'
	| 'EMAIL'
	| 'NTFY'
	| 'GOTIFY'
	| 'MATTERMOST'
	| 'TEAMS'
	| 'CUSTOM';

const PROVIDERS: {
	id: ProviderId;
	label: string;
	icon: typeof Mail;
	tile: string;
	description: string;
	fields: {key: string; label: string; placeholder?: string; type?: string; optional?: boolean}[];
}[] = [
	{
		id: 'DISCORD',
		label: 'Discord',
		icon: MessageSquare,
		tile: 'bg-indigo-500/10 text-indigo-400',
		description: 'Post alerts to a channel webhook.',
		fields: [{key: 'webhook_url', label: 'Webhook URL', placeholder: 'https://discord.com/api/webhooks/…'}],
	},
	{
		id: 'SLACK',
		label: 'Slack',
		icon: GitBranch,
		tile: 'bg-emerald-500/10 text-emerald-400',
		description: 'Incoming webhook to any Slack channel.',
		fields: [
			{key: 'webhook_url', label: 'Webhook URL', placeholder: 'https://hooks.slack.com/services/…'},
			{key: 'channel', label: 'Channel (optional)', optional: true},
		],
	},
	{
		id: 'TELEGRAM',
		label: 'Telegram',
		icon: Send,
		tile: 'bg-sky-500/10 text-sky-400',
		description: 'Bot message to a chat.',
		fields: [
			{key: 'bot_token', label: 'Bot token', placeholder: '123456:ABC-DEF…'},
			{key: 'chat_id', label: 'Chat ID', placeholder: '-1001234567890'},
		],
	},
	{
		id: 'EMAIL',
		label: 'SMTP Email',
		icon: Mail,
		tile: 'bg-rose-500/10 text-rose-400',
		description: 'Send alerts over SMTP.',
		fields: [
			{key: 'smtp_server', label: 'SMTP server', placeholder: 'smtp.example.com'},
			{key: 'smtp_port', label: 'SMTP port', type: 'number', placeholder: '587'},
			{key: 'username', label: 'Username'},
			{key: 'password', label: 'Password', type: 'password'},
			{key: 'from_address', label: 'From address', placeholder: 'alerts@example.com'},
			{key: 'to_addresses', label: 'To addresses', placeholder: 'a@b.com,c@d.com'},
		],
	},
	{
		id: 'NTFY',
		label: 'Ntfy',
		icon: Voicemail,
		tile: 'bg-amber-500/10 text-amber-400',
		description: 'Pub-sub push notifications.',
		fields: [
			{key: 'server_url', label: 'Server URL', placeholder: 'https://ntfy.sh'},
			{key: 'topic', label: 'Topic'},
			{key: 'access_token', label: 'Access token (optional)', type: 'password', optional: true},
			{key: 'priority', label: 'Priority (1-5)', type: 'number', placeholder: '3'},
		],
	},
	{
		id: 'GOTIFY',
		label: 'Gotify',
		icon: Zap,
		tile: 'bg-violet-500/10 text-violet-400',
		description: 'Self-hosted message server.',
		fields: [
			{key: 'server_url', label: 'Server URL', placeholder: 'https://gotify.example.com'},
			{key: 'app_token', label: 'App token', type: 'password'},
			{key: 'priority', label: 'Priority', type: 'number', placeholder: '5'},
		],
	},
	{
		id: 'MATTERMOST',
		label: 'Mattermost',
		icon: MessageSquare,
		tile: 'bg-blue-500/10 text-blue-400',
		description: 'Team webhook.',
		fields: [{key: 'webhook_url', label: 'Webhook URL'}],
	},
	{
		id: 'TEAMS',
		label: 'Microsoft Teams',
		icon: GitBranch,
		tile: 'bg-cyan-500/10 text-cyan-400',
		description: 'Workflows / connector webhook.',
		fields: [{key: 'webhook_url', label: 'Webhook URL'}],
	},
	{
		id: 'CUSTOM',
		label: 'Custom webhook',
		icon: Wrench,
		tile: 'bg-muted text-muted-foreground',
		description: 'POST the alert JSON to any endpoint.',
		fields: [
			{key: 'endpoint', label: 'Endpoint URL'},
			{key: 'headers', label: 'Headers JSON (optional)', optional: true, placeholder: '{"Authorization":"Bearer …"}'},
		],
	},
];

const TRIGGERS: {key: keyof NotificationPayload; label: string}[] = [
	{key: 'on_app_deploy', label: 'App deployed'},
	{key: 'on_app_build_error', label: 'Build failed'},
	{key: 'on_database_backup', label: 'Database backup'},
	{key: 'on_volume_backup', label: 'Volume backup'},
	{key: 'on_panel_restart', label: 'Panel restart'},
	{key: 'on_docker_cleanup', label: 'Docker cleanup'},
	{key: 'on_server_threshold', label: 'Server threshold'},
	{key: 'on_panel_backup', label: 'Panel backup'},
	{key: 'on_schedule_success', label: 'Schedule success'},
	{key: 'on_schedule_failure', label: 'Schedule failure'},
];

function providerMeta(type: string) {
	return PROVIDERS.find(p => p.id === type) ?? {
		id: type as ProviderId,
		label: type,
		icon: Bell,
		tile: 'bg-muted text-muted-foreground',
		description: '',
		fields: [],
	};
}

function triggerSummary(n: NotificationPayload): string {
	const on = TRIGGERS.filter(t => n[t.key] === true).map(t => t.label);
	if (on.length === 0) return 'No events selected';
	if (on.length === TRIGGERS.length) return 'All events';
	return on.slice(0, 3).join(', ') + (on.length > 3 ? ` +${on.length - 3} more` : '');
}

type DraftState = {
	provider: ProviderId;
	name: string;
	config: Record<string, string>;
	triggers: Record<string, boolean>;
};

const emptyDraft = (): DraftState => ({
	provider: 'DISCORD',
	name: '',
	config: {},
	triggers: Object.fromEntries(TRIGGERS.map(t => [t.key, false])),
});

export function NotificationsPage() {
	const orgId = useActiveOrgId();
	const {data: channels = [], isLoading} = useNotificationListByOrganization(orgId);
	const createMutation = useNotificationCreate();
	const updateMutation = useNotificationUpdate();
	const deleteMutation = useNotificationDelete();
	const testMutation = useNotificationTest();

	const [draft, setDraft] = useState<DraftState | null>(null);
	const [editing, setEditing] = useState<NotificationPayload | null>(null);
	const [deleting, setDeleting] = useState<NotificationPayload | null>(null);

	const provider = useMemo(
		() => (draft ? providerMeta(draft.provider) : null),
		[draft?.provider],
	);

	const openCreate = () => {
		setEditing(null);
		setDraft(emptyDraft());
	};

	const openEdit = (n: NotificationPayload) => {
		setEditing(n);
		setDraft({
			provider: (n.notification_type as ProviderId) ?? 'CUSTOM',
			name: n.name,
			config: {},
			triggers: Object.fromEntries(TRIGGERS.map(t => [t.key, n[t.key] === true])),
		});
	};

	const save = async () => {
		if (!draft) return;
		if (!draft.name.trim()) {
			toast.error('Name is required');
			return;
		}
		try {
			if (editing) {
				// Config edits need the provider fields re-supplied; the patch
				// covers name + triggers, which is what the edit dialog exposes.
				await updateMutation.mutateAsync({
					id: editing.id,
					name: draft.name.trim(),
					...draft.triggers,
				});
				toast.success('Channel updated');
			} else {
				for (const f of provider?.fields ?? []) {
					if (!f.optional && !(draft.config[f.key] ?? '').trim()) {
						toast.error(`${f.label} is required`);
						return;
					}
				}
				const config: Record<string, unknown> = {...draft.config};
				for (const f of provider?.fields ?? []) {
					if (f.type === 'number' && config[f.key] !== undefined) {
						config[f.key] = Number(config[f.key]);
					}
					if (f.key === 'to_addresses' && typeof config[f.key] === 'string') {
						config[f.key] = (config[f.key] as string)
							.split(',')
							.map(s => s.trim())
							.filter(Boolean);
					}
				}
				await createMutation.mutateAsync({
					settings: {
						name: draft.name.trim(),
						organization_id: Number(orgId),
						...draft.triggers,
					},
					provider: {
						notification_type: draft.provider,
						config,
					},
				});
				toast.success('Channel created');
			}
			setDraft(null);
		} catch (err) {
			toast.error(formatApiError(err));
		}
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
							<div key={i} className="h-14 animate-pulse rounded-xl bg-muted/30" />
						))}
					</div>
				) : channels.length === 0 ? (
					<ListEmpty
						icon={Bell}
						title="No channels yet"
						description="Add a Discord, Slack, Telegram, or email channel to get deployment and server alerts."
						action={
							<Button size="sm" onClick={openCreate} className="gap-1.5">
								<Plus className="size-3.5" /> Add channel
							</Button>
						}
					/>
				) : (
					<div className="divide-y divide-border/40">
						{channels.map(n => {
							const meta = providerMeta(n.notification_type);
							const Icon = meta.icon;
							return (
								<ListRow
									key={n.id}
									icon={
										<RowTile className={meta.tile}>
											<Icon className="size-4" />
										</RowTile>
									}
									title={n.name}
									subtitle={triggerSummary(n)}
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
												onClick={() => void test(n)}
												className="gap-2 text-xs">
												<FlaskConical className="size-3.5" /> Send test
											</DropdownMenuItem>
											<DropdownMenuItem
												onClick={() => openEdit(n)}
												className="gap-2 text-xs">
												<Pencil className="size-3.5" /> Edit
											</DropdownMenuItem>
											<DropdownMenuItem
												onClick={() => setDeleting(n)}
												className="gap-2 text-xs text-destructive focus:text-destructive">
												<Trash2 className="size-3.5" /> Delete
											</DropdownMenuItem>
										</DropdownMenuContent>
									</DropdownMenu>
								</ListRow>
							);
						})}
					</div>
				)}
			</ListCard>

			{/* Create / edit dialog */}
			<Dialog open={draft !== null} onOpenChange={open => !open && setDraft(null)}>
				<DialogContent className="max-h-[85vh] overflow-y-auto sm:max-w-lg">
					<DialogHeader>
						<DialogTitle>
							{editing ? 'Edit channel' : 'Add notification channel'}
						</DialogTitle>
						<DialogDescription>
							{editing
								? 'Rename the channel or change which events it fires on.'
								: 'Pick a provider, fill its connection details, then choose events.'}
						</DialogDescription>
					</DialogHeader>

					{draft && (
						<div className="flex flex-col gap-4">
							<div className="flex flex-col gap-1.5">
								<Label className="text-xs font-semibold">Name</Label>
								<Input
									placeholder="e.g. Ops alerts"
									value={draft.name}
									onChange={e =>
										setDraft({...draft, name: e.target.value})
									}
									className="h-9 text-sm"
								/>
							</div>

							{!editing && (
								<div className="flex flex-col gap-1.5">
									<Label className="text-xs font-semibold">Provider</Label>
									<Select
										value={draft.provider}
										onValueChange={v =>
											setDraft({...draft, provider: (v as ProviderId) ?? 'DISCORD', config: {}})
										}>
										<SelectTrigger className="h-9 text-sm">
											<SelectValue />
										</SelectTrigger>
										<SelectContent>
											{PROVIDERS.map(p => (
												<SelectItem key={p.id} value={p.id} className="text-xs">
													{p.label}
												</SelectItem>
											))}
										</SelectContent>
									</Select>
									{provider && (
										<p className="text-xs text-muted-foreground">
											{provider.description}
										</p>
									)}
								</div>
							)}

							{!editing &&
								provider &&
								provider.fields.length > 0 && (
									<div className="flex flex-col gap-3 rounded-xl bg-muted/20 p-4">
										{provider.fields.map(f => (
											<label key={f.key} className="flex flex-col gap-1.5">
												<span className="text-xs font-medium text-foreground/80">
													{f.label}
												</span>
												<Input
													type={f.type ?? 'text'}
													placeholder={f.placeholder}
													value={draft.config[f.key] ?? ''}
													onChange={e =>
														setDraft({
															...draft,
															config: {...draft.config, [f.key]: e.target.value},
														})
													}
													className="h-9 bg-muted/40 text-sm"
												/>
											</label>
										))}
									</div>
								)}

							<div>
								<p className="mb-2 flex items-center gap-1.5 text-xs font-semibold text-foreground">
									<ShieldAlert className="size-3.5 text-muted-foreground" />
									Fire on
								</p>
								<div className="divide-y divide-border/40 rounded-xl bg-muted/20">
									{TRIGGERS.map(t => (
										<div
											key={t.key}
											className="flex items-center justify-between px-4 py-2.5">
											<span className="text-sm text-foreground">{t.label}</span>
											<Switch
												checked={draft.triggers[t.key] === true}
												onCheckedChange={v =>
													setDraft({
														...draft,
														triggers: {...draft.triggers, [t.key]: v},
													})
												}
											/>
										</div>
									))}
								</div>
							</div>

							<DialogFooter>
								<Button type="button" variant="ghost" onClick={() => setDraft(null)}>
									Cancel
								</Button>
								<Button type="button" onClick={() => void save()}>
									{editing ? 'Save changes' : 'Create channel'}
								</Button>
							</DialogFooter>
						</div>
					)}
				</DialogContent>
			</Dialog>

			{/* Delete confirmation */}
			<AlertDialog open={deleting !== null} onOpenChange={open => !open && setDeleting(null)}>
				<AlertDialogContent>
					<AlertDialogHeader>
						<AlertDialogTitle>Delete channel</AlertDialogTitle>
						<AlertDialogDescription>
							Alerts will stop reaching{' '}
							<span className="font-medium text-foreground">{deleting?.name}</span>.
							This cannot be undone.
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
