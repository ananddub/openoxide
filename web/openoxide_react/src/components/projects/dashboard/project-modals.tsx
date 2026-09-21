import {CreateEnvDialog} from '#/components/projects/env/create-env-dialog';
import {EnvVariablesModal} from '#/components/projects/env/env-variables-modal';

interface ProjectModalsProps {
	projectId: number;
	project: any;
	selectedEnv: any;
	showCreateEnv: boolean;
	setShowCreateEnv: (open: boolean) => void;
	showProjectEnv: boolean;
	setShowProjectEnv: (open: boolean) => void;
	showEnvVars: boolean;
	setShowEnvVars: (open: boolean) => void;
	handleRefresh: () => void;
	envsRefetch: () => void;
}

export function ProjectModals({
	projectId,
	project,
	selectedEnv,
	showCreateEnv,
	setShowCreateEnv,
	showProjectEnv,
	setShowProjectEnv,
	showEnvVars,
	setShowEnvVars,
	handleRefresh,
	envsRefetch,
}: ProjectModalsProps) {
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
		</>
	);
}
