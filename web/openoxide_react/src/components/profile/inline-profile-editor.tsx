import {useState, useEffect} from 'react';
import {Loader2} from 'lucide-react';
import {useUserUpdate} from '#/api/live';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {getAvatarInitials} from '#/lib/avatar-utils';
import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';
import {AvatarPicker} from './avatar-picker';

/** The editor body that unfolds under the identity row. */
export function InlineProfileEditor({
	open,
	initialFirstName,
	initialLastName,
	initialAvatar,
	email,
	onDone,
}: {
	open: boolean;
	initialFirstName: string;
	initialLastName: string;
	initialAvatar: string;
	email: string;
	onDone: () => void;
}) {
	const updateUserMutation = useUserUpdate();
	const [firstName, setFirstName] = useState(initialFirstName);
	const [lastName, setLastName] = useState(initialLastName);
	const [avatarValue, setAvatarValue] = useState(initialAvatar);

	// Re-seed from the live profile every time the card opens, so a collapse (Cancel) discards unsaved edits.
	useEffect(() => {
		if (open) {
			setFirstName(initialFirstName);
			setLastName(initialLastName);
			setAvatarValue(initialAvatar);
		}
	}, [open, initialFirstName, initialLastName, initialAvatar]);

	const userInitials = getAvatarInitials(
		`${firstName} ${lastName}`.trim() || email,
	);

	const handleSubmit = async (e: React.FormEvent) => {
		e.preventDefault();
		try {
			await updateUserMutation.mutateAsync({
				first_name: firstName.trim() || undefined,
				last_name: lastName.trim() || undefined,
				avatar: avatarValue,
			});
			toast.success('Profile updated');
			onDone();
		} catch (err) {
			toast.error(formatApiError(err) || 'Error updating profile');
		}
	};

	return (
		<form onSubmit={handleSubmit} className="border-t px-4 py-4">
			<div className="flex flex-col gap-5">
				<div className="grid grid-cols-1 gap-3 sm:grid-cols-2">
					<label className="flex flex-col gap-1.5">
						<span className="text-[13px] font-medium text-foreground">
							First name
						</span>
						<Input
							placeholder="First name"
							value={firstName}
							onChange={e => setFirstName(e.target.value)}
							className="h-10 text-sm"
						/>
					</label>
					<label className="flex flex-col gap-1.5">
						<span className="text-[13px] font-medium text-foreground">
							Last name
						</span>
						<Input
							placeholder="Last name"
							value={lastName}
							onChange={e => setLastName(e.target.value)}
							className="h-10 text-sm"
						/>
					</label>
				</div>

				<AvatarPicker
					avatarValue={avatarValue}
					initials={userInitials}
					onChange={setAvatarValue}
				/>

				<div className="flex items-center justify-end gap-2">
					<Button
						type="button"
						variant="ghost"
						onClick={onDone}
						className="h-9 text-sm">
						Cancel
					</Button>
					<Button
						type="submit"
						disabled={updateUserMutation.isPending}
						className="h-9 gap-1.5 px-4 text-sm">
						{updateUserMutation.isPending ? (
							<>
								<Loader2 className="size-4 animate-spin" />
								Saving...
							</>
						) : (
							'Save changes'
						)}
					</Button>
				</div>
			</div>
		</form>
	);
}
