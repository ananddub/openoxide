import {useState, useEffect} from 'react';
import {toast} from 'sonner';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {
	Dialog,
	DialogContent,
	DialogDescription,
	DialogHeader,
	DialogTitle,
} from '#/components/ui/dialog';
import {$api} from '#/api/query';
import {formatApiError} from '#/api/utils';

interface CreateAppDialogProps {
	isOpen: boolean;
	onClose: () => void;
	environmentId: number;
	onCreated: (app: any) => void;
}

export function CreateAppDialog({
	isOpen,
	onClose,
	environmentId,
	onCreated,
}: CreateAppDialogProps) {
	const [name, setName] = useState('');
	const [description, setDescription] = useState('');
	const [isSubmitting, setIsSubmitting] = useState(false);

	const createMutation = $api.useMutation('post', '/applications');

	useEffect(() => {
		if (isOpen) {
			setName('');
			setDescription('');
		}
	}, [isOpen]);

	const handleSubmit = async (e: React.FormEvent) => {
		e.preventDefault();
		if (!name.trim()) {
			toast.error('Please specify application name');
			return;
		}

		setIsSubmitting(true);
		try {
			const res = await createMutation.mutateAsync({
				body: {
					name: name.trim(),
					description: description.trim() || undefined,
					environment_id: environmentId,
					build_type: 'NIXPACKS',
					source_type: 'GITHUB',
				},
			});
			toast.success('Application created successfully');
			onCreated(res);
			onClose();
		} catch (err: any) {
			toast.error(formatApiError(err));
		} finally {
			setIsSubmitting(false);
		}
	};

	return (
		<Dialog open={isOpen} onOpenChange={open => !open && onClose()}>
			<DialogContent className="sm:max-w-md bg-card border border-border shadow-2xl p-6 flex flex-col gap-5 rounded-2xl">
				<DialogHeader className="space-y-1">
					<DialogTitle className="text-lg font-bold tracking-tight text-foreground">
						Create Application
					</DialogTitle>
					<DialogDescription className="text-xs text-muted-foreground">
						Add a new web application service to this environment.
					</DialogDescription>
				</DialogHeader>

				<form onSubmit={handleSubmit} className="flex flex-col gap-5">
					<div className="flex flex-col gap-4">
						<div className="flex flex-col gap-1.5">
							<label className="text-[11px] font-bold text-muted-foreground uppercase tracking-wider">
								App Name <span className="text-destructive">*</span>
							</label>
							<Input
								placeholder="e.g. node-api"
								value={name}
								onChange={e => setName(e.target.value)}
								required
								className="h-9 rounded-lg border border-border/80 bg-muted/20 px-3 text-xs shadow-inner focus:outline-none"
							/>
						</div>
						<div className="flex flex-col gap-1.5">
							<label className="text-[11px] font-bold text-muted-foreground uppercase tracking-wider">
								Description
							</label>
							<textarea
								rows={3}
								placeholder="Optional details about this application..."
								value={description}
								onChange={e => setDescription(e.target.value)}
								className="w-full rounded-lg border border-border/80 bg-muted/20 px-3 py-2 text-xs shadow-inner focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring leading-relaxed resize-none"
							/>
						</div>
					</div>

					{/* Modal Actions */}
					<div className="flex justify-end pt-3 border-t border-border/30">
						<Button
							type="submit"
							disabled={isSubmitting}
							className="bg-primary hover:bg-primary/95 text-primary-foreground text-xs h-9 px-5 font-semibold shadow-lg shadow-primary/10 rounded-lg">
							{isSubmitting ? 'Creating...' : 'Create Application'}
						</Button>
					</div>
				</form>
			</DialogContent>
		</Dialog>
	);
}
