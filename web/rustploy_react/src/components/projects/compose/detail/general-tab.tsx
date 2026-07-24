import {useState, useEffect} from 'react';
import Editor from '@monaco-editor/react';
import {Play, RotateCw, Square, RefreshCw, Save, Code2, Upload, GitBranch} from 'lucide-react';
import {GithubIcon} from '#/components/icons/data-tools-icons';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {toast} from 'sonner';
import {$api} from '#/api/query';
import {formatApiError} from '#/api/utils';

interface GeneralTabProps {
	compose: any;
	onAction: (action: 'deploy' | 'redeploy' | 'start' | 'stop' | 'reload' | 'cancel') => void;
	onUpdated: () => void;
}

export function ComposeGeneralTab({compose, onAction, onUpdated}: GeneralTabProps) {
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
			toast.success('Compose configuration saved');
			onUpdated();
		} catch (err: any) {
			toast.error(formatApiError(err));
		} finally {
			setSaving(false);
		}
	};

	return (
		<div className="flex flex-col gap-6 w-full">
			{/* Action bar */}
			<div className="bg-card border border-border/80 rounded-xl p-4 flex flex-wrap items-center justify-between gap-3 shadow-sm">
				<div className="flex items-center gap-2">
					<Button
						onClick={() => onAction('deploy')}
						className="bg-primary hover:bg-primary/90 text-primary-foreground font-semibold text-xs h-9 px-4 rounded-lg flex items-center gap-1.5 shadow-md shadow-primary/10">
						<Play className="w-3.5 h-3.5 fill-current" /> Deploy Stack
					</Button>
					<Button
						onClick={() => onAction('redeploy')}
						variant="outline"
						className="border-border text-foreground hover:bg-muted text-xs h-9 px-3.5 rounded-lg flex items-center gap-1.5 font-semibold">
						<RotateCw className="w-3.5 h-3.5" /> Redeploy
					</Button>
					<Button
						onClick={() => onAction('start')}
						variant="outline"
						className="border-emerald-500/30 text-emerald-500 hover:bg-emerald-500/10 text-xs h-9 px-3 rounded-lg font-semibold flex items-center gap-1">
						Start
					</Button>
					<Button
						onClick={() => onAction('stop')}
						variant="outline"
						className="border-rose-500/30 text-rose-500 hover:bg-rose-500/10 text-xs h-9 px-3 rounded-lg font-semibold flex items-center gap-1">
						<Square className="w-3 h-3 fill-current" /> Stop
					</Button>
					<Button
						onClick={() => onAction('reload')}
						variant="outline"
						className="border-border text-muted-foreground hover:text-foreground text-xs h-9 px-3 rounded-lg font-semibold flex items-center gap-1">
						<RefreshCw className="w-3.5 h-3.5" /> Reload
					</Button>
				</div>
			</div>

			{/* Source selection */}
			<section className="bg-card border border-border rounded-xl p-5 flex flex-col gap-4">
				<div className="flex items-center justify-between">
					<div>
						<h3 className="text-sm font-bold text-foreground">Compose Source Provider</h3>
						<p className="text-xs text-muted-foreground mt-0.5">Select how your Docker Compose file is specified.</p>
					</div>
					<div className="flex items-center gap-1 bg-muted/40 p-1 rounded-lg border border-border/50">
						<button
							onClick={() => setSourceType('RAW')}
							className={`px-3 py-1.5 rounded-md text-xs font-semibold flex items-center gap-1.5 transition-all ${
								sourceType === 'RAW' ? 'bg-background text-foreground shadow-sm' : 'text-muted-foreground hover:text-foreground'
							}`}>
							<Code2 className="w-3.5 h-3.5" /> Raw YAML
						</button>
						<button
							onClick={() => setSourceType('GITHUB')}
							className={`px-3 py-1.5 rounded-md text-xs font-semibold flex items-center gap-1.5 transition-all ${
								sourceType === 'GITHUB' ? 'bg-background text-foreground shadow-sm' : 'text-muted-foreground hover:text-foreground'
							}`}>
							<GithubIcon className="w-3.5 h-3.5" /> GitHub
						</button>
					</div>
				</div>

				{sourceType === 'RAW' ? (
					<div className="flex flex-col gap-3">
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

				<div className="flex justify-end pt-2 border-t border-border/40">
					<Button
						onClick={handleSaveConfig}
						disabled={saving}
						className="bg-primary hover:bg-primary/95 text-primary-foreground font-semibold text-xs h-9 px-4 rounded-lg flex items-center gap-1.5">
						<Save className="w-3.5 h-3.5" /> {saving ? 'Saving...' : 'Save Configuration'}
					</Button>
				</div>
			</section>
		</div>
	);
}
