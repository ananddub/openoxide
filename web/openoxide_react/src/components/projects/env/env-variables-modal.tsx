import {useProjectUpdate} from '#/api/live/projects';
import {useEnvironmentUpdate} from '#/api/live/environments';
import {useState, useEffect} from 'react';
import {toast} from 'sonner';
import {Info, Save} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {
	Sheet,
	SheetContent,
	SheetDescription,
	SheetFooter,
	SheetHeader,
	SheetTitle,
} from '#/components/ui/sheet';
import Editor from '@monaco-editor/react';
import {formatApiError} from '#/api/utils';
import {useTheme} from '#/hooks/use-theme';

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
	const {theme} = useTheme();
	const [projectEnv, setProjectEnv] = useState(project.env_var || '');
	const [envVars, setEnvVars] = useState(environment?.env_var || '');
	const [isSubmitting, setIsSubmitting] = useState(false);

	const patchProjectMutation = useProjectUpdate();
	const patchEnvironmentMutation = useEnvironmentUpdate();

	useEffect(() => {
		setProjectEnv(project.env_var || '');
		setEnvVars(environment?.env_var || '');
	}, [project, environment, isOpen]);

	const isModified =
		mode === 'PROJECT'
			? projectEnv !== (project.env_var || '')
			: envVars !== (environment?.env_var || '');

	const handleSubmit = async (e: React.FormEvent) => {
		e.preventDefault();
		setIsSubmitting(true);
		try {
			if (mode === 'PROJECT') {
				await patchProjectMutation.mutateAsync({
					id: project.id,
					environment_variables: projectEnv,
				});
				toast.success('Project global environment variables updated');
			} else if (environment) {
				await patchEnvironmentMutation.mutateAsync({
					id: environment.id,
					environment_variables: envVars,
				});
				toast.success(
					`Environment variables updated for ${environment.name}`,
				);
			}
			onUpdated();
			onClose();
		} catch (err: any) {
			toast.error(formatApiError(err));
		} finally {
			setIsSubmitting(false);
		}
	};

	const title =
		mode === 'PROJECT'
			? 'Project Environment Variables'
			: `${environment?.name || 'Environment'} Variables`;

	const description =
		mode === 'PROJECT'
			? 'Configure global variables accessible by all environments inside this project.'
			: `Configure variables specific to the ${environment?.name || 'selected'} environment.`;

	const filename =
		mode === 'PROJECT'
			? 'project.env'
			: `${environment?.name?.toLowerCase() || 'env'}.env`;

	return (
		<Sheet open={isOpen} onOpenChange={open => !open && onClose()}>
			<SheetContent>
				<SheetHeader>
					<SheetTitle>{title}</SheetTitle>
					<SheetDescription>{description}</SheetDescription>
				</SheetHeader>

				<form
					id="env-variables-sheet-form"
					onSubmit={handleSubmit}
					className="flex min-h-0 flex-1 flex-col gap-4 px-6 pt-6 pb-6">
					{/* Info syntax tip */}
					<div className="flex items-start gap-2.5 rounded-lg bg-muted/40 p-3">
						<Info className="mt-0.5 size-4 shrink-0 text-primary" />
						<div className="text-xs leading-relaxed text-muted-foreground">
							{mode === 'PROJECT' ? (
								<span>
									Define global key-value environment variables. One
									variable definition per line.
								</span>
							) : (
								<span>
									Reference project-level variables using{' '}
									<code className="rounded border border-border/60 bg-muted/80 px-1 py-px font-mono text-[10.5px] font-medium text-foreground">
										{'{{ project.DB_URL }}'}
									</code>
								</span>
							)}
						</div>
					</div>

					<div className="flex min-h-0 flex-1 flex-col gap-2">
						<div className="flex items-center justify-between">
							<span className="font-mono text-xs font-bold tracking-wider text-muted-foreground uppercase">
								{filename}
							</span>
							{isModified && (
								<span className="rounded bg-amber-500/5 px-2 py-0.5 text-[10px] font-semibold text-amber-500">
									Unsaved Changes
								</span>
							)}
						</div>

						{/* Monaco Code Editor */}
						<div className="min-h-0 flex-1 overflow-hidden rounded-xl border border-border bg-white p-1 shadow-inner dark:bg-[#1e1e1e]">
							<Editor
								height="100%"
								language="ini"
								theme={theme === 'dark' ? 'vs-dark' : 'light'}
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
				</form>

				<SheetFooter>
					<Button
						type="submit"
						form="env-variables-sheet-form"
						disabled={isSubmitting || !isModified}
						className="h-10 gap-1.5 px-5">
						<Save className="size-4" />
						{isSubmitting ? 'Saving…' : 'Save'}
					</Button>
				</SheetFooter>
			</SheetContent>
		</Sheet>
	);
}
