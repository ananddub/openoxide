import {useEffect, useState, type ComponentType, type ReactNode} from 'react';
import {
	Activity,
	BadgeCheck,
	Container,
	HardDrive,
	RotateCw,
	Save,
	Server,
	Shield,
	Sparkles,
	Trash2,
	Wrench,
} from 'lucide-react';
import {toast} from 'sonner';
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
import {ResourceGuideCard} from '#/components/shared/resource-sidebar';

// Rendered as the "Web Server" tab of pages/infra.tsx — no standalone route.

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

export function WebServerSettingsPage() {
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

	const set = <K extends keyof SettingsForm>(
		key: K,
		value: SettingsForm[K],
	) => {
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
		if (
			form.certificate_type === 'Letsencrypt' &&
			!form.lets_encrypt_email.includes('@')
		) {
			toast.error('Letsencrypt requires a valid email');
			return;
		}
		if (
			form.certificate_type === 'Custom' &&
			!form.custom_cert_resolver.trim()
		) {
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

	const reclaimable = (diskQuery.data ?? []).reduce((sum, row) => {
		const match = /^([\d.]+)([kMG]?)B?$/.exec(row.Reclaimable?.trim() ?? '');
		if (!match) return sum;
		const mult = {k: 1e3, M: 1e6, G: 1e9}[match[2]] ?? 1;
		return sum + Number(match[1]) * mult;
	}, 0);

	return (
		<div className="flex w-full flex-col gap-6">
			{/* ── Content grid + right rail ── */}
			<div className="grid grid-cols-1 items-start gap-6 lg:grid-cols-[minmax(0,1fr)_340px]">
				<div className="flex min-w-0 flex-col gap-6">
					{/* Instance settings */}
					{canManage ? (
						<div className="overflow-hidden rounded-2xl bg-card ring-1 ring-border/50">
							<div className="flex items-center gap-3 border-b border-border/50 px-5 py-4">
								<div className="flex size-9 shrink-0 items-center justify-center rounded-xl bg-muted">
									<Shield className="size-[18px] text-muted-foreground" />
								</div>
								<div className="min-w-0 flex-1">
									<h3 className="text-[15px] font-semibold text-foreground">
										Host Configuration
									</h3>
									<p className="text-xs text-muted-foreground">
										How this panel is addressed, secured, and kept tidy.
									</p>
								</div>
								<Button
									size="sm"
									onClick={handleSave}
									disabled={
										updateMutation.isPending || settingsQuery.isLoading
									}
									className="h-9 shrink-0 cursor-pointer gap-2 px-4 text-xs font-semibold">
									<Save className="size-3.5" />
									{updateMutation.isPending ? 'Saving…' : 'Save Settings'}
								</Button>
							</div>
							<div className="space-y-6 p-5">
								{/* Panel address */}
								<section className="flex flex-col gap-3">
									<SectionLabel icon={Server}>
										Panel Address
									</SectionLabel>
									<div className="grid gap-4 sm:grid-cols-2">
										<Field
											label="Server IP"
											hint="IPv4/IPv6 of this host">
											<Input
												value={form.server_ip}
												onChange={e => set('server_ip', e.target.value)}
												placeholder="203.0.113.10"
												className="h-9 font-mono text-xs"
											/>
										</Field>
										<Field
											label="Panel Host"
											hint="Domain the panel is served on">
											<Input
												value={form.host}
												onChange={e => set('host', e.target.value)}
												placeholder="panel.example.com"
												className="h-9 font-mono text-xs"
											/>
										</Field>
									</div>
								</section>

								{/* TLS */}
								<section className="flex flex-col gap-3">
									<SectionLabel icon={BadgeCheck}>
										TLS &amp; Certificates
									</SectionLabel>
									<div className="flex items-center justify-between gap-3 rounded-xl bg-muted/20 px-4 py-3 ring-1 ring-border/40">
										<div className="min-w-0">
											<p className="text-[13px] font-medium text-foreground">
												HTTPS
											</p>
											<p className="mt-0.5 text-xs text-muted-foreground">
												Serve the panel over TLS
											</p>
										</div>
										<Switch
											checked={form.https}
											onCheckedChange={checked => set('https', checked)}
										/>
									</div>
									<div className="grid gap-4 sm:grid-cols-2">
										<Field label="Certificate Type">
											<Select
												value={form.certificate_type}
												onValueChange={value =>
													value && set('certificate_type', value as CertType)
												}>
												<SelectTrigger className="h-9 text-xs">
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
											<Field
												label="Letsencrypt Email"
												hint="Used for expiry notices">
												<Input
													type="email"
													value={form.lets_encrypt_email}
													onChange={e =>
														set('lets_encrypt_email', e.target.value)
													}
													placeholder="admin@example.com"
													className="h-9 text-xs"
												/>
											</Field>
										)}
										{form.certificate_type === 'Custom' && (
											<Field
												label="Custom Resolver"
												hint="Traefik cert resolver name">
												<Input
													value={form.custom_cert_resolver}
													onChange={e =>
														set('custom_cert_resolver', e.target.value)
													}
													placeholder="my-resolver"
													className="h-9 font-mono text-xs"
												/>
											</Field>
										)}
									</div>
								</section>

								{/* Maintenance */}
								<section className="flex flex-col gap-3">
									<SectionLabel icon={Wrench}>Maintenance</SectionLabel>
									<div className="grid gap-4 sm:grid-cols-2">
										<Field
											label="Log Cleanup Cron"
											hint="Blank disables the schedule">
											<Input
												value={form.log_cleanup_cron}
												onChange={e =>
													set('log_cleanup_cron', e.target.value)
												}
												placeholder="0 4 * * *"
												className="h-9 font-mono text-xs"
											/>
										</Field>
										<div className="flex items-center justify-between gap-3 rounded-xl bg-muted/20 px-4 py-3 ring-1 ring-border/40">
											<div className="min-w-0">
												<p className="text-[13px] font-medium text-foreground">
													Docker Cleanup
												</p>
												<p className="mt-0.5 text-xs text-muted-foreground">
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
								</section>
							</div>
						</div>
					) : (
						<p className="rounded-xl bg-card px-4 py-3 text-xs text-muted-foreground ring-1 ring-border/50">
							Host configuration is managed by the organization owner
							(requires org:write).
						</p>
					)}

					{/* Docker disk usage + prune */}
					{canSeeHost && (
						<div className="overflow-hidden rounded-2xl bg-card ring-1 ring-border/50">
							<div className="flex items-center gap-3 border-b border-border/50 px-5 py-4">
								<div className="flex size-9 shrink-0 items-center justify-center rounded-xl bg-muted">
									<Container className="size-[18px] text-muted-foreground" />
								</div>
								<div className="min-w-0 flex-1">
									<h3 className="text-[15px] font-semibold text-foreground">
										Docker Disk Usage
									</h3>
									<p className="text-xs text-muted-foreground">
										{'`docker system df`'} on this host
										{reclaimable > 0 && (
											<span className="text-amber-500">
												{' · '}
												{formatBytes(reclaimable)} reclaimable
											</span>
										)}
									</p>
								</div>
								<Button
									variant="outline"
									size="sm"
									onClick={() => void diskQuery.refetch()}
									className="h-8 shrink-0 cursor-pointer gap-1.5 text-xs">
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
										<tr className="border-b border-border/50 text-left text-muted-foreground">
											<th className="px-5 py-2.5 font-medium">Type</th>
											<th className="px-2 py-2.5 font-medium">Count</th>
											<th className="px-2 py-2.5 font-medium">Active</th>
											<th className="px-2 py-2.5 font-medium">Size</th>
											<th className="px-5 py-2.5 text-right font-medium">
												Reclaimable
											</th>
										</tr>
									</thead>
									<tbody>
										{(diskQuery.data ?? []).map(row => (
											<tr
												key={row.Type}
												className="border-b border-border/40 last:border-0">
												<td className="px-5 py-2.5 font-semibold">
													{row.Type}
												</td>
												<td className="px-2 py-2.5 tabular-nums">
													{row.TotalCount}
												</td>
												<td className="px-2 py-2.5 tabular-nums">
													{row.Active}
												</td>
												<td className="px-2 py-2.5 tabular-nums">
													{row.Size}
												</td>
												<td className="px-5 py-2.5 text-right tabular-nums text-amber-600 dark:text-amber-500">
													{row.Reclaimable}
												</td>
											</tr>
										))}
									</tbody>
								</table>
							)}
							{can('server:create') && (
								<div className="flex flex-wrap items-center gap-4 border-t border-border/50 bg-muted/10 px-5 py-3.5">
									<span className="inline-flex items-center gap-1.5 text-xs font-semibold text-muted-foreground">
										<Trash2 className="size-3.5" />
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
											className="flex cursor-pointer items-center gap-2 text-xs text-foreground/80">
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
										}
										className="ml-auto h-8 cursor-pointer text-xs">
										{pruneMutation.isPending ? 'Pruning…' : 'Run Prune'}
									</Button>
								</div>
							)}
						</div>
					)}
				</div>

				{/* ── Right rail ── */}
				<div className="flex flex-col gap-4 lg:sticky lg:top-6 lg:self-start">
					<div className="overflow-hidden rounded-2xl bg-card ring-1 ring-border/50">
						<div className="flex items-center gap-3 border-b border-border/50 px-5 py-4">
							<div className="flex size-9 shrink-0 items-center justify-center rounded-xl bg-muted">
								<Activity className="size-[18px] text-muted-foreground" />
							</div>
							<div className="min-w-0">
								<h2 className="truncate text-[15px] font-semibold text-foreground">
									Live Health
								</h2>
								<p className="text-xs text-muted-foreground">
									Services backing this page
								</p>
							</div>
						</div>
						<div className="divide-y divide-border/40 px-5">
							<HealthRow
								icon={Activity}
								title="Panel Engine"
								subtitle="Axum · Tokio · SQLite"
								gated={can('org:read')}
								loading={healthQuery.isPending}
								ok={healthQuery.data?.success === true}
								okLabel="Online"
								badLabel="Degraded"
								onRetry={() => void healthQuery.refetch()}
							/>
							<HealthRow
								icon={Server}
								title="Reverse Proxy"
								subtitle="Traefik v3"
								gated={can('traefik:read')}
								loading={traefikQuery.isPending}
								ok={traefikQuery.data?.is_healthy === true}
								okLabel="Active"
								badLabel="Unreachable"
								onRetry={() => void traefikQuery.refetch()}
							/>
							<HealthRow
								icon={HardDrive}
								title="Database Storage"
								subtitle="SQLite WAL mode"
								gated={can('org:read')}
								loading={healthQuery.isPending}
								ok={
									healthQuery.data !== undefined &&
									!/database/.test(healthQuery.data.message ?? '')
								}
								okLabel="Healthy"
								badLabel="Unhealthy"
								onRetry={() => void healthQuery.refetch()}
							/>
						</div>
					</div>

					<ResourceGuideCard
						title="How serving works"
						icon={Sparkles}
						steps={[
							{
								title: 'Traefik routes everything',
								text: 'Domains you attach to apps, compose stacks and databases are routed automatically — no nginx config.',
							},
							{
								title: 'Certificates are per-domain',
								text: 'Letsencrypt issues and renews TLS for each domain; Custom defers to a resolver you manage in Traefik.',
							},
							{
								title: 'Cleanup keeps the host healthy',
								text: 'Scheduled pruning stops unused images and logs from eating the disk. Run it manually any time.',
							},
						]}
					/>
				</div>
			</div>
		</div>
	);
}

