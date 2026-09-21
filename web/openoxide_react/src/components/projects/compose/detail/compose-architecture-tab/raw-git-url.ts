/** Build a raw-file HTTPS URL for the compose file hosted in a git repo. */
export function buildRawGitUrl(compose: any): string | null {
	if (!compose) return null;
	const repo =
		compose.repository ||
		compose.custom_git_url ||
		compose.gitlab_repository ||
		compose.gitea_repository ||
		compose.bitbucket_repository;
	if (!repo || typeof repo !== 'string') return null;

	const cleanRepo = repo.trim().replace(/\.git$/, '');
	const branch =
		compose.branch ||
		compose.custom_git_branch ||
		compose.gitlab_branch ||
		compose.gitea_branch ||
		compose.bitbucket_branch ||
		'main';
	const rawPath = (compose.compose_path || 'docker-compose.yml').replace(
		/^\.\//,
		'',
	);

	const strip = (prefixes: string[]) => {
		let pathPart = cleanRepo;
		for (const prefix of prefixes) {
			if (pathPart.startsWith(prefix)) {
				pathPart = pathPart.replace(prefix, '');
				break;
			}
		}
		return pathPart.split('/').filter(Boolean);
	};

	if (cleanRepo.includes('github.com')) {
		const parts = strip([
			'https://github.com/',
			'http://github.com/',
			'github.com/',
		]);
		if (parts.length >= 2) {
			return `https://raw.githubusercontent.com/${parts[0]}/${parts[1]}/${branch}/${rawPath}`;
		}
	} else if (cleanRepo.includes('gitlab.com')) {
		const parts = strip(['https://gitlab.com/', 'gitlab.com/']);
		if (parts.length >= 2) {
			return `https://gitlab.com/${parts[0]}/${parts[1]}/-/raw/${branch}/${rawPath}`;
		}
	} else if (cleanRepo.includes('bitbucket.org')) {
		const parts = strip(['https://bitbucket.org/']);
		if (parts.length >= 2) {
			return `https://bitbucket.org/${parts[0]}/${parts[1]}/raw/${branch}/${rawPath}`;
		}
	}
	return null;
}
