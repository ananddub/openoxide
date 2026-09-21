import {useState, useEffect} from 'react';
import {useNavigate} from '@tanstack/react-router';
import {
	AlertTriangle,
	Copy,
	Globe,
	HardDrive,
	KeyRound,
	Link2,
	RefreshCw,
	Save,
	Settings2,
	Shield,
	Trash2,
	Zap,
} from 'lucide-react';
import {toast} from 'sonner';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {Label} from '#/components/ui/label';
import {Switch} from '#/components/ui/switch';
import {Badge} from '#/components/ui/badge';
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
	useApplicationClearHistory,
	useApplicationCleanupQueue,
	useApplicationDelete,
	useApplicationForceKill,
	useApplicationMountDelete,
	useApplicationMountList,
	useApplicationMountUpsert,
	useApplicationPortCreate,
	useApplicationPortDelete,
	useApplicationPortList,
	useApplicationRedirectDelete,
	useApplicationRedirectList,
	useApplicationRedirectUpsert,
	useApplicationRotateWebhook,
	useApplicationSecurityDelete,
	useApplicationSecurityList,
	useApplicationSecurityUpsert,
	useApplicationUpdateBuildConfig,
	useApplicationUpdateResourceConfig,
	type ApplicationPortPayload,
	type ApplicationRedirectPayload,
	type ApplicationSecurityPayload,
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

interface AdvancedTabProps {
	app: any;
	onUpdated: () => void;
}

export function AdvancedTab({app, onUpdated}: AdvancedTabProps) {
	return (
		<div className="flex flex-col gap-6 pb-6">
			<GeneralSection app={app} onUpdated={onUpdated} />
			<PortsSection app={app} />
			<VolumesSection app={app} />
			<RedirectsSection app={app} />
			<SecuritySection app={app} />
			<WebhookSection app={app} />
			<ResourceSection app={app} onUpdated={onUpdated} />
			<DangerSection app={app} />
		</div>
	);
}

// ── General: command / args / watch paths ──────────────────────────────

