import {useEnvironmentCreate} from '#/api/live/environments';
import {useState} from 'react';
import {toast} from 'sonner';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {
	Sheet,
	SheetContent,
	SheetDescription,
	SheetFooter,
	SheetHeader,
	SheetTitle,
} from '#/components/ui/sheet';
import {formatApiError} from '#/api/utils';

interface CreateEnvDialogProps {
	isOpen: boolean;
	onClose: () => void;
	projectId: number;
	onCreated: (env: any) => void;
}

export function CreateEnvDialog({
	isOpen,
	onClose,
	projectId,
	onCreated,
}: CreateEnvDialogProps) {
	const [name, setName] = useState('');
	const [description, setDescription] = useState('');
	const [isSubmitting, setIsSubmitting] = useState(false);

	const createMutation = useEnvironmentCreate();

	const handleSubmit = async (e: React.FormEvent) => {
		e.preventDefault();
		if (!name.trim()) return;

		setIsSubmitting(true);
		try {
			const res = await createMutation.mutateAsync({
				name: name.trim(),
				description: description.trim() || undefined,
				project_id: projectId,
				is_default: false,
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
		<Sheet open={isOpen} onOpenChange={open => !open && onClose()}>
			<SheetContent>
				<SheetHeader>
					<SheetTitle>New Environment</SheetTitle>
					<SheetDescription>
						Add a new deployment target environment (e.g. Staging,
						Production).
					</SheetDescription>
				</SheetHeader>

				<div className="flex-1 overflow-y-auto px-6 py-6">
					<form
						id="create-env-sheet-form"
						onSubmit={handleSubmit}
						className="flex flex-col gap-4">
						<div className="flex flex-col gap-1.5">
							<label className="text-xs font-semibold text-foreground">
								Name *
							</label>
							<Input
								placeholder="e.g. Production"
								value={name}
								onChange={e => setName(e.target.value)}
								required
							/>
						</div>

						<div className="flex flex-col gap-1.5">
							<label className="text-xs font-semibold text-foreground">
								Description
							</label>
							<Input
								placeholder="Brief details"
								value={description}
								onChange={e => setDescription(e.target.value)}
							/>
						</div>
					</form>
				</div>

				<SheetFooter>
					<Button
						type="submit"
						form="create-env-sheet-form"
						disabled={isSubmitting || !name.trim()}
						className="h-10 gap-1.5 px-5">
						{isSubmitting ? 'Saving…' : 'Save'}
					</Button>
				</SheetFooter>
			</SheetContent>
		</Sheet>
	);
}
