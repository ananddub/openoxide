import {
	deriveServiceStatus,
	type ComposeService,
} from '#/components/projects/compose/visualizer/compose-parse';

/** Single-service topology for an application (shared by tab + preview). */
export function buildAppArchService(app: any): ComposeService[] {
	return [
		{
			name: app?.name || app?.app_name || 'app',
			image: app?.repository || app?.dockerfile || app?.build_type || 'app',
			dependsOn: [],
			envVars: {},
			volumes: app?.port ? [`port:${app.port}`] : [],
			ports: app?.port ? [String(app.port)] : [],
			// Kept fresh by the realtime table-watch stream (SERVICES →
			// app_status), so the node dot is effectively live.
			status: deriveServiceStatus(
				app?.app_status || app?.status || undefined,
			),
		},
	];
}

export function filterAppArchBackups(backups: unknown, app: any): any[] {
	const list = Array.isArray(backups) ? backups : [];
	return list.filter(
		(b: any) => b.application_id === app?.id || b.app_name === app?.app_name,
	);
}
