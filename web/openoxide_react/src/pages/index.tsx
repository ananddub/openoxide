import {createFileRoute} from '@tanstack/react-router';
import {useHomeStats} from '#/hooks/home/use-home-stats';
import {HomeHeader} from '#/components/home/home-header';
import {StatCard} from '#/components/home/stat-card';
import {StatusListCard} from '#/components/home/status-list-card';
import {RecentDeploymentsCard} from '#/components/home/recent-deployments-card';

export const Route = createFileRoute('/_app/')({
	component: Home,
});

function Home() {
	const {firstName, stats, deployStats, recentDeployments, isLoading} =
		useHomeStats();

	return (
		<div className="mx-auto flex w-full max-w-7xl animate-in flex-col gap-6 p-4 duration-200 fade-in">
			{/* Header: Welcome back & Go to projects */}
			<HomeHeader firstName={firstName} />

			{/* 4 Stat Cards: Projects, Services, Deploys / 7d, Status */}
			<div className="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-4">
				<StatCard
					label="Projects"
					value={String(stats.projects)}
					delta={`${stats.environments} ${stats.environments === 1 ? 'environment' : 'environments'}`}
				/>
				<StatCard
					label="Services"
					value={String(stats.services)}
					delta={`${stats.applications} apps · ${stats.compose} compose · ${stats.databases} db`}
				/>
				<StatCard
					label="Deploys / 7d"
					value={deployStats.value}
					delta={deployStats.delta}
				/>
				<StatusListCard
					label="Status"
					items={[
						{
							dotClass: 'bg-emerald-500',
							label: 'running',
							count: stats.status.running,
						},
						{
							dotClass: 'bg-red-500',
							label: 'errored',
							count: stats.status.error,
						},
						{
							dotClass: 'bg-muted-foreground/40',
							label: 'idle',
							count: stats.status.idle,
						},
					]}
				/>
			</div>

			{/* Recent Deployments Section */}
			<RecentDeploymentsCard
				deployments={recentDeployments}
				isLoading={isLoading}
				canReadDeployments={true}
			/>
		</div>
	);
}
