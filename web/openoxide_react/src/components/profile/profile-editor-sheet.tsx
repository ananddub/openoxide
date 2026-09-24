import {useState} from 'react';
import {Loader2} from 'lucide-react';
import {toast} from 'sonner';
import {useUserUpdate} from '#/api/live';
import {formatApiError} from '#/api/utils';
import {getAvatarInitials} from '#/lib/avatar-utils';
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
import {AvatarPicker} from './avatar-picker';

interface ProfileEditorSheetProps {
	open: boolean;
	onOpenChange: (open: boolean) => void;
	initialFirstName: string;
	initialLastName: string;
	initialAvatar: string;
	email: string;
}

/** Settings → Account editor — right-side sheet, same grammar as the rest. */
export function ProfileEditorSheet({
	open,
	onOpenChange,
	initialFirstName,
	initialLastName,
	initialAvatar,
	email,
}: ProfileEditorSheetProps) {
	return (
		<Sheet open={open} onOpenChange={onOpenChange}>
			{open && (
				<EditorBody
					initialFirstName={initialFirstName}
					initialLastName={initialLastName}
					initialAvatar={initialAvatar}
					email={email}
					onDone={() => onOpenChange(false)}
				/>
			)}
		</Sheet>
	);
}

/* Seeded on mount — closing and reopening discards unsaved edits. */
function EditorBody({
	initialFirstName,
	initialLastName,
	initialAvatar,
	email,
	onDone,
}: {
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
		<SheetContent
			side="right"
			className="gap-0 p-0 sm:data-[side=right]:w-[560px]">
			<SheetHeader className="px-5 pt-5 pb-4">
				<SheetTitle>Edit profile</SheetTitle>
				<SheetDescription>
					Your name and avatar across the panel — {email}.
				</SheetDescription>
			</SheetHeader>

			<form
				id="profile-editor-form"
				onSubmit={handleSubmit}
				className="flex min-h-0 flex-1 flex-col gap-5 overflow-y-auto px-5 pt-4 pb-5">
				<div className="grid gap-4 sm:grid-cols-2">
					<label className="flex flex-col gap-1.5">
						<span className="text-xs font-semibold text-foreground">
							First name
						</span>
						<Input
							placeholder="First name"
							value={firstName}
							onChange={e => setFirstName(e.target.value)}
						/>
					</label>
					<label className="flex flex-col gap-1.5">
						<span className="text-xs font-semibold text-foreground">
							Last name
						</span>
						<Input
							placeholder="Last name"
							value={lastName}
							onChange={e => setLastName(e.target.value)}
						/>
					</label>
				</div>

				<AvatarPicker
					avatarValue={avatarValue}
					initials={userInitials}
					onChange={setAvatarValue}
				/>
			</form>

			<SheetFooter className="px-5 py-4">
				<Button
					type="button"
					variant="ghost"
					onClick={onDone}
					className="h-8 text-xs">
					Cancel
				</Button>
				<Button
					type="submit"
					form="profile-editor-form"
					disabled={updateUserMutation.isPending}
					className="h-8 gap-1.5 px-4 text-xs">
					{updateUserMutation.isPending ? (
						<>
							<Loader2 className="size-3.5 animate-spin" />
							Saving…
						</>
					) : (
						'Save changes'
					)}
				</Button>
			</SheetFooter>
		</SheetContent>
	);
}
