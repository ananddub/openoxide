import {useState} from 'react';
import {useNavigate} from '@tanstack/react-router';
import {
	AlertTriangle,
	Copy,
	FileCode2,
	HardDrive,
	KeyRound,
	RefreshCw,
	Save,
	Trash2,
	Zap,
} from 'lucide-react';
import {toast} from 'sonner';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {Label} from '#/components/ui/label';
import {Textarea} from '#/components/ui/textarea';
import {Badge} from '#/components/ui/badge';
import {Switch} from '#/components/ui/switch';
import {
	Dialog,
	DialogContent,
	DialogDescription,
	DialogFooter,
	DialogHeader,
	DialogTitle,
} from '#/components/ui/dialog';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import {
	useComposeClearHistory,
	useComposeCleanupQueue,
	useComposeDelete,
	useComposeForceKill,
	useComposeMountDelete,
	useComposeMountList,
	useComposeMountUpsert,
	useComposePatchDelete,
	useComposePatchList,
	useComposePatchUpsert,
	useComposeRotateWebhook,
	useComposeUpdate,
	type ComposePatchPayload,
} from '#/api/live';
import {formatApiError} from '#/api/utils';
import {
	AddResourceButton,
	AdvancedSection,
	DangerAction,
	EmptyRow,
	ResourceRow,
} from '#/components/projects/detail/advanced/shared';
import {
	MountDialog,
	type MountValue,
} from '#/components/projects/detail/advanced/mount-dialog';

interface ComposeAdvancedTabProps {
	compose: any;
	projectId?: number;
	onUpdated?: () => void;
}

export function ComposeAdvancedTab({
	compose,
	projectId,
	onUpdated,
}: ComposeAdvancedTabProps) {
	if (!compose?.id) return null;
	return (
		<div className="flex flex-col gap-6 pb-6">
			<CommandSection compose={compose} onUpdated={onUpdated} />
			<VolumesSection compose={compose} />
			<PatchesSection compose={compose} />
			<WebhookSection compose={compose} />
			<DangerSection compose={compose} projectId={projectId} />
		</div>
	);
}

// ── Deployment command ─────────────────────────────────────────────────

function CommandSection({
	compose,
	onUpdated,
}: ComposeAdvancedTabProps) {
	const update = useComposeUpdate();
	const [command, setCommand] = useState(compose.command || '');
	const [saving, setSaving] = useState(false);

	const save = async () => {
		setSaving(true);
		try {
			await update.mutateAsync({id: compose.id, command: command || ''});
			toast.success('Deployment command saved');
			onUpdated?.();
		} catch (err) {
			toast.error(formatApiError(err));
		} finally {
			setSaving(false);
		}
	};

	return (
		<AdvancedSection
			icon={<Zap className="size-4 text-primary" />}
			title="Deployment Command"
			description="Shell command executed instead of the default `docker compose up`. Leave empty for the default flow.">
			<div className="flex flex-col gap-1.5">
				<Label className="text-xs font-bold text-muted-foreground">
					Command
				</Label>
				<Input
					placeholder="docker compose -f docker-compose.yml up -d"
					value={command}
					onChange={e => setCommand(e.target.value)}
					className="font-mono text-xs"
				/>
			</div>
			<div className="flex justify-end">
				<Button
					onClick={save}
					disabled={saving}
					className="flex h-9 items-center gap-1.5 rounded-lg bg-primary text-xs font-semibold text-primary-foreground hover:bg-primary/95">
					<Save className="h-3.5 w-3.5" /> {saving ? 'Saving…' : 'Save'}
				</Button>
			</div>
		</AdvancedSection>
	);
}

// ── Volumes / Mounts ───────────────────────────────────────────────────

