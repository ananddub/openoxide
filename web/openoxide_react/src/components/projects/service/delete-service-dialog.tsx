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
		if (confirmText.trim() !== serviceName.trim() && confirmText.trim().toLowerCase() !== 'delete') {
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

	const isMatch = confirmText.trim() === serviceName.trim() || confirmText.trim().toLowerCase() === 'delete';

	return (
		<Dialog open={isOpen} onOpenChange={handleOpenChange}>
			<DialogContent className="sm:max-w-md bg-card border border-destructive/30 shadow-2xl p-6 flex flex-col gap-5 rounded-2xl">
				<DialogHeader className="space-y-2">
					<div className="size-10 rounded-full bg-destructive/10 border border-destructive/20 flex items-center justify-center text-destructive">
						<AlertTriangle className="size-5" />
					</div>
					<DialogTitle className="text-lg font-bold tracking-tight text-destructive flex items-center gap-2">
						Delete {serviceType || 'Service'}
					</DialogTitle>
					<DialogDescription className="text-xs text-muted-foreground leading-relaxed">
						Are you sure you want to delete <strong className="text-foreground">{serviceName}</strong>? This action cannot be undone. All associated deployments, containers, and configurations will be permanently removed.
					</DialogDescription>
				</DialogHeader>

				<form onSubmit={handleDelete} className="flex flex-col gap-4">
					<div className="flex flex-col gap-2">
						<label className="text-[11px] font-bold text-muted-foreground uppercase tracking-wider">
							Type <span className="text-foreground font-mono font-bold select-all">{serviceName}</span> or <span className="text-destructive font-mono font-bold">delete</span> to confirm:
						</label>
						<Input
							placeholder={serviceName}
							value={confirmText}
							onChange={e => setConfirmText(e.target.value)}
							className="h-9 text-xs font-mono"
							autoFocus
						/>
					</div>

					<div className="flex items-center justify-end gap-2 pt-3 border-t border-border/30">
						<Button
							type="button"
							variant="ghost"
							onClick={() => handleOpenChange(false)}
							disabled={isSubmitting}
							className="text-xs h-9 px-4">
							Cancel
						</Button>
						<Button
							type="submit"
							disabled={isSubmitting || !isMatch}
							className="bg-destructive hover:bg-destructive/90 text-destructive-foreground text-xs h-9 px-5 font-semibold shadow-md gap-1.5">
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
