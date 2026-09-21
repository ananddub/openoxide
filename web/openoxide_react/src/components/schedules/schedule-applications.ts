/** Flatten applications (direct + per-environment) out of a project list. */
export function collectProjectApplications(
	projectsList: any[],
): {id: number; name: string}[] {
	if (!Array.isArray(projectsList)) return [];
	const apps: {id: number; name: string}[] = [];
	const push = (app: any) =>
		apps.push({
			id: app.id || app.application_id,
			name: app.name || app.app_name || `App #${app.id}`,
		});
	projectsList.forEach((proj: any) => {
		(proj.applications ?? []).forEach(push);
		(proj.environments ?? []).forEach((env: any) =>
			(env.applications ?? []).forEach(push),
		);
	});
	return apps;
}
