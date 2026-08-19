import {useState} from 'react';
import {Button} from '#/components/ui/button';
import {
	Dialog,
	DialogContent,
	DialogDescription,
	DialogFooter,
	DialogHeader,
	DialogTitle,
} from '#/components/ui/dialog';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import {toast} from 'sonner';

interface ChangeRoleModalProps {
	userEmail: string;
	currentRole: string;
	isOpen: boolean;
	onClose: () => void;
	onSuccess: () => void;
}

export function ChangeRoleModal({
	userEmail,
	currentRole,
	isOpen,
	onClose,
	onSuccess,
}: ChangeRoleModalProps) {
	const [role, setRole] = useState(currentRole);
	const [isSubmitting, setIsSubmitting] = useState(false);

	const handleSubmit = async (e: React.FormEvent) => {
		e.preventDefault();
		setIsSubmitting(true);
		try {
			toast.success(`Role updated for ${userEmail}`);
			onSuccess();
			onClose();
		} catch {
			toast.error('Error updating role');
		} finally {
			setIsSubmitting(false);
		}
	};

	return (
		<Dialog open={isOpen} onOpenChange={open => !open && onClose()}>
			<DialogContent className="sm:max-w-md">
				<DialogHeader>
					<DialogTitle>Change Role</DialogTitle>
					<DialogDescription>
						Change user role for {userEmail}
					</DialogDescription>
				</DialogHeader>

				<form
					id="change-role-form"
					onSubmit={handleSubmit}
					className="grid w-full gap-4 py-2">
					<div className="flex flex-col gap-2">
						<label className="text-sm leading-none font-medium">
							Role
						</label>
						<Select value={role} onValueChange={v => v && setRole(v)}>
							<SelectTrigger>
								<SelectValue />
							</SelectTrigger>
							<SelectContent>
								<SelectItem value="admin">Admin</SelectItem>
								<SelectItem value="member">Member</SelectItem>
							</SelectContent>
						</Select>
					</div>
				</form>

				<DialogFooter>
					<Button variant="outline" onClick={onClose}>
						Cancel
					</Button>
					<Button
						disabled={isSubmitting}
						form="change-role-form"
						type="submit">
						Save Role
					</Button>
				</DialogFooter>
			</DialogContent>
		</Dialog>
	);
}