function isIpAddress(value: string): boolean {
	const ipv4 =
		/^\d{1,3}(\.\d{1,3}){3}$/.test(value) &&
		value.split('.').every(part => Number(part) <= 255);
	return ipv4 || value.includes(':');
}

function formatBytes(bytes: number): string {
	if (bytes >= 1e9) return `${(bytes / 1e9).toFixed(1)} GB`;
	if (bytes >= 1e6) return `${(bytes / 1e6).toFixed(1)} MB`;
	return `${Math.round(bytes / 1e3)} kB`;
}

function SectionLabel({
	icon: Icon,
	children,
}: {
	icon: ComponentType<{className?: string}>;
	children: ReactNode;
}) {
	return (
		<p className="flex items-center gap-2 text-[11px] font-semibold uppercase tracking-wider text-muted-foreground">
			<Icon className="size-3.5" />
			{children}
		</p>
	);
}

function HealthRow(props: {
	icon: ComponentType<{className?: string}>;
	title: string;
	subtitle: string;
	loading: boolean;
	ok: boolean;
	okLabel: string;
	badLabel: string;
	gated?: boolean;
	onRetry: () => void;
}) {
	const Icon = props.icon;
	return (
		<div className="flex items-center gap-3 py-3">
			<div className="flex size-8 shrink-0 items-center justify-center rounded-lg bg-muted/60">
				<Icon className="size-4 text-muted-foreground" />
			</div>
			<div className="min-w-0 flex-1">
				<p className="truncate text-[13px] font-medium text-foreground">
					{props.title}
				</p>
				<p className="truncate text-[11px] text-muted-foreground">
					{props.subtitle}
				</p>
			</div>
			{props.loading ? (
				<Skeleton className="h-5 w-14 shrink-0 rounded-full" />
			) : props.gated === false ? (
				<span className="shrink-0 rounded-full bg-muted px-2.5 py-1 text-[11px] font-medium text-muted-foreground">
					No access
				</span>
			) : (
				<span
					title={`${props.ok ? props.okLabel : props.badLabel} — click to re-check`}
					onClick={props.onRetry}
					className={`inline-flex shrink-0 cursor-pointer items-center gap-1.5 rounded-full px-2.5 py-1 text-[11px] font-medium transition-opacity hover:opacity-80 ${
						props.ok
							? 'bg-emerald-500/10 text-emerald-500'
							: 'bg-rose-500/10 text-rose-500'
					}`}>
					<span
						className={`size-1.5 rounded-full ${props.ok ? 'bg-emerald-500' : 'bg-rose-500'}`}
					/>
					{props.ok ? props.okLabel : props.badLabel}
				</span>
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
