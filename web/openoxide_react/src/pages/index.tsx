import {createFileRoute, Link} from '@tanstack/react-router';
import {
	Activity,
	ArrowRight,
	BookOpen,
	Clock,
	FolderKanban,
	GitBranch,
	Plus,
	Rocket,
	Server,
	Boxes,
} from 'lucide-react';
import {useMemo} from 'react';
import {useHomeStats} from '#/hooks/home/use-home-stats';
import {
	useOverviewServices,
	useOverviewDomains,
} from '#/api/live/overview';
import {ProjectRow} from '#/components/home/project-row';
import {ActivitySummary} from '#/components/activity/activity-feed';

export const Route = createFileRoute('/_app/')({
	component: Home,
});

function Home() {
	const {firstName, stats, deployStats, projects, isLoading} =
		useHomeStats();
	const {data: services = []} = useOverviewServices();
	const {data: domains = []} = useOverviewDomains();

	// Group live services by project so each row can render its stack,
	// hosting, and aggregated status pill without extra fetches.
	const servicesByProject = useMemo(() => {
		const map = new Map<number, typeof services>();
		for (const s of services) {
			const pid = Number(s.project_id);
			const list = map.get(pid);
			if (list) list.push(s);
			else map.set(pid, [s]);
		}
		return map;
	}, [services]);

	return (
		<div className="mx-auto w-full max-w-[1600px] px-4 py-6 sm:px-6 lg:px-8">
			{/* ── Header: greeting only (OpenShip) — no CTA button ────── */}
			<div className="mb-6">
				<h1
					className="text-2xl font-medium text-foreground/80"
					style={{letterSpacing: '-0.2px'}}>
					{firstName ? `Welcome back, ${firstName}` : 'Welcome back'}
				</h1>
				<p className="mt-1 text-sm text-muted-foreground/70">
					Infrastructure overview &amp; service status metrics
				</p>
			</div>
			{/* ── Main Grid ─────────────────────────────────────────── */}
			<div className="grid grid-cols-1 gap-6 lg:grid-cols-[1fr_340px]">
				<div className="flex min-w-0 flex-col gap-6">
					{/* Your Projects — one card, header + divided rows (OpenShip) */}
					<div className="flex flex-1 flex-col rounded-2xl bg-card">
						<div className="flex items-center justify-between border-b  px-5 py-4">
							<div className="flex items-center gap-3">
								<div className="flex size-9 items-center justify-center rounded-xl bg-primary/10">
									<FolderKanban className="size-[18px] text-primary" />
								</div>
								<div>
									<h2 className="text-[15px] font-semibold text-foreground">
										Your Projects
									</h2>
									<p className="text-xs text-muted-foreground">
										{isLoading
											? 'Loading…'
											: `${stats.projects} project${stats.projects === 1 ? '' : 's'} · ${stats.services} services`}
									</p>
								</div>
							</div>
							<Link
								to="/projects"
								className="flex items-center gap-1 text-sm font-medium text-muted-foreground transition-colors hover:text-foreground">
								View all
								<ArrowRight className="size-3.5" />
							</Link>
						</div>

						{isLoading ? (
							<div className="flex-1 divide-y divide-border/50">
								{[...Array(3)].map((_, i) => (
									<div
										key={i}
										className="flex animate-pulse items-center gap-4 px-5 py-4">
										<div className="size-10 rounded-xl bg-muted" />
										<div className="flex-1 space-y-2">
											<div className="h-4 w-32 rounded bg-muted" />
											<div className="h-3 w-48 rounded bg-muted" />
										</div>
									</div>
								))}
							</div>
						) : projects.length === 0 ? (
							<div className="flex flex-1 flex-col items-center justify-center px-6 py-12 text-center">
								<div className="mb-4 flex size-12 items-center justify-center rounded-2xl bg-primary/10">
									<Plus className="size-6 text-primary" />
								</div>
								<h3
									className="mb-1 text-lg font-medium text-foreground/80"
									style={{letterSpacing: '-0.2px'}}>
									No projects yet
								</h3>
								<p className="mx-auto mb-4 max-w-sm text-sm leading-relaxed text-muted-foreground/70">
									Create your first project to deploy applications, compose
									stacks, and databases.
								</p>
								<Link
									to="/projects"
									className="inline-flex items-center gap-2 rounded-xl bg-primary px-5 py-2.5 text-sm font-medium text-primary-foreground transition-all hover:-translate-y-0.5 hover:bg-primary/90 hover:shadow-lg hover:shadow-primary/25">
									<Plus className="size-4" />
									New Project
								</Link>
							</div>
						) : (
							<div className="flex-1 divide-y divide-border/50">
								{projects.slice(0, 6).map(p => (
									<ProjectRow
										key={p.id}
										project={p}
										services={servicesByProject.get(p.id) ?? []}
										domains={domains}
									/>
								))}
								{projects.length > 6 && (
									<Link
										to="/projects"
										className="block px-5 py-3 text-center text-sm text-muted-foreground/70 transition-colors hover:bg-muted/40 hover:text-foreground">
										View all {projects.length} projects
									</Link>
								)}
							</div>
						)}
					</div>
					{/* Shortcuts */}
					<div className="grid grid-cols-2 gap-3 sm:grid-cols-4">
						<Link
							to="/projects"
							search={{create: 1} as never}
							className="group rounded-xl bg-muted/40 p-4 transition-colors hover:bg-muted/70">
							<div className="mb-3 flex size-9 items-center justify-center rounded-lg bg-muted transition-transform group-hover:scale-105">
								<Plus className="size-[18px] text-muted-foreground" />
							</div>
							<p className="text-sm font-medium text-foreground">
								New Project
							</p>
							<p className="mt-0.5 text-xs text-muted-foreground">
								Deploy an app or database
							</p>
						</Link>
						<Link
							to="/Deployments"
							className="group rounded-xl bg-muted/40 p-4 transition-colors hover:bg-muted/70">
							<div className="mb-3 flex size-9 items-center justify-center rounded-lg bg-muted transition-transform group-hover:scale-105">
								<Rocket className="size-[18px] text-muted-foreground" />
							</div>
							<p className="text-sm font-medium text-foreground">
								Deployments
							</p>
							<p className="mt-0.5 text-xs text-muted-foreground">
								Build & deploy history
							</p>
						</Link>
						<Link
							to="/remote-servers"
							className="group rounded-xl bg-muted/40 p-4 transition-colors hover:bg-muted/70">
							<div className="mb-3 flex size-9 items-center justify-center rounded-lg bg-muted transition-transform group-hover:scale-105">
								<Server className="size-[18px] text-muted-foreground" />
							</div>
							<p className="text-sm font-medium text-foreground">
								Servers
							</p>
							<p className="mt-0.5 text-xs text-muted-foreground">
								Per-host health & metrics
							</p>
						</Link>
						<a
							href="https://github.com/vajra-labs/goploy"
							target="_blank"
							rel="noopener noreferrer"
							className="group rounded-xl bg-muted/40 p-4 transition-colors hover:bg-muted/70">
							<div className="mb-3 flex size-9 items-center justify-center rounded-lg bg-muted transition-transform group-hover:scale-105">
								<BookOpen className="size-[18px] text-muted-foreground" />
							</div>
							<p className="flex items-center gap-1 text-sm font-medium text-foreground">
								Docs
								<ArrowRight className="size-3 text-muted-foreground" />
							</p>
							<p className="mt-0.5 text-xs text-muted-foreground">
								Guides & API reference
							</p>
						</a>
					</div>
				</div>

				{/* ── RIGHT COLUMN (sticky) ───────────────────────────── */}
				<div className="space-y-4 lg:sticky lg:top-6 lg:self-start">
					{/* Activity overview */}
					<div className="rounded-2xl bg-card p-5">
						<div className="mb-4 flex items-center gap-2">
							<Activity className="size-4 text-muted-foreground" />
							<h3 className="text-sm font-semibold text-foreground">
								Activity
							</h3>
						</div>

						<div className="space-y-3">
							<div className="flex items-center justify-between">
								<div className="flex items-center gap-2">
									<div className="flex size-8 items-center justify-center rounded-lg bg-primary/10">
										<FolderKanban className="size-4 text-primary" />
									</div>
									<span className="text-sm text-muted-foreground">
										Projects
									</span>
								</div>
								<span className="text-lg font-semibold text-foreground">
									{isLoading ? '–' : stats.projects}
								</span>
							</div>
							<div className="flex items-center justify-between">
								<div className="flex items-center gap-2">
									<div className="flex size-8 items-center justify-center rounded-lg bg-orange-500/10">
										<Boxes className="size-4 text-orange-500" />
									</div>
									<span className="text-sm text-muted-foreground">
										Services
									</span>
								</div>
								<span className="text-lg font-semibold text-foreground">
									{isLoading ? '–' : stats.services}
								</span>
							</div>
							<div className="flex items-center justify-between">
								<div className="flex items-center gap-2">
									<div className="flex size-8 items-center justify-center rounded-lg bg-sky-500/10">
										<Rocket className="size-4 text-sky-500" />
									</div>
									<span className="text-sm text-muted-foreground">
										Deploys / 7d
									</span>
								</div>
								<span className="text-lg font-semibold text-foreground">
									{isLoading ? '–' : deployStats.value}
								</span>
							</div>

							<div className="my-2 h-px bg-border/60" />

							{/* System status */}
							<div className="flex items-center justify-between">
								<div className="flex items-center gap-2">
									<GitBranch className="size-4 text-muted-foreground" />
									<span className="text-sm text-muted-foreground">
										Status
									</span>
								</div>
								<div className="flex items-center gap-3 text-sm">
									<span className="flex items-center gap-1.5">
										<span className="size-2 rounded-full bg-emerald-500" />
										<span className="font-semibold text-foreground">
											{stats.status.running}
										</span>
									</span>
									<span className="flex items-center gap-1.5">
										<span className="size-2 rounded-full bg-rose-500" />
										<span className="font-semibold text-foreground">
											{stats.status.error}
										</span>
									</span>
									<span className="flex items-center gap-1.5">
										<span className="size-2 rounded-full bg-muted-foreground/40" />
										<span className="font-semibold text-foreground">
											{stats.status.idle}
										</span>
									</span>
								</div>
							</div>
						</div>
					</div>

					{/* Recent activity — deployments + audit, merged feed */}
					<div className="rounded-2xl bg-card p-5">
						<div className="mb-4 flex items-center justify-between">
							<div className="flex items-center gap-2">
								<Clock className="size-4 text-muted-foreground" />
								<h3 className="text-sm font-semibold text-foreground">
									Recent activity
								</h3>
							</div>
							<Link
								to="/settings/audit-logs"
								className="flex items-center gap-1 text-xs font-medium text-muted-foreground transition-colors hover:text-foreground">
								View all
								<ArrowRight className="size-3" />
							</Link>
						</div>
						<ActivitySummary count={6} />
					</div>
				</div>
			</div>
		</div>
	);
}
