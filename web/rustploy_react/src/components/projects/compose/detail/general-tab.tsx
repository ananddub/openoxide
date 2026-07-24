import {useState, useEffect} from 'react';
import Editor from '@monaco-editor/react';
import {Save, Code2} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {toast} from 'sonner';
import {$api} from '#/api/query';
import {formatApiError} from '#/api/utils';
import {DeploySettingsCard} from '#/components/projects/app/detail/general-tab/deploy-settings-card';
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
			{/* Shared Deploy Settings Card — Exact same card component as Application! */}
			<DeploySettingsCard app={compose} handleAction={onAction} onUpdated={onUpdated} />

			{/* Source Settings Provider Card */}
			<section className="bg-card border border-border rounded-xl p-5 flex flex-col gap-5 shadow-sm">
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