function VolumesSection({compose}: {compose: any}) {
	const {data: mounts = []} = useComposeMountList(compose.id);
	const upsert = useComposeMountUpsert();
	const remove = useComposeMountDelete();
	const [open, setOpen] = useState(false);
	const [editing, setEditing] = useState<MountValue | null>(null);
	const [saving, setSaving] = useState(false);
	const [deletingId, setDeletingId] = useState<number | null>(null);

	const save = async (value: Omit<MountValue, 'id'>) => {
		setSaving(true);
		try {
			await upsert.mutateAsync({
				compose_id: compose.id,
				id: editing?.id,
				...value,
			});
			toast.success(editing ? 'Volume updated' : 'Volume added');
			setOpen(false);
			setEditing(null);
		} catch (err) {
			toast.error(formatApiError(err));
		} finally {
			setSaving(false);
		}
	};

	const del = async (id: number) => {
		setDeletingId(id);
		try {
			await remove.mutateAsync({compose_id: compose.id, id});
			toast.success('Volume removed');
		} catch (err) {
			toast.error(formatApiError(err));
		} finally {
			setDeletingId(null);
		}
	};

	return (
		<AdvancedSection
			icon={<HardDrive className="size-4 text-primary" />}
			title="Volumes / Mounts"
			description="Persistent storage and injected files applied to every service in the stack."
			action={
				<AddResourceButton
					onClick={() => {
						setEditing(null);
						setOpen(true);
					}}
					label="Add Volume"
				/>
			}>
			<div className="flex flex-col gap-2">
				{mounts.length === 0 ? (
					<EmptyRow label="No volumes or mounts." />
				) : (
					mounts.map(mount => (
						<ResourceRow
							key={mount.id}
							title={mount.mount_path}
							subtitle={
								mount.volume_name ||
								mount.host_path ||
								mount.file_path ||
								''
							}
							badges={
								<Badge
									variant="secondary"
									className="text-[10px] font-semibold">
									{mount.mount_type}
								</Badge>
							}
							onEdit={() => {
								setEditing({...mount});
								setOpen(true);
							}}
							onDelete={() => del(mount.id)}
							deleting={deletingId === mount.id}
						/>
					))
				)}
			</div>
			<MountDialog
				open={open}
				mount={editing}
				saving={saving}
				onOpenChange={setOpen}
				onSave={save}
			/>
		</AdvancedSection>
	);
}

// ── Patches (env/secret/config overlays) ───────────────────────────────

const patchTypes = [
	{value: 'UPDATE', label: 'Update (merge YAML)'},
	{value: 'DELETE', label: 'Delete (remove keys)'},
];

