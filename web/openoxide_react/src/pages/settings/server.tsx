import {useEffect, useState, type ReactNode} from 'react';
import {createFileRoute} from '@tanstack/react-router';
import {Activity, Container, HardDrive, RotateCw, Save, Server} from 'lucide-react';
import {toast} from 'sonner';
import {Badge} from '#/components/ui/badge';
import {Button} from '#/components/ui/button';
import {Checkbox} from '#/components/ui/checkbox';
import {Input} from '#/components/ui/input';
import {Label} from '#/components/ui/label';
import {Skeleton} from '#/components/ui/skeleton';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import {Switch} from '#/components/ui/switch';
import {
	useDockerDiskUsage,
	useDockerPrune,
	useSettingsGet,
	useSettingsUpdate,
	useSystemHealth,
} from '#/api/live';
import {useTraefikHealth} from '#/api/live/traefik';
import {usePermissions} from '#/hooks/use-permissions';

export const Route = createFileRoute('/_app/settings/server')({
	component: WebServerSettingsPage,
});

type CertType = 'None' | 'Letsencrypt' | 'Custom';

type SettingsForm = {
	server_ip: string;
	host: string;
	https: boolean;
	certificate_type: CertType;
	lets_encrypt_email: string;
	custom_cert_resolver: string;
	enable_docker_cleanup: boolean;
	log_cleanup_cron: string;
};

const EMPTY_FORM: SettingsForm = {
	server_ip: '',
	host: '',
	https: false,
	certificate_type: 'None',
	lets_encrypt_email: '',
	custom_cert_resolver: '',
	enable_docker_cleanup: false,
	log_cleanup_cron: '',
};

