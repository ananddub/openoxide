import React, {useId, useMemo, useState, useEffect} from 'react';
import {Trash2, Server, Layers, KeyRound} from 'lucide-react';
import {Link} from '@tanstack/react-router';
import {cn} from '#/api/utils';
import {
	useContainerMonitoring,
	type MonitoringEntityType,
} from '#/hooks/monitoring/use-container-monitoring';
import {
	Sheet,
	SheetContent,
	SheetHeader,
	SheetTitle,
	SheetDescription,
} from '#/components/ui/sheet';
import {
	useEnvironmentListByProject,
	type EnvironmentPayload,
} from '#/api/live/environments';
import {DeleteServiceDialog} from './delete-service-dialog';
import {ServiceTypeIcon, getStatusDotColor} from './service-card-icon';
import {useServiceCard, type ServiceCardProps} from './use-service-card';

export type {ServiceCardProps} from './use-service-card';

/** Variable names from a raw .env-style string (values stay hidden). */
function envVarKeys(raw?: string): string[] {
	return (raw || '')
		.split('\n')
		.map(line => line.trim())
		.filter(line => line && !line.startsWith('#') && line.includes('='))
		.map(line => line.slice(0, line.indexOf('=')).trim());
}

const ENTITY_BY_TYPE: Record<
	ServiceCardProps['type'],
	MonitoringEntityType
> = {
	APP: 'application',
	COMPOSE: 'compose',
	DATABASE: 'database',
};

/** Deep, saturated palette per usage band; the ring stroke and its halo share one tone. */
const TONES = {
	ok: {
		from: '#2563eb',
		to: '#0284c7',
		glow: 'rgba(37,99,235,0.35)',
	},
	warm: {
		from: '#d97706',
		to: '#ea580c',
		glow: 'rgba(217,119,6,0.35)',
	},
	hot: {
		from: '#dc2626',
		to: '#b91c1c',
		glow: 'rgba(220,38,38,0.4)',
	},
};

function toneFor(percent: number) {
	if (percent >= 85) return TONES.hot;
	if (percent >= 60) return TONES.warm;
	return TONES.ok;
}

/** Geist bolt glyph shown in the gauge centre instead of the raw number. */
function GeistBolt() {
	return (
		<svg
			viewBox="0 0 16 16"
			className="size-2.5"
			fill="currentColor"
			aria-hidden>
			<path
				fillRule="evenodd"
				d="M5.51 3.62 3.76 8.35a1 1 0 0 1-.93.65H0V7.5h2.48l2.09-5.64a1 1 0 0 1 1.87-.01l4.07 10.6 1.73-4.32a1 1 0 0 1 .93-.63H16V9h-2.49l-2.08 5.19a1 1 0 0 1-1.86-.02z"
				clipRule="evenodd"
			/>
		</svg>
	);
}

/** MagicUI-style animated circular gauge — value arc grows clockwise from the top, the leftover arc sits after a moving gap, gradient stroke + glow, bolt glyph in the centre. */
function UsageRing({
	percent,
	ready,
	title,
}: {
	percent: number;
	ready: boolean;
	title: string;
}) {
	const C = 2 * Math.PI * 45;
	const percentPx = C / 100;
	const GAP = 5; // gap-percent — 18° of breathing room between the two arcs
	const pct = Math.round(Math.min(100, Math.max(0, percent)));
	const current = ready ? pct : 0;
	const tone = toneFor(pct);
	const gradientId = useId();
	const origin = {transformOrigin: '50px 50px'};
	return (
		<div className="relative size-8 shrink-0" title={title}>
			<svg viewBox="0 0 100 100" className="size-full" fill="none">
				<defs>
					<linearGradient id={gradientId} x1="0" y1="0" x2="1" y2="1">
						<stop offset="0%" stopColor={tone.from} />
						<stop offset="100%" stopColor={tone.to} />
					</linearGradient>
				</defs>
				{current < 90 && (
					<circle
						cx="50"
						cy="50"
						r="45"
						strokeWidth="10"
						strokeLinecap="round"
						strokeDasharray={`${(90 - current) * percentPx} ${C}`}
						className={cn(
							'stroke-muted-foreground/30 transition-all duration-700 ease-out',
							!ready && 'opacity-30',
						)}
						style={{
							...origin,
							transform: `rotate(${360 - 90 - GAP * 3.6}deg) scaleY(-1)`,
						}}
					/>
				)}
				<circle
					cx="50"
					cy="50"
					r="45"
					strokeWidth="10"
					strokeLinecap="round"
					strokeDasharray={`${current * percentPx} ${C}`}
					stroke={`url(#${gradientId})`}
					style={{
						...origin,
						transform: 'rotate(-90deg)',
						filter: `drop-shadow(0 0 3px ${tone.glow})`,
					}}
					className={cn(
						'transition-all duration-700 ease-out',
						!ready && 'opacity-30',
					)}
				/>
			</svg>
			<span
				className={cn(
					'absolute inset-0 flex items-center justify-center',
					ready ? 'text-foreground' : 'text-muted-foreground',
				)}>
				<GeistBolt />
			</span>
		</div>
	);
}