function PatchesSection({compose}: {compose: any}) {
	const {data: patches = []} = useComposePatchList(compose.id);
	const upsert = useComposePatchUpsert();
	const remove = useComposePatchDelete();
	const [open, setOpen] = useState(false);
	const [editing, setEditing] = useState<ComposePatchPayload | null>(null);
	const [saving, setSaving] = useState(false);
	const [deletingId, setDeletingId] = useState<number | null>(null);
	const [type, setType] = useState('UPDATE');
	const [filePath, setFilePath] = useState('');
	const [enabled, setEnabled] = useState(true);
	const [content, setContent] = useState('');

	const show = (patch: ComposePatchPayload | null) => {
		setEditing(patch);
		setType(patch?.patch_type || 'UPDATE');
		setFilePath(patch?.file_path || '');
		setEnabled(patch ? patch.enabled : true);
		setContent(patch?.content || '');
		setOpen(true);
	};

	const save = async () => {
		setSaving(true);
		try {
			await upsert.mutateAsync({
				compose_id: compose.id,
				id: editing?.id,
				patch_type: type,
				payload: JSON.stringify({
					file_path: filePath,
					enabled,
					content,
				}),
			});
			toast.success(editing ? 'Patch updated' : 'Patch added');
			setOpen(false);
		} catch (err) {
			toast.error(formatApiError(err));
		} finally {
			setSaving(false);
		}
	};

	const del = async (id: number) => {
		setDeletingId(id);
		try {
			await remove.mutateAsync({compose_id: compose.id, id});
			toast.success('Patch removed');
		} catch (err) {
			toast.error(formatApiError(err));
		} finally {
			setDeletingId(null);
		}
	};

	return (
		<AdvancedSection
			icon={<FileCode2 className="size-4 text-primary" />}
			title="Compose Patches"
			description="Overlay files merged into the compose definition at deploy time."
			action={<AddResourceButton onClick={() => show(null)} label="Add Patch" />}>
			<div className="flex flex-col gap-2">
				{patches.length === 0 ? (
					<EmptyRow label="No patches." />
				) : (
					patches.map(patch => (
						<ResourceRow
							key={patch.id}
							title={patch.file_path}
							subtitle={patch.content?.slice(0, 80)}
							badges={
								<Badge
									variant="secondary"
									className="text-[10px] font-semibold">
									{patch.patch_type}
								</Badge>
							}
							onEdit={() => show(patch)}
							onDelete={() => del(patch.id)}
							deleting={deletingId === patch.id}
						/>
					))
				)}
			</div>
			<Dialog open={open} onOpenChange={setOpen}>
				<DialogContent className="max-h-[90vh] overflow-y-auto sm:max-w-2xl">
					<DialogHeader>
						<DialogTitle>Compose Patch</DialogTitle>
						<DialogDescription>
							Target one service file in the compose project with a
							patch overlay.
						</DialogDescription>
					</DialogHeader>
					<div className="grid grid-cols-2 gap-3">
						<div className="flex flex-col gap-1.5">
							<Label className="text-xs font-bold text-muted-foreground">
								Patch Type
							</Label>
							<Select value={type} onValueChange={v => v && setType(v)}>
								<SelectTrigger className="h-9 text-xs">
									<SelectValue />
								</SelectTrigger>
								<SelectContent>
									{patchTypes.map(option => (
										<SelectItem
											key={option.value}
											value={option.value}>
											{option.label}
										</SelectItem>
									))}
								</SelectContent>
							</Select>
						</div>
						<div className="flex flex-col gap-1.5">
							<Label className="text-xs font-bold text-muted-foreground">
								Service File
							</Label>
							<Input
								placeholder="docker-compose.yml"
								value={filePath}
								onChange={e => setFilePath(e.target.value)}
								className="font-mono text-xs"
							/>
						</div>
					</div>
					<div className="flex items-center justify-between rounded-lg border border-border/40 px-3 py-2.5">
						<Label className="text-xs font-bold text-muted-foreground">
							Enabled
						</Label>
						<Switch checked={enabled} onCheckedChange={setEnabled} />
					</div>
					<div className="flex flex-col gap-1.5">
						<Label className="text-xs font-bold text-muted-foreground">
							Content (YAML)
						</Label>
						<Textarea
							rows={8}
							placeholder={'environment:\n  - FOO=bar'}
							value={content}
							onChange={e => setContent(e.target.value)}
							className="font-mono text-xs"
						/>
					</div>
					<DialogFooter>
						<Button
							variant="outline"
							onClick={() => setOpen(false)}
							className="text-xs">
							Cancel
						</Button>
						<Button
							disabled={saving || !filePath.trim() || !content.trim()}
							onClick={save}
							className="text-xs">
							{saving ? 'Saving…' : editing ? 'Update' : 'Add'}
						</Button>
					</DialogFooter>
				</DialogContent>
			</Dialog>
		</AdvancedSection>
	);
}

// ── Webhook ────────────────────────────────────────────────────────────

function WebhookSection({compose}: {compose: any}) {
	const rotate = useComposeRotateWebhook();
	const [token, setToken] = useState<string>('');
	const [busy, setBusy] = useState(false);

	const rotateToken = async () => {
		setBusy(true);
		try {
			const next = await rotate.mutateAsync(compose.id);
			setToken(next);
			toast.success('Webhook token rotated');
		} catch (err) {
			toast.error(formatApiError(err));
		} finally {
			setBusy(false);
		}
	};

	const copy = async () => {
		if (!token) return;
		await navigator.clipboard.writeText(token);
		toast.success('Token copied');
	};

	return (
		<AdvancedSection
			icon={<KeyRound className="size-4 text-primary" />}
			title="Deploy Webhook"
			description="Git providers call this token to trigger auto-deploys. Rotating it invalidates the previous secret.">
			{token ? (
				<div className="flex items-center gap-2">
					<code className="min-w-0 flex-1 truncate rounded-lg bg-muted/40 px-3 py-2 font-mono text-xs text-foreground">
						{token}
					</code>
					<Button
						variant="outline"
						size="icon"
						title="Copy token"
						onClick={copy}
						className="size-8 rounded-lg">
						<Copy className="size-3.5" />
					</Button>
					<Button
						variant="outline"
						size="icon"
						title="Rotate again"
						disabled={busy}
						onClick={rotateToken}
						className="size-8 rounded-lg">
						<RefreshCw className="size-3.5" />
					</Button>
				</div>
			) : (
				<Button
					variant="outline"
					disabled={busy}
					onClick={rotateToken}
					className="flex h-9 w-fit items-center gap-1.5 rounded-lg text-xs font-semibold">
					<RefreshCw className="size-3.5" />
					{busy ? 'Rotating…' : 'Generate / Rotate Token'}
				</Button>
			)}
		</AdvancedSection>
	);
}