function WebServerSettingsPage() {
	const {can} = usePermissions();
	const canManage = can('org:write');
	const canSeeHost = can('server:read');

	const healthQuery = useSystemHealth({enabled: can('org:read')});
	const traefikQuery = useTraefikHealth({enabled: can('traefik:read')});
	const diskQuery = useDockerDiskUsage({enabled: canSeeHost});
	const settingsQuery = useSettingsGet({enabled: canManage});
	const updateMutation = useSettingsUpdate();
	const pruneMutation = useDockerPrune();

	const [form, setForm] = useState<SettingsForm>(EMPTY_FORM);
	const [prune, setPrune] = useState({
		containers: true,
		images: true,
		networks: false,
		volumes: false,
	});

	// Seed the form once settings land (and on realtime refreshes while the
	// form is untouched, so external edits don't get silently overwritten).
	const [dirty, setDirty] = useState(false);
	useEffect(() => {
		const s = settingsQuery.data;
		if (!s || dirty) return;
		setForm({
			server_ip: s.server_ip ?? '',
			host: s.host ?? '',
			https: s.https,
			certificate_type:
				s.certificate_type === 'LETSENCRYPT'
					? 'Letsencrypt'
					: s.certificate_type === 'CUSTOM'
						? 'Custom'
						: 'None',
			lets_encrypt_email: s.lets_encrypt_email ?? '',
			custom_cert_resolver: s.custom_cert_resolver ?? '',
			enable_docker_cleanup: s.enable_docker_cleanup,
			log_cleanup_cron: s.log_cleanup_cron ?? '',
		});
	}, [settingsQuery.data, dirty]);

	const set = <K extends keyof SettingsForm>(key: K, value: SettingsForm[K]) => {
		setDirty(true);
		setForm(prev => ({...prev, [key]: value}));
	};

	const handleSave = () => {
		const trimmedIp = form.server_ip.trim();
		if (trimmedIp && !isIpAddress(trimmedIp)) {
			toast.error('Server IP must be a valid IPv4/IPv6 address');
			return;
		}
		if (form.host.trim().match(/\s/)) {
			toast.error('Host must not contain whitespace');
			return;
		}
		if (form.certificate_type === 'Letsencrypt' && !form.lets_encrypt_email.includes('@')) {
			toast.error('Letsencrypt requires a valid email');
			return;
		}
		if (form.certificate_type === 'Custom' && !form.custom_cert_resolver.trim()) {
			toast.error('Custom certificate requires a resolver name');
			return;
		}
		updateMutation.mutate(
			{
				// Backend patch semantics: null keeps the stored value, "" clears it.
				server_ip: trimmedIp,
				host: form.host.trim(),
				https: form.https,
				certificate_type: form.certificate_type,
				lets_encrypt_email:
					form.certificate_type === 'Letsencrypt'
						? form.lets_encrypt_email.trim()
						: null,
				custom_cert_resolver:
					form.certificate_type === 'Custom'
						? form.custom_cert_resolver.trim()
						: null,
				enable_docker_cleanup: form.enable_docker_cleanup,
				log_cleanup_cron: form.log_cleanup_cron.trim(),
			},
			{
				onSuccess: () => {
					setDirty(false);
					toast.success('Web server settings saved');
				},
				onError: error => toast.error(error.message),
			},
		);
	};

	const handlePrune = () => {
		pruneMutation.mutate(
			{...prune},
			{
				onSuccess: result => {
					toast.success(
						`Docker prune finished (${result.operation || 'prune'})`,
					);
				},
				onError: error => toast.error(error.message),
			},
		);
	};

	return (
		<div className="mx-auto flex w-full max-w-7xl animate-in flex-col gap-6 p-6 duration-200 fade-in">
			<div className="flex flex-col gap-1">
				<h1 className="text-2xl font-bold tracking-tight text-foreground">
					Web Server
				</h1>
				<p className="text-xs text-muted-foreground">
					Panel instance health, host web server configuration, and local
					Docker maintenance.
				</p>
			</div>

			{/* Live status cards */}
			<div className="grid grid-cols-1 gap-4 md:grid-cols-3">
				<StatusCard
					icon={<Activity className="size-5" />}
					title="Panel Engine"
					subtitle="Axum · Tokio · SQLite"
					gated={can('org:read')}
					loading={healthQuery.isPending}
					ok={healthQuery.data?.success === true}
					okLabel="ONLINE"
					badLabel="DEGRADED"
					detail={
						healthQuery.data && !healthQuery.data.success
							? (healthQuery.data.message ?? 'unhealthy')
							: undefined
					}
					onRetry={() => void healthQuery.refetch()}
				/>
				<StatusCard
					icon={<Server className="size-5" />}
					title="Reverse Proxy"
					subtitle="Traefik v3"
					gated={can('traefik:read')}
					loading={traefikQuery.isPending}
					ok={traefikQuery.data?.is_healthy === true}
					okLabel="ACTIVE"
					badLabel="UNREACHABLE"
					detail={
						traefikQuery.data && !traefikQuery.data.is_healthy
							? (traefikQuery.data.configuration_errors?.join(', ') ??
								traefikQuery.data.rawdata_status)
							: undefined
					}
					onRetry={() => void traefikQuery.refetch()}
				/>
				<StatusCard
					icon={<HardDrive className="size-5" />}
					title="Database Storage"
					subtitle="SQLite WAL mode"
					gated={can('org:read')}
					loading={healthQuery.isPending}
					ok={
						healthQuery.data !== undefined &&
						!/database/.test(healthQuery.data.message ?? '')
					}
					okLabel="HEALTHY"
					badLabel="UNHEALTHY"
					onRetry={() => void healthQuery.refetch()}
				/>
			</div>

			{/* Instance settings form */}
			{canManage ? (
				<div className="rounded-xl border bg-card">
					<div className="border-b px-5 py-4">
						<h3 className="text-sm font-bold">Host Configuration</h3>
						<p className="text-xs text-muted-foreground">
							Panel address, HTTPS, and certificate issuance used when
							exposing deployments.
						</p>
					</div>
					<div className="grid gap-4 p-5 md:grid-cols-2">
						<Field label="Server IP" hint="IPv4/IPv6 of this host">
							<Input
								value={form.server_ip}
								onChange={e => set('server_ip', e.target.value)}
								placeholder="203.0.113.10"
							/>
						</Field>
						<Field label="Panel Host" hint="Domain the panel is served on">
							<Input
								value={form.host}
								onChange={e => set('host', e.target.value)}
								placeholder="panel.example.com"
							/>
						</Field>
						<div className="flex items-center justify-between rounded-lg border px-4 py-3">
							<div>
								<p className="text-sm font-semibold">HTTPS</p>
								<p className="text-xs text-muted-foreground">
									Serve the panel over TLS
								</p>
							</div>
							<Switch
								checked={form.https}
								onCheckedChange={checked => set('https', checked)}
							/>
						</div>
						<Field label="Certificate Type">
							<Select
								value={form.certificate_type}
								onValueChange={value =>
									value &&
									set('certificate_type', value as CertType)
								}>
								<SelectTrigger>
									<SelectValue />
								</SelectTrigger>
								<SelectContent>
									<SelectItem value="None">None</SelectItem>
									<SelectItem value="Letsencrypt">
										Letsencrypt
									</SelectItem>
									<SelectItem value="Custom">Custom</SelectItem>
								</SelectContent>
							</Select>
						</Field>
						{form.certificate_type === 'Letsencrypt' && (
							<Field label="Letsencrypt Email" hint="Used for expiry notices">
								<Input
									type="email"
									value={form.lets_encrypt_email}
									onChange={e =>
										set('lets_encrypt_email', e.target.value)
									}
									placeholder="admin@example.com"
								/>
							</Field>
						)}
						{form.certificate_type === 'Custom' && (
							<Field label="Custom Resolver" hint="Traefik cert resolver name">
								<Input
									value={form.custom_cert_resolver}
									onChange={e =>
										set('custom_cert_resolver', e.target.value)
									}
									placeholder="my-resolver"
								/>
							</Field>
						)}
						<Field label="Log Cleanup Cron" hint="Blank disables the schedule">
							<Input
								value={form.log_cleanup_cron}
								onChange={e => set('log_cleanup_cron', e.target.value)}
								placeholder="0 4 * * *"
							/>
						</Field>
						<div className="flex items-center justify-between rounded-lg border px-4 py-3">
							<div>
								<p className="text-sm font-semibold">
									Docker Cleanup Schedule
								</p>
								<p className="text-xs text-muted-foreground">
									Periodically prune unused images/containers
								</p>
							</div>
							<Switch
								checked={form.enable_docker_cleanup}
								onCheckedChange={checked =>
									set('enable_docker_cleanup', checked)
								}
							/>
						</div>
					</div>
					<div className="flex justify-end border-t px-5 py-4">
						<Button
							onClick={handleSave}
							disabled={updateMutation.isPending || settingsQuery.isLoading}
							className="gap-2">
							<Save className="size-4" />
							{updateMutation.isPending ? 'Saving…' : 'Save Settings'}
						</Button>
					</div>
				</div>
			) : (
				<p className="rounded-lg border border-dashed px-4 py-3 text-xs text-muted-foreground">
					Host configuration is managed by the organization owner
					(requires org:write).
				</p>
			)}

			{/* Docker disk usage + prune */}
			{canSeeHost && (
				<div className="rounded-xl border bg-card">
					<div className="flex items-center justify-between border-b px-5 py-4">
						<div>
							<h3 className="flex items-center gap-2 text-sm font-bold">
								<Container className="size-4" />
								Docker Disk Usage
							</h3>
							<p className="text-xs text-muted-foreground">
								`docker system df` on this host
							</p>
						</div>
						<Button
							variant="ghost"
							size="sm"
							onClick={() => void diskQuery.refetch()}
							className="gap-1.5">
							<RotateCw
								className={`size-3.5 ${diskQuery.isFetching ? 'animate-spin' : ''}`}
							/>
							Refresh
						</Button>
					</div>
					{diskQuery.isLoading ? (
						<div className="space-y-2 p-5">
							<Skeleton className="h-8 w-full" />
							<Skeleton className="h-8 w-full" />
							<Skeleton className="h-8 w-full" />
						</div>
					) : (
						<table className="w-full text-xs">
							<thead>
								<tr className="border-b text-left text-muted-foreground">
									<th className="px-5 py-2 font-medium">Type</th>
									<th className="px-2 py-2 font-medium">Count</th>
									<th className="px-2 py-2 font-medium">Active</th>
									<th className="px-2 py-2 font-medium">Size</th>
									<th className="px-2 py-2 font-medium">
										Reclaimable
									</th>
								</tr>
							</thead>
							<tbody>
								{(diskQuery.data ?? []).map(row => (
									<tr
										key={row.Type}
										className="border-b border-border/40 last:border-0">
										<td className="px-5 py-2 font-semibold">
											{row.Type}
										</td>
										<td className="px-2 py-2">{row.TotalCount}</td>
										<td className="px-2 py-2">{row.Active}</td>
										<td className="px-2 py-2">{row.Size}</td>
										<td className="px-2 py-2 text-amber-600 dark:text-amber-500">
											{row.Reclaimable}
										</td>
									</tr>
								))}
							</tbody>
						</table>
					)}
					{can('server:create') && (
						<div className="flex flex-wrap items-center gap-4 border-t px-5 py-4">
							<span className="text-xs font-semibold text-muted-foreground">
								Prune:
							</span>
							{(
								[
									['containers', 'Containers'],
									['images', 'Images'],
									['networks', 'Networks'],
									['volumes', 'Volumes'],
								] as const
							).map(([key, label]) => (
								<label
									key={key}
									className="flex items-center gap-2 text-xs">
									<Checkbox
										checked={prune[key]}
										onCheckedChange={checked =>
											setPrune(prev => ({
												...prev,
												[key]: checked === true,
											}))
										}
									/>
									{label}
								</label>
							))}
							<Button
								size="sm"
								variant="destructive"
								onClick={handlePrune}
								disabled={
									pruneMutation.isPending ||
									!Object.values(prune).some(Boolean)
								}>
								{pruneMutation.isPending ? 'Pruning…' : 'Run Prune'}
							</Button>
						</div>
					)}
				</div>
			)}
		</div>
	);
}

