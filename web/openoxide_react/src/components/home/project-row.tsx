import {Link} from '@tanstack/react-router';
import {
	FolderOpen,
	GitBranch,
	Globe,
	Server,
	ArrowRight,
	Boxes,
	Database,
	AppWindow,
} from 'lucide-react';
import type {
	OverviewServiceItemPayload,
	OverviewDomainItemPayload,
} from '#/api/live/overview';

export type ProjectRowProject = {
	id: number;
	name: string;
	description?: string;
	created_at?: string | number | bigint;
};

function timeAgo(dateStr?: string | number | bigint): string {
	if (dateStr === undefined) return 'recently';
	const numeric = typeof dateStr === 'bigint' ? Number(dateStr) : dateStr;
	const ms =
		typeof numeric === 'number' && numeric < 1e12
			? numeric * 1000
			: numeric;
	const date = new Date(ms as string | number);
	const diffSec = Math.floor((Date.now() - date.getTime()) / 1000);
	if (isNaN(diffSec) || diffSec < 0) return 'just now';
	if (diffSec < 60) return `${diffSec}s ago`;
	const min = Math.floor(diffSec / 60);
	if (min < 60) return `${min}m ago`;
	const hr = Math.floor(min / 60);
	if (hr < 24) return `${hr}h ago`;
	return `${Math.floor(hr / 24)}d ago`;
}

/** Aggregate a project's services into one status pill (OpenShip style). */
function statusPill(services: OverviewServiceItemPayload[]) {
	if (services.length === 0)
		return {
			label: 'No services',
			cls: 'text-muted-foreground bg-muted/60',
		};
	const statuses = services.map(s => (s.status || 'done').toLowerCase());
	if (
		statuses.some(
			s =>
				s.includes('build') ||
				s.includes('deploy') ||
				s.includes('run') ||
				s.includes('up'),
		)
	)
		return {label: 'Running', cls: 'text-emerald-500 bg-emerald-500/10'};
	if (
		statuses.some(
			s =>
				s.includes('fail') || s.includes('error') || s.includes('crash'),
		)
	)
		return {label: 'Attention', cls: 'text-rose-500 bg-rose-500/10'};
	return {label: 'Idle', cls: 'text-muted-foreground bg-muted/60'};
}

/** The dominant service type drives the row icon + stack badge. */
function primaryService(services: OverviewServiceItemPayload[]) {
	const app = services.find(s => s.service_type === 'APP');
	const compose = services.find(s => s.service_type === 'COMPOSE');
	const db = services.find(s => s.service_type === 'DATABASE');
	return app || compose || db || services[0];
}

function serviceGlyph(svc?: OverviewServiceItemPayload) {
	if (!svc) return <Boxes className="size-6 text-muted-foreground" />;
	if (svc.service_type === 'DATABASE')
		return <Database className="size-6 text-muted-foreground" />;
	if (svc.service_type === 'COMPOSE')
		return <Boxes className="size-6 text-muted-foreground" />;
	return <AppWindow className="size-6 text-muted-foreground" />;
}

function stackName(svc?: OverviewServiceItemPayload): string {
	if (!svc) return 'Empty';
	if (svc.service_type === 'DATABASE')
		return (svc.db_kind || 'database').toUpperCase();
	if (svc.service_type === 'COMPOSE') return 'Docker Compose';
	return 'Application';
}

type Props = {
	project: ProjectRowProject;
	services: OverviewServiceItemPayload[];
	domains: OverviewDomainItemPayload[];
};

/**
 * One project row inside a card, laid out like OpenShip's dashboard
 * ProjectCard: icon tile, name + domain, meta badges (stack, hosting,
 * source), status pill, relative time, trailing arrow. The whole row is a
 * stretched link.
 */
export function ProjectRow({project, services, domains}: Props) {
	const svc = primaryService(services);
	const pill = statusPill(services);
	const domain = domains.find(d => d.project_name === project.name)?.host;
	const serverName = svc?.server_name;
	const hasGit = services.some(s => s.service_type === 'APP');

	return (
		<div className="group relative flex items-center gap-4 px-5 py-3.5 transition-colors hover:bg-muted/40">
			<Link
				to="/projects/$id"
				params={{id: String(project.id)}}
				preload="intent"
				aria-label={project.name}
				className="absolute inset-0 z-0"
			/>

			{/* Icon tile */}
			<div className="flex size-10 shrink-0 items-center justify-center overflow-hidden rounded-xl bg-muted/60 transition-colors group-hover:bg-muted">
				{serviceGlyph(svc)}
			</div>

			{/* Name + domain */}
			<div className="w-44 min-w-0 flex-shrink-0 text-start lg:w-56">
				<p className="truncate text-sm font-medium text-foreground">
					{project.name}
				</p>
				{domain ? (
					<p className="mt-0.5 truncate text-xs text-muted-foreground">
						{domain}
					</p>
				) : (
					<p className="mt-0.5 truncate text-xs text-muted-foreground">
						Created {timeAgo(project.created_at)}
					</p>
				)}
			</div>

			{/* Meta badges */}
			<div className="flex min-w-0 flex-1 items-center gap-3 overflow-hidden">
				<span className="inline-flex shrink-0 items-center gap-1.5 rounded-md bg-muted/60 px-2 py-0.5 text-xs text-muted-foreground">
					{stackName(svc)}
				</span>
				{services.length > 1 && (
					<span className="inline-flex shrink-0 items-center gap-1.5 rounded-md bg-primary/10 px-2 py-0.5 text-xs font-medium text-primary">
						{services.length} services
					</span>
				)}
				{serverName && (
					<span className="hidden shrink-0 items-center gap-1.5 text-xs text-muted-foreground sm:inline-flex">
						<Server className="size-3.5" />
						<span className="max-w-[120px] truncate">{serverName}</span>
					</span>
				)}
				{hasGit && (
					<span className="hidden shrink-0 items-center gap-1.5 text-xs text-muted-foreground md:inline-flex">
						<GitBranch className="size-3.5" />
						<span className="max-w-[140px] truncate">Git</span>
					</span>
				)}
				{!hasGit && services.length > 0 && (
					<span className="hidden shrink-0 items-center gap-1.5 text-xs text-muted-foreground md:inline-flex">
						<FolderOpen className="size-3.5" />
						<span className="max-w-[140px] truncate">Local</span>
					</span>
				)}
				{!serverName && domain && (
					<span className="hidden shrink-0 items-center gap-1.5 text-xs text-muted-foreground lg:inline-flex">
						<Globe className="size-3.5" />
						<span className="max-w-[140px] truncate">{domain}</span>
					</span>
				)}
			</div>

			{/* Right: status pill + time + arrow */}
			<div className="flex shrink-0 items-center gap-3">
				<span className="hidden text-xs text-muted-foreground lg:block">
					{timeAgo(project.created_at)}
				</span>
				<span
					className={`inline-flex shrink-0 items-center rounded-full px-2 py-0.5 text-[10px] font-medium ${pill.cls}`}>
					{pill.label}
				</span>
				<ArrowRight className="size-4 text-muted-foreground/40 transition-colors group-hover:text-muted-foreground" />
			</div>
		</div>
	);
}
