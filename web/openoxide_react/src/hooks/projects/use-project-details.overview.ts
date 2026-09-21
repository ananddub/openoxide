// Overview-services fallback for useProjectDetails: fills any project service
// not already present in the direct application/compose/database live lists.
import {normalizeDatabaseKind} from './use-project-details.mappers';

export function appendOverviewServices(
	result: any[],
	seenKeys: Set<string>,
	args: {
		overviewServices: any[];
		projectId: number;
		activeEnvId: number | null;
		getServerName: (service: any) => string;
	},
): void {
	const {overviewServices, projectId, activeEnvId, getServerName} = args;
	overviewServices.forEach((s: any) => {
		if (!s || String(s.project_id || s.projectId) !== String(projectId))
			return;
		const envId = s.environment_id ?? s.environmentId;
		if (
			activeEnvId !== null &&
			envId !== undefined &&
			envId !== null &&
			Number(envId) !== Number(activeEnvId)
		)
			return;
		const sType = String(s.service_type || s.type || '').toUpperCase();
		const isDb =
			sType === 'DATABASE' ||
			sType.includes('DB') ||
			!!s.db_kind ||
			!!s.dbKind;
		const isCompose = sType === 'COMPOSE' || sType.includes('COMPOSE');
		const normalizedType = isDb
			? 'DATABASE'
			: isCompose
				? 'COMPOSE'
				: 'APP';
		const dbKind = isDb
			? normalizeDatabaseKind(s.db_kind || s.dbKind || s.kind)
			: undefined;
		const key = isDb
			? `DATABASE-${dbKind}-${s.id}`
			: `${normalizedType}-${s.id}`;
		if (seenKeys.has(key)) return;
		seenKeys.add(key);
		result.push({
			id: s.id,
			type: normalizedType,
			name: s.name,
			subtitle: isDb
				? dbKind
					? dbKind.toUpperCase()
					: 'DATABASE'
				: s.name || s.app_name,
			status: s.status || s.app_status || 'STOPPED',
			createdAt: s.created_at || Date.now(),
			...(dbKind ? {dbKind} : {}),
			projectId: Number(s.project_id || projectId),
			environmentId: envId,
			serverName: getServerName(s),
		});
	});
}
