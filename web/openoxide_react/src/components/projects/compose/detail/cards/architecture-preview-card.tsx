import {useMemo, useState, type ReactNode} from 'react';
import {
	Network,
	Boxes,
	Activity,
	Globe,
	GitBranch,
	Clock,
	Database,
	Cable,
	AlertCircle,
	Copy,
	Check,
	type LucideIcon,
} from 'lucide-react';
import {toast} from 'sonner';
import {
	Tooltip,
	TooltipTrigger,
	TooltipContent,
} from '#/components/ui/tooltip';
import {ComposeVisualizer} from '#/components/projects/compose/visualizer/compose-visualizer';
import {parseComposeFile} from '#/components/projects/compose/visualizer/compose-parse';
import type {ComposeService} from '#/components/projects/compose/visualizer/compose-parse';
import {useRemoteComposeFile} from '../compose-architecture-tab/use-remote-compose-file';
import {useComposeServiceStatus} from '../containers/use-compose-service-status';
import {UsageRingsStrip} from './architecture-usage-rings';

import type {MonitoringEntityType} from '#/hooks/monitoring/use-container-monitoring';

export type StatusTone = 'ok' | 'busy' | 'off' | 'bad';

const TONE_DOT: Record<StatusTone, string> = {
	ok: 'bg-emerald-500',
	busy: 'animate-pulse bg-amber-500',
	off: 'bg-muted-foreground/50',
	bad: 'bg-red-500',
};

export function statusToneFor(raw?: string): StatusTone {
	const s = (raw || '').toUpperCase();
	if (['ERROR', 'FAILED'].includes(s)) return 'bad';
	if (
		[
			'BUILDING',
			'DEPLOYING',
			'STARTING',
			'PENDING',
			'QUEUED',
			'PREPARING',
			'STOPPING',
			'CANCELLING',
		].includes(s)
	)
		return 'busy';
	if (['RUNNING', 'ACTIVE', 'READY', 'HEALTHY', 'DEPLOYED'].includes(s))
		return 'ok';
	return 'off';
}

export interface PreviewRow {
	label: string;
	value: ReactNode;
	icon?: LucideIcon;
}

const ROW_ICONS: Record<string, LucideIcon> = {
	Services: Boxes,
	Status: Activity,
	Domains: Globe,
	Source: GitBranch,
	Created: Clock,
	Engine: Database,
	Port: Cable,
};

/** Metadata rows — plain divider list, no box. */
function DetailsList({rows}: {rows: PreviewRow[]}) {
	return (
		<div className="divide-y divide-border/70">
			{rows.map(row => {
				const RowIcon = row.icon ?? ROW_ICONS[row.label];
				return (
					<div
						key={row.label}
						className="flex items-center justify-between gap-3 py-2 text-xs">
						<span className="flex shrink-0 items-center gap-2 font-medium text-muted-foreground">
							{RowIcon && <RowIcon className="size-3.5 shrink-0" />}
							{row.label}
						</span>
						<span className="min-w-0 truncate text-right">
							{row.value}
						</span>
					</div>
				);
			})}
		</div>
	);
}

