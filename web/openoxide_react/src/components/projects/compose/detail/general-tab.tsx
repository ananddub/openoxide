import {useState, useEffect} from 'react';
import {toast} from 'sonner';
import {$api} from '#/api/query';
import { useAppStore } from '#/stores/app-store';
import {formatApiError} from '#/api/utils';
import {ComposeDeployCard} from './compose-deploy-card';
import {ComposeSourceCard} from './cards/compose-source-card';
import {TerminalModal} from '#/components/projects/common/terminal-modal';

interface GeneralTabProps {
	compose: any;
	onAction: (action: 'deploy' | 'reload' | 'rebuild' | 'start' | 'cancel') => Promise<void>;
	onUpdated: () => void;
}

export function ComposeGeneralTab({compose, onUpdated}: GeneralTabProps) {
	const [provider, setProvider] = useState<string>(compose?.source_type || 'GITHUB');
	const [composeFile, setComposeFile] = useState<string>(compose?.compose_file || '');
	const [command, setCommand] = useState<string>(compose?.command || '');
	const [showTerminal, setShowTerminal] = useState(false);
	
	const [repoOwner, setRepoOwner] = useState<string>(compose?.owner || compose?.gitlab_owner || compose?.gitea_owner || compose?.bitbucket_owner || '');
	const [repoName, setRepoName] = useState<string>(compose?.repository || compose?.gitlab_repository || compose?.gitea_repository || compose?.bitbucket_repository || '');
	const [gitUrl, setGitUrl] = useState<string>(compose?.custom_git_url || compose?.repository || '');
	const [gitBranch, setGitBranch] = useState<string>(compose?.branch || compose?.custom_git_branch || 'main');
	const [gitBuildPath, setGitBuildPath] = useState<string>(compose?.build_path || compose?.custom_git_build_path || 'docker-compose.yml');
	const [gitSshKeyId, setGitSshKeyId] = useState<number | undefined>(compose?.custom_git_ssh_key_id || undefined);
	const [savingSource, setSavingSource] = useState(false);

	const sshKeys = useAppStore((state) => state.sshKeys || []);

	const patchRawMutation = $api.useMutation('patch', '/compose/{id}/source/raw');
	const patchGithubMutation = $api.useMutation('patch', '/compose/{id}/source/github');
	const patchGitlabMutation = $api.useMutation('patch', '/compose/{id}/source/gitlab');
	const patchGiteaMutation = $api.useMutation('patch', '/compose/{id}/source/gitea');
	const patchBitbucketMutation = $api.useMutation('patch', '/compose/{id}/source/bitbucket');
	const patchGitMutation = $api.useMutation('patch', '/compose/{id}/source/git');

	useEffect(() => {
		if (compose) {
			setProvider(compose.source_type || 'GITHUB');
			setComposeFile(compose.compose_file || '');
			setCommand(compose.command || '');
			setRepoOwner(compose.owner || compose.gitlab_owner || compose.gitea_owner || compose.bitbucket_owner || '');
			setRepoName(compose.repository || compose.gitlab_repository || compose.gitea_repository || compose.bitbucket_repository || '');
			setGitUrl(compose.custom_git_url || compose.repository || '');
			setGitBranch(compose.branch || compose.custom_git_branch || 'main');
			setGitBuildPath(compose.build_path || compose.custom_git_build_path || 'docker-compose.yml');
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
					if (text && (text.includes('services:') || text.includes('version:'))) {
						setComposeFile(text);
					}
				})
				.catch(() => {});
		}
	}, [provider, repoOwner, repoName, gitBranch, gitBuildPath, composeFile]);

	const handleSaveSource = async () => {
		setSavingSource(true);
		try {
			if (!compose?.id) return;
			if (provider === 'RAW') {
				await patchRawMutation.mutateAsync({
					params: {path: {id: compose.id}},
					body: {
						compose_file: composeFile,
					},
				});
			} else if (provider === 'GIT') {
				await patchGitMutation.mutateAsync({
					params: {path: {id: compose.id}},
					body: {
						custom_git_url: gitUrl,
						custom_git_branch: gitBranch,
						custom_git_build_path: gitBuildPath,
						custom_git_ssh_key_id: gitSshKeyId,
					} as any,
				});
			} else if (provider === 'GITHUB') {
				await patchGithubMutation.mutateAsync({
					params: {path: {id: compose.id}},
					body: {
						owner: repoOwner,
						repository: repoName,
						branch: gitBranch,
						build_path: gitBuildPath,
					} as any,
				});
			} else if (provider === 'GITLAB') {
				await patchGitlabMutation.mutateAsync({
					params: {path: {id: compose.id}},
					body: {
						gitlab_owner: repoOwner,
						gitlab_repository: repoName,
						gitlab_branch: gitBranch,
						gitlab_build_path: gitBuildPath,
					} as any,
				});
			} else if (provider === 'GITEA') {
				await patchGiteaMutation.mutateAsync({
					params: {path: {id: compose.id}},
					body: {
						gitea_owner: repoOwner,
						gitea_repository: repoName,
						gitea_branch: gitBranch,
						gitea_build_path: gitBuildPath,
					} as any,
				});
			} else if (provider === 'BITBUCKET') {
				await patchBitbucketMutation.mutateAsync({
					params: {path: {id: compose.id}},
					body: {
						bitbucket_owner: repoOwner,
						bitbucket_repository: repoName,
						bitbucket_branch: gitBranch,
					} as any,
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
