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
import {$api} from '#/api/query';
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

	const kind = (database?.kind || database?.database_kind || 'postgres').toLowerCase();
	let endpoint: '/postgres/{id}' | '/mysql/{id}' | '/mariadb/{id}' | '/mongo/{id}' | '/redis/{id}' | '/libsql/{id}' = '/postgres/{id}';
	if (kind.includes('mysql')) endpoint = '/mysql/{id}';
	else if (kind.includes('mariadb')) endpoint = '/mariadb/{id}';
	else if (kind.includes('mongo')) endpoint = '/mongo/{id}';
	else if (kind.includes('redis')) endpoint = '/redis/{id}';
	else if (kind.includes('libsql')) endpoint = '/libsql/{id}';

	const deleteMutation = $api.useMutation('delete', endpoint as any);

	const dbName = database?.name || database?.app_name || 'database';

	const handleDelete = async (e: React.FormEvent) => {
		e.preventDefault();
		if (!database?.id) return;

		setIsSubmitting(true);
		try {
			await deleteMutation.mutateAsync({
				params: {path: {id: database.id}},
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
			<DialogContent className="sm:max-w-md bg-card border border-destructive/30 shadow-2xl p-6 flex flex-col gap-5 rounded-2xl">
				<DialogHeader className="space-y-2">
					<div className="size-10 rounded-full bg-destructive/10 border border-destructive/20 flex items-center justify-center text-destructive">
						<AlertTriangle className="size-5" />
					</div>
					<DialogTitle className="text-lg font-bold tracking-tight text-destructive flex items-center gap-2">
						Delete Database
					</DialogTitle>
					<DialogDescription className="text-xs text-muted-foreground leading-relaxed">
						Are you sure you want to delete <strong className="text-foreground">{dbName}</strong>? This will permanently remove the database container, volumes, and associated configs.
					</DialogDescription>
				</DialogHeader>

				<form onSubmit={handleDelete} className="flex flex-col gap-4">
					<div className="flex flex-col gap-2">
						<label className="text-[11px] font-bold text-muted-foreground uppercase tracking-wider">
							Type <span className="text-foreground font-mono font-bold select-all">{dbName}</span> to confirm:
						</label>
						<Input
							placeholder={dbName}
							value={confirmText}
							onChange={e => setConfirmText(e.target.value)}
							className="h-9 text-xs font-mono"
						/>
					</div>

					<div className="flex items-center justify-end pt-3 border-t border-border/30">
						<Button
							type="submit"
							disabled={isSubmitting || (confirmText.trim() !== dbName.trim() && confirmText.trim() !== 'delete')}
							className="w-full sm:w-auto bg-destructive hover:bg-destructive/90 text-destructive-foreground text-xs h-9 px-6 font-semibold shadow-md gap-1.5">
							<Trash2 className="size-3.5" />
							{isSubmitting ? 'Deleting...' : 'Delete Database'}
						</Button>
					</div>
				</form>
			</DialogContent>
		</Dialog>
	);
}
