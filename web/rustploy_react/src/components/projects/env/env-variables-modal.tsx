import {useState, useEffect} from 'react';
import {toast} from 'sonner';
import {Info, Save} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {
	Dialog,
	DialogContent,
	DialogDescription,
	DialogHeader,
	DialogTitle,
} from '#/components/ui/dialog';
import Editor from '@monaco-editor/react';
import {$api} from '#/api/query';
import {formatApiError} from '#/api/utils';

interface EnvVariablesModalProps {
	isOpen: boolean;
	onClose: () => void;
	mode: 'PROJECT' | 'ENVIRONMENT';
	project: {id: number; env_var?: string; name: string};
	environment: {id: number; env_var?: string; name: string} | null;
	onUpdated: () => void;
}

export function EnvVariablesModal({
	isOpen,
	onClose,
	mode,
	project,
	environment,
	onUpdated,
}: EnvVariablesModalProps) {
	const [projectEnv, setProjectEnv] = useState(project.env_var || '');
	const [envVars, setEnvVars] = useState(environment?.env_var || '');
	const [isSubmitting, setIsSubmitting] = useState(false);

	const patchProjectMutation = $api.useMutation('patch', '/projects/{id}');
	const patchEnvironmentMutation = $api.useMutation('patch', '/environments/{id}');

	useEffect(() => {
		setProjectEnv(project.env_var || '');
		setEnvVars(environment?.env_var || '');
	}, [project, environment, isOpen]);

	const isModified = mode === 'PROJECT'
		? projectEnv !== (project.env_var || '')
		: envVars !== (environment?.env_var || '');

	const handleSubmit = async (e: React.FormEvent) => {
		e.preventDefault();
		setIsSubmitting(true);
		try {
			if (mode === 'PROJECT') {
				await patchProjectMutation.mutateAsync({
					params: {path: {id: project.id}},
					body: {env_var: projectEnv},
				});
				toast.success('Project global environment variables updated');
			} else if (environment) {
				await patchEnvironmentMutation.mutateAsync({
					params: {path: {id: environment.id}},
					body: {env_var: envVars},
				});
				toast.success(`Environment variables updated for ${environment.name}`);
			}
			onUpdated();
			onClose();
		} catch (err: any) {
			toast.error(formatApiError(err));
		} finally {
			setIsSubmitting(false);
		}
	};

	const title = mode === 'PROJECT'
		? 'Project Environment Variables'
		: `${environment?.name || 'Environment'} Variables`;

	const description = mode === 'PROJECT'
		? 'Configure global variables accessible by all environments inside this project.'
		: `Configure variables specific to the ${environment?.name || 'selected'} environment.`;

	const filename = mode === 'PROJECT'
		? 'project.env'
		: `${environment?.name?.toLowerCase() || 'env'}.env`;

	return (
		<Dialog open={isOpen} onOpenChange={open => !open && onClose()}>
			<DialogContent className="sm:max-w-xl bg-card border border-border shadow-2xl p-6 flex flex-col gap-5 rounded-2xl">
				<DialogHeader className="space-y-1.5">
					<DialogTitle className="text-lg font-bold tracking-tight text-foreground">
						{title}
					</DialogTitle>
					<DialogDescription className="text-xs text-muted-foreground">
						{description}
					</DialogDescription>
				</DialogHeader>

				{/* Info syntax tip */}
				<div className="flex items-start gap-2.5 bg-muted/40 border border-border/55 rounded-lg p-3">
					<Info className="size-4 text-primary shrink-0 mt-0.5" />
					<div className="text-xs text-muted-foreground leading-relaxed">
						{mode === 'PROJECT' ? (
							<span>Define global key-value environment variables. One variable definition per line.</span>
						) : (
							<span>
								Reference project-level variables using:
								<code className="font-mono text-foreground font-semibold bg-muted/80 border border-border px-1 py-0.5 rounded ml-1.5">
									{"DB_URL={{ project.DB_URL }}"}
								</code>
							</span>
						)}
					</div>
				</div>

				<form onSubmit={handleSubmit} className="flex flex-col gap-4">
					<div className="flex flex-col gap-2">
						<div className="flex items-center justify-between">
							<span className="text-xs font-bold text-muted-foreground uppercase tracking-wider font-mono">
								{filename}
							</span>
							{isModified && (
								<span className="text-[10px] font-semibold text-amber-500 bg-amber-500/5 px-2 py-0.5 rounded border border-amber-500/20">
									Unsaved Changes
								</span>
							)}
						</div>

						{/* Monaco Code Editor */}
						<div className="rounded-xl border border-border overflow-hidden bg-[#1e1e1e] p-1 shadow-inner">
							<Editor
								height="320px"
								language="ini"
								theme="vs-dark"
								value={mode === 'PROJECT' ? projectEnv : envVars}
								onChange={val => {
									if (mode === 'PROJECT') {
										setProjectEnv(val || '');
									} else {
										setEnvVars(val || '');
									}
								}}
								options={{
									minimap: {enabled: false},
									lineNumbers: 'on',
									wordWrap: 'on',
									fontFamily: 'monospace',
									fontSize: 12.5,
									automaticLayout: true,
									scrollBeyondLastLine: false,
									tabSize: 2,
								}}
							/>
						</div>
					</div>

					{/* Modal Actions */}
					<div className="flex justify-end pt-3 border-t border-border/30">
						<Button
							type="submit"
							disabled={isSubmitting}
							className="bg-primary hover:bg-primary/95 text-primary-foreground text-xs h-9 px-4 font-semibold flex items-center gap-1.5 shadow-lg shadow-primary/10">
							<Save className="size-3.5" />
							{isSubmitting ? 'Saving...' : 'Save Configuration'}
						</Button>
					</div>
				</form>
			</DialogContent>
		</Dialog>
	);
}
