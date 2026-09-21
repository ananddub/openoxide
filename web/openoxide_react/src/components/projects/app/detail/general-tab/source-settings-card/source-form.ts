export interface SourceForm {
	gitUrl: string;
	gitBranch: string;
	gitBuildPath: string;
	dockerImage: string;
	registryUrl: string;
	dockerUsername: string;
	dockerPassword: string;
	sshKeyId?: number;
	repoOwner: string;
	repoName: string;
}

export const EMPTY_SOURCE_FORM: SourceForm = {
	gitUrl: '',
	gitBranch: 'main',
	gitBuildPath: '/',
	dockerImage: '',
	registryUrl: '',
	dockerUsername: '',
	dockerPassword: '',
	repoOwner: '',
	repoName: '',
};

/** Seed the source form state from a persisted application record. */
export function sourceFormFromApp(app: any): SourceForm {
	return {
		gitUrl: app.custom_git_url || '',
		gitBranch:
			app.custom_git_branch ||
			app.branch ||
			app.gitlab_branch ||
			app.gitea_branch ||
			app.bitbucket_branch ||
			'main',
		gitBuildPath:
			app.custom_git_build_path ||
			app.build_path ||
			app.gitlab_build_path ||
			app.gitea_build_path ||
			'/',
		dockerImage: app.docker_image || '',
		registryUrl: app.registry_url || '',
		dockerUsername: app.docker_username || '',
		dockerPassword: app.docker_password || '',
		sshKeyId: app.custom_git_ssh_key_id || undefined,
		repoOwner:
			app.owner ||
			app.gitlab_owner ||
			app.gitea_owner ||
			app.bitbucket_owner ||
			'',
		repoName:
			app.repository ||
			app.gitlab_repository ||
			app.gitea_repository ||
			app.bitbucket_repository ||
			'',
	};
}
