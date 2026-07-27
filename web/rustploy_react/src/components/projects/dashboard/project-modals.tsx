import {useNavigate} from '@tanstack/react-router';
import {CreateEnvDialog} from '#/components/projects/env/create-env-dialog';
import {EnvVariablesModal} from '#/components/projects/env/env-variables-modal';
import {CreateAppDialog} from '#/components/projects/app/create-app-dialog';
import {CreateComposeDialog} from '#/components/projects/compose/create-compose-dialog';
import {CreateDatabaseDialog} from '#/components/projects/database/create-database-dialog';

interface ProjectModalsProps {
	projectId: number;
	project: any;
	selectedEnvId: number | null;
	selectedEnv: any;
	servers: any[];
	showCreateEnv: boolean;
	setShowCreateEnv: (open: boolean) => void;
	showProjectEnv: boolean;
	setShowProjectEnv: (open: boolean) => void;
	showEnvVars: boolean;
	setShowEnvVars: (open: boolean) => void;
	showCreateApp: boolean;
	setShowCreateApp: (open: boolean) => void;
	showCreateCompose: boolean;
	setShowCreateCompose: (open: boolean) => void;
	showCreateDatabase: boolean;
	setShowCreateDatabase: (open: boolean) => void;
	handleRefresh: () => void;
	envsRefetch: () => void;
}

export function ProjectModals({
	projectId,
	project,
	selectedEnvId,
	selectedEnv,
	servers,
	showCreateEnv,
	setShowCreateEnv,
	showProjectEnv,
	setShowProjectEnv,
	showEnvVars,
	setShowEnvVars,
	showCreateApp,
	setShowCreateApp,
	showCreateCompose,
	setShowCreateCompose,
	showCreateDatabase,
	setShowCreateDatabase,
	handleRefresh,
	envsRefetch,
}: ProjectModalsProps) {
	const navigate = useNavigate();
	return (
		<>
			{showCreateEnv && (
				<CreateEnvDialog
					isOpen={showCreateEnv}
					onClose={() => setShowCreateEnv(false)}
					projectId={projectId}
					onCreated={() => {
						envsRefetch();
					}}
				/>
			)}

			{showProjectEnv && project && (
				<EnvVariablesModal
					isOpen={showProjectEnv}
					onClose={() => setShowProjectEnv(false)}
					mode="PROJECT"
					project={project}
					environment={null}
					onUpdated={handleRefresh}
				/>
			)}

			{showEnvVars && project && selectedEnv && (
				<EnvVariablesModal
					isOpen={showEnvVars}
					onClose={() => setShowEnvVars(false)}
					mode="ENVIRONMENT"
					project={project}
					environment={selectedEnv}
					onUpdated={handleRefresh}
				/>
			)}

			{showCreateApp && selectedEnvId && (
				<CreateAppDialog
					isOpen={showCreateApp}
					onClose={() => setShowCreateApp(false)}
					environmentId={selectedEnvId}
					onCreated={handleRefresh}
				/>
			)}

			{showCreateCompose && selectedEnvId && (
				<CreateComposeDialog
					isOpen={showCreateCompose}
					onClose={() => setShowCreateCompose(false)}
					environmentId={selectedEnvId}
					onCreated={(compose) => {
						handleRefresh();
						if (compose?.id) {
							navigate({ to: `/projects/${projectId}/compose/${compose.id}` as any });
						}
					}}
				/>
			)}

			{showCreateDatabase && selectedEnvId && (
				<CreateDatabaseDialog
					isOpen={showCreateDatabase}
					onClose={() => setShowCreateDatabase(false)}
					environmentId={selectedEnvId}
					servers={servers}
					onCreated={(db) => {
						setShowCreateDatabase(false);
						handleRefresh();
						const targetDbId = db?.id || db?.data?.id;
						const targetKind = db?.kind || db?.data?.kind || 'postgres';
						if (targetDbId) {
							navigate({
								to: `/projects/${projectId}/database/${targetDbId}` as any,
								search: {kind: targetKind} as any,
							});
						}
					}}
				/>
			)}
		</>
	);
}
