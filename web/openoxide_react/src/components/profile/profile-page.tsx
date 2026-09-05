import {useState, useEffect, useRef} from 'react';
import {User, Loader2, Palette, Upload} from 'lucide-react';
import {useUserUpdate, useProfile} from '#/api/live';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {
	Card,
	CardContent,
	CardDescription,
	CardHeader,
	CardTitle,
} from '#/components/ui/card';
import {isSolidColorAvatar, getAvatarInitials} from '#/lib/avatar-utils';
import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';
import {useAuthSession} from '#/stores/session-context';

export interface ApiKeyItem {
	id: string;
	name: string;
	keyPrefix: string;
	createdAt: string;
}

const PRESET_AVATARS = [
	'https://api.dicebear.com/7.x/bottts/svg?seed=OpenOxide1',
	'https://api.dicebear.com/7.x/bottts/svg?seed=OpenOxide2',
	'https://api.dicebear.com/7.x/bottts/svg?seed=OpenOxide3',
	'https://api.dicebear.com/7.x/bottts/svg?seed=OpenOxide4',
	'https://api.dicebear.com/7.x/avataaars/svg?seed=Alex',
	'https://api.dicebear.com/7.x/avataaars/svg?seed=Sarah',
	'https://api.dicebear.com/7.x/avataaars/svg?seed=David',
	'https://api.dicebear.com/7.x/avataaars/svg?seed=Elena',
	'https://api.dicebear.com/7.x/shapes/svg?seed=Shape1',
	'https://api.dicebear.com/7.x/shapes/svg?seed=Shape2',
];