/** Live CPU and memory rings side by side across the card. Mounted only for services that have a container to measure — a stopped one renders nothing rather than a pair of dead rings. */
function UsageRings({
	type,
	id,
}: {
	type: ServiceCardProps['type'];
	id: number;
}) {
	const {metrics, triggerRefresh} = useContainerMonitoring(
		id,
		ENTITY_BY_TYPE[type],
		true,
		false,
	);
	useEffect(() => {
		const timer = setInterval(triggerRefresh, 15_000);
		return () => clearInterval(timer);
	}, [triggerRefresh]);
	return (
		<div className="flex shrink-0 items-start gap-2">
			<span className="flex flex-col items-center gap-0.5">
				<UsageRing
					percent={metrics?.cpuPercent ?? 0}
					ready={!!metrics}
					title={
						metrics
							? `Live CPU ${metrics.cpuPercent.toFixed(1)}%`
							: 'CPU — no stats'
					}
				/>
				<span className="font-mono text-[8px] tracking-wider text-muted-foreground uppercase">
					CPU
				</span>
			</span>
			<span className="flex flex-col items-center gap-0.5">
				<UsageRing
					percent={metrics?.memPercent ?? 0}
					ready={!!metrics}
					title={
						metrics
							? `Memory ${metrics.memUsage} of ${metrics.memLimit}`
							: 'Memory — no stats'
					}
				/>
				<span className="font-mono text-[8px] tracking-wider text-muted-foreground uppercase">
					MEM
				</span>
			</span>
		</div>
	);
}

