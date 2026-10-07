import {type ReactNode, useState} from 'react';
import {useNavigate} from '@tanstack/react-router';
import {
	Ban,
	CheckCircle2,
	Plug,
	RotateCw,
	Save,
	ShieldCheck,
	Trash2,
} from 'lucide-react';
import {toast} from 'sonner';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import {Switch} from '#/components/ui/switch';
import {Textarea} from '#/components/ui/textarea';
import {Field} from '#/components/shared/field';
import {formatApiError} from '#/api/utils';
import {
	useRemoteServerActivate,
	useRemoteServerDeactivate,
	useRemoteServerTestConnection,
	useRemoteServerUpdate,
} from '#/api/live/servers';
import {useSshKeyList} from '#/api/live/ssh_keys';
import {usePermissions} from '#/hooks/permissions/use-permissions';
import {describeCron} from '#/utils/cron';
import type {RemoteServerPayload} from '#/api/live/servers';
import type {RemoteServerResponse} from '#/types/api-helpers';

import {DeleteServerModal} from '../delete-server-modal';
import {
	buildSettingsPatch,
	cleanupProblem,
	isBuildNode,
	settingsProblem,
	toSettingsForm,
	type ServerSettingsForm,
} from './settings-form';

const SERVER_TYPES = [
	{value: 'DEPLOY', label: 'Deploy', hint: 'Runs your services.'},
	{value: 'BUILD', label: 'Build', hint: 'Only builds images.'},
];

