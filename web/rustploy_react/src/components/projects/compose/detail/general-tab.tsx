import {useState, useEffect} from 'react';
import Editor from '@monaco-editor/react';
import {
	Rocket,
	RotateCw,
	Play,
	Square,
	RefreshCw,
	Save,
	Code2,
	UploadCloud,
} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {toast} from 'sonner';
import {$api} from '#/api/query';
import {formatApiError} from '#/api/utils';
import {
	GithubIcon,
	GitlabIcon,
	BitbucketIcon,
	GiteaIcon,
	GitIcon,
} from '#/components/icons/data-tools-icons';

interface GeneralTabProps {
	compose: any;
	onAction: (action: 'deploy' | 'redeploy' | 'start' | 'stop' | 'reload' | 'cancel') => void;
	onUpdated: () => void;
}

export function ComposeGeneralTab({compose, onAction, onUpdated}: GeneralTabProps) {
	const PROVIDERS = [
		{id: 'RAW', label: 'Raw YML', icon: <Code2 className="w-4 h-4 text-primary" />},
		{id: 'GITHUB', label: 'GitHub', icon: <GithubIcon className="w-4 h-4" />},
		{id: 'GITLAB', label: 'GitLab', icon: <GitlabIcon className="w-4.5 h-4.5" />},
		{id: 'GITEA', label: 'Gitea', icon: <GiteaIcon className="w-4.5 h-4.5" />},
		{id: 'BITBUCKET', label: 'Bitbucket', icon: <BitbucketIcon className="w-4 h-4" />},
		{id: 'GIT', label: 'Git', icon: <GitIcon className="w-4.5 h-4.5" />},
	] as const;

	const [sourceType, setSourceType] = useState<string>(compose?.source_type || 'RAW');
	const [composeFile, setComposeFile] = useState<string>(compose?.compose_file || '');
	const [command, setCommand] = useState<string>(compose?.command || '');
	const [repoUrl, setRepoUrl] = useState<string>(compose?.repository || '');
	const [branch, setBranch] = useState<string>(compose?.branch || 'main');
	const [buildPath, setBuildPath] = useState<string>(compose?.build_path || 'docker-compose.yml');
	const [saving, setSaving] = useState(false);

	const patchRawMutation = $api.useMutation('patch', '/compose/{id}/source/raw');
	const patchGithubMutation = $api.useMutation('patch', '/compose/{id}/source/github');

	useEffect(() => {
		if (compose) {
			setSourceType(compose.source_type || 'RAW');
			setComposeFile(compose.compose_file || '');
			setCommand(compose.command || '');
			setRepoUrl(compose.repository || '');
			setBranch(compose.branch || 'main');
			setBuildPath(compose.build_path || 'docker-compose.yml');
		}
	}, [compose]);

	const handleSaveConfig = async () => {
		setSaving(true);
		try {
			if (sourceType === 'RAW') {
				await patchRawMutation.mutateAsync({
					params: {path: {id: compose.id}},
					body: {
						compose_file: composeFile,
						command: command || undefined,
					},
				});
			} else if (sourceType === 'GITHUB') {
				await patchGithubMutation.mutateAsync({
					params: {path: {id: compose.id}},
					body: {
						repository: repoUrl,
						branch: branch,
						build_path: buildPath,
						auto_deploy: 1,
					},
				});
			}
			toast.success('Compose configuration saved successfully');
			onUpdated();
		} catch (err: any) {
			toast.error(formatApiError(err));
		} finally {
			setSaving(false);
		}
	};

	return (
		<div className="flex flex-col gap-6 w-full">
			{/* Deploy Controls Settings Card */}
			<section className="bg-card border border-border/80 rounded-xl p-5 flex flex-col gap-4 shadow-sm">
				<div className="flex flex-wrap items-center justify-between gap-3 border-b border-border/40 pb-4">
					<div className="flex items-center gap-3">
						<div className="size-9 rounded-lg bg-primary/10 border border-primary/20 flex items-center justify-center shrink-0">
							<Rocket className="w-4.5 h-4.5 text-primary" />
						</div>
						<div>
							<h3 className="text-sm font-bold text-foreground">Deploy Stack</h3>
							<p className="text-xs text-muted-foreground">Manage deployments, restarts, and runtime controls for this compose stack.</p>
						</div>
					</div>
					<div className="flex items-center gap-2">
						<span className="inline-flex items-center gap-1.5 px-2.5 py-1 rounded-full bg-emerald-500/10 border border-emerald-500/20 text-[11px] font-bold text-emerald-500">
							<span className="size-1.5 rounded-full bg-emerald-500 animate-pulse" />
							{compose?.compose_status || 'Active'}
						</span>
					</div>
				</div>

				<div className="flex flex-wrap items-center gap-2.5 pt-1">
					<Button
						onClick={() => onAction('deploy')}
						className="bg-primary hover:bg-primary/90 text-primary-foreground font-semibold text-xs h-9 px-4 rounded-lg flex items-center gap-1.5 shadow-md shadow-primary/10">
						<Rocket className="w-3.5 h-3.5" /> Deploy Stack
					</Button>
					<Button
						onClick={() => onAction('redeploy')}
						variant="outline"
						className="border-border text-foreground hover:bg-muted text-xs h-9 px-3.5 rounded-lg flex items-center gap-1.5 font-semibold">
						<RotateCw className="w-3.5 h-3.5 text-muted-foreground" /> Redeploy
					</Button>
					<Button
						onClick={() => onAction('start')}
						variant="outline"
						className="border-emerald-500/30 text-emerald-500 hover:bg-emerald-500/10 text-xs h-9 px-3.5 rounded-lg font-semibold flex items-center gap-1.5">
						<Play className="w-3.5 h-3.5 fill-current" /> Start
					</Button>
					<Button
						onClick={() => onAction('stop')}
						variant="outline"
						className="border-rose-500/30 text-rose-500 hover:bg-rose-500/10 text-xs h-9 px-3.5 rounded-lg font-semibold flex items-center gap-1.5">
						<Square className="w-3.5 h-3.5 fill-current" /> Stop
					</Button>
					<Button
						onClick={() => onAction('reload')}
						variant="outline"
						className="border-border text-muted-foreground hover:text-foreground text-xs h-9 px-3.5 rounded-lg font-semibold flex items-center gap-1.5">
						<RefreshCw className="w-3.5 h-3.5" /> Reload
					</Button>
				</div>
			</section>

			{/* Source Settings Provider Card */}
			<section className="bg-card border border-border/80 rounded-xl p-5 flex flex-col gap-5 shadow-sm">
				<div>
					<h3 className="text-sm font-bold text-foreground">Compose Source Provider</h3>
					<p className="text-xs text-muted-foreground mt-0.5">Choose your compose source: write inline Raw YML or connect to a Git repository.</p>
				</div>

				{/* Provider Tabs Bar */}
				<div className="flex flex-wrap gap-2">
					{PROVIDERS.map(p => (
						<button
							key={p.id}
							type="button"
							onClick={() => setSourceType(p.id)}
							className={`flex items-center gap-2 px-3.5 py-2 rounded-lg text-xs font-semibold border transition-all ${
								sourceType === p.id
									? 'bg-primary/10 border-primary/40 text-primary shadow-sm'
									: 'bg-muted/20 border-border/60 text-muted-foreground hover:bg-muted/40 hover:text-foreground'
							}`}>
							{p.icon}
							{p.label}
						</button>
					))}
				</div>

				{/* Source Form Content */}
				{sourceType === 'RAW' ? (
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
				) : (
					<div className="grid grid-cols-1 md:grid-cols-2 gap-4 pt-2">
						<div className="flex flex-col gap-1.5">
							<span className="text-xs font-bold text-muted-foreground">Repository URL</span>
							<Input
								placeholder="e.g. owner/repo"
								value={repoUrl}
								onChange={e => setRepoUrl(e.target.value)}
								className="bg-card border-border text-xs"
							/>
						</div>
						<div className="flex flex-col gap-1.5">
							<span className="text-xs font-bold text-muted-foreground">Branch</span>
							<Input
								placeholder="e.g. main"
								value={branch}
								onChange={e => setBranch(e.target.value)}
								className="bg-card border-border text-xs"
							/>
						</div>
						<div className="flex flex-col gap-1.5 md:col-span-2">
							<span className="text-xs font-bold text-muted-foreground">Compose File Path</span>
							<Input
								placeholder="docker-compose.yml"
								value={buildPath}
								onChange={e => setBuildPath(e.target.value)}
								className="bg-card border-border text-xs"
							/>
						</div>
					</div>
				)}

				<div className="flex justify-end pt-3 border-t border-border/40">
					<Button
						onClick={handleSaveConfig}
						disabled={saving}
						className="bg-primary hover:bg-primary/95 text-primary-foreground font-semibold text-xs h-9 px-4 rounded-lg flex items-center gap-1.5 shadow-md shadow-primary/10">
						<Save className="w-3.5 h-3.5" /> {saving ? 'Saving...' : 'Save Configuration'}
					</Button>
				</div>
			</section>
		</div>
	);
}
