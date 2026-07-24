import {Save, Code2} from 'lucide-react';
import Editor from '@monaco-editor/react';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {Select, SelectContent, SelectItem, SelectTrigger, SelectValue} from '#/components/ui/select';
import {
	GithubIcon,
	GitlabIcon,
	BitbucketIcon,
	GiteaIcon,
	GitIcon,
} from '#/components/icons/data-tools-icons';

interface ComposeSourceCardProps {
	provider: string;
	setProvider: (p: string) => void;
	composeFile: string;
	setComposeFile: (v: string) => void;
	command: string;
	setCommand: (v: string) => void;
	repoOwner: string;
	setRepoOwner: (v: string) => void;
	repoName: string;
	setRepoName: (v: string) => void;
	gitUrl: string;
	setGitUrl: (v: string) => void;
	gitBranch: string;
	setGitBranch: (v: string) => void;
	gitBuildPath: string;
	setGitBuildPath: (v: string) => void;
	gitSshKeyId: number | undefined;
	setGitSshKeyId: (v: number | undefined) => void;
	sshKeys: any[] | undefined;
	savingSource: boolean;
	onSave: () => void;
}

export function ComposeSourceCard({
	provider,
	setProvider,
	composeFile,
	setComposeFile,
	command,
	setCommand,
	repoOwner,
	setRepoOwner,
	repoName,
	setRepoName,
	gitUrl,
	setGitUrl,
	gitBranch,
	setGitBranch,
	gitBuildPath,
	setGitBuildPath,
	gitSshKeyId,
	setGitSshKeyId,
	sshKeys,
	savingSource,
	onSave,
}: ComposeSourceCardProps) {
	const PROVIDERS = [
		{id: 'GITHUB', label: 'GitHub', icon: <GithubIcon className="w-4 h-4" />},
		{id: 'GITLAB', label: 'GitLab', icon: <GitlabIcon className="w-4.5 h-4.5" />},
		{id: 'GITEA', label: 'Gitea', icon: <GiteaIcon className="w-4.5 h-4.5" />},
		{id: 'BITBUCKET', label: 'Bitbucket', icon: <BitbucketIcon className="w-4 h-4" />},
		{id: 'GIT', label: 'Git', icon: <GitIcon className="w-4.5 h-4.5" />},
		{id: 'RAW', label: 'Raw YML', icon: <Code2 className="w-4 h-4 text-primary" />},
	] as const;

	return (
		<section className="bg-card border border-border rounded-xl p-5 flex flex-col gap-5 shadow-sm">
			<div className="flex items-center justify-between border-b border-border/40 pb-4">
				<h3 className="text-sm font-bold text-foreground">Compose Source & Provider Configuration</h3>
				<Button size="sm" onClick={onSave} disabled={savingSource} className="h-8 text-xs font-semibold flex items-center gap-1.5">
					<Save className="w-3.5 h-3.5" /> Save Source Settings
				</Button>
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

			{provider === 'RAW' ? (
				<div className="flex flex-col gap-3">
					<div className="border border-border/80 rounded-xl overflow-hidden shadow-inner bg-zinc-950">
						<Editor
							height="360px"
							defaultLanguage="yaml"
							theme="vs-dark"
							value={composeFile}
							onChange={val => setComposeFile(val || '')}
							options={{
								minimap: {enabled: false},
								fontSize: 12,
								scrollBeyondLastLine: false,
								lineNumbers: 'on',
							}}
						/>
					</div>
					<div className="flex flex-col gap-1">
						<label className="text-xs font-semibold text-foreground">Compose Command Override (Optional)</label>
						<Input
							value={command}
							onChange={e => setCommand(e.target.value)}
							placeholder="docker compose up -d --build"
							className="h-9 text-xs font-mono"
						/>
					</div>
				</div>
			) : provider === 'GIT' ? (
				<div className="grid grid-cols-1 md:grid-cols-3 gap-4 items-end">
					<div className="flex flex-col gap-1.5 md:col-span-2">
						<label className="text-xs font-semibold text-foreground">Repository URL *</label>
						<Input value={gitUrl} onChange={e => setGitUrl(e.target.value)} placeholder="https://github.com/user/repo.git or git@github.com:user/repo.git" className="h-9 text-xs font-mono" />
					</div>

					<div className="flex flex-col gap-1.5 md:col-span-1">
						<label className="text-xs font-semibold text-foreground">SSH Key (Optional for private repos)</label>
						{(() => {
							const selectedKey = sshKeys?.find((k: any) => Number(k.id) === Number(gitSshKeyId));
							const selectedName = gitSshKeyId ? (selectedKey?.name || `SSH Key #${gitSshKeyId}`) : 'None (Public Repo)';
							return (
								<Select
									value={gitSshKeyId ? String(gitSshKeyId) : 'none'}
									onValueChange={v => setGitSshKeyId(v === 'none' ? undefined : Number(v))}
								>
									<SelectTrigger className="w-full h-9 text-xs font-sans bg-card border-border">
										<SelectValue placeholder="No SSH Key Selected">
											{selectedName}
										</SelectValue>
									</SelectTrigger>
									<SelectContent>
										<SelectItem value="none" className="text-xs font-sans">None (Public Repo)</SelectItem>
										{sshKeys?.map((key: any) => (
											<SelectItem key={key.id} value={String(key.id)} className="text-xs font-sans">
												{key.name}
											</SelectItem>
										))}
									</SelectContent>
								</Select>
							);
						})()}
					</div>

					<div className="flex flex-col gap-1.5 md:col-span-1">
						<label className="text-xs font-semibold text-foreground">Git Branch *</label>
						<Input value={gitBranch} onChange={e => setGitBranch(e.target.value)} placeholder="main" className="h-9 text-xs font-mono" />
					</div>

					<div className="flex flex-col gap-1.5 md:col-span-2">
						<label className="text-xs font-semibold text-foreground">Compose File Path *</label>
						<Input value={gitBuildPath} onChange={e => setGitBuildPath(e.target.value)} placeholder="docker-compose.yml" className="h-9 text-xs font-mono" />
					</div>
				</div>
			) : (
				<div className="grid grid-cols-1 md:grid-cols-4 gap-4 items-end">
					<div className="flex flex-col gap-1.5 md:col-span-2">
						<label className="text-xs font-semibold text-foreground">Owner / Username *</label>
						<Input value={repoOwner} onChange={e => setRepoOwner(e.target.value)} placeholder="e.g. ananddub" className="h-9 text-xs font-mono" />
					</div>
					<div className="flex flex-col gap-1.5 md:col-span-2">
						<label className="text-xs font-semibold text-foreground">Repository Name *</label>
						<Input value={repoName} onChange={e => setRepoName(e.target.value)} placeholder="e.g. sample-node-project" className="h-9 text-xs font-mono" />
					</div>

					<div className="flex flex-col gap-1.5 md:col-span-2">
						<label className="text-xs font-semibold text-foreground">Git Branch *</label>
						<Input value={gitBranch} onChange={e => setGitBranch(e.target.value)} placeholder="main" className="h-9 text-xs font-mono" />
					</div>
					<div className="flex flex-col gap-1.5 md:col-span-2">
						<label className="text-xs font-semibold text-foreground">Compose File Path *</label>
						<Input value={gitBuildPath} onChange={e => setGitBuildPath(e.target.value)} placeholder="docker-compose.yml" className="h-9 text-xs font-mono" />
					</div>
				</div>
			)}
		</section>
	);
}