export function ServiceCard(props: ServiceCardProps) {
	const {
		projectId,
		type,
		id,
		name,
		subtitle,
		createdAt,
		dbKind,
		serverName,
	} = props;
	const {
		routeConfig,
		actions,
		statusKey,
		isStopped,
		confirmDeleteService,
		...flags
	} = useServiceCard(props);

	const [contextMenu, setContextMenu] = useState<{
		x: number;
		y: number;
	} | null>(null);
	const [isDeleteDialogOpen, setIsDeleteDialogOpen] = useState(false);
	const [envSheet, setEnvSheet] = useState<EnvironmentPayload | null>(
		null,
	);

	const projectIdBig = useMemo(
		() => BigInt(Number(projectId) || 0),
		[projectId],
	);
	const {data: envs = []} = useEnvironmentListByProject(projectIdBig);
	const openEnvSheet = (env: EnvironmentPayload) => {
		setContextMenu(null);
		setEnvSheet(env);
	};

	// Close context menu on outside click or scroll
	useEffect(() => {
		if (!contextMenu) return;
		const handleClose = () => setContextMenu(null);
		window.addEventListener('click', handleClose);
		window.addEventListener('scroll', handleClose, true);
		return () => {
			window.removeEventListener('click', handleClose);
			window.removeEventListener('scroll', handleClose, true);
		};
	}, [contextMenu]);

	const dotColor = getStatusDotColor({...flags, statusKey});
	const closeMenu = () => setContextMenu(null);
	const openContextMenu = (e: React.MouseEvent) => {
		e.preventDefault();
		e.stopPropagation();
		setContextMenu({x: e.clientX, y: e.clientY});
	};
	const handleDelete = (e?: React.MouseEvent) => {
		e?.stopPropagation();
		closeMenu();
		setIsDeleteDialogOpen(true);
	};

	return (
		<>
			<div
				onContextMenu={openContextMenu}
				className="group relative flex w-full cursor-pointer gap-3 overflow-hidden rounded-xl border bg-card p-4 text-left transition-colors duration-200 hover:border-primary/30">
				{/* Clickable Card Overlay */}
				<Link
					to={routeConfig.to as any}
					params={routeConfig.params as any}
					search={(routeConfig as any).search}
					preload="intent"
					className="absolute inset-0 z-0 rounded-xl"
					aria-label={`Open ${name}`}
				/>

				{/* Identity + deployment facts on the left, gauges top-right. */}
				<div className="flex min-w-0 flex-1 flex-col justify-between gap-2.5">
					<div className="pointer-events-none relative z-10 flex items-center justify-between gap-2">
						<div className="flex min-w-0 flex-1 items-center gap-3">
							<div className="relative flex size-9 shrink-0 items-center justify-center rounded-lg bg-muted/40 text-foreground">
								<ServiceTypeIcon type={type} dbKind={dbKind} />
								<span
									className={cn(
										'absolute -top-0.5 -right-0.5 size-2 rounded-full border-2 border-card',
										dotColor,
									)}
								/>
							</div>
							<div className="min-w-0 flex-1">
								<p
									title={name}
									className="truncate text-xs leading-snug font-bold text-foreground transition-colors group-hover:text-primary">
									{name}
								</p>
								<p className="truncate font-mono text-[10px] tracking-wider text-muted-foreground uppercase">
									{subtitle}
								</p>
							</div>
						</div>
						{/* Stopped service has no container to measure — no gauges. */}
						{!isStopped && <UsageRings type={type} id={Number(id)} />}
					</div>

					<div className="pointer-events-none relative z-10 flex flex-col gap-1.5 border-t border-border pt-2">
						<div
							className="flex min-w-0 items-center gap-1.5 text-[10px] text-muted-foreground"
							title="Deployment server">
							<Server className="size-3 shrink-0" />
							<span className="shrink-0">Server</span>
							<span className="truncate font-semibold text-foreground">
								{serverName || 'Localhost (Main server)'}
							</span>
						</div>
						<div className="flex items-center justify-between">
							<span className="flex items-center gap-1.5 text-[10px] font-bold tracking-wider text-muted-foreground uppercase">
								<span className={cn('size-1.5 rounded-full', dotColor)} />
								{statusKey || 'idle'}
							</span>
							<span className="font-mono text-[10px] text-muted-foreground/70">
								{new Date(Number(createdAt) * 1000).toLocaleDateString(
									undefined,
									{day: '2-digit', month: 'short'},
								)}
							</span>
						</div>
					</div>
				</div>
			</div>

			{/* Custom Right-Click Context Menu */}
			{contextMenu && (
				<div
					style={{top: `${contextMenu.y}px`, left: `${contextMenu.x}px`}}
					className="fixed z-50 w-44 animate-in rounded-lg bg-popover/95 p-1 text-foreground shadow-md backdrop-blur-md duration-100 fade-in"
					onClick={e => e.stopPropagation()}>
					{actions.map(({id, label, Icon, onClick, disabled}) => (
						<button
							key={id}
							disabled={disabled}
							onClick={onClick}
							className={cn(
								'flex w-full items-center gap-2 rounded-md px-2.5 py-1.5 text-left text-xs font-medium text-foreground transition-colors hover:bg-muted',
								disabled &&
									'cursor-not-allowed opacity-40 hover:bg-transparent',
							)}>
							<Icon className="size-3.5 text-foreground" />
							{label}
						</button>
					))}
					<div className="my-1 h-px bg-border/60" />
					{envs.length > 0 && (
						<>
							<div className="px-2.5 py-1 text-[10px] font-semibold tracking-wider text-muted-foreground uppercase">
								Environments
							</div>
							{envs.map(env => (
								<button
									key={String(env.id)}
									onClick={() => openEnvSheet(env)}
									className="flex w-full items-center gap-2 rounded-md px-2.5 py-1.5 text-left text-xs font-medium text-foreground transition-colors hover:bg-muted">
									<Layers className="size-3.5 shrink-0 text-muted-foreground" />
									<span className="truncate">{env.name}</span>
									<span className="ml-auto shrink-0 font-mono text-[10px] text-muted-foreground">
										{envVarKeys(env.env_var).length}
									</span>
								</button>
							))}
							<div className="my-1 h-px bg-border/60" />
						</>
					)}
					<button
						onClick={handleDelete}
						className="flex w-full items-center gap-2 rounded-md px-2.5 py-1.5 text-left text-xs font-medium text-destructive transition-colors hover:bg-destructive/10">
						<Trash2 className="size-3.5 text-destructive" />
						Delete
					</button>
				</div>
			)}

			<DeleteServiceDialog
				isOpen={isDeleteDialogOpen}
				onClose={() => setIsDeleteDialogOpen(false)}
				serviceName={name}
				serviceType={type === 'DATABASE' ? dbKind || 'Database' : type}
				onConfirm={confirmDeleteService}
			/>

			{/* Environment basics sidebar — keeps the menu itself clean */}
			<Sheet
				open={!!envSheet}
				onOpenChange={open => {
					if (!open) setEnvSheet(null);
				}}>
				<SheetContent
					side="right"
					className="gap-0 p-0 sm:data-[side=right]:w-[640px]">
					<SheetHeader className="px-5 pt-5 pb-4">
						<SheetTitle className="flex items-center gap-2">
							{envSheet?.name}
							{envSheet?.is_default && (
								<span className="rounded-full border border-border px-2 py-0.5 text-[10px] font-semibold text-muted-foreground">
									Default
								</span>
							)}
						</SheetTitle>
						<SheetDescription>
							{envSheet?.description?.trim() ||
								'Basic environment details'}
						</SheetDescription>
					</SheetHeader>

					<div className="flex min-h-0 flex-1 flex-col px-5 pt-4 pb-5">
						{/* Same card grammar as the env editors — read-only */}
						<div className="flex min-h-0 flex-1 flex-col overflow-hidden rounded-xl border border-border bg-card">
							<div className="flex shrink-0 items-center gap-2 border-b border-border bg-muted/40 px-3 py-2">
								<KeyRound className="size-3.5 shrink-0 text-muted-foreground/70" />
								<span className="truncate font-mono text-[10px] font-medium tracking-wider text-muted-foreground uppercase">
									{`${
										String(envSheet?.name || 'env')
											.toLowerCase()
											.replace(/[^a-z0-9]+/g, '-')
											.replace(/^-|-$/g, '') || 'env'
									}.env`}
								</span>
								<span className="shrink-0 rounded-full bg-foreground/[0.06] px-1.5 py-px font-mono text-[10px] text-muted-foreground tabular-nums">
									{envVarKeys(envSheet?.env_var).length}
								</span>
							</div>
							<div className="min-h-0 flex-1 overflow-y-auto">
								{envVarKeys(envSheet?.env_var).length > 0 ? (
									envVarKeys(envSheet?.env_var).map(key => (
										<div
											key={key}
											className="flex h-11 items-center border-b border-border/60 px-3 transition-colors last:border-b-0 hover:bg-accent/30">
											<span className="min-w-0 flex-1 truncate font-mono text-xs font-semibold tracking-wide text-foreground uppercase">
												{key}
											</span>
											<span className="shrink-0 font-mono text-xs text-muted-foreground/40 select-none">
												••••••••
											</span>
										</div>
									))
								) : (
									<p className="px-4 py-8 text-center text-xs text-muted-foreground">
										No variables defined.
									</p>
								)}
							</div>
						</div>

						{envSheet && (
							<p className="mt-3 shrink-0 text-[11px] leading-normal text-muted-foreground">
								Edited from Projects → Environments. Values are hidden here
								to keep this view quick to scan.
							</p>
						)}
					</div>
				</SheetContent>
			</Sheet>
		</>
	);
}