function GeneralSection({app, onUpdated}: AdvancedTabProps) {
	const update = useApplicationUpdateBuildConfig();
	const [command, setCommand] = useState(app.command || '');
	const [args, setArgs] = useState(app.args || '');
	const [watchPaths, setWatchPaths] = useState(app.watch_paths || '');
	const [submodules, setSubmodules] = useState(
		Boolean(app.enable_submodules),
	);
	const [saving, setSaving] = useState(false);

	useEffect(() => {
		setCommand(app.command || '');
		setArgs(app.args || '');
		setWatchPaths(app.watch_paths || '');
		setSubmodules(Boolean(app.enable_submodules));
	}, [app.command, app.args, app.watch_paths, app.enable_submodules]);

	const save = async () => {
		setSaving(true);
		try {
			await update.mutateAsync({
				id: app.id,
				command,
				args,
				watch_paths: watchPaths,
				enable_submodules: submodules,
			});
			toast.success('General settings saved');
			onUpdated();
		} catch (err) {
			toast.error(formatApiError(err));
		} finally {
			setSaving(false);
		}
	};

	return (
		<AdvancedSection
			icon={<Settings2 className="size-4 text-primary" />}
			title="General"
			description="Container start command, arguments, and git watch behaviour.">
			<div className="grid gap-4 sm:grid-cols-2">
				<div className="flex flex-col gap-1.5">
					<Label className="text-xs font-bold text-muted-foreground">
						Start Command
					</Label>
					<Input
						placeholder="node server.js"
						value={command}
						onChange={e => setCommand(e.target.value)}
						className="font-mono text-xs"
					/>
				</div>
				<div className="flex flex-col gap-1.5">
					<Label className="text-xs font-bold text-muted-foreground">
						Command Arguments
					</Label>
					<Input
						placeholder="--port 3000"
						value={args}
						onChange={e => setArgs(e.target.value)}
						className="font-mono text-xs"
					/>
				</div>
				<div className="flex flex-col gap-1.5">
					<Label className="text-xs font-bold text-muted-foreground">
						Watch Paths
					</Label>
					<Input
						placeholder="src, package.json"
						value={watchPaths}
						onChange={e => setWatchPaths(e.target.value)}
						className="font-mono text-xs"
					/>
					<p className="text-[11px] text-muted-foreground">
						Comma-separated paths that trigger a rebuild on push.
						Empty watches the whole repository.
					</p>
				</div>
				<div className="flex items-center justify-between rounded-lg border border-border/40 px-3 py-2.5">
					<div>
						<Label className="text-xs font-bold text-muted-foreground">
							Git Submodules
						</Label>
						<p className="text-[11px] text-muted-foreground">
							Clone submodules during build
						</p>
					</div>
					<Switch checked={submodules} onCheckedChange={setSubmodules} />
				</div>
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

// ── Ports ─────────────────────────────────────────────────────────────

function PortsSection({app}: {app: any}) {
	const {data: ports = []} = useApplicationPortList(app.id);
	const create = useApplicationPortCreate();
	const remove = useApplicationPortDelete();
	const [open, setOpen] = useState(false);
	const [saving, setSaving] = useState(false);
	const [deletingId, setDeletingId] = useState<number | null>(null);
	const [published, setPublished] = useState('');
	const [target, setTarget] = useState('');
	const [protocol, setProtocol] = useState('TCP');
	const [mode, setMode] = useState('HOST');

	const reset = () => {
		setPublished('');
		setTarget('');
		setProtocol('TCP');
		setMode('HOST');
	};

	const save = async () => {
		setSaving(true);
		try {
			await create.mutateAsync({
				application_id: app.id,
				published_port: Number(published),
				target_port: Number(target),
				protocol,
				publish_mode: mode,
			});
			toast.success('Port added');
			setOpen(false);
			reset();
		} catch (err) {
			toast.error(formatApiError(err));
		} finally {
			setSaving(false);
		}
	};

	const del = async (port: ApplicationPortPayload) => {
		setDeletingId(port.id);
		try {
			await remove.mutateAsync({application_id: app.id, id: port.id});
			toast.success('Port removed');
		} catch (err) {
			toast.error(formatApiError(err));
		} finally {
			setDeletingId(null);
		}
	};

	return (
		<AdvancedSection
			icon={<Globe className="size-4 text-primary" />}
			title="Ports"
			description="Publish container ports on the host or through the swarm ingress mesh."
			action={
				<AddResourceButton
					onClick={() => setOpen(true)}
					label="Add Port"
				/>
			}>
			<div className="flex flex-col gap-2">
				{ports.length === 0 ? (
					<EmptyRow label="No published ports." />
				) : (
					ports.map(port => (
						<ResourceRow
							key={port.id}
							title={`${port.published_port} → ${port.target_port}/${port.protocol.toLowerCase()}`}
							subtitle={
								port.publish_mode === 'INGRESS'
									? 'Swarm ingress mesh'
									: 'Host binding'
							}
							badges={
								<Badge
									variant="secondary"
									className="text-[10px] font-semibold">
									{port.publish_mode}
								</Badge>
							}
							onDelete={() => del(port)}
							deleting={deletingId === port.id}
						/>
					))
				)}
			</div>
			<Dialog open={open} onOpenChange={setOpen}>
				<DialogContent className="sm:max-w-md">
					<DialogHeader>
						<DialogTitle>Add Port</DialogTitle>
						<DialogDescription>
							Map a host or ingress port to a container port.
						</DialogDescription>
					</DialogHeader>
					<div className="grid grid-cols-2 gap-3">
						<div className="flex flex-col gap-1.5">
							<Label className="text-xs font-bold text-muted-foreground">
								Published Port
							</Label>
							<Input
								type="number"
								placeholder="8080"
								value={published}
								onChange={e => setPublished(e.target.value)}
								className="text-xs"
							/>
						</div>
						<div className="flex flex-col gap-1.5">
							<Label className="text-xs font-bold text-muted-foreground">
								Target Port
							</Label>
							<Input
								type="number"
								placeholder="3000"
								value={target}
								onChange={e => setTarget(e.target.value)}
								className="text-xs"
							/>
						</div>
						<div className="flex flex-col gap-1.5">
							<Label className="text-xs font-bold text-muted-foreground">
								Protocol
							</Label>
							<Select
								value={protocol}
								onValueChange={v => v && setProtocol(v)}>
								<SelectTrigger className="h-9 text-xs">
									<SelectValue />
								</SelectTrigger>
								<SelectContent>
									<SelectItem value="TCP">TCP</SelectItem>
									<SelectItem value="UDP">UDP</SelectItem>
								</SelectContent>
							</Select>
						</div>
						<div className="flex flex-col gap-1.5">
							<Label className="text-xs font-bold text-muted-foreground">
								Publish Mode
							</Label>
							<Select value={mode} onValueChange={v => v && setMode(v)}>
								<SelectTrigger className="h-9 text-xs">
									<SelectValue />
								</SelectTrigger>
								<SelectContent>
									<SelectItem value="HOST">Host</SelectItem>
									<SelectItem value="INGRESS">Ingress</SelectItem>
								</SelectContent>
							</Select>
						</div>
					</div>
					<DialogFooter>
						<Button
							variant="outline"
							onClick={() => setOpen(false)}
							className="text-xs">
							Cancel
						</Button>
						<Button
							disabled={saving || !published || !target}
							onClick={save}
							className="text-xs">
							{saving ? 'Saving…' : 'Add'}
						</Button>
					</DialogFooter>
				</DialogContent>
			</Dialog>
		</AdvancedSection>
	);
}

// ── Volumes / Mounts ───────────────────────────────────────────────────

function VolumesSection({app}: {app: any}) {
	const {data: mounts = []} = useApplicationMountList(app.id);
	const upsert = useApplicationMountUpsert();
	const remove = useApplicationMountDelete();
	const [open, setOpen] = useState(false);
	const [editing, setEditing] = useState<MountValue | null>(null);
	const [saving, setSaving] = useState(false);
	const [deletingId, setDeletingId] = useState<number | null>(null);

	const save = async (value: Omit<MountValue, 'id'>) => {
		setSaving(true);
		try {
			await upsert.mutateAsync({
				application_id: app.id,
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
			await remove.mutateAsync({application_id: app.id, id});
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
			description="Persistent storage and injected files for this container."
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

// ── Redirects ──────────────────────────────────────────────────────────

function RedirectsSection({app}: {app: any}) {
	const {data: redirects = []} = useApplicationRedirectList(app.id);
	const upsert = useApplicationRedirectUpsert();
	const remove = useApplicationRedirectDelete();
	const [open, setOpen] = useState(false);
	const [editing, setEditing] =
		useState<ApplicationRedirectPayload | null>(null);
	const [saving, setSaving] = useState(false);
	const [deletingId, setDeletingId] = useState<number | null>(null);
	const [regex, setRegex] = useState('');
	const [replacement, setReplacement] = useState('');
	const [permanent, setPermanent] = useState(true);

	const show = (redirect: ApplicationRedirectPayload | null) => {
		setEditing(redirect);
		setRegex(redirect?.regex || '');
		setReplacement(redirect?.replacement || '');
		setPermanent((redirect?.permanent ?? 1) !== 0);
		setOpen(true);
	};

	const save = async () => {
		setSaving(true);
		try {
			await upsert.mutateAsync({
				application_id: app.id,
				id: editing?.id,
				regex,
				replacement,
				permanent,
			});
			toast.success(editing ? 'Redirect updated' : 'Redirect added');
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
			await remove.mutateAsync({application_id: app.id, id});
			toast.success('Redirect removed');
		} catch (err) {
			toast.error(formatApiError(err));
		} finally {
			setDeletingId(null);
		}
	};

	return (
		<AdvancedSection
			icon={<Link2 className="size-4 text-primary" />}
			title="Traefik Redirects"
			description="Regex-based request rewrites handled at the edge router."
			action={<AddResourceButton onClick={() => show(null)} label="Add Redirect" />}>
			<div className="flex flex-col gap-2">
				{redirects.length === 0 ? (
					<EmptyRow label="No redirects." />
				) : (
					redirects.map(redirect => (
						<ResourceRow
							key={redirect.id}
							title={redirect.regex}
							subtitle={`→ ${redirect.replacement}`}
							badges={
								<Badge
									variant="secondary"
									className="text-[10px] font-semibold">
									{redirect.permanent ? '301' : '302'}
								</Badge>
							}
							onEdit={() => show(redirect)}
							onDelete={() => del(redirect.id)}
							deleting={deletingId === redirect.id}
						/>
					))
				)}
			</div>
			<Dialog open={open} onOpenChange={setOpen}>
				<DialogContent className="sm:max-w-md">
					<DialogHeader>
						<DialogTitle>Redirect</DialogTitle>
						<DialogDescription>
							Match a request pattern and rewrite it to a target.
						</DialogDescription>
					</DialogHeader>
					<div className="flex flex-col gap-3">
						<div className="flex flex-col gap-1.5">
							<Label className="text-xs font-bold text-muted-foreground">
								Regex
							</Label>
							<Input
								placeholder="https://old.example.com/(.*)"
								value={regex}
								onChange={e => setRegex(e.target.value)}
								className="font-mono text-xs"
							/>
						</div>
						<div className="flex flex-col gap-1.5">
							<Label className="text-xs font-bold text-muted-foreground">
								Replacement
							</Label>
							<Input
								placeholder="https://example.com/$1"
								value={replacement}
								onChange={e => setReplacement(e.target.value)}
								className="font-mono text-xs"
							/>
						</div>
						<div className="flex items-center justify-between rounded-lg border border-border/40 px-3 py-2.5">
							<Label className="text-xs font-bold text-muted-foreground">
								Permanent (301)
							</Label>
							<Switch checked={permanent} onCheckedChange={setPermanent} />
						</div>
					</div>
					<DialogFooter>
						<Button
							variant="outline"
							onClick={() => setOpen(false)}
							className="text-xs">
							Cancel
						</Button>
						<Button
							disabled={saving || !regex.trim() || !replacement.trim()}
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

// ── Basic Auth ─────────────────────────────────────────────────────────

function SecuritySection({app}: {app: any}) {
	const {data: securities = []} = useApplicationSecurityList(app.id);
	const upsert = useApplicationSecurityUpsert();
	const remove = useApplicationSecurityDelete();
	const [open, setOpen] = useState(false);
	const [editing, setEditing] = useState<ApplicationSecurityPayload | null>(
		null,
	);
	const [saving, setSaving] = useState(false);
	const [deletingId, setDeletingId] = useState<number | null>(null);
	const [username, setUsername] = useState('');
	const [password, setPassword] = useState('');

	const show = (security: ApplicationSecurityPayload | null) => {
		setEditing(security);
		setUsername(security?.username || '');
		setPassword('');
		setOpen(true);
	};

	const save = async () => {
		setSaving(true);
		try {
			await upsert.mutateAsync({
				application_id: app.id,
				id: editing?.id,
				username,
				password,
			});
			toast.success(editing ? 'Credentials updated' : 'Basic auth enabled');
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
			await remove.mutateAsync({application_id: app.id, id});
			toast.success('Basic auth removed');
		} catch (err) {
			toast.error(formatApiError(err));
		} finally {
			setDeletingId(null);
		}
	};

	return (
		<AdvancedSection
			icon={<Shield className="size-4 text-primary" />}
			title="Basic Auth"
			description="Gate the whole service behind HTTP basic authentication."
			action={
				<AddResourceButton
					onClick={() => show(null)}
					label={securities.length ? 'Add Credential' : 'Enable Basic Auth'}
				/>
			}>
			<div className="flex flex-col gap-2">
				{securities.length === 0 ? (
					<EmptyRow label="Basic auth is disabled." />
				) : (
					securities.map(security => (
						<ResourceRow
							key={security.id}
							title={security.username}
							subtitle={
								security.has_password
									? 'Password set'
									: 'No password'
							}
							onEdit={() => show(security)}
							onDelete={() => del(security.id)}
							deleting={deletingId === security.id}
						/>
					))
				)}
			</div>
			<Dialog open={open} onOpenChange={setOpen}>
				<DialogContent className="sm:max-w-md">
					<DialogHeader>
						<DialogTitle>Basic Auth Credential</DialogTitle>
						<DialogDescription>
							Visitors must authenticate with these credentials.
						</DialogDescription>
					</DialogHeader>
					<div className="flex flex-col gap-3">
						<div className="flex flex-col gap-1.5">
							<Label className="text-xs font-bold text-muted-foreground">
								Username
							</Label>
							<Input
								placeholder="admin"
								value={username}
								onChange={e => setUsername(e.target.value)}
								className="text-xs"
							/>
						</div>
						<div className="flex flex-col gap-1.5">
							<Label className="text-xs font-bold text-muted-foreground">
								Password
							</Label>
							<Input
								type="password"
								placeholder={editing ? '••••••••' : 'password'}
								value={password}
								onChange={e => setPassword(e.target.value)}
								className="text-xs"
							/>
							{editing && (
								<p className="text-[11px] text-muted-foreground">
									Leave unchanged to keep the current password.
								</p>
							)}
						</div>
					</div>
					<DialogFooter>
						<Button
							variant="outline"
							onClick={() => setOpen(false)}
							className="text-xs">
							Cancel
						</Button>
						<Button
							disabled={saving || !username.trim() || !password.trim()}
							onClick={save}
							className="text-xs">
							{saving ? 'Saving…' : editing ? 'Update' : 'Enable'}
						</Button>
					</DialogFooter>
				</DialogContent>
			</Dialog>
		</AdvancedSection>
	);
}

// ── Webhook ────────────────────────────────────────────────────────────

function WebhookSection({app}: {app: any}) {
	const rotate = useApplicationRotateWebhook();
	const [token, setToken] = useState<string>('');
	const [busy, setBusy] = useState(false);

	const rotateToken = async () => {
		setBusy(true);
		try {
			const next = await rotate.mutateAsync(app.id);
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

// ── Resource limits ────────────────────────────────────────────────────

function ResourceSection({app, onUpdated}: AdvancedTabProps) {
	const [memRes, setMemRes] = useState(app.memory_reservation || '');
	const [memLimit, setMemLimit] = useState(app.memory_limit || '');
	const [cpuRes, setCpuRes] = useState(app.cpu_reservation || '');
	const [cpuLimit, setCpuLimit] = useState(app.cpu_limit || '');
	const [replicas, setReplicas] = useState(String(app.replicas || '1'));
	const [saving, setSaving] = useState(false);

	useEffect(() => {
		setMemRes(app.memory_reservation || '');
		setMemLimit(app.memory_limit || '');
		setCpuRes(app.cpu_reservation || '');
		setCpuLimit(app.cpu_limit || '');
		setReplicas(String(app.replicas || '1'));
	}, [
		app.memory_reservation,
		app.memory_limit,
		app.cpu_reservation,
		app.cpu_limit,
		app.replicas,
	]);

	const patchResources = useApplicationUpdateResourceConfig();

	const handleSaveResources = async () => {
		setSaving(true);
		try {
			await patchResources.mutateAsync({
				id: app.id,
				memory_reservation: memRes || undefined,
				memory_limit: memLimit || undefined,
				cpu_reservation: cpuRes || undefined,
				cpu_limit: cpuLimit || undefined,
				replicas: replicas ? parseInt(replicas) : 1,
			});
			toast.success('Resource limits saved successfully');
			onUpdated();
		} catch (err: unknown) {
			toast.error(formatApiError(err));
		} finally {
			setSaving(false);
		}
	};

	return (
		<AdvancedSection
			icon={<Zap className="size-4 text-primary" />}
			title="Container Replicas & Resource Limits"
			description="Memory reservation/limit (e.g. 512m, 2g) and CPU allocation.">
			<div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-4">
				<div className="flex flex-col gap-1.5">
					<Label className="text-xs font-bold text-muted-foreground">
						Memory Reservation
					</Label>
					<Input
						placeholder="128m"
						value={memRes}
						onChange={e => setMemRes(e.target.value)}
						className="text-xs"
					/>
				</div>
				<div className="flex flex-col gap-1.5">
					<Label className="text-xs font-bold text-muted-foreground">
						Memory Limit
					</Label>
					<Input
						placeholder="512m"
						value={memLimit}
						onChange={e => setMemLimit(e.target.value)}
						className="text-xs"
					/>
				</div>
				<div className="flex flex-col gap-1.5">
					<Label className="text-xs font-bold text-muted-foreground">
						CPU Reservation
					</Label>
					<Input
						placeholder="0.1"
						value={cpuRes}
						onChange={e => setCpuRes(e.target.value)}
						className="text-xs"
					/>
				</div>
				<div className="flex flex-col gap-1.5">
					<Label className="text-xs font-bold text-muted-foreground">
						CPU Limit
					</Label>
					<Input
						placeholder="0.5"
						value={cpuLimit}
						onChange={e => setCpuLimit(e.target.value)}
						className="text-xs"
					/>
				</div>
			</div>
			<div className="flex w-32 flex-col gap-1.5">
				<Label className="text-xs font-bold text-muted-foreground">
					Replicas
				</Label>
				<Input
					type="number"
					min="1"
					max="10"
					value={replicas}
					onChange={e => setReplicas(e.target.value)}
					className="text-xs"
				/>
			</div>
			<div className="flex justify-end">
				<Button
					onClick={handleSaveResources}
					disabled={saving}
					className="flex h-9 items-center gap-1.5 rounded-lg bg-primary text-xs font-semibold text-primary-foreground hover:bg-primary/95">
					<Save className="h-3.5 w-3.5" />{' '}
					{saving ? 'Saving…' : 'Save Resource Limits'}
				</Button>
			</div>
		</AdvancedSection>
	);
}

// ── Danger zone ────────────────────────────────────────────────────────

function DangerSection({app}: {app: any}) {
	const navigate = useNavigate();
	const deleteApplication = useApplicationDelete();
	const clearHistory = useApplicationClearHistory();
	const cleanupQueue = useApplicationCleanupQueue();
	const forceKill = useApplicationForceKill();
	const [confirmDelete, setConfirmDelete] = useState(false);
	const [busy, setBusy] = useState<string | null>(null);
	const [deleting, setDeleting] = useState(false);

	const run = async (key: string, fn: () => Promise<unknown>, message: string) => {
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

	const handleDeleteApp = async () => {
		setDeleting(true);
		try {
			await deleteApplication.mutateAsync(app.id);
			toast.success('Application deleted');
			navigate({to: `/projects/${app.project_id}`});
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
						run('history', () => clearHistory.mutateAsync(app.id), 'Deployment history cleared')
					}
				/>
				<DangerAction
					label="Cleanup Deployment Queue"
					busy={busy === 'queue'}
					onConfirm={() =>
						run('queue', () => cleanupQueue.mutateAsync(app.id), 'Deployment queue cleaned')
					}
				/>
				<DangerAction
					label="Force Kill Running Deployment"
					busy={busy === 'kill'}
					onConfirm={() =>
						run('kill', () => forceKill.mutateAsync(app.id), 'Deployment killed')
					}
				/>
			</div>
			<p className="text-xs text-muted-foreground">
				Permanently remove this application and all associated data,
				settings, domains, and routes.
			</p>
			{confirmDelete ? (
				<div className="flex flex-col gap-3 rounded-lg bg-destructive/5 p-4">
					<p className="text-xs font-semibold text-foreground">
						Are you sure? This action is absolutely irreversible.
					</p>
					<div className="flex items-center gap-2">
						<Button
							onClick={handleDeleteApp}
							disabled={deleting}
							className="h-9 bg-destructive px-6 text-xs font-semibold text-destructive-foreground shadow-md hover:bg-destructive/90">
							{deleting ? 'Deleting…' : 'Yes, Delete application'}
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
						<Trash2 className="h-4 w-4" /> Delete Application
					</Button>
				</div>
			)}
		</section>
	);
}