export function ProfilePage() {
	const {data: storeProfile} = useProfile();
	const {user: authUser} = useAuthSession();
	const isLoading = false;

	useEffect(() => {
		const initialEmail = storeProfile?.email || authUser?.email || '';
		const initialFirstName =
			storeProfile?.name?.split(' ')[0] || authUser?.firstName || '';
		const initialLastName =
			storeProfile?.name?.split(' ').slice(1).join(' ') ||
			authUser?.lastName ||
			'';
		const initialAvatar = storeProfile?.avatar || '';

		setEmail(initialEmail);
		setFirstName(initialFirstName);
		setLastName(initialLastName);
		setAvatarValue(initialAvatar);
	}, [storeProfile, authUser]);

	// Profile updates flow through AuthService.UpdateUser; the table-watch
	// stream (USERS table → ['auth']) refetches useProfile() on change.
	const updateUserMutation = useUserUpdate();

	const [firstName, setFirstName] = useState('');
	const [lastName, setLastName] = useState('');
	const [email, setEmail] = useState('');
	const [avatarValue, setAvatarValue] = useState<string>(
		PRESET_AVATARS[0],
	);

	const colorInputRef = useRef<HTMLInputElement>(null);
	const fileInputRef = useRef<HTMLInputElement>(null);

	const handleImageFileUpload = (
		e: React.ChangeEvent<HTMLInputElement>,
	) => {
		const file = e.target.files?.[0];
		if (!file) return;

		if (file.size > 2 * 1024 * 1024) {
			toast.error('Image size must be less than 2MB');
			return;
		}

		const reader = new FileReader();
		reader.onload = event => {
			const result = event.target?.result as string;
			setAvatarValue(result);
			toast.success('Avatar image loaded');
		};
		reader.readAsDataURL(file);
	};

	const handleSubmitProfile = async (e: React.FormEvent) => {
		e.preventDefault();
		try {
			await updateUserMutation.mutateAsync({
				first_name: firstName.trim() || undefined,
				last_name: lastName.trim() || undefined,
				avatar: avatarValue,
			});
			toast.success('Profile Updated Successfully');
		} catch (err) {
			toast.error(formatApiError(err) || 'Error updating profile');
		}
	};

	const userInitials = getAvatarInitials(
		`${firstName} ${lastName}`.trim() || email,
	);

	return (
		<div className="mx-auto flex w-full max-w-5xl animate-in flex-col gap-6 pb-12 duration-150 fade-in">
			{/* Account Card */}
			<Card className="overflow-hidden rounded-2xl border border-border/80 bg-card shadow-xs">
				<CardHeader className="flex flex-row flex-wrap items-center justify-between gap-4 pb-4">
					<div>
						<CardTitle className="flex flex-row items-center gap-2 text-xl font-bold">
							<User className="size-5 text-muted-foreground" />
							<span>Account</span>
						</CardTitle>
						<CardDescription className="mt-0.5 text-xs text-muted-foreground">
							Manage your avatar and view profile details.
						</CardDescription>
					</div>
				</CardHeader>

				<CardContent className="space-y-6 border-t border-border/60 pt-4">
					{isLoading ? (
						<div className="flex min-h-[25vh] flex-row items-center justify-center gap-2 text-sm text-muted-foreground">
							<Loader2 className="size-4 animate-spin text-primary" />
							<span>Loading account profile...</span>
						</div>
					) : (
						<form
							onSubmit={handleSubmitProfile}
							className="flex flex-col gap-6">
							{/* Editable Account Information Grid */}
							<div className="flex flex-col gap-3 rounded-xl border border-border/60 bg-muted/20 p-4">
								<div className="flex items-center gap-1.5 text-xs font-bold text-foreground">
									<User className="size-3.5 text-primary" />
									<span>Account Information</span>
								</div>

								<div className="grid grid-cols-1 gap-4 sm:grid-cols-2">
									<div className="flex flex-col gap-1.5">
										<label className="text-[11px] font-semibold text-muted-foreground">
											First Name
										</label>
										<Input
											placeholder="First Name"
											value={firstName}
											onChange={e => setFirstName(e.target.value)}
											className="h-9 border-border/70 bg-muted/30 text-xs"
										/>
									</div>

									<div className="flex flex-col gap-1.5">
										<label className="text-[11px] font-semibold text-muted-foreground">
											Last Name
										</label>
										<Input
											placeholder="Last Name"
											value={lastName}
											onChange={e => setLastName(e.target.value)}
											className="h-9 border-border/70 bg-muted/30 text-xs"
										/>
									</div>
								</div>
							</div>

							{/* Dokploy 1:1 Avatar Selection */}
							<div className="flex flex-col gap-2 pt-2">
								<label className="text-xs font-semibold text-foreground">
									Avatar Selection
								</label>

								<div className="flex flex-row flex-wrap items-center gap-3">
									{/* Default Initials Avatar */}
									<div
										onClick={() => setAvatarValue('')}
										className={`flex h-12 w-12 cursor-pointer items-center justify-center rounded-full border text-xs font-bold transition-all ${
											!avatarValue
												? 'border-primary ring-2 ring-primary ring-offset-2 ring-offset-background'
												: 'border-border hover:border-primary'
										}`}>
										{userInitials}
									</div>

									{/* Custom Uploaded Avatar */}
									<div
										onClick={() => fileInputRef.current?.click()}
										className={`relative flex h-12 w-12 cursor-pointer items-center justify-center overflow-hidden rounded-full border border-dashed bg-muted/40 transition-all hover:border-primary ${
											(avatarValue || '').startsWith('data:')
												? 'border-primary ring-2 ring-primary ring-offset-2 ring-offset-background'
												: 'border-border'
										}`}>
										{(avatarValue || '').startsWith('data:') ? (
											<img
												src={avatarValue}
												alt="Custom avatar"
												className="h-full w-full rounded-full object-cover"
											/>
										) : (
											<Upload className="h-4 w-4 text-muted-foreground" />
										)}
									</div>
									<input
										ref={fileInputRef}
										type="file"
										accept="image/*"
										className="hidden"
										onChange={handleImageFileUpload}
									/>

									{/* Color Picker Avatar */}
									<div
										onClick={() => colorInputRef.current?.click()}
										className={`relative flex h-12 w-12 cursor-pointer items-center justify-center overflow-hidden rounded-full border transition-all hover:border-primary ${
											isSolidColorAvatar(avatarValue)
												? 'border-primary ring-2 ring-primary ring-offset-2 ring-offset-background'
												: 'border-border'
										}`}
										style={{
											backgroundColor: isSolidColorAvatar(avatarValue)
												? avatarValue
												: undefined,
										}}>
										{!isSolidColorAvatar(avatarValue) && (
											<Palette className="h-4 w-4 text-muted-foreground" />
										)}
									</div>
									<input
										ref={colorInputRef}
										type="color"
										className="pointer-events-none absolute h-12 w-12 opacity-0"
										value={
											isSolidColorAvatar(avatarValue)
												? avatarValue
												: '#3b82f6'
										}
										onChange={e => setAvatarValue(e.target.value)}
									/>

									{/* Dokploy Preset Avatar Images */}
									{PRESET_AVATARS.map((imgUrl, idx) => (
										<div
											key={idx}
											onClick={() => setAvatarValue(imgUrl)}
											className={`h-12 w-12 cursor-pointer overflow-hidden rounded-full border transition-all hover:scale-105 ${
												avatarValue === imgUrl
													? 'border-primary ring-2 ring-primary ring-offset-2 ring-offset-background'
													: 'border-border hover:border-primary'
											}`}>
											<img
												src={imgUrl}
												alt={`Preset avatar ${idx}`}
												className="h-full w-full rounded-full object-cover"
											/>
										</div>
									))}
								</div>
							</div>

							<div className="flex justify-end border-t border-border/40 pt-2">
								<Button
									type="submit"
									disabled={updateUserMutation.isPending}
									className="h-9 cursor-pointer bg-primary px-5 text-xs font-bold text-primary-foreground hover:bg-primary/90">
									{updateUserMutation.isPending
										? 'Saving...'
										: 'Save Profile Changes'}
								</Button>
							</div>
						</form>
					)}
				</CardContent>
			</Card>
		</div>
	);
}
