import {useMemo} from 'react';
import {useAuthStore} from '#/stores/auth-store';
import {useOrganizationStore} from '#/stores/organization-store';
import type {RecentDeploymentItem} from '#/components/home/recent-deployments-card';
import {useAppStore} from '#/stores/app-store';
import {
	useProjectListByOrganization,
	useDeploymentList,
	useDeploymentRunning,
} from 'virtual:openoxide-live';

export function useHomeStats() {
	const user = useAuthStore(state => state.user);
	const activeOrg = useOrganizationStore((state) => state.activeOrg);
	const firstName = user?.firstName || (user?.email ? user.email.split('@')[0] : undefined);

	const storeProjects = useAppStore(state => state.projects);
	const storeDeployments = useAppStore(state => state.deployments);

	const rawProjects = storeProjects || [];
	const rawDeployments = storeDeployments || [];
	const isProjectsLoading = false;
	const isDeploymentsLoading = false;

	// Fetch active running deployments (/deployments/running)
	const {data: rawRunning, loading: isRunningLoading} = useDeploymentRunning({
		status: null,
		state: null,
		application_id: null,
		compose_id: null,
		database_id: null,
		server_id: null,
		limit: 50n,
		offset: null,
	});

	// Totals & Status Calculations
	const stats = useMemo(() => {
		const projectsList = Array.isArray(rawProjects) ? rawProjects : [];
		const runningList = Array.isArray(rawRunning) ? rawRunning : [];
		const deploymentsList = Array.isArray(rawDeployments) ? rawDeployments : [];

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
				if (Array.isArray(env.applications)) applications += env.applications.length;
				if (Array.isArray(env.composes)) compose += env.composes.length;
				if (Array.isArray(env.postgreses)) databases += env.postgreses.length;
				if (Array.isArray(env.mysqls)) databases += env.mysqls.length;
				if (Array.isArray(env.mariadbs)) databases += env.mariadbs.length;
				if (Array.isArray(env.mongos)) databases += env.mongos.length;
				if (Array.isArray(env.redises)) databases += env.redises.length;
			});
		});

		// Fallback for applications count
		if (applications === 0 && (runningList.length > 0 || deploymentsList.length > 0)) {
			applications = Math.max(runningList.length, deploymentsList.length);
		}

		const totalServices = applications + compose + databases;

		let running = 0;
		let error = 0;
		let idle = 0;

		const listToAnalyze = deploymentsList.length > 0 ? deploymentsList : runningList;

		listToAnalyze.forEach((r: unknown) => {
			const item = r as Record<string, unknown>;
			const statusStr = String(item.status || item.state || 'done').toUpperCase();
			if (statusStr.includes('RUN') || statusStr.includes('UP') || statusStr.includes('BUILD')) running++;
			else if (statusStr.includes('FAIL') || statusStr.includes('ERR') || statusStr.includes('CRASH')) error++;
			else idle++;
		});

		return {
			projects: projectsList.length,
			environments: environments || (projectsList.length > 0 ? projectsList.length : 1),
			applications,
			compose,
			databases,
			services: totalServices || listToAnalyze.length,
			status: {running: running || listToAnalyze.length, error, idle},
		};
	}, [rawProjects, rawRunning, rawDeployments]);

	// 7-day deploy stats calculation
	const deployStats = useMemo(() => {
		const deploymentsList = Array.isArray(rawDeployments) && rawDeployments.length > 0
			? rawDeployments
			: (Array.isArray(rawRunning) ? rawRunning : []);
		const now = Date.now();
		const weekMs = 7 * 24 * 60 * 60 * 1000;
		const lastStart = now - weekMs;
		const prevStart = now - 2 * weekMs;

		let lastCount = 0;
		let prevCount = 0;

		deploymentsList.forEach((r: unknown) => {
			const item = r as Record<string, unknown>;
			const rawDate = item.created_at || item.createdAt;
			const t = rawDate ? new Date(typeof rawDate === 'number' ? rawDate : String(rawDate)).getTime() : now;
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

	// Transform recent deployments from real /deployments API data
	const recentDeployments: RecentDeploymentItem[] = useMemo(() => {
		const deploymentsList = Array.isArray(rawDeployments) && rawDeployments.length > 0
			? rawDeployments
			: (Array.isArray(rawRunning) ? rawRunning : []);

		return deploymentsList.map((rawItem: unknown, idx: number) => {
			const item = rawItem as Record<string, unknown>;
			const statusStr = String(item.status || item.state || 'done').toUpperCase();
			let status: 'idle' | 'running' | 'done' | 'error' = 'done';
			if (statusStr.includes('RUN') || statusStr.includes('BUILD') || statusStr.includes('QUEUED')) status = 'running';
			else if (statusStr.includes('ERR') || statusStr.includes('FAIL') || statusStr.includes('CRASH')) status = 'error';
			else if (statusStr.includes('DONE') || statusStr.includes('SUCCESS') || statusStr.includes('DEPLOY')) status = 'done';

			const title = String(item.title || item.app_name || item.name || `deployment-${idx + 1}`);
			const rawDate = item.created_at || item.createdAt;

			return {
				deploymentId: String(item.id || item.deployment_id || `dep-${idx + 1}`),
				name: title,
				projectName: String(item.project_name || item.description || 'Production'),
				environment: String(item.environment || item.type || 'production'),
				serverName: String(item.server_name || 'Dokploy'),
				status,
				createdAt: rawDate
					? (typeof rawDate === 'number' ? new Date(rawDate).toISOString() : String(rawDate))
					: new Date().toISOString(),
			};
		});
	}, [rawDeployments, rawRunning]);

	return {
		firstName,
		stats,
		deployStats,
		recentDeployments,
		isLoading: isProjectsLoading || isDeploymentsLoading || isRunningLoading,
	};
}
