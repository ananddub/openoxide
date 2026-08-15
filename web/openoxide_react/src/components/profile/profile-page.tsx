import {useState, useEffect, useRef} from 'react';
import {
	User,
	KeyRound,
	Plus,
	Copy,
	Check,
	Trash2,
	Loader2,
	CheckCircle2,
	Palette,
	Upload,
	Lock,
} from 'lucide-react';
import {$api} from '#/api/query';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {
	Card,
	CardContent,
	CardDescription,
	CardHeader,
	CardTitle,
} from '#/components/ui/card';
import {
	Dialog,
	DialogContent,
	DialogDescription,
	DialogHeader,
	DialogTitle,
} from '#/components/ui/dialog';
import {
	Table,
	TableBody,
	TableCell,
	TableHead,
	TableHeader,
	TableRow,
} from '#/components/ui/table';
import {isSolidColorAvatar, getAvatarInitials} from '#/lib/avatar-utils';
import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';
import {useAuthWhoAmI} from 'virtual:openoxide-live';
import {useAuthStore} from '#/stores/auth-store';
import {useAppStore} from '#/stores/app-store';

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
	const {
		data: whoamiData,
		loading: isLoading,
		setData: setWhoamiData,
	} = useAuthWhoAmI();
	const {data: twoFactorStatus} = $api.useQuery('get', '/auth/2fa/status' as any);

	// Real API Tokens Query
	const {
		data: apiTokensData,
		refetch: refetchApiTokens,
		isLoading: isLoadingTokens,
	} = $api.useQuery('get', '/auth/api-tokens' as any);

	const updateUserMutation = $api.useMutation('patch', '/auth/user', {
		onSuccess: (data: any) => {
			if (data) {
				setWhoamiData(data);
				useAuthStore.getState().setAuth({
					id: data.user_id,
					email: data.email || '',
					firstName: data.first_name,
					lastName: data.last_name,
				});
				useAppStore.getState().setProfile({
					id: data.user_id,
					email: data.email || '',
					name: `${data.first_name || ''} ${data.last_name || ''}`.trim(),
					avatar: data.avatar,
				} as any);
			}
			toast.success('Profile Updated Successfully');
		},
		onError: (err: any) => {
			toast.error(formatApiError(err) || 'Error updating profile');
		},
	});

	const createApiTokenMutation = $api.useMutation('post', '/auth/api-tokens' as any);
	const revokeApiTokenMutation = $api.useMutation(
		'delete',
		'/auth/api-tokens/{id}' as any,
	);

	const [firstName, setFirstName] = useState('');
	const [lastName, setLastName] = useState('');
	const [email, setEmail] = useState('');
	const [avatarValue, setAvatarValue] = useState<string>(PRESET_AVATARS[0]);

	const colorInputRef = useRef<HTMLInputElement>(null);
	const fileInputRef = useRef<HTMLInputElement>(null);

	// API Keys Modal & State
	const [isCreateKeyOpen, setIsCreateKeyOpen] = useState(false);
	const [keyName, setKeyName] = useState('');
	const [keyConfirmPassword, setKeyConfirmPassword] = useState('');
	const [newlyCreatedKey, setNewlyCreatedKey] = useState<string | null>(null);
	const [copiedKeyId, setCopiedKeyId] = useState<string | number | null>(null);
	const [isCreatingKey, setIsCreatingKey] = useState(false);

	useEffect(() => {
		if (!whoamiData) return;

		setEmail(whoamiData.email ?? '');
		setFirstName(whoamiData.first_name ?? '');
		setLastName(whoamiData.last_name ?? '');
		setAvatarValue(whoamiData.avatar);
	}, [whoamiData]);

	const handleImageFileUpload = (e: React.ChangeEvent<HTMLInputElement>) => {
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
				body: {
					first_name: firstName.trim() || undefined,
					last_name: lastName.trim() || undefined,
					avatar: avatarValue,
				},
			});
		} catch {
			// handled in onError callback
		}
	};

	const handleCreateApiKey = async (e: React.FormEvent) => {
		e.preventDefault();
		if (!keyName.trim()) {
			toast.error('Please enter a key name');
			return;
		}
		if (!keyConfirmPassword) {
			toast.error('Account password is required to generate API Key');
			return;
		}

		setIsCreatingKey(true);
		try {
			const res = (await createApiTokenMutation.mutateAsync({
				body: {
					name: keyName.trim(),
					password: keyConfirmPassword,
				},
			} as any)) as any;

			const tokenString = res?.token || '';
			setNewlyCreatedKey(tokenString);
			setKeyName('');
			setKeyConfirmPassword('');
			refetchApiTokens();
			toast.success('API Key Created');
		} catch (error) {
			toast.error(formatApiError(error));
		} finally {
			setIsCreatingKey(false);
		}
	};

	const handleDeleteApiKey = async (id: number | string) => {
		try {
			await revokeApiTokenMutation.mutateAsync({
				params: {
					path: {
						id: Number(id),
					},
				},
			} as any);
			toast.success('API Key Revoked');
			refetchApiTokens();
		} catch (error) {
			toast.error(formatApiError(error));
		}
	};

	const handleCopyKey = (key: string, id: string | number) => {
		navigator.clipboard.writeText(key);
		setCopiedKeyId(id);
		toast.success('Copied to clipboard');
		setTimeout(() => setCopiedKeyId(null), 2000);
	};

	const userInitials = getAvatarInitials(`${firstName} ${lastName}`.trim() || email);

	return (
		<div className="w-full max-w-5xl mx-auto flex flex-col gap-6 pb-12 animate-in fade-in duration-150">
			{/* Account Card */}
			<Card className="bg-card border border-border/80 rounded-2xl shadow-xs overflow-hidden">
				<CardHeader className="flex flex-row items-center justify-between flex-wrap gap-4 pb-4">
					<div>
						<CardTitle className="text-xl flex flex-row items-center gap-2 font-bold">
							<User className="size-5 text-muted-foreground" />
							<span>Account</span>
						</CardTitle>
						<CardDescription className="text-xs text-muted-foreground mt-0.5">
							Manage your avatar and view profile details.
						</CardDescription>
					</div>
				</CardHeader>

				<CardContent className="space-y-6 pt-4 border-t border-border/60">
					{isLoading ? (
						<div className="flex flex-row gap-2 items-center justify-center text-sm text-muted-foreground min-h-[25vh]">
							<Loader2 className="animate-spin size-4 text-primary" />
							<span>Loading account profile...</span>
						</div>
					) : (
						<form onSubmit={handleSubmitProfile} className="flex flex-col gap-6">
							{/* Editable Account Information Grid */}
							<div className="flex flex-col gap-3 p-4 bg-muted/20 border border-border/60 rounded-xl">
								<div className="flex items-center gap-1.5 text-xs font-bold text-foreground">
									<User className="size-3.5 text-primary" />
									<span>Account Information</span>
								</div>

								<div className="grid grid-cols-1 sm:grid-cols-2 gap-4">
									<div className="flex flex-col gap-1.5">
										<label className="text-[11px] font-semibold text-muted-foreground">First Name</label>
										<Input
											placeholder="First Name"
											value={firstName}
											onChange={e => setFirstName(e.target.value)}
											className="h-9 text-xs bg-muted/30 border-border/70"
										/>
									</div>

									<div className="flex flex-col gap-1.5">
										<label className="text-[11px] font-semibold text-muted-foreground">Last Name</label>
										<Input
											placeholder="Last Name"
											value={lastName}
											onChange={e => setLastName(e.target.value)}
											className="h-9 text-xs bg-muted/30 border-border/70"
										/>
									</div>
								</div>
							</div>

							{/* Dokploy 1:1 Avatar Selection */}
							<div className="flex flex-col gap-2 pt-2">
								<label className="text-xs font-semibold text-foreground">Avatar Selection</label>

								<div className="flex flex-row flex-wrap items-center gap-3">
									{/* Default Initials Avatar */}
									<div
										onClick={() => setAvatarValue('')}
										className={`h-12 w-12 rounded-full border flex items-center justify-center font-bold text-xs cursor-pointer transition-all ${
											!avatarValue ? 'border-primary ring-2 ring-primary ring-offset-2 ring-offset-background' : 'border-border hover:border-primary'
										}`}
									>
										{userInitials}
									</div>

									{/* Custom Uploaded Avatar */}
									<div
										onClick={() => fileInputRef.current?.click()}
										className={`h-12 w-12 rounded-full border border-dashed hover:border-primary transition-all flex items-center justify-center bg-muted/40 cursor-pointer overflow-hidden relative ${
											avatarValue.startsWith('data:') ? 'border-primary ring-2 ring-primary ring-offset-2 ring-offset-background' : 'border-border'
										}`}
									>
										{avatarValue.startsWith('data:') ? (
											<img src={avatarValue} alt="Custom avatar" className="h-full w-full object-cover rounded-full" />
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
										className={`h-12 w-12 rounded-full border hover:border-primary transition-all flex items-center justify-center cursor-pointer overflow-hidden relative ${
											isSolidColorAvatar(avatarValue) ? 'ring-2 ring-primary ring-offset-2 ring-offset-background border-primary' : 'border-border'
										}`}
										style={{
											backgroundColor: isSolidColorAvatar(avatarValue) ? avatarValue : undefined,
										}}
									>
										{!isSolidColorAvatar(avatarValue) && <Palette className="h-4 w-4 text-muted-foreground" />}
									</div>
									<input
										ref={colorInputRef}
										type="color"
										className="absolute opacity-0 pointer-events-none w-12 h-12"
										value={isSolidColorAvatar(avatarValue) ? avatarValue : '#3b82f6'}
										onChange={e => setAvatarValue(e.target.value)}
									/>

									{/* Dokploy Preset Avatar Images */}
									{PRESET_AVATARS.map((imgUrl, idx) => (
										<div
											key={idx}
											onClick={() => setAvatarValue(imgUrl)}
											className={`h-12 w-12 rounded-full border overflow-hidden cursor-pointer transition-all hover:scale-105 ${
												avatarValue === imgUrl ? 'ring-2 ring-primary ring-offset-2 ring-offset-background border-primary' : 'border-border hover:border-primary'
											}`}
										>
											<img src={imgUrl} alt={`Preset avatar ${idx}`} className="h-full w-full object-cover rounded-full" />
										</div>
									))}
								</div>
							</div>

							<div className="flex justify-end pt-2 border-t border-border/40">
								<Button
									type="submit"
									disabled={updateUserMutation.isPending}
									className="h-9 text-xs font-bold px-5 bg-primary text-primary-foreground hover:bg-primary/90 cursor-pointer"
								>
									{updateUserMutation.isPending ? 'Saving...' : 'Save Profile Changes'}
								</Button>
							</div>
						</form>
					)}
				</CardContent>
			</Card>
		</div>
	);
}
