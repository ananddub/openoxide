import {useState, useEffect} from 'react';
import {Save, UploadCloud} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {Select, SelectContent, SelectItem, SelectTrigger, SelectValue} from '#/components/ui/select';
import {toast} from 'sonner';
import {$api} from '#/api/query';
import { useAppStore } from '#/stores/app-store';
import {formatApiError} from '#/api/utils';
import {DropSourceForm} from './drop-source-form';
import {
	GithubIcon,
	GitlabIcon,
	BitbucketIcon,
	GiteaIcon,
	DockerIcon,
	GitIcon,
} from '#/components/icons/data-tools-icons';

interface SourceSettingsCardProps {
	app: any;
	onUpdated: () => void;
}

export function SourceSettingsCard({app, onUpdated}: SourceSettingsCardProps) {
	const PROVIDERS = [
		{id: 'GITHUB', label: 'GitHub', icon: <GithubIcon className="w-4 h-4" />},
		{id: 'GITLAB', label: 'GitLab', icon: <GitlabIcon className="w-4.5 h-4.5" />},
		{id: 'GITEA', label: 'Gitea', icon: <GiteaIcon className="w-4.5 h-4.5" />},
		{id: 'BITBUCKET', label: 'Bitbucket', icon: <BitbucketIcon className="w-4 h-4" />},
		{id: 'GIT', label: 'Git', icon: <GitIcon className="w-4.5 h-4.5" />},
		{id: 'DOCKER', label: 'Docker', icon: <DockerIcon className="w-4.5 h-4.5" />},
		{id: 'DROP', label: 'Drop', icon: <UploadCloud className="w-4 h-4" />},
	] as const;

	const [provider, setProvider] = useState(app.source_type || 'GIT');
	const [gitUrl, setGitUrl] = useState(app.custom_git_url || '');
	const [gitBranch, setGitBranch] = useState(app.custom_git_branch || 'main');
	const [gitBuildPath, setGitBuildPath] = useState(app.custom_git_build_path || '/');
	const [dockerImage, setDockerImage] = useState(app.docker_image || '');
	const [registryUrl, setRegistryUrl] = useState(app.registry_url || '');
	const [dockerUsername, setDockerUsername] = useState(app.docker_username || '');
	const [dockerPassword, setDockerPassword] = useState(app.docker_password || '');
	const [gitSshKeyId, setGitSshKeyId] = useState<number | undefined>(app.custom_git_ssh_key_id || undefined);
	
	const [repoOwner, setRepoOwner] = useState(app.owner || app.gitlab_owner || app.gitea_owner || app.bitbucket_owner || '');
	const [repoName, setRepoName] = useState(app.repository || app.gitlab_repository || app.gitea_repository || app.bitbucket_repository || '');

	const [savingSource, setSavingSource] = useState(false);

	const sshKeys = useAppStore((state) => state.sshKeys || []);

	const patchDocker = $api.useMutation('patch', '/applications/{id}/source/docker');
	const patchGit = $api.useMutation('patch', '/applications/{id}/source/git');
	const patchGithub = $api.useMutation('patch', '/applications/{id}/source/github');
	const patchGitlab = $api.useMutation('patch', '/applications/{id}/source/gitlab');
	const patchGitea = $api.useMutation('patch', '/applications/{id}/source/gitea');
	const patchBitbucket = $api.useMutation('patch', '/applications/{id}/source/bitbucket');

	useEffect(() => {
		if (app) {
			setProvider(app.source_type || 'GIT');
			setGitUrl(app.custom_git_url || '');
			setGitBranch(app.custom_git_branch || app.branch || app.gitlab_branch || app.gitea_branch || app.bitbucket_branch || 'main');
			setGitBuildPath(app.custom_git_build_path || app.build_path || app.gitlab_build_path || app.gitea_build_path || '/');
			setDockerImage(app.docker_image || '');
			setRegistryUrl(app.registry_url || '');
			setDockerUsername(app.docker_username || '');
			setDockerPassword(app.docker_password || '');
			setGitSshKeyId(app.custom_git_ssh_key_id || undefined);
			setRepoOwner(app.owner || app.gitlab_owner || app.gitea_owner || app.bitbucket_owner || '');
			setRepoName(app.repository || app.gitlab_repository || app.gitea_repository || app.bitbucket_repository || '');
		}
	}, [app]);

	const handleSaveSource = async () => {
		setSavingSource(true);
		try {
			if (provider === 'DOCKER') {
				await patchDocker.mutateAsync({
					params: {path: {id: app.id}},
					body: {
						docker_image: dockerImage,
						docker_username: dockerUsername || undefined,
						docker_password: dockerPassword || undefined,
						registry_url: registryUrl || undefined,
					},
				});
			} else if (provider === 'GIT') {
				await patchGit.mutateAsync({
					params: {path: {id: app.id}},
					body: {
						custom_git_url: gitUrl,
						custom_git_branch: gitBranch || 'main',
						custom_git_build_path: gitBuildPath || '/',
						custom_git_ssh_key_id: gitSshKeyId || undefined,
					} as any,
				});
			} else if (provider === 'GITHUB') {
				await patchGithub.mutateAsync({
					params: {path: {id: app.id}},
					body: {
						owner: repoOwner,
						repository: repoName,
						branch: gitBranch,
						build_path: gitBuildPath,
					} as any,
				});
			} else if (provider === 'GITLAB') {
				await patchGitlab.mutateAsync({
					params: {path: {id: app.id}},
					body: {
						gitlab_owner: repoOwner,
						gitlab_repository: repoName,
						gitlab_branch: gitBranch,
						gitlab_build_path: gitBuildPath,
					} as any,
				});
			} else if (provider === 'GITEA') {
				await patchGitea.mutateAsync({
					params: {path: {id: app.id}},
					body: {
						gitea_owner: repoOwner,
						gitea_repository: repoName,
						gitea_branch: gitBranch,
						gitea_build_path: gitBuildPath,
					} as any,
				});
			} else if (provider === 'BITBUCKET') {
				await patchBitbucket.mutateAsync({
					params: {path: {id: app.id}},
					body: {
						bitbucket_owner: repoOwner,
						bitbucket_repository: repoName,
						bitbucket_branch: gitBranch,
					} as any,
				});
			}
			toast.success('Source configuration saved');
			onUpdated();
		} catch (err: any) {
			toast.error(formatApiError(err));
		} finally {
			setSavingSource(false);
		}
	};

	return (
		<section className="bg-card border border-border rounded-xl p-5 flex flex-col gap-4">
			<div>
				<h3 className="text-sm font-bold text-foreground">Provider Source</h3>
				<p className="text-xs text-muted-foreground mt-1">Select and configure the repository source code of your application</p>
			</div>

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
							}`}
						>
							{p.icon}
							{p.label}
						</button>
					);
				})}
			</div>

			{provider === 'DROP' ? (
				<DropSourceForm app={app} onUpdated={onUpdated} />
			) : (
				<>
					{provider === 'DOCKER' ? (
						<div className="grid grid-cols-1 md:grid-cols-2 gap-4">
							<div className="flex flex-col gap-1.5 col-span-1 md:col-span-2">
								<span className="text-xs font-bold text-foreground">Docker Image</span>
								<Input placeholder="node:16" value={dockerImage} onChange={e => setDockerImage(e.target.value)} className="bg-card border-border font-mono text-xs h-9" />
							</div>
							<div className="flex flex-col gap-1.5 col-span-1 md:col-span-2">
								<span className="text-xs font-bold text-foreground">Registry URL</span>
								<Input placeholder="Registry URL (Optional)" value={registryUrl} onChange={e => setRegistryUrl(e.target.value)} className="bg-card border-border text-xs font-mono h-9" />
							</div>
							<div className="flex flex-col gap-1.5">
								<span className="text-xs font-bold text-foreground">Username</span>
								<Input placeholder="Username (Optional)" value={dockerUsername} onChange={e => setDockerUsername(e.target.value)} className="bg-card border-border text-xs h-9" />
							</div>
							<div className="flex flex-col gap-1.5">
								<span className="text-xs font-bold text-foreground">Password</span>
								<Input type="password" placeholder="Password (Optional)" value={dockerPassword} onChange={e => setDockerPassword(e.target.value)} className="bg-card border-border text-xs h-9" />
							</div>
						</div>
					) : provider === 'GIT' ? (
						<div className="grid grid-cols-1 md:grid-cols-4 gap-4 items-end">
							<div className="flex flex-col gap-1.5 md:col-span-3">
								<span className="text-xs font-bold text-foreground">Repository URL</span>
								<Input placeholder="https://github.com/username/repo.git" value={gitUrl} onChange={e => setGitUrl(e.target.value)} className="bg-card border-border font-mono text-xs h-9" />
							</div>
							<div className="flex flex-col gap-1.5 md:col-span-1">
								<span className="text-xs font-bold text-foreground">SSH Key</span>
								{(() => {
									const selectedKey = sshKeys?.find((k: any) => Number(k.id) === Number(gitSshKeyId));
									const selectedName = gitSshKeyId ? (selectedKey?.name || `SSH Key #${gitSshKeyId}`) : 'None (Public Repo)';
									return (
										<Select 
											value={gitSshKeyId ? String(gitSshKeyId) : 'none'} 
											onValueChange={(val) => setGitSshKeyId(val === 'none' ? undefined : Number(val))}
										>
											<SelectTrigger className="w-full bg-card border-border text-xs h-9">
												<SelectValue placeholder="No SSH Key Selected">
													{selectedName}
												</SelectValue>
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
									);
								})()}
							</div>
							<div className="flex flex-col gap-1.5 md:col-span-2">
								<span className="text-xs font-bold text-foreground">Branch</span>
								<Input placeholder="main" value={gitBranch} onChange={e => setGitBranch(e.target.value)} className="bg-card border-border text-xs h-9" />
							</div>
							<div className="flex flex-col gap-1.5 md:col-span-2">
								<span className="text-xs font-bold text-foreground">Build Path</span>
								<Input placeholder="/" value={gitBuildPath} onChange={e => setGitBuildPath(e.target.value)} className="bg-card border-border text-xs h-9" />
							</div>
						</div>
					) : (
						<div className="grid grid-cols-1 md:grid-cols-4 gap-4 items-end">
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
								<Input placeholder="/" value={gitBuildPath} onChange={e => setGitBuildPath(e.target.value)} className="bg-card border-border text-xs h-9" />
							</div>
						</div>
					)}

					<div className="flex justify-end mt-2">
						<Button onClick={handleSaveSource} disabled={savingSource} className="bg-primary hover:bg-primary/95 text-primary-foreground font-semibold flex items-center gap-1.5 h-9 rounded-lg text-xs">
							<Save className="w-3.5 h-3.5" /> {savingSource ? 'Saving...' : 'Save Source'}
						</Button>
					</div>
				</>
			)}
		</section>
	);
}