/** Settings tab: everything about this server that an operator can change — SSH target, role, build limits, cleanup schedule, lifecycle. */
export function ServerSettingsTab({
	server,
}: {
	server: RemoteServerPayload;
}) {
	const {can} = usePermissions();
	const canEdit = can('server:create');
	const navigate = useNavigate();
	const [deleting, setDeleting] = useState(false);

	const initial = toSettingsForm(server);
	const [form, setForm] = useState<ServerSettingsForm>(initial);
	const sshKeys = useSshKeyList();
	const update = useRemoteServerUpdate();
	const activate = useRemoteServerActivate();
	const deactivate = useRemoteServerDeactivate();
	const testConnection = useRemoteServerTestConnection();
	const [testing, setTesting] = useState(false);

	const patch = (changes: Partial<ServerSettingsForm>) =>
		setForm(prev => ({...prev, ...changes}));

	const body = buildSettingsPatch(form, initial);
	const dirty = Object.keys(body).length > 0;
	const problem = settingsProblem(form) ?? cleanupProblem(form);
	const active =
		(server.server_status || 'INACTIVE').toUpperCase() === 'ACTIVE';

	const onSave = () => {
		update.mutate(
			{id: server.id, ...body},
			{
				onSuccess: () => toast.success('Server settings saved'),
				onError: error => toast.error(formatApiError(error)),
			},
		);
	};

	const runLifecycle = (kind: 'activate' | 'deactivate') => {
		const mutation = kind === 'activate' ? activate : deactivate;
		mutation.mutate(server.id, {
			onSuccess: () =>
				toast.success(
					kind === 'activate'
						? 'Server marked active'
						: 'Server marked inactive',
				),
			onError: error => toast.error(formatApiError(error)),
		});
	};

	const busy = update.isPending || testing;

	return (
		<div className="flex flex-col gap-4">
			<Section
				title="Connection"
				caption="How the panel reaches this host over SSH.">
				<div className="grid gap-4 p-4 sm:grid-cols-2">
					<Field label="Server Name" htmlFor="settings-name">
						<Input
							id="settings-name"
							value={form.name}
							disabled={!canEdit || busy}
							onChange={event => patch({name: event.target.value})}
							className="h-8 text-xs"
						/>
					</Field>
					<Field label="SSH Key" htmlFor="settings-ssh-key">
						<Select
							value={form.ssh_key_id}
							onValueChange={value => value && patch({ssh_key_id: value})}
							disabled={!canEdit || busy}>
							<SelectTrigger id="settings-ssh-key" className="h-8 text-xs">
								<SelectValue
									placeholder={
										sshKeys.isLoading
											? 'Loading keys…'
											: 'Agent-managed key'
									}>
									{sshKeys.data?.find(
										key => String(key.id) === form.ssh_key_id,
									)?.name ?? 'Agent-managed key'}
								</SelectValue>
							</SelectTrigger>
							<SelectContent>
								{(sshKeys.data ?? []).map(key => (
									<SelectItem key={key.id} value={String(key.id)}>
										{key.name}
									</SelectItem>
								))}
							</SelectContent>
						</Select>
					</Field>
					<Field label="Address" htmlFor="settings-ip">
						<Input
							id="settings-ip"
							value={form.ip_address}
							disabled={!canEdit || busy}
							onChange={event => patch({ip_address: event.target.value})}
							className="h-8 font-mono text-xs"
						/>
					</Field>
					<div className="grid grid-cols-[1fr_2fr] gap-4">
						<Field label="Port" htmlFor="settings-port">
							<Input
								id="settings-port"
								value={form.port}
								inputMode="numeric"
								disabled={!canEdit || busy}
								onChange={event => patch({port: event.target.value})}
								className="h-8 text-xs"
							/>
						</Field>
						<Field label="SSH User" htmlFor="settings-user">
							<Input
								id="settings-user"
								value={form.username}
								disabled={!canEdit || busy}
								onChange={event => patch({username: event.target.value})}
								className="h-8 text-xs"
							/>
						</Field>
					</div>
					<Field
						label="Description"
						htmlFor="settings-description"
						className="sm:col-span-2">
						<Textarea
							id="settings-description"
							value={form.description}
							maxLength={1000}
							disabled={!canEdit || busy}
							placeholder="What runs on this host."
							onChange={event => patch({description: event.target.value})}
							className="min-h-16 text-xs"
						/>
					</Field>
				</div>
			</Section>

			<Section
				title="Role & Build Limits"
				caption="Whether this host serves traffic, builds images, or both.">
				<div className="flex flex-col gap-4 p-4">
					<div className="grid gap-2 sm:grid-cols-2">
						{SERVER_TYPES.map(type => {
							const selected = form.server_type === type.value;
							return (
								<button
									key={type.value}
									type="button"
									disabled={!canEdit || busy}
									onClick={() => patch({server_type: type.value})}
									className={`flex cursor-pointer flex-col gap-0.5 rounded-lg border px-4 py-3 text-left transition-colors disabled:cursor-default ${
										selected
											? 'border-primary/60 bg-primary/[0.06]'
											: 'border-border/60 bg-muted/10 hover:bg-muted/30'
									}`}>
									<span className="flex items-center gap-1.5 text-xs font-semibold text-foreground">
										{selected && (
											<ShieldCheck className="size-3.5 text-primary" />
										)}
										{type.label} server
									</span>
									<span className="text-[11px] text-muted-foreground">
										{type.hint}
									</span>
								</button>
							);
						})}
					</div>
					{isBuildNode(form) && (
						<div className="grid gap-4 sm:grid-cols-2">
							<Field
								label="Build Memory Limit"
								htmlFor="settings-build-mem"
								hint="Bytes suffixes the builder understands, e.g. 2g or 512m. Empty means unlimited.">
								<Input
									id="settings-build-mem"
									value={form.build_memory_limit}
									placeholder="2g"
									disabled={!canEdit || busy}
									onChange={event =>
										patch({build_memory_limit: event.target.value})
									}
									className="h-8 font-mono text-xs"
								/>
							</Field>
							<Field
								label="Build CPU Limit"
								htmlFor="settings-build-cpu"
								hint="Cores, e.g. 2 or 0.5. Empty means every core.">
								<Input
									id="settings-build-cpu"
									value={form.build_cpu_limit}
									placeholder="2"
									disabled={!canEdit || busy}
									onChange={event =>
										patch({build_cpu_limit: event.target.value})
									}
									className="h-8 font-mono text-xs"
								/>
							</Field>
						</div>
					)}
				</div>
			</Section>

			<Section
				title="Docker Cleanup"
				caption="The agent prunes this host's engine on the schedule below.">
				<div className="flex flex-col gap-4 p-4">
					<div className="flex items-start justify-between gap-4">
						<div className="min-w-0">
							<span className="text-xs font-semibold text-foreground">
								Automatic cleanup
							</span>
							<p className="text-[11px] text-muted-foreground">
								Uses this server’s cleanup policy from the Actions tab.
							</p>
						</div>
						<Switch
							checked={form.enable_docker_cleanup}
							disabled={!canEdit || busy}
							onCheckedChange={checked =>
								patch({enable_docker_cleanup: checked === true})
							}
						/>
					</div>
					{form.enable_docker_cleanup && (
						<Field
							label="Schedule"
							htmlFor="settings-cleanup-cron"
							hint={
								describeCron(form.log_cleanup_cron) ??
								'Five-field cron, e.g. “0 2 * * *”.'
							}>
							<Input
								id="settings-cleanup-cron"
								value={form.log_cleanup_cron}
								placeholder="0 2 * * *"
								disabled={!canEdit || busy}
								onChange={event =>
									patch({log_cleanup_cron: event.target.value})
								}
								className="h-8 font-mono text-xs"
							/>
						</Field>
					)}
				</div>
			</Section>

			<div className="flex items-center gap-3">
				<Button
					size="sm"
					disabled={!canEdit || !dirty || busy || problem !== null}
					onClick={onSave}
					className="h-8 w-fit cursor-pointer gap-1.5 text-xs disabled:cursor-default">
					<Save className="size-3.5" />
					{update.isPending ? 'Saving…' : 'Save Settings'}
				</Button>
				{problem && dirty && (
					<span className="text-[11px] text-rose-500">{problem}</span>
				)}
				{!dirty && (
					<span className="text-[11px] text-muted-foreground">
						No changes to save.
					</span>
				)}
				{!canEdit && (
					<span className="text-[11px] text-muted-foreground">
						Read-only: you need the server:create permission.
					</span>
				)}
			</div>

			<Section
				title="Lifecycle"
				caption="Connection check, availability, and removing this host from the panel.">
				<div className="flex flex-wrap items-center gap-2 p-4">
					<Button
						size="sm"
						variant="outline"
						disabled={!canEdit || testing}
						onClick={async () => {
							setTesting(true);
							try {
								await testConnection.mutateAsync(server.id);
								toast.success('SSH connection OK');
							} catch (error) {
								toast.error(formatApiError(error));
							} finally {
								setTesting(false);
							}
						}}
						className="h-8 cursor-pointer gap-1.5 text-xs">
						<Plug
							className={`size-3.5 ${testing ? 'animate-spin' : ''}`}
						/>
						{testing ? 'Testing…' : 'Test Connection'}
					</Button>
					<Button
						size="sm"
						variant="outline"
						disabled={!canEdit || activate.isPending || active}
						onClick={() => runLifecycle('activate')}
						className="h-8 cursor-pointer gap-1.5 text-xs">
						{activate.isPending ? (
							<RotateCw className="size-3.5 animate-spin" />
						) : (
							<CheckCircle2 className="size-3.5" />
						)}
						{active ? 'Active' : 'Mark Active'}
					</Button>
					<Button
						size="sm"
						variant="outline"
						disabled={!canEdit || deactivate.isPending || !active}
						onClick={() => runLifecycle('deactivate')}
						className="h-8 cursor-pointer gap-1.5 text-xs">
						<Ban className="size-3.5" />
						Mark Inactive
					</Button>
					<Button
						size="sm"
						variant="destructive"
						disabled={!canEdit}
						onClick={() => setDeleting(true)}
						className="ml-auto h-8 cursor-pointer gap-1.5 text-xs">
						<Trash2 className="size-3.5" />
						Delete Server
					</Button>
				</div>
			</Section>

			{deleting && (
				<DeleteServerModal
					isOpen
					server={server as unknown as RemoteServerResponse}
					onClose={() => setDeleting(false)}
					onSuccess={() =>
						void navigate({to: '/remote-servers', replace: true})
					}
				/>
			)}
		</div>
	);
}

function Section({
	title,
	caption,
	children,
}: {
	title: string;
	caption: string;
	children: ReactNode;
}) {
	return (
		<section className="overflow-hidden rounded-xl border border-border bg-card">
			<div className="border-b border-border/50 px-5 py-4">
				<h3 className="text-[15px] font-semibold text-foreground">
					{title}
				</h3>
				<p className="text-xs text-muted-foreground">{caption}</p>
			</div>
			{children}
		</section>
	);
}
