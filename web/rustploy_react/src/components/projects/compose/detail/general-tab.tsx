import {useState, useEffect} from 'react';
import Editor from '@monaco-editor/react';
import {Save, Code2} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {Select, SelectContent, SelectItem, SelectTrigger, SelectValue} from '#/components/ui/select';
import {toast} from 'sonner';
import {$api} from '#/api/query';
import {formatApiError} from '#/api/utils';
import {ComposeDeployCard} from './compose-deploy-card';
import {
	GithubIcon,
	GitlabIcon,
	BitbucketIcon,
	GiteaIcon,
	GitIcon,
} from '#/components/icons/data-tools-icons';

interface GeneralTabProps {
	compose: any;
	onAction: (action: 'deploy' | 'reload' | 'rebuild' | 'start' | 'cancel') => Promise<void>;
	onUpdated: () => void;
}

export function ComposeGeneralTab({compose, onAction, onUpdated}: GeneralTabProps) {
	// Providers list matching Application SourceSettingsCard — Raw YML at the LAST!
	const PROVIDERS = [
		{id: 'GITHUB', label: 'GitHub', icon: <GithubIcon className="w-4 h-4" />},
		{id: 'GITLAB', label: 'GitLab', icon: <GitlabIcon className="w-4.5 h-4.5" />},
		{id: 'GITEA', label: 'Gitea', icon: <GiteaIcon className="w-4.5 h-4.5" />},
		{id: 'BITBUCKET', label: 'Bitbucket', icon: <BitbucketIcon className="w-4 h-4" />},
		{id: 'GIT', label: 'Git', icon: <GitIcon className="w-4.5 h-4.5" />},
		{id: 'RAW', label: 'Raw YML', icon: <Code2 className="w-4 h-4 text-primary" />},
	] as const;

	const [provider, setProvider] = useState<string>(compose?.source_type || 'GITHUB');
	const [composeFile, setComposeFile] = useState<string>(compose?.compose_file || '');
	const [command, setCommand] = useState<string>(compose?.command || '');
	
	// Form fields matching Application SourceSettingsCard
	const [repoOwner, setRepoOwner] = useState<string>(compose?.owner || compose?.gitlab_owner || compose?.gitea_owner || compose?.bitbucket_owner || '');
	const [repoName, setRepoName] = useState<string>(compose?.repository || compose?.gitlab_repository || compose?.gitea_repository || compose?.bitbucket_repository || '');
	const [gitUrl, setGitUrl] = useState<string>(compose?.custom_git_url || compose?.repository || '');
	const [gitBranch, setGitBranch] = useState<string>(compose?.branch || compose?.custom_git_branch || 'main');
	const [gitBuildPath, setGitBuildPath] = useState<string>(compose?.build_path || compose?.custom_git_build_path || 'docker-compose.yml');
	const [gitSshKeyId, setGitSshKeyId] = useState<number | undefined>(compose?.custom_git_ssh_key_id || undefined);
	const [savingSource, setSavingSource] = useState(false);

	const {data: sshKeys} = $api.useQuery('get', '/ssh-keys');

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

	const handleSaveSource = async () => {
		setSavingSource(true);
		try {
			if (!compose?.id) return;
			if (provider === 'RAW') {
				await patchRawMutation.mutateAsync({
					params: {path: {id: compose.id}},
					body: {
						compose_file: composeFile,
						command: command || undefined,
					},
				});
			} else if (provider === 'GIT') {
				await patchGitMutation.mutateAsync({
					params: {path: {id: compose.id}},
					body: {
						custom_git_url: gitUrl,
						custom_git_branch: gitBranch || 'main',
						custom_git_build_path: gitBuildPath || 'docker-compose.yml',
						custom_git_ssh_key_id: gitSshKeyId || undefined,
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
						bitbucket_build_path: gitBuildPath,
					} as any,
				});
			}
			toast.success('Compose source saved successfully');
			onUpdated();
		} catch (err: any) {
			toast.error(formatApiError(err));
		} finally {
			setSavingSource(false);
		}
	};

	return (
		<div className="flex flex-col gap-6 w-full">
			{/* Dedicated Compose Deploy Settings Card */}
			<ComposeDeployCard compose={compose} handleAction={onAction} onUpdated={onUpdated} />

			{/* Source Settings Provider Card — 100% Identical to Application SourceSettingsCard! */}
			<section className="bg-card border border-border rounded-xl p-5 flex flex-col gap-4 shadow-sm">
				<div>
					<h3 className="text-sm font-bold text-foreground">Provider Source</h3>
					<p className="text-xs text-muted-foreground mt-1">Select and configure the repository source code of your compose stack</p>
				</div>

				{/* Provider Tabs Bar — Identical to Application SourceSettingsCard! */}
				<div className="flex flex-wrap border-b border-border/60 gap-1 pb-1 relative w-full -mb-[1px]">
					{PROVIDERS.map(p => {
						const isActive = provider === p.id;
						return (
							<button
								key={p.id}
								type="button"
								onClick={() => setProvider(p.id)}
								className={`text-xs font-bold px-3.5 pb-2 pt-2 transition-all flex items-center gap-1.5 cursor-pointer border-b-2 -mb-[1px] ${
									isActive 
										? 'border-foreground text-foreground font-bold' 
										: 'border-transparent text-muted-foreground hover:text-foreground hover:border-border/40'
								}`}>
								{p.icon}
								{p.label}
							</button>
						);
					})}
				</div>

				{/* Source Form Content — Exact same forms layout as Application! */}
				{provider === 'RAW' ? (
					<div className="flex flex-col gap-3 pt-2">
						<div className="flex items-center justify-between">
							<span className="text-xs font-bold text-muted-foreground uppercase tracking-wider">docker-compose.yml</span>
							<span className="text-[11px] text-muted-foreground font-mono">YAML Syntax</span>
						</div>
						<div className="border border-border/80 rounded-xl overflow-hidden shadow-inner bg-[#1e1e1e]">
							<Editor
								height="420px"
								defaultLanguage="yaml"
								language="yaml"
								theme="vs-dark"
								value={composeFile}
								onChange={val => setComposeFile(val || '')}
								options={{
									fontSize: 13,
									minimap: {enabled: false},
									scrollBeyondLastLine: false,
									wordWrap: 'on',
									tabSize: 2,
								}}
							/>
						</div>
					</div>
				) : provider === 'GIT' ? (
					<div className="grid grid-cols-1 md:grid-cols-4 gap-4 items-end pt-2">
						<div className="flex flex-col gap-1.5 md:col-span-3">
							<span className="text-xs font-bold text-foreground">Repository URL</span>
							<Input placeholder="https://github.com/username/repo.git" value={gitUrl} onChange={e => setGitUrl(e.target.value)} className="bg-card border-border font-mono text-xs h-9" />
						</div>
						<div className="flex flex-col gap-1.5 md:col-span-1">
							<span className="text-xs font-bold text-foreground">SSH Key</span>
							<Select 
								value={gitSshKeyId ? String(gitSshKeyId) : 'none'} 
								onValueChange={(val) => setGitSshKeyId(val === 'none' ? undefined : Number(val))}
							>
								<SelectTrigger className="w-full bg-card border-border text-xs h-9">
									<SelectValue placeholder="No SSH Key Selected" />
								</SelectTrigger>
								<SelectContent className="bg-popover border border-border">
									<SelectItem value="none" className="text-xs cursor-pointer hover:bg-muted font-bold text-primary">
										None (Public Repo)
									</SelectItem>
									{sshKeys?.map(key => (
										<SelectItem key={key.id} value={String(key.id)} className="text-xs cursor-pointer hover:bg-muted font-mono">
											{key.name}
										</SelectItem>
									))}
								</SelectContent>
							</Select>
						</div>
						<div className="flex flex-col gap-1.5 md:col-span-2">
							<span className="text-xs font-bold text-foreground">Branch</span>
							<Input placeholder="main" value={gitBranch} onChange={e => setGitBranch(e.target.value)} className="bg-card border-border text-xs h-9" />
						</div>
						<div className="flex flex-col gap-1.5 md:col-span-2">
							<span className="text-xs font-bold text-foreground">Build Path</span>
							<Input placeholder="docker-compose.yml" value={gitBuildPath} onChange={e => setGitBuildPath(e.target.value)} className="bg-card border-border text-xs h-9" />
						</div>
					</div>
				) : (
					<div className="grid grid-cols-1 md:grid-cols-4 gap-4 items-end pt-2">
						<div className="flex flex-col gap-1.5 md:col-span-2">
							<span className="text-xs font-bold text-foreground">Owner / Username</span>
							<Input placeholder="e.g. facebook" value={repoOwner} onChange={e => setRepoOwner(e.target.value)} className="bg-card border-border text-xs h-9" />
						</div>
						<div className="flex flex-col gap-1.5 md:col-span-2">
							<span className="text-xs font-bold text-foreground">Repository Name</span>
							<Input placeholder="e.g. react" value={repoName} onChange={e => setRepoName(e.target.value)} className="bg-card border-border text-xs h-9" />
						</div>
						<div className="flex flex-col gap-1.5 md:col-span-2">
							<span className="text-xs font-bold text-foreground">Branch</span>
							<Input placeholder="main" value={gitBranch} onChange={e => setGitBranch(e.target.value)} className="bg-card border-border text-xs h-9" />
						</div>
						<div className="flex flex-col gap-1.5 md:col-span-2">
							<span className="text-xs font-bold text-foreground">Build Path</span>
							<Input placeholder="docker-compose.yml" value={gitBuildPath} onChange={e => setGitBuildPath(e.target.value)} className="bg-card border-border text-xs h-9" />
						</div>
					</div>
				)}

				<div className="flex justify-end mt-2">
					<Button onClick={handleSaveSource} disabled={savingSource} className="bg-primary hover:bg-primary/95 text-primary-foreground font-semibold flex items-center gap-1.5 h-9 rounded-lg text-xs">
						<Save className="w-3.5 h-3.5" /> {savingSource ? 'Saving...' : 'Save Source'}
					</Button>
				</div>
			</section>
		</div>
	);
}