/** Status pill; on error tone it doubles as a hover card with the reason. */
function StatusValue({
	status,
	errorDetail,
}: {
	status: {label: string; tone: StatusTone};
	errorDetail?: string;
}) {
	const [copied, setCopied] = useState(false);

	const handleCopy = async () => {
		if (!errorDetail) return;
		try {
			await navigator.clipboard.writeText(errorDetail);
			setCopied(true);
			toast.success('Error copied to clipboard');
			setTimeout(() => setCopied(false), 1500);
		} catch {
			toast.error('Could not copy');
		}
	};

	const pill = (
		<span className="flex w-fit cursor-default items-center gap-2 text-[13px] font-semibold">
			<span
				className={`size-2 shrink-0 rounded-full ${TONE_DOT[status.tone]}`}
			/>
			{status.label}
		</span>
	);
	if (status.tone !== 'bad' || !errorDetail) return pill;
	return (
		<Tooltip>
			<TooltipTrigger render={pill} />
			{/* Geist relative-time-card style: dark elevated surface,
			    header + mono body. Forces dark even in light theme and
			    overrides the default bg-foreground tooltip colors. */}
			<TooltipContent
				side="right"
				align="start"
				className="max-w-sm items-start gap-0 rounded-lg border-none bg-neutral-900 px-0 py-0 text-neutral-100 shadow-xl ring-1 ring-white/10 [&>svg]:hidden">
				<div className="flex w-full flex-col">
					<span className="flex items-center gap-1.5 border-b border-white/10 px-3 py-1.5 text-[10.5px] font-bold tracking-wider text-neutral-400 uppercase">
						<AlertCircle className="size-3.5 shrink-0 text-red-400" />
						Last Error
						<button
							type="button"
							onClick={handleCopy}
							aria-label="Copy error"
							className="ml-auto flex size-6 cursor-pointer items-center justify-center rounded-md text-neutral-400 transition-colors hover:bg-white/10 hover:text-white">
							{copied ? (
								<Check className="size-3.5 text-emerald-400" />
							) : (
								<Copy className="size-3.5" />
							)}
						</button>
					</span>
					<span className="px-3 py-2 font-mono text-[11.5px] leading-relaxed break-all whitespace-pre-wrap text-neutral-200">
						{errorDetail}
					</span>
				</div>
			</TooltipContent>
		</Tooltip>
	);
}

interface ArchitecturePreviewCardProps {
	/** Compose path: card resolves the compose file (DB or remote Git). */
	compose?: any;
	/** App/database path: caller supplies the topology services directly. */
	customServices?: ComposeService[];
	domains?: any[];
	schedules?: any[];
	backups?: any[];
	status?: {label: string; tone: StatusTone};
	rows?: PreviewRow[];
	/** Small icon action buttons (deploy/reload/...) shown in the header. */
	actions?: ReactNode;
	/** Deploy controls rendered full-width at the bottom of the card. */
	footer?: ReactNode;
	/** Live docker-stats ring row (CPU/mem/disk) shown above the footer. */
	monitor?: {id: number; type: MonitoringEntityType};
	/** Shown in a hover card when the status tone is an error. */
	errorDetail?: string;
	/** Extra collapsible strips embedded at the bottom of the card
	 *  (source config, credentials, ...) — keeps the page to one card. */
	sections?: ReactNode;
	onOpen: () => void;
}

