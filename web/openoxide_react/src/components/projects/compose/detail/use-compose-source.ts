import {useState, useEffect} from 'react';
import {toast} from 'sonner';
import {
	useComposeSetBitbucketSource,
	useComposeSetCustomGitSource,
	useComposeSetGiteaSource,
	useComposeSetGithubSource,
	useComposeSetGitlabSource,
	useComposeSetRawSource,
} from '#/api/live';
import {formatApiError} from '#/api/utils';

export interface ComposeSourceState {
	provider: string;
	composeFile: string;
	command: string;
	repoOwner: string;
	repoName: string;
	gitUrl: string;
	gitBranch: string;
	gitBuildPath: string;
	sshKeyId?: number;
}

function stateFromCompose(compose: any): ComposeSourceState {
	return {
		provider: compose?.source_type || 'GITHUB',
		composeFile: compose?.compose_file || '',
		command: compose?.command || '',
		repoOwner:
			compose?.owner ||
			compose?.gitlab_owner ||
			compose?.gitea_owner ||
			compose?.bitbucket_owner ||
			'',
		repoName:
			compose?.repository ||
			compose?.gitlab_repository ||
			compose?.gitea_repository ||
			compose?.bitbucket_repository ||
			'',
		gitUrl: compose?.custom_git_url || compose?.repository || '',
		gitBranch: compose?.branch || compose?.custom_git_branch || 'main',
		gitBuildPath:
			compose?.build_path ||
			compose?.custom_git_build_path ||
			'docker-compose.yml',
		sshKeyId: compose?.custom_git_ssh_key_id || undefined,
	};
}

/** Owns the compose source form state, GitHub file auto-preview and the provider-dispatched save (mirrors the app source-settings-card). */
export function useComposeSource(compose: any, onUpdated: () => void) {
	const [source, setSource] = useState<ComposeSourceState>(() =>
		stateFromCompose(compose),
	);
	const [savingSource, setSavingSource] = useState(false);

	const patchRaw = useComposeSetRawSource();
	const patchGithub = useComposeSetGithubSource();
	const patchGitlab = useComposeSetGitlabSource();
	const patchGitea = useComposeSetGiteaSource();
	const patchBitbucket = useComposeSetBitbucketSource();
	const patchGit = useComposeSetCustomGitSource();

	useEffect(() => {
		if (compose) setSource(stateFromCompose(compose));
	}, [compose]);

	const onPatch = (patch: Partial<ComposeSourceState>) =>
		setSource(s => ({...s, ...patch}));

	// Preview helper: fetch the compose file from GitHub raw when the editor is empty and a GitHub repo is configured.
	const {
		provider,
		repoOwner,
		repoName,
		gitBranch,
		gitBuildPath,
		composeFile,
	} = source;
	useEffect(() => {
		if (provider === 'GITHUB' && repoOwner && repoName && !composeFile) {
			const branch = gitBranch || 'main';
			const path = gitBuildPath || 'docker-compose.yml';
			const rawUrl = `https://raw.githubusercontent.com/${repoOwner}/${repoName}/${branch}/${path}`;
			fetch(rawUrl)
				.then(res => (res.ok ? res.text() : null))
				.then(text => {
					if (
						text &&
						(text.includes('services:') || text.includes('version:'))
					) {
						setSource(s => ({...s, composeFile: text}));
					}
				})
				.catch(() => {});
		}
	}, [
		provider,
		repoOwner,
		repoName,
		gitBranch,
		gitBuildPath,
		composeFile,
	]);

	const handleSaveSource = async () => {
		setSavingSource(true);
		try {
			if (!compose?.id) return;
			// Guard before hitting the server: an empty required field used to surface as an opaque "[not_found] related resource not found".
			if (provider === 'RAW' && !composeFile.trim()) {
				toast.error('Compose file content is required');
				return;
			}
			if (provider === 'GIT' && !source.gitUrl.trim()) {
				toast.error('Repository URL is required');
				return;
			}
			if (
				['GITHUB', 'GITLAB', 'GITEA', 'BITBUCKET'].includes(provider) &&
				(!repoOwner.trim() || !repoName.trim())
			) {
				toast.error('Owner and repository are required');
				return;
			}
			const id = compose.id;
			const {gitUrl, sshKeyId} = source;
			const repoPayload = {
				id,
				owner: repoOwner,
				repository: repoName,
				branch: gitBranch,
				build_path: gitBuildPath,
			};
			if (provider === 'RAW') {
				await patchRaw.mutateAsync({id, content: composeFile});
			} else if (provider === 'GIT') {
				await patchGit.mutateAsync({
					id,
					url: gitUrl,
					branch: gitBranch,
					build_path: gitBuildPath,
					ssh_key_id: sshKeyId,
				});
			} else if (provider === 'GITHUB') {
				await patchGithub.mutateAsync(repoPayload);
			} else if (provider === 'GITLAB') {
				await patchGitlab.mutateAsync(repoPayload);
			} else if (provider === 'GITEA') {
				await patchGitea.mutateAsync(repoPayload);
			} else if (provider === 'BITBUCKET') {
				await patchBitbucket.mutateAsync(repoPayload);
			}
			toast.success('Compose source settings saved successfully');
			onUpdated();
		} catch (err: any) {
			toast.error(formatApiError(err));
		} finally {
			setSavingSource(false);
		}
	};

	return {source, onPatch, savingSource, handleSaveSource};
}
