import {useProjectUpdate} from '#/api/live/projects';
import {useEnvironmentUpdate} from '#/api/live/environments';
import {useState} from 'react';
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
import {formatApiError} from '#/api/utils';
import {
	EnvRowsCard,
	parseEnv,
	serializeEnv,
	type EnvRow,
} from '#/components/projects/common/env-rows-editor';

interface EnvVariablesModalProps {
	isOpen: boolean;
	onClose: () => void;
	mode: 'PROJECT' | 'ENVIRONMENT';
	project: {id: number; env_var?: string; name: string};
	environment: {id: number; env_var?: string; name: string} | null;
	onUpdated: () => void;
}

function EnvVariablesSheet({
	onClose,
	mode,
	project,
	environment,
	onUpdated,
}: EnvVariablesModalProps) {
	const source = mode === 'PROJECT' ? project : environment;
	const original = String(source?.env_var || '');
	const [rows, setRows] = useState<EnvRow[]>(() => parseEnv(original));
	const [isSubmitting, setIsSubmitting] = useState(false);

	const isModified =
		serializeEnv(rows) !== serializeEnv(parseEnv(original));

	const patchProjectMutation = useProjectUpdate();
	const patchEnvironmentMutation = useEnvironmentUpdate();

	const handleSubmit = async (e: React.FormEvent) => {
		e.preventDefault();
		setIsSubmitting(true);
		const text = serializeEnv(rows);
		try {
			if (mode === 'PROJECT') {
				await patchProjectMutation.mutateAsync({
					id: project.id,
					environment_variables: text,
				});
				toast.success('Project global environment variables updated');
			} else if (environment) {
				await patchEnvironmentMutation.mutateAsync({
					id: environment.id,
					environment_variables: text,
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

	const fileName =
		mode === 'PROJECT'
			? 'project.env'
			: `${environment?.name?.toLowerCase() || 'env'}.env`;

	return (
		<SheetContent className="gap-0 p-0 sm:data-[side=right]:w-[640px]">
			<SheetHeader className="px-5 pt-5 pb-4">
				<SheetTitle>{title}</SheetTitle>
				<SheetDescription>{description}</SheetDescription>
			</SheetHeader>

			<form
				id="env-variables-sheet-form"
				onSubmit={handleSubmit}
				className="flex min-h-0 flex-1 flex-col px-5 pt-4 pb-5">
				{/* Syntax tip — kept as the only note above the editor */}
				<div className="mb-3 flex shrink-0 items-start gap-2.5 rounded-lg bg-muted/40 p-3">
					<Info className="mt-0.5 size-4 shrink-0 text-primary" />
					<div className="text-xs leading-relaxed text-muted-foreground">
						{mode === 'PROJECT' ? (
							<span>
								Define global key-value environment variables for every
								environment in this project.
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

				<EnvRowsCard
					rows={rows}
					onRowsChange={setRows}
					fileName={fileName}
					isModified={isModified}
				/>
			</form>

			<SheetFooter className="px-5 py-4">
				<Button
					type="submit"
					form="env-variables-sheet-form"
					disabled={isSubmitting || !isModified}
					className="h-8 gap-1.5 px-4 text-xs">
					<Save className="size-3.5" />
					{isSubmitting ? 'Saving…' : 'Save'}
				</Button>
			</SheetFooter>
		</SheetContent>
	);
}

export function EnvVariablesModal(props: EnvVariablesModalProps) {
	return (
		<Sheet
			open={props.isOpen}
			onOpenChange={open => !open && props.onClose()}>
			{props.isOpen && <EnvVariablesSheet {...props} />}
		</Sheet>
	);
}
