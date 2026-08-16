import { load as yamlLoad } from 'js-yaml';

export function buildRawGitUrl(compose: any): string | null {
	if (!compose) return null;
	const repo = compose.repository || compose.custom_git_url || compose.gitlab_repository || compose.gitea_repository || compose.bitbucket_repository;
	if (!repo || typeof repo !== 'string') return null;

	let cleanRepo = repo.trim().replace(/\.git$/, '');
	const branch = compose.branch || compose.custom_git_branch || compose.gitlab_branch || compose.gitea_branch || compose.bitbucket_branch || 'main';
	const rawPath = (compose.compose_path || 'docker-compose.yml').replace(/^\.\//, '');

	if (cleanRepo.includes('github.com')) {
		let pathPart = cleanRepo;
		if (pathPart.startsWith('https://github.com/')) pathPart = pathPart.replace('https://github.com/', '');
		else if (pathPart.startsWith('http://github.com/')) pathPart = pathPart.replace('http://github.com/', '');
		else if (pathPart.startsWith('github.com/')) pathPart = pathPart.replace('github.com/', '');
		const parts = pathPart.split('/').filter(Boolean);
		if (parts.length >= 2) {
			return `https://raw.githubusercontent.com/${parts[0]}/${parts[1]}/${branch}/${rawPath}`;
		}
	} else if (cleanRepo.includes('gitlab.com')) {
		let pathPart = cleanRepo;
		if (pathPart.startsWith('https://gitlab.com/')) pathPart = pathPart.replace('https://gitlab.com/', '');
		else if (pathPart.startsWith('gitlab.com/')) pathPart = pathPart.replace('gitlab.com/', '');
		const parts = pathPart.split('/').filter(Boolean);
		if (parts.length >= 2) {
			return `https://gitlab.com/${parts[0]}/${parts[1]}/-/raw/${branch}/${rawPath}`;
		}
	} else if (cleanRepo.includes('bitbucket.org')) {
		let pathPart = cleanRepo;
		if (pathPart.startsWith('https://bitbucket.org/')) pathPart = pathPart.replace('https://bitbucket.org/', '');
		const parts = pathPart.split('/').filter(Boolean);
		if (parts.length >= 2) {
			return `https://bitbucket.org/${parts[0]}/${parts[1]}/raw/${branch}/${rawPath}`;
		}
	}
	return null;
}

export function parseServicesFromYaml(yaml?: string): string[] {
	if (!yaml || !yaml.trim()) return [];
	try {
		let cleanYaml = yaml.trim();
		if (cleanYaml.startsWith('```')) {
			cleanYaml = cleanYaml.replace(/^```[a-zA-Z]*\n?/, '').replace(/\n?```$/, '').trim();
		}
		let doc: any = yamlLoad(cleanYaml);
		if (doc && typeof doc === 'object' && doc.services && typeof doc.services === 'object') {
			const keys = Object.keys(doc.services);
			if (keys.length > 0) return keys;
		}
	} catch {}
	return [];
}

export function getComposeServiceNames(compose: any, rawYaml?: string): string[] {
	const parsedFromRaw = parseServicesFromYaml(rawYaml);
	if (parsedFromRaw.length > 0) return parsedFromRaw;

	const parsedFromComposeFile = parseServicesFromYaml(compose?.compose_file);
	if (parsedFromComposeFile.length > 0) return parsedFromComposeFile;

	if (Array.isArray(compose?.services) && compose.services.length > 0) {
		return compose.services;
	}

	return ['app'];
}
