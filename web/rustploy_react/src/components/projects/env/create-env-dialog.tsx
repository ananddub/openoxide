import {useState} from 'react';
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

interface CreateEnvDialogProps {
	isOpen: boolean;
	onClose: () => void;
	projectId: number;
	onCreated: (env: any) => void;
}

export function CreateEnvDialog({isOpen, onClose, projectId, onCreated}: CreateEnvDialogProps) {
	const [name, setName] = useState('');
	const [description, setDescription] = useState('');
	const [isSubmitting, setIsSubmitting] = useState(false);

	const createMutation = $api.useMutation('post', '/environments');

	const handleSubmit = async (e: React.FormEvent) => {
		e.preventDefault();
		if (!name.trim()) return;

		setIsSubmitting(true);
		try {
			const res = await createMutation.mutateAsync({
				body: {
					name: name.trim(),
					description: description.trim() || undefined,
					project_id: projectId,
					is_default: false,
					env_var: '',
				},
			});
			toast.success('Environment created successfully');
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
			<DialogContent className="sm:max-w-md bg-card border-border">
				<DialogHeader>
					<DialogTitle className="text-base font-bold">Create Environment</DialogTitle>
					<DialogDescription className="text-xs text-muted-foreground">
						Add a new deployment target environment (e.g. Staging, Production).
					</DialogDescription>
				</DialogHeader>

				<form onSubmit={handleSubmit} className="space-y-4 mt-2">
					<div className="space-y-1">
						<label className="text-xs font-semibold text-foreground">Name *</label>
						<Input
							placeholder="e.g. Production"
							value={name}
							onChange={e => setName(e.target.value)}
							required
							className="h-9"
						/>
					</div>

					<div className="space-y-1">
						<label className="text-xs font-semibold text-foreground">Description</label>
						<Input
							placeholder="Brief details"
							value={description}
							onChange={e => setDescription(e.target.value)}
							className="h-9"
						/>
					</div>

					<div className="flex justify-end pt-3 border-t border-border/20">
						<Button
							type="submit"
							disabled={isSubmitting}
							className="bg-primary hover:bg-primary/95 text-primary-foreground text-xs h-9 px-6 font-semibold shadow-xs w-full sm:w-auto">
							{isSubmitting ? 'Creating...' : 'Create'}
						</Button>
					</div>
				</form>
			</DialogContent>
		</Dialog>
	);
}
