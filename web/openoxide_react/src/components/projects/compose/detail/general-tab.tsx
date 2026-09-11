import {useState, useEffect} from 'react';
import {toast} from 'sonner';
import {
	useComposeSetBitbucketSource,
	useComposeSetCustomGitSource,
	useComposeSetGiteaSource,
	useComposeSetGithubSource,
	useComposeSetGitlabSource,
	useComposeSetRawSource,
	useSshKeyItems,
} from '#/api/live';
import {formatApiError} from '#/api/utils';
import {ComposeDeployCard} from './compose-deploy-card';
import {ComposeSourceCard} from './cards/compose-source-card';
import {TerminalModal} from '#/components/projects/common/terminal-modal';

interface GeneralTabProps {
	compose: any;
	onAction: (
		action: 'deploy' | 'reload' | 'rebuild' | 'start' | 'cancel',
	) => Promise<void>;
	onUpdated: () => void;
}

export function ComposeGeneralTab({compose, onUpdated}: GeneralTabProps) {
	const [provider, setProvider] = useState<string>(
		compose?.source_type || 'GITHUB',
	);
	const [composeFile, setComposeFile] = useState<string>(
		compose?.compose_file || '',
	);
	const [command, setCommand] = useState<string>(compose?.command || '');
	const [showTerminal, setShowTerminal] = useState(false);

	const [repoOwner, setRepoOwner] = useState<string>(
		compose?.owner ||
			compose?.gitlab_owner ||
			compose?.gitea_owner ||
			compose?.bitbucket_owner ||
			'',
	);
	const [repoName, setRepoName] = useState<string>(
		compose?.repository ||
			compose?.gitlab_repository ||
			compose?.gitea_repository ||
			compose?.bitbucket_repository ||
			'',
	);
	const [gitUrl, setGitUrl] = useState<string>(
		compose?.custom_git_url || compose?.repository || '',
	);
	const [gitBranch, setGitBranch] = useState<string>(
		compose?.branch || compose?.custom_git_branch || 'main',
	);
	const [gitBuildPath, setGitBuildPath] = useState<string>(
		compose?.build_path ||
			compose?.custom_git_build_path ||
			'docker-compose.yml',
	);
	const [gitSshKeyId, setGitSshKeyId] = useState<number | undefined>(
		compose?.custom_git_ssh_key_id || undefined,
	);
	const [savingSource, setSavingSource] = useState(false);

	const sshKeys = useSshKeyItems().data ?? [];

	const patchRawMutation = useComposeSetRawSource();
	const patchGithubMutation = useComposeSetGithubSource();
	const patchGitlabMutation = useComposeSetGitlabSource();
	const patchGiteaMutation = useComposeSetGiteaSource();
	const patchBitbucketMutation = useComposeSetBitbucketSource();
	const patchGitMutation = useComposeSetCustomGitSource();

	useEffect(() => {
		if (compose) {
			setProvider(compose.source_type || 'GITHUB');
			setComposeFile(compose.compose_file || '');
			setCommand(compose.command || '');
			setRepoOwner(
				compose.owner ||
					compose.gitlab_owner ||
					compose.gitea_owner ||
					compose.bitbucket_owner ||
					'',
			);
			setRepoName(
				compose.repository ||
					compose.gitlab_repository ||
					compose.gitea_repository ||
					compose.bitbucket_repository ||
					'',
			);
			setGitUrl(compose.custom_git_url || compose.repository || '');
			setGitBranch(compose.branch || compose.custom_git_branch || 'main');
			setGitBuildPath(
				compose.build_path ||
					compose.custom_git_build_path ||
					'docker-compose.yml',
			);
			setGitSshKeyId(compose.custom_git_ssh_key_id || undefined);
		}
	}, [compose]);

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
						setComposeFile(text);
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
			// Guard before hitting the server: an empty required field used to
			// surface as an opaque "[not_found] related resource not found".
			if (provider === 'RAW' && !composeFile.trim()) {
				toast.error('Compose file content is required');
				return;
			}
			if (provider === 'GIT' && !gitUrl.trim()) {
				toast.error('Repository URL is required');
				return;
			}
			if (
				provider === 'GITHUB' ||
				provider === 'GITLAB' ||
				provider === 'GITEA' ||
				provider === 'BITBUCKET'
			) {
				if (!repoOwner.trim() || !repoName.trim()) {
					toast.error('Owner and repository are required');
					return;
				}
			}
			if (provider === 'RAW') {
				await patchRawMutation.mutateAsync({
					id: compose.id,
					content: composeFile,
				});
			} else if (provider === 'GIT') {
				await patchGitMutation.mutateAsync({
					id: compose.id,
					url: gitUrl,
					branch: gitBranch,
					build_path: gitBuildPath,
					ssh_key_id: gitSshKeyId,
				});
			} else if (provider === 'GITHUB') {
				await patchGithubMutation.mutateAsync({
					id: compose.id,
					owner: repoOwner,
					repository: repoName,
					branch: gitBranch,
					build_path: gitBuildPath,
				});
			} else if (provider === 'GITLAB') {
				await patchGitlabMutation.mutateAsync({
					id: compose.id,
					owner: repoOwner,
					repository: repoName,
					branch: gitBranch,
					build_path: gitBuildPath,
				});
			} else if (provider === 'GITEA') {
				await patchGiteaMutation.mutateAsync({
					id: compose.id,
					owner: repoOwner,
					repository: repoName,
					branch: gitBranch,
					build_path: gitBuildPath,
				});
			} else if (provider === 'BITBUCKET') {
				await patchBitbucketMutation.mutateAsync({
					id: compose.id,
					owner: repoOwner,
					repository: repoName,
					branch: gitBranch,
					build_path: gitBuildPath,
				});
			}
			toast.success('Compose source settings saved successfully');
			onUpdated();
		} catch (err: any) {
			toast.error(formatApiError(err));
		} finally {
			setSavingSource(false);
		}
	};

	return (
		<div className="flex flex-col gap-6">
			{/* Deploy Settings Card Component */}
			<ComposeDeployCard
				compose={compose}
				onUpdated={onUpdated}
				onOpenTerminal={() => setShowTerminal(true)}
			/>

			{/* Source Settings Card Component */}
			<ComposeSourceCard
				provider={provider}
				setProvider={setProvider}
				composeFile={composeFile}
				setComposeFile={setComposeFile}
				command={command}
				setCommand={setCommand}
				repoOwner={repoOwner}
				setRepoOwner={setRepoOwner}
				repoName={repoName}
				setRepoName={setRepoName}
				gitUrl={gitUrl}
				setGitUrl={setGitUrl}
				gitBranch={gitBranch}
				setGitBranch={setGitBranch}
				gitBuildPath={gitBuildPath}
				setGitBuildPath={setGitBuildPath}
				gitSshKeyId={gitSshKeyId}
				setGitSshKeyId={setGitSshKeyId}
				sshKeys={sshKeys}
				savingSource={savingSource}
				onSave={handleSaveSource}
			/>

			{/* Interactive WebSockets Terminal Modal for Compose Containers */}
			<TerminalModal
				app={compose}
				open={showTerminal}
				onClose={() => setShowTerminal(false)}
			/>
		</div>
	);
}
