import {useMemo} from 'react';

import {useAuthSession, useOrganizations} from '#/stores/session-context';
import {
	useDeploymentItems,
	useDeploymentRunning,
	useProjectListByOrganization,
} from '#/api/live';

export function useHomeStats() {
	const {user} = useAuthSession();
	const firstName =
		user?.firstName ||
		(user?.email ? user.email.split('@')[0] : undefined);

	// Live projects query keyed ['projects','list',org] — table-watch
	// invalidates it while the home page is mounted.
	const {activeOrg} = useOrganizations();
	const orgId = activeOrg?.id || 1;
	const projectsQuery = useProjectListByOrganization(BigInt(orgId));
	const rawProjects = projectsQuery.data;
	const isProjectsLoading = projectsQuery.isLoading;
	// Live deployments query; the table-watch stream invalidates it while mounted.
	const {data: rawDeployments = [], isLoading: isDeploymentsLoading} =
		useDeploymentItems();

	const {data: rawRunning, isLoading: isRunningLoading} =
		useDeploymentRunning();

	const stats = useMemo(() => {
		const projectsList = Array.isArray(rawProjects) ? rawProjects : [];
		const runningList = Array.isArray(rawRunning) ? rawRunning : [];
		const deploymentsList = Array.isArray(rawDeployments)
			? rawDeployments
			: [];

		let environments = 0;
		let applications = 0;
		let compose = 0;
		let databases = 0;

		projectsList.forEach((p: unknown) => {
			const proj = p as Record<string, unknown>;
			const envs = (proj.environments as unknown[]) || [];
			environments += envs.length || 1;

			envs.forEach((e: unknown) => {
				const env = e as Record<string, unknown>;
				if (Array.isArray(env.applications))
					applications += env.applications.length;
				if (Array.isArray(env.composes)) compose += env.composes.length;
				if (Array.isArray(env.postgreses))
					databases += env.postgreses.length;
				if (Array.isArray(env.mysqls)) databases += env.mysqls.length;
				if (Array.isArray(env.mariadbs)) databases += env.mariadbs.length;
				if (Array.isArray(env.mongos)) databases += env.mongos.length;
				if (Array.isArray(env.redises)) databases += env.redises.length;
			});
		});

		// Fallback for applications count
		if (
			applications === 0 &&
			(runningList.length > 0 || deploymentsList.length > 0)
		) {
			applications = Math.max(runningList.length, deploymentsList.length);
		}

		const totalServices = applications + compose + databases;

		let running = 0;
		let error = 0;
		let idle = 0;

		const listToAnalyze =
			deploymentsList.length > 0 ? deploymentsList : runningList;

		listToAnalyze.forEach((r: unknown) => {
			const item = r as Record<string, unknown>;
			const statusStr = String(
				item.status || item.state || 'done',
			).toUpperCase();
			if (
				statusStr.includes('RUN') ||
				statusStr.includes('UP') ||
				statusStr.includes('BUILD')
			)
				running++;
			else if (
				statusStr.includes('FAIL') ||
				statusStr.includes('ERR') ||
				statusStr.includes('CRASH')
			)
				error++;
			else idle++;
		});

		return {
			projects: projectsList.length,
			environments:
				environments ||
				(projectsList.length > 0 ? projectsList.length : 1),
			applications,
			compose,
			databases,
			services: totalServices || listToAnalyze.length,
			status: {running: running || listToAnalyze.length, error, idle},
		};
	}, [rawProjects, rawRunning, rawDeployments]);

	// 7-day deploy stats calculation
	const deployStats = useMemo(() => {
		const deploymentsList =
			Array.isArray(rawDeployments) && rawDeployments.length > 0
				? rawDeployments
				: Array.isArray(rawRunning)
					? rawRunning
					: [];
		const now = Date.now();
		const weekMs = 7 * 24 * 60 * 60 * 1000;
		const lastStart = now - weekMs;
		const prevStart = now - 2 * weekMs;

		let lastCount = 0;
		let prevCount = 0;

		deploymentsList.forEach((r: unknown) => {
			const item = r as Record<string, unknown>;
			const rawDate = item.created_at || item.createdAt;
			const t = rawDate
				? new Date(
						typeof rawDate === 'bigint'
							? Number(rawDate) * 1000
							: typeof rawDate === 'number'
								? rawDate < 1e12
									? rawDate * 1000
									: rawDate
								: String(rawDate),
					).getTime()
				: now;
			if (t >= lastStart) lastCount++;
			else if (t >= prevStart) prevCount++;
		});

		let delta: string | undefined;
		if (prevCount > 0) {
			const pct = Math.round(((lastCount - prevCount) / prevCount) * 100);
			delta = `${pct >= 0 ? '+' : ''}${pct}% vs prev 7d`;
		} else if (lastCount > 0) {
			delta = 'no prior data';
		} else {
			delta = 'no activity yet';
		}

		return {value: String(lastCount || deploymentsList.length), delta};
	}, [rawDeployments, rawRunning]);

	return {
		firstName,
		stats,
		deployStats,
		projects: (Array.isArray(rawProjects) ? rawProjects : []).map(
			(raw: unknown) => {
				const p = raw as Record<string, unknown>;
				return {
					id: Number(p.id),
					name: String(p.name ?? `Project #${p.id}`),
					created_at: p.created_at as string | number | bigint | undefined,
				};
			},
		),
		isLoading:
			isProjectsLoading || isDeploymentsLoading || isRunningLoading,
	};
}