// ── Danger zone ────────────────────────────────────────────────────────

function DangerSection({
	compose,
	projectId,
}: {
	compose: any;
	projectId?: number;
}) {
	const navigate = useNavigate();
	const deleteCompose = useComposeDelete();
	const clearHistory = useComposeClearHistory();
	const cleanupQueue = useComposeCleanupQueue();
	const forceKill = useComposeForceKill();
	const [confirmDelete, setConfirmDelete] = useState(false);
	const [busy, setBusy] = useState<string | null>(null);
	const [deleting, setDeleting] = useState(false);

	const run = async (
		key: string,
		fn: () => Promise<unknown>,
		message: string,
	) => {
		setBusy(key);
		try {
			await fn();
			toast.success(message);
		} catch (err) {
			toast.error(formatApiError(err));
		} finally {
			setBusy(null);
		}
	};

	const handleDelete = async () => {
		setDeleting(true);
		try {
			await deleteCompose.mutateAsync(compose.id);
			toast.success('Compose stack deleted');
			navigate({
				to: `/projects/${projectId ?? compose.project_id ?? ''}`,
			});
		} catch (err) {
			toast.error(formatApiError(err));
			setDeleting(false);
		}
	};

	return (
		<section className="flex flex-col gap-4 rounded-2xl bg-card p-6 shadow-sm">
			<div className="flex items-center gap-2 text-destructive">
				<AlertTriangle className="h-4 w-4" />
				<h3 className="text-sm font-bold">Danger Zone</h3>
			</div>
			<div className="flex flex-wrap gap-2">
				<DangerAction
					label="Clear Deployment History"
					busy={busy === 'history'}
					onConfirm={() =>
						run(
							'history',
							() => clearHistory.mutateAsync(compose.id),
							'Deployment history cleared',
						)
					}
				/>
				<DangerAction
					label="Cleanup Deployment Queue"
					busy={busy === 'queue'}
					onConfirm={() =>
						run(
							'queue',
							() => cleanupQueue.mutateAsync(compose.id),
							'Deployment queue cleaned',
						)
					}
				/>
				<DangerAction
					label="Force Kill Running Deployment"
					busy={busy === 'kill'}
					onConfirm={() =>
						run(
							'kill',
							() => forceKill.mutateAsync(compose.id),
							'Deployment killed',
						)
					}
				/>
			</div>
			<p className="text-xs text-muted-foreground">
				Permanently remove this compose stack and all associated
				containers, networks, and volumes.
			</p>
			{confirmDelete ? (
				<div className="flex flex-col gap-3 rounded-lg bg-destructive/5 p-4">
					<p className="text-xs font-semibold text-foreground">
						Are you sure? This action is absolutely irreversible.
					</p>
					<div className="flex items-center gap-2">
						<Button
							onClick={handleDelete}
							disabled={deleting}
							className="h-9 bg-destructive px-6 text-xs font-semibold text-destructive-foreground shadow-md hover:bg-destructive/90">
							{deleting ? 'Deleting…' : 'Yes, Delete stack'}
						</Button>
						<Button
							variant="outline"
							onClick={() => setConfirmDelete(false)}
							className="h-9 text-xs">
							Cancel
						</Button>
					</div>
				</div>
			) : (
				<div>
					<Button
						variant="outline"
						onClick={() => setConfirmDelete(true)}
						className="flex h-9 items-center gap-1.5 border-destructive/40 text-xs font-semibold text-destructive/80 hover:bg-destructive/5 hover:text-destructive">
						<Trash2 className="h-4 w-4" /> Delete Compose Stack
					</Button>
				</div>
			)}
		</section>
	);
}
