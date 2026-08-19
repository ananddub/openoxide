import {useState} from 'react';
import {AlertTriangle, Trash2, Loader2} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {
	Dialog,
	DialogContent,
	DialogDescription,
	DialogHeader,
	DialogTitle,
} from '#/components/ui/dialog';

interface DeleteServiceDialogProps {
	isOpen: boolean;
	onClose: () => void;
	serviceName: string;
	serviceType: string;
	onConfirm: () => Promise<void>;
}

export function DeleteServiceDialog({
	isOpen,
	onClose,
	serviceName,
	serviceType,
	onConfirm,
}: DeleteServiceDialogProps) {
	const [confirmText, setConfirmText] = useState('');
	const [isSubmitting, setIsSubmitting] = useState(false);

	const handleOpenChange = (open: boolean) => {
		if (!open) {
			setConfirmText('');
			onClose();
		}
	};

	const handleDelete = async (e: React.FormEvent) => {
		e.preventDefault();
		if (
			confirmText.trim() !== serviceName.trim() &&
			confirmText.trim().toLowerCase() !== 'delete'
		) {
			return;
		}

		setIsSubmitting(true);
		try {
			await onConfirm();
			setConfirmText('');
			onClose();
		} finally {
			setIsSubmitting(false);
		}
	};

	const isMatch =
		confirmText.trim() === serviceName.trim() ||
		confirmText.trim().toLowerCase() === 'delete';

	return (
		<Dialog open={isOpen} onOpenChange={handleOpenChange}>
			<DialogContent className="flex flex-col gap-5 rounded-2xl border border-destructive/30 bg-card p-6 shadow-2xl sm:max-w-md">
				<DialogHeader className="space-y-2">
					<div className="flex size-10 items-center justify-center rounded-full border border-destructive/20 bg-destructive/10 text-destructive">
						<AlertTriangle className="size-5" />
					</div>
					<DialogTitle className="flex items-center gap-2 text-lg font-bold tracking-tight text-destructive">
						Delete {serviceType || 'Service'}
					</DialogTitle>
					<DialogDescription className="text-xs leading-relaxed text-muted-foreground">
						Are you sure you want to delete{' '}
						<strong className="text-foreground">{serviceName}</strong>?
						This action cannot be undone. All associated deployments,
						containers, and configurations will be permanently removed.
					</DialogDescription>
				</DialogHeader>

				<form onSubmit={handleDelete} className="flex flex-col gap-4">
					<div className="flex flex-col gap-2">
						<label className="text-[11px] font-bold tracking-wider text-muted-foreground uppercase">
							Type{' '}
							<span className="font-mono font-bold text-foreground select-all">
								{serviceName}
							</span>{' '}
							or{' '}
							<span className="font-mono font-bold text-destructive">
								delete
							</span>{' '}
							to confirm:
						</label>
						<Input
							placeholder={serviceName}
							value={confirmText}
							onChange={e => setConfirmText(e.target.value)}
							className="h-9 font-mono text-xs"
							autoFocus
						/>
					</div>

					<div className="flex items-center justify-end gap-2 border-t border-border/30 pt-3">
						<Button
							type="button"
							variant="ghost"
							onClick={() => handleOpenChange(false)}
							disabled={isSubmitting}
							className="h-9 px-4 text-xs">
							Cancel
						</Button>
						<Button
							type="submit"
							disabled={isSubmitting || !isMatch}
							className="text-destructive-foreground h-9 gap-1.5 bg-destructive px-5 text-xs font-semibold shadow-md hover:bg-destructive/90">
							{isSubmitting ? (
								<>
									<Loader2 className="size-3.5 animate-spin" />
									Deleting...
								</>
							) : (
								<>
									<Trash2 className="size-3.5" />
									Delete {serviceType || 'Service'}
								</>
							)}
						</Button>
					</div>
				</form>
			</DialogContent>
		</Dialog>
	);
}
