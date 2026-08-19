import {useState} from 'react';
import {PlusIcon} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {
	Dialog,
	DialogContent,
	DialogDescription,
	DialogFooter,
	DialogHeader,
	DialogTitle,
	DialogTrigger,
} from '#/components/ui/dialog';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import {AlertBlock} from '#/components/shared/alert-block';
import {toast} from 'sonner';

interface AddInvitationModalProps {
	onSuccess?: () => void;
}

export function AddInvitationModal({onSuccess}: AddInvitationModalProps) {
	const [isOpen, setIsOpen] = useState(false);
	const [mode, setMode] = useState<'invitation' | 'credentials'>(
		'credentials',
	);
	const [email, setEmail] = useState('');
	const [role, setRole] = useState('member');
	const [password, setPassword] = useState('');
	const [confirmPassword, setConfirmPassword] = useState('');
	const [isSubmitting, setIsSubmitting] = useState(false);
	const [errorMessage, setErrorMessage] = useState<string | null>(null);

	const handleSubmit = async (e: React.FormEvent) => {
		e.preventDefault();
		const trimmedEmail = email.trim();
		if (!trimmedEmail) {
			setErrorMessage('Email is required');
			return;
		}

		if (mode === 'credentials') {
			if (!password) {
				setErrorMessage('Password is required');
				return;
			}
			if (password.length < 8) {
				setErrorMessage('Password must be at least 8 characters');
				return;
			}
			if (password !== confirmPassword) {
				setErrorMessage('Passwords do not match');
				return;
			}
		}

		setIsSubmitting(true);
		setErrorMessage(null);
		try {
			toast.success(
				mode === 'credentials'
					? 'User Created Successfully'
					: 'Invitation Sent',
			);
			setIsOpen(false);
			setEmail('');
			setPassword('');
			setConfirmPassword('');
			onSuccess?.();
		} catch {
			setErrorMessage('Error creating user invitation');
			toast.error('Error creating user invitation');
		} finally {
			setIsSubmitting(false);
		}
	};

	return (
		<Dialog open={isOpen} onOpenChange={setIsOpen}>
			<DialogTrigger
				render={
					<Button size="sm" className="h-9 gap-1.5 text-xs font-semibold">
						<PlusIcon className="h-4 w-4" />
						Create User / Invitation
					</Button>
				}
			/>
			<DialogContent className="sm:max-w-lg">
				<DialogHeader>
					<DialogTitle>Create Invitation</DialogTitle>
					<DialogDescription>
						Invite users to your organization or create credentials
						directly.
					</DialogDescription>
				</DialogHeader>

				{errorMessage && (
					<AlertBlock type="error">{errorMessage}</AlertBlock>
				)}

				<form
					id="add-invitation-form"
					onSubmit={handleSubmit}
					className="grid w-full gap-4 py-2">
					<div className="flex flex-col gap-2">
						<label className="text-sm leading-none font-medium">
							Mode
						</label>
						<Select
							value={mode}
							onValueChange={v => v && setMode(v as any)}>
							<SelectTrigger>
								<SelectValue />
							</SelectTrigger>
							<SelectContent>
								<SelectItem value="credentials">
									Create Credentials Directly
								</SelectItem>
								<SelectItem value="invitation">
									Send Email Invitation
								</SelectItem>
							</SelectContent>
						</Select>
					</div>

					<div className="flex flex-col gap-2">
						<label className="text-sm leading-none font-medium">
							Email
						</label>
						<Input
							type="email"
							placeholder="user@example.com"
							value={email}
							onChange={e => setEmail(e.target.value)}
							required
						/>
					</div>

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

					{mode === 'credentials' && (
						<>
							<div className="flex flex-col gap-2">
								<label className="text-sm leading-none font-medium">
									Password
								</label>
								<Input
									type="password"
									placeholder="••••••••"
									value={password}
									onChange={e => setPassword(e.target.value)}
									required
								/>
							</div>

							<div className="flex flex-col gap-2">
								<label className="text-sm leading-none font-medium">
									Confirm Password
								</label>
								<Input
									type="password"
									placeholder="••••••••"
									value={confirmPassword}
									onChange={e => setConfirmPassword(e.target.value)}
									required
								/>
							</div>
						</>
					)}
				</form>

				<DialogFooter>
					<Button
						disabled={isSubmitting}
						form="add-invitation-form"
						type="submit">
						{mode === 'credentials' ? 'Create User' : 'Send Invitation'}
					</Button>
				</DialogFooter>
			</DialogContent>
		</Dialog>
	);
}
