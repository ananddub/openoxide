import {createFileRoute} from '@tanstack/react-router';
import {useMemo} from 'react';
import {useHomeStats} from '#/hooks/home/use-home-stats';
import {
	useOverviewServices,
	useOverviewDomains,
} from '#/api/live/overview';
import {Titlebar} from '#/components/shared/titlebar';
import {StatCell} from '#/components/home/overview-widgets';
import {ProjectsCard} from '#/components/home/projects-card';
import {OverviewRightColumn} from '#/components/home/overview-right-column';
import {buildDeployHistogram} from '#/components/home/deploy-histogram';

export const Route = createFileRoute('/_app/')({
	component: Home,
});

function Home() {
	const {firstName, stats, deployStats, projects, isLoading} =
		useHomeStats();
	const {data: servicesData} = useOverviewServices();
	const {data: domainsData} = useOverviewDomains();
	// Live queries can hand back null while streaming; normalize once here.
	const services = Array.isArray(servicesData) ? servicesData : [];
	const domains = Array.isArray(domainsData) ? domainsData : [];

	// Group live services by project so each row can render its stack and
	// aggregated status without extra fetches.
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

	// 14-day deploy histogram for the activity card.
	const deployDays = useMemo(() => {
		const {days} = buildDeployHistogram(deployStats as never, services);
		return days;
	}, [deployStats, services]);

	const runningPct = Math.round(
		(stats.status.running /
			Math.max(
				stats.status.running + stats.status.error + stats.status.idle,
				1,
			)) *
			100,
	);

	return (
		<div className="mx-auto w-full max-w-[1600px] px-4 py-6 sm:px-6 lg:px-8">
			<Titlebar
				breadcrumbs={[{label: 'Overview'}]}
				title="Overview"
				description={
					firstName
						? `A summary of ${firstName}'s workspace activity and health`
						: 'Workspace activity and health at a glance'
				}
			/>

			{/* ── Stats band: one card, internal hairline dividers ───── */}
			<div className="mb-6 overflow-hidden rounded-lg border bg-card shadow-[0_1px_1px_rgba(0,0,0,0.02),0_2px_2px_rgba(0,0,0,0.04)]">
				<div className="grid grid-cols-2 divide-border md:grid-cols-4 md:divide-x">
					<StatCell
						label="Projects"
						value={isLoading ? '…' : stats.projects}
						sub={`${stats.environments} environments`}
					/>
					<StatCell
						label="Services"
						value={isLoading ? '…' : stats.services}
						sub={`${stats.applications} apps · ${stats.databases} databases`}
					/>
					<StatCell
						label="Deployments / 7d"
						value={isLoading ? '…' : deployStats.value}
						sub={deployStats.delta}
					/>
					<StatCell
						label="Active domains"
						value={isLoading ? '…' : domains.length}
						sub="https routes"
					/>
				</div>
			</div>

			{/* ── Main grid ──────────────────────────────────────────── */}
			<div className="grid grid-cols-1 gap-6 lg:grid-cols-[1fr_340px]">
				<div className="flex min-w-0 flex-col gap-6">
					<ProjectsCard
						isLoading={isLoading}
						projects={projects}
						servicesByProject={servicesByProject}
						domains={domains}
					/>
				</div>

				<OverviewRightColumn
					runningPct={runningPct}
					status={stats.status}
					isLoading={isLoading}
					deployStats={deployStats}
					deployDays={deployDays}
				/>
			</div>
		</div>
	);
}

export default Home;
