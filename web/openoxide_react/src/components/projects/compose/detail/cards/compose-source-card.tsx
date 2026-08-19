import {Save, Code2} from 'lucide-react';
import Editor from '@monaco-editor/react';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
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
		{
			id: 'GITHUB',
			label: 'GitHub',
			icon: <GithubIcon className="h-4 w-4" />,
		},
		{
			id: 'GITLAB',
			label: 'GitLab',
			icon: <GitlabIcon className="h-4.5 w-4.5" />,
		},
		{
			id: 'GITEA',
			label: 'Gitea',
			icon: <GiteaIcon className="h-4.5 w-4.5" />,
		},
		{
			id: 'BITBUCKET',
			label: 'Bitbucket',
			icon: <BitbucketIcon className="h-4 w-4" />,
		},
		{id: 'GIT', label: 'Git', icon: <GitIcon className="h-4.5 w-4.5" />},
		{
			id: 'RAW',
			label: 'Raw YML',
			icon: <Code2 className="h-4 w-4 text-primary" />,
		},
	] as const;

	return (
		<section className="flex flex-col gap-5 rounded-xl border border-border bg-card p-5 shadow-sm">
			<div className="flex items-center justify-between border-b border-border/40 pb-4">
				<h3 className="text-sm font-bold text-foreground">
					Compose Source & Provider Configuration
				</h3>
				<Button
					size="sm"
					onClick={onSave}
					disabled={savingSource}
					className="flex h-8 items-center gap-1.5 text-xs font-semibold">
					<Save className="h-3.5 w-3.5" /> Save Source Settings
				</Button>
			</div>

			<div className="relative -mb-[1px] flex w-full flex-wrap gap-1 border-b border-border/60 pb-1">
				{PROVIDERS.map(p => {
					const isActive = provider === p.id;
					return (
						<button
							key={p.id}
							type="button"
							onClick={() => setProvider(p.id)}
							className={`-mb-[1px] flex cursor-pointer items-center gap-1.5 border-b-2 px-3.5 pt-2 pb-2 text-xs font-bold transition-all ${
								isActive
									? 'border-foreground font-bold text-foreground'
									: 'border-transparent text-muted-foreground hover:border-border/40 hover:text-foreground'
							}`}>
							{p.icon}
							{p.label}
						</button>
					);
				})}
			</div>

			{provider === 'RAW' ? (
				<div className="flex flex-col gap-3">
					<div className="overflow-hidden rounded-xl border border-border/80 bg-zinc-950 shadow-inner">
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
						<label className="text-xs font-semibold text-foreground">
							Compose Command Override (Optional)
						</label>
						<Input
							value={command}
							onChange={e => setCommand(e.target.value)}
							placeholder="docker compose up -d --build"
							className="h-9 font-mono text-xs"
						/>
					</div>
				</div>
			) : provider === 'GIT' ? (
				<div className="grid grid-cols-1 items-end gap-4 md:grid-cols-3">
					<div className="flex flex-col gap-1.5 md:col-span-2">
						<label className="text-xs font-semibold text-foreground">
							Repository URL *
						</label>
						<Input
							value={gitUrl}
							onChange={e => setGitUrl(e.target.value)}
							placeholder="https://github.com/user/repo.git or git@github.com:user/repo.git"
							className="h-9 font-mono text-xs"
						/>
					</div>

					<div className="flex flex-col gap-1.5 md:col-span-1">
						<label className="text-xs font-semibold text-foreground">
							SSH Key (Optional for private repos)
						</label>
						{(() => {
							const selectedKey = sshKeys?.find(
								(k: any) => Number(k.id) === Number(gitSshKeyId),
							);
							const selectedName = gitSshKeyId
								? selectedKey?.name || `SSH Key #${gitSshKeyId}`
								: 'None (Public Repo)';
							return (
								<Select
									value={gitSshKeyId ? String(gitSshKeyId) : 'none'}
									onValueChange={v =>
										setGitSshKeyId(v === 'none' ? undefined : Number(v))
									}>
									<SelectTrigger className="h-9 w-full border-border bg-card font-sans text-xs">
										<SelectValue placeholder="No SSH Key Selected">
											{selectedName}
										</SelectValue>
									</SelectTrigger>
									<SelectContent>
										<SelectItem value="none" className="font-sans text-xs">
											None (Public Repo)
										</SelectItem>
										{sshKeys?.map((key: any) => (
											<SelectItem
												key={key.id}
												value={String(key.id)}
												className="font-sans text-xs">
												{key.name}
											</SelectItem>
										))}
									</SelectContent>
								</Select>
							);
						})()}
					</div>

					<div className="flex flex-col gap-1.5 md:col-span-1">
						<label className="text-xs font-semibold text-foreground">
							Git Branch *
						</label>
						<Input
							value={gitBranch}
							onChange={e => setGitBranch(e.target.value)}
							placeholder="main"
							className="h-9 font-mono text-xs"
						/>
					</div>

					<div className="flex flex-col gap-1.5 md:col-span-2">
						<label className="text-xs font-semibold text-foreground">
							Compose File Path *
						</label>
						<Input
							value={gitBuildPath}
							onChange={e => setGitBuildPath(e.target.value)}
							placeholder="docker-compose.yml"
							className="h-9 font-mono text-xs"
						/>
					</div>
				</div>
			) : (
				<div className="grid grid-cols-1 items-end gap-4 md:grid-cols-4">
					<div className="flex flex-col gap-1.5 md:col-span-2">
						<label className="text-xs font-semibold text-foreground">
							Owner / Username *
						</label>
						<Input
							value={repoOwner}
							onChange={e => setRepoOwner(e.target.value)}
							placeholder="e.g. ananddub"
							className="h-9 font-mono text-xs"
						/>
					</div>
					<div className="flex flex-col gap-1.5 md:col-span-2">
						<label className="text-xs font-semibold text-foreground">
							Repository Name *
						</label>
						<Input
							value={repoName}
							onChange={e => setRepoName(e.target.value)}
							placeholder="e.g. sample-node-project"
							className="h-9 font-mono text-xs"
						/>
					</div>

					<div className="flex flex-col gap-1.5 md:col-span-2">
						<label className="text-xs font-semibold text-foreground">
							Git Branch *
						</label>
						<Input
							value={gitBranch}
							onChange={e => setGitBranch(e.target.value)}
							placeholder="main"
							className="h-9 font-mono text-xs"
						/>
					</div>
					<div className="flex flex-col gap-1.5 md:col-span-2">
						<label className="text-xs font-semibold text-foreground">
							Compose File Path *
						</label>
						<Input
							value={gitBuildPath}
							onChange={e => setGitBuildPath(e.target.value)}
							placeholder="docker-compose.yml"
							className="h-9 font-mono text-xs"
						/>
					</div>

					{/* Collapsible / Optional YAML Code editor for previewing graph */}
					<div className="flex flex-col gap-2 border-t border-border/40 pt-2 md:col-span-4">
						<details className="group">
							<summary className="flex cursor-pointer items-center gap-1.5 py-1 text-xs font-semibold text-muted-foreground select-none hover:text-foreground">
								<Code2 className="size-3.5 text-primary" />
								<span>
									Compose YAML Content (Paste code here to preview
									Dependency Graph without building)
								</span>
							</summary>
							<div className="flex flex-col gap-2 pt-3">
								<div className="overflow-hidden rounded-xl border border-border/80 bg-zinc-950 shadow-inner">
									<Editor
										height="240px"
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
								<p className="text-[11px] text-muted-foreground italic">
									Pasting your docker-compose.yml content here instantly
									renders the service graph below.
								</p>
							</div>
						</details>
					</div>
				</div>
			)}
		</section>
	);
}
