import type {ComponentType} from 'react';
import {Activity, HardDrive, Server, Sparkles} from 'lucide-react';
import {Skeleton} from '#/components/ui/skeleton';
import {useSystemHealth} from '#/api/live';
import {useTraefikHealth} from '#/api/live/traefik';
import {usePermissions} from '#/hooks/use-permissions';
import {ResourceGuideCard} from '#/components/shared/resource-sidebar';

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

/** Sticky right rail: live health of panel engine, proxy and storage. */
export function HealthRail() {
	const {can} = usePermissions();
	const healthQuery = useSystemHealth({enabled: can('org:read')});
	const traefikQuery = useTraefikHealth({enabled: can('traefik:read')});

	return (
		<div className="flex flex-col gap-4 lg:sticky lg:top-6 lg:self-start">
			<div className="overflow-hidden rounded-xl bg-card ring-1 ring-border/50">
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
	);
}
