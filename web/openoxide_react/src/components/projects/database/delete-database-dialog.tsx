import {useState} from 'react';
import {useNavigate} from '@tanstack/react-router';
import {AlertTriangle, Trash2} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {
	Dialog,
	DialogContent,
	DialogDescription,
	DialogHeader,
	DialogTitle,
} from '#/components/ui/dialog';
import {toast} from 'sonner';
import {parseDatabaseKind, useDatabaseDelete} from '#/api/live';
import {formatApiError} from '#/api/utils';

interface DeleteDatabaseDialogProps {
	isOpen: boolean;
	onClose: () => void;
	database: any;
}

export function DeleteDatabaseDialog({
	isOpen,
	onClose,
	database,
}: DeleteDatabaseDialogProps) {
	const navigate = useNavigate();
	const [confirmText, setConfirmText] = useState('');
	const [isSubmitting, setIsSubmitting] = useState(false);

	const kind = (
		database?.kind ||
		database?.database_kind ||
		'postgres'
	).toLowerCase();

	const deleteMutation = useDatabaseDelete();

	const dbName = database?.name || database?.app_name || 'database';

	const handleDelete = async (e: React.FormEvent) => {
		e.preventDefault();
		if (!database?.id) return;

		setIsSubmitting(true);
		try {
			await deleteMutation.mutateAsync({
				kind: parseDatabaseKind(kind),
				id: database.id,
			});
			toast.success('Database deleted successfully');
			onClose();
			if (database?.project_id) {
				navigate({to: `/projects/${database.project_id}` as any});
			} else {
				navigate({to: '/projects' as any});
			}
		} catch (err: any) {
			toast.error(formatApiError(err));
		} finally {
			setIsSubmitting(false);
		}
	};

	return (
		<Dialog open={isOpen} onOpenChange={open => !open && onClose()}>
			<DialogContent className="flex flex-col gap-5 rounded-xl bg-card p-6 shadow-lg sm:max-w-md">
				<DialogHeader className="space-y-2">
					<div className="flex size-10 items-center justify-center rounded-full bg-destructive/10 text-destructive">
						<AlertTriangle className="size-5" />
					</div>
					<DialogTitle className="flex items-center gap-2 text-lg font-bold tracking-tight text-destructive">
						Delete Database
					</DialogTitle>
					<DialogDescription className="text-xs leading-relaxed text-muted-foreground">
						Are you sure you want to delete{' '}
						<strong className="text-foreground">{dbName}</strong>? This
						will permanently remove the database container, volumes, and
						associated configs.
					</DialogDescription>
				</DialogHeader>

				<form onSubmit={handleDelete} className="flex flex-col gap-4">
					<div className="flex flex-col gap-2">
						<label className="text-[11px] font-bold tracking-wider text-muted-foreground uppercase">
							Type{' '}
							<span className="font-mono font-bold text-foreground select-all">
								{dbName}
							</span>{' '}
							to confirm:
						</label>
						<Input
							placeholder={dbName}
							value={confirmText}
							onChange={e => setConfirmText(e.target.value)}
							className="h-9 font-mono text-xs"
						/>
					</div>

					<div className="flex items-center justify-end border-t  pt-3">
						<Button
							type="submit"
							disabled={
								isSubmitting ||
								(confirmText.trim() !== dbName.trim() &&
									confirmText.trim() !== 'delete')
							}
							className="text-destructive-foreground h-9 w-full gap-1.5 bg-destructive px-6 text-xs font-semibold shadow-md hover:bg-destructive/90 sm:w-auto">
							<Trash2 className="size-3.5" />
							{isSubmitting ? 'Deleting...' : 'Delete Database'}
						</Button>
					</div>
				</form>
			</DialogContent>
		</Dialog>
	);
}