/** Vercel "Production Deployment"-style summary card: header with actions, left = read-only architecture map (click opens the full interactive graph), right = key metadata rows. General visitors get the topology here; deploy controls live in the collapsible settings below. */
export function ArchitecturePreviewCard({
	compose,
	customServices,
	domains,
	schedules,
	backups,
	status,
	rows,
	actions,
	footer,
	monitor,
	errorDetail,
	sections,
	onOpen,
}: ArchitecturePreviewCardProps) {
	const fetchedComposeFile = useRemoteComposeFile(compose);
	const serviceStatus = useComposeServiceStatus(
		customServices ? undefined : compose,
		fetchedComposeFile || compose?.compose_file,
	);

	const composeBackups = useMemo(() => {
		const list = Array.isArray(backups) ? backups : [];
		return customServices
			? list
			: list.filter(
					(b: any) =>
						b.compose_id === compose?.id ||
						b.app_name === compose?.app_name,
				);
	}, [backups, compose, customServices]);

	const servicesCount = useMemo(() => {
		if (customServices) return customServices.length;
		const parsed = parseComposeFile(fetchedComposeFile || '');
		if (parsed.length > 0) return parsed.length;
		return compose?.name || compose?.app_name ? 1 : 0;
	}, [customServices, fetchedComposeFile, compose]);

	const isGit = compose?.source_type && compose.source_type !== 'RAW';
	const gitBuildPath =
		compose?.build_path ||
		compose?.custom_git_build_path ||
		'docker-compose.yml';

	const domainList = Array.isArray(domains) ? domains : [];
	const domainHost = domainList[0]?.host || domainList[0]?.domain || '';

	const allRows: PreviewRow[] = [
		{
			label: 'Services',
			value: (
				<span className="font-mono text-[13px]">
					{servicesCount} service{servicesCount === 1 ? '' : 's'}
				</span>
			),
		},
		...(status
			? [
					{
						label: 'Status',
						value: (
							<StatusValue status={status} errorDetail={errorDetail} />
						),
					},
				]
			: []),
		{
			label: 'Domains',
			value:
				domainList.length > 0 ? (
					<span className="flex items-center gap-1.5 text-[13px] font-semibold">
						<span className="truncate">{domainHost}</span>
						{domainList.length > 1 && (
							<span className="shrink-0 text-xs font-medium text-muted-foreground">
								+{domainList.length - 1} more
							</span>
						)}
					</span>
				) : (
					<span className="text-[13px] text-muted-foreground">
						No domain attached
					</span>
				),
		},
		...(rows ?? []),
	];

	return (
		<section className="overflow-hidden rounded-xl bg-card shadow-sm ring-1 ring-border/60">
			{/* Header */}
			<div className="flex flex-wrap items-center justify-between gap-3 border-b border-border px-5 py-3.5">
				<h3 className="flex items-center gap-2 text-sm font-bold text-foreground">
					<Network className="size-4 text-primary" />
					Production Architecture
				</h3>
				{actions}
			</div>

			{/* Body: preview + metadata — side-by-side only from lg up,
			    stacked so nothing gets squeezed on smaller screens */}
			<div className="grid gap-6 p-4 sm:p-5 lg:grid-cols-[minmax(0,520px)_minmax(0,1fr)]">
				{/* Left: clickable static topology map */}
				<div
					role="button"
					tabIndex={0}
					onClick={onOpen}
					onKeyDown={e => {
						if (e.key === 'Enter' || e.key === ' ') {
							e.preventDefault();
							onOpen();
						}
					}}
					aria-label="Open full architecture view"
					className="group relative flex min-h-[280px] cursor-pointer overflow-hidden rounded-lg bg-muted/40 ring-1 ring-border/70 transition-[box-shadow] outline-none group-hover:shadow-md focus-visible:ring-2 focus-visible:ring-ring sm:min-h-[360px]">
					<div className="pointer-events-none h-full w-full">
						<ComposeVisualizer
							readOnly
							monitorId={monitor?.id}
							monitorType={monitor?.type}
							wrapperClassName="h-full min-h-0 rounded-none border-0 bg-transparent"
							serviceStatus={serviceStatus}
							customServices={customServices}
							composeFile={
								fetchedComposeFile || compose?.compose_file || ''
							}
							stackName={compose?.name || compose?.app_name}
							gitBuildPath={gitBuildPath}
							isGitSource={isGit}
							backups={composeBackups as any}
							schedules={
								(Array.isArray(schedules) ? schedules : []) as any
							}
							domains={domainList as any}
						/>
					</div>
					<span className="pointer-events-none absolute inset-x-0 bottom-0 flex h-12 items-end justify-center bg-gradient-to-t from-background to-transparent pb-2.5 opacity-0 transition-opacity duration-150 group-hover:opacity-100">
						<span className="rounded-full border border-border bg-card/90 px-2.5 py-1 text-[10.5px] font-semibold text-foreground shadow-sm backdrop-blur-sm">
							Open full architecture view
						</span>
					</span>
				</div>

				{/* Right: details list + live usage rings */}
				<div className="flex min-w-0 flex-col justify-center gap-4 pt-0.5">
					<DetailsList rows={allRows} />

					{monitor && monitor.id > 0 && (
						<UsageRingsStrip id={monitor.id} entityType={monitor.type} />
					)}
				</div>
			</div>

			{/* Extra embedded strips (source config / credentials) */}
			{sections}

			{/* Deploy controls — full width inside the same card */}
			{footer && (
				<div className="border-t border-border bg-background/40 px-5 py-4">
					{footer}
				</div>
			)}
		</section>
	);
}
