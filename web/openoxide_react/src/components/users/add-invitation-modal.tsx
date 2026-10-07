import {useState} from 'react';
import {KeyRound, Mail, PlusIcon, ShieldCheck, User} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {
	Sheet,
	SheetContent,
	SheetDescription,
	SheetFooter,
	SheetHeader,
	SheetTitle,
	SheetTrigger,
} from '#/components/ui/sheet';
import {AlertBlock} from '#/components/shared/alert-block';
import {Field} from '#/components/shared/field';
import {
	OptionRow,
	StepTimeline,
	type WizardStep,
} from '#/components/shared/sheet-wizard';
import {toast} from 'sonner';

interface AddInvitationModalProps {
	onSuccess?: () => void;
}

const STEPS: WizardStep[] = [
	{id: 'account', label: 'Account'},
	{id: 'access', label: 'Access'},
];

const FORM_ID = 'add-invitation-form';

const STEP_DESCRIPTIONS: Record<string, string> = {
	account: 'How should this person get in, and what can they do?',
	access: 'Finish the account — credentials or an email invite.',
};

/** New User / Invitation — checkpoint wizard: Account (mode + email + role) → Access, the same StepTimeline grammar as the new-server sheet. */
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
	const [stepIdx, setStepIdx] = useState(0);

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
			setStepIdx(0);
			onSuccess?.();
		} catch {
			setErrorMessage('Error creating user invitation');
			toast.error('Error creating user invitation');
		} finally {
			setIsSubmitting(false);
		}
	};

	const last = stepIdx === STEPS.length - 1;

	return (
		<Sheet open={isOpen} onOpenChange={setIsOpen}>
			<SheetTrigger
				render={
					<Button size="sm" className="h-9 gap-1.5 text-xs font-semibold">
						<PlusIcon className="h-4 w-4" />
						New User / Invitation
					</Button>
				}
			/>
			<SheetContent>
				<SheetHeader>
					<SheetTitle>New Invitation</SheetTitle>
					<SheetDescription>
						{STEP_DESCRIPTIONS[STEPS[stepIdx].id]}
					</SheetDescription>
				</SheetHeader>

				{errorMessage && (
					<AlertBlock type="error" className="mx-6 mt-6">
						{errorMessage}
					</AlertBlock>
				)}

				<form
					id={FORM_ID}
					onSubmit={handleSubmit}
					className="flex min-h-0 flex-1 flex-col">
					<div className="flex min-h-0 flex-1 flex-col overflow-y-auto px-6 py-6">
						<StepTimeline steps={STEPS} idx={stepIdx}>
							<div
								key={STEPS[stepIdx].id}
								className="animate-in duration-200 ease-out fade-in-0 slide-in-from-right-4">
								{stepIdx === 0 ? (
									<div className="flex flex-col gap-4">
										<Field label="Mode">
											<div className="flex flex-col gap-2">
												<OptionRow
													selected={mode === 'credentials'}
													onClick={() => setMode('credentials')}
													icon={<KeyRound className="size-4.5" />}
													title="Create credentials directly"
													subtitle="Set a password here — the user signs in right away."
												/>
												<OptionRow
													selected={mode === 'invitation'}
													onClick={() => setMode('invitation')}
													icon={<Mail className="size-4.5" />}
													title="Send email invitation"
													subtitle="The user sets their own password from the invite."
												/>
											</div>
										</Field>

										<Field label="Email">
											<Input
												type="email"
												placeholder="user@example.com"
												value={email}
												onChange={e => setEmail(e.target.value)}
												required
											/>
										</Field>

										<Field label="Role">
											<div className="flex flex-col gap-2">
												<OptionRow
													selected={role === 'admin'}
													onClick={() => setRole('admin')}
													icon={<ShieldCheck className="size-4.5" />}
													title="Admin"
													subtitle="Full control over the organization."
												/>
												<OptionRow
													selected={role === 'member'}
													onClick={() => setRole('member')}
													icon={<User className="size-4.5" />}
													title="Member"
													subtitle="Deploy and manage services, no org settings."
												/>
											</div>
										</Field>
									</div>
								) : mode === 'credentials' ? (
									<div className="flex flex-col gap-4">
										<Field label="Password">
											<Input
												type="password"
												placeholder="••••••••"
												value={password}
												onChange={e => setPassword(e.target.value)}
												required
											/>
										</Field>
										<Field label="Confirm Password">
											<Input
												type="password"
												placeholder="••••••••"
												value={confirmPassword}
												onChange={e => setConfirmPassword(e.target.value)}
												required
											/>
										</Field>
									</div>
								) : (
									<p className="text-xs text-muted-foreground">
										No password needed —{' '}
										<span className="font-medium text-foreground">
											{email.trim() || 'the user'}
										</span>{' '}
										receives an email invitation and picks their own.
									</p>
								)}
							</div>
						</StepTimeline>
					</div>

					<SheetFooter className="justify-between">
						{stepIdx > 0 ? (
							<Button
								type="button"
								variant="outline"
								onClick={() => setStepIdx(i => i - 1)}>
								Back
							</Button>
						) : (
							<span />
						)}
						{last ? (
							<Button
								disabled={isSubmitting || !email.trim()}
								form={FORM_ID}
								type="submit">
								{isSubmitting
									? 'Saving…'
									: mode === 'credentials'
										? 'Save'
										: 'Send Invitation'}
							</Button>
						) : (
							<Button
								type="button"
								disabled={!email.trim()}
								onClick={() => setStepIdx(i => i + 1)}>
								Continue
							</Button>
						)}
					</SheetFooter>
				</form>
			</SheetContent>
		</Sheet>
	);
}