function isIpAddress(value: string): boolean {
	const ipv4 =
		/^\d{1,3}(\.\d{1,3}){3}$/.test(value) &&
		value.split('.').every(part => Number(part) <= 255);
	return ipv4 || value.includes(':');
}

function StatusCard(props: {
	icon: ReactNode;
	title: string;
	subtitle: string;
	loading: boolean;
	ok: boolean;
	okLabel: string;
	badLabel: string;
	detail?: string;
	/** False when the underlying query is permission-disabled. */
	gated?: boolean;
	onRetry: () => void;
}) {
	return (
		<div className="flex flex-col gap-3 rounded-xl border bg-card p-5">
			<div className="flex items-center gap-3">
				<div className="flex size-9 items-center justify-center rounded-lg bg-primary/10 text-primary">
					{props.icon}
				</div>
				<div>
					<h3 className="text-sm font-bold text-foreground">
						{props.title}
					</h3>
					<p className="text-[10px] text-muted-foreground">
						{props.subtitle}
					</p>
				</div>
			</div>
			<div className="flex items-center justify-between border-t border-border/40 pt-3">
				<span className="max-w-[70%] truncate text-xs text-muted-foreground">
					{props.detail ?? 'Status'}
				</span>
				{props.loading ? (
					<Skeleton className="h-5 w-16" />
				) : props.gated === false ? (
					<Badge variant="secondary" className="text-[10px] font-bold">
						NO ACCESS
					</Badge>
				) : (
					<Badge
						variant={props.ok ? 'default' : 'destructive'}
						className={
							props.ok
								? 'bg-emerald-500 text-[10px] font-bold'
								: 'text-[10px] font-bold'
						}>
						{props.ok ? props.okLabel : props.badLabel}
					</Badge>
				)}
			</div>
			{props.gated !== false && (
				<button
					type="button"
					onClick={props.onRetry}
					className="text-left text-[10px] text-muted-foreground underline-offset-2 hover:underline">
					Re-check
				</button>
			)}
		</div>
	);
}

function Field(props: {
	label: string;
	hint?: string;
	children: ReactNode;
}) {
	return (
		<div className="flex flex-col gap-1.5">
			<Label className="text-xs font-semibold">{props.label}</Label>
			{props.children}
			{props.hint && (
				<p className="text-[10px] text-muted-foreground">{props.hint}</p>
			)}
		</div>
	);
}
