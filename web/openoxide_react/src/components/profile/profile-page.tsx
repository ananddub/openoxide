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
import {useQueryClient} from '@tanstack/react-query';
import {isSolidColorAvatar, getAvatarInitials} from '#/lib/avatar-utils';
import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';

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
	const queryClient = useQueryClient();
	const {data: whoamiData, isLoading} = $api.useQuery('get', '/auth/whoami');
	const {data: twoFactorStatus} = $api.useQuery('get', '/auth/2fa/status' as any);

	// Real API Tokens Query
	const {
		data: apiTokensData,
		refetch: refetchApiTokens,
		isLoading: isLoadingTokens,
	} = $api.useQuery('get', '/auth/api-tokens' as any);

	const updateUserMutation = $api.useMutation('patch', '/auth/user', {
		onSuccess: () => {
			queryClient.invalidateQueries({queryKey: ['get', '/auth/whoami']});
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
		if (whoamiData) {
			if (whoamiData.email) setEmail(whoamiData.email);
			if (whoamiData.first_name) setFirstName(whoamiData.first_name);
			if (whoamiData.last_name) setLastName(whoamiData.last_name);
			if (whoamiData.avatar) setAvatarValue(whoamiData.avatar);
		}
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
					email: email.trim() || undefined,
					avatar: avatarValue || undefined,
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

								<div className="flex flex-col gap-1.5">
									<label className="text-[11px] font-semibold text-muted-foreground">Email Address</label>
									<Input
										type="email"
										placeholder="user@example.com"
										value={email}
										onChange={e => setEmail(e.target.value)}
										className="h-9 text-xs bg-muted/30 border-border/70 font-mono"
									/>
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

			{/* Real Backend API Keys Card */}
			<Card className="bg-card border border-border/80 rounded-2xl shadow-xs overflow-hidden">
				<CardHeader className="flex flex-row items-center justify-between flex-wrap gap-4 pb-4">
					<div>
						<CardTitle className="text-xl flex flex-row items-center gap-2 font-bold">
							<KeyRound className="size-5 text-muted-foreground" />
							<span>API Keys</span>
						</CardTitle>
						<CardDescription className="text-xs text-muted-foreground mt-0.5">
							Manage your Personal Access Tokens for authenticating with the OpenOxide API.
						</CardDescription>
					</div>

					<Button
						size="sm"
						onClick={() => {
							setNewlyCreatedKey(null);
							setKeyConfirmPassword('');
							setIsCreateKeyOpen(true);
						}}
						className="h-8 text-xs font-semibold gap-1.5 cursor-pointer"
					>
						<Plus className="size-3.5" />
						<span>Create API Key</span>
					</Button>
				</CardHeader>

				<CardContent className="space-y-4 pt-4 border-t border-border/60">
					{isLoadingTokens ? (
						<div className="flex flex-row gap-2 items-center justify-center text-sm text-muted-foreground py-8">
							<Loader2 className="animate-spin size-4 text-primary" />
							<span>Loading API Keys...</span>
						</div>
					) : !apiTokensData || !Array.isArray(apiTokensData) || apiTokensData.length === 0 ? (
						<div className="flex flex-col items-center justify-center p-8 text-center border border-dashed border-border rounded-xl bg-muted/10 gap-2">
							<KeyRound className="size-8 text-muted-foreground/40" />
							<span className="text-sm font-semibold text-foreground">No API keys created</span>
							<span className="text-xs text-muted-foreground max-w-sm">
								Create a personal access token to interact with OpenOxide CLI and REST APIs programmatically.
							</span>
						</div>
					) : (
						<Table>
							<TableHeader className="bg-muted/40 border-b border-border/60">
								<TableRow className="hover:bg-transparent border-border/60">
									<TableHead className="px-4 py-3 text-[11px] font-bold uppercase tracking-wider text-muted-foreground">Name</TableHead>
									<TableHead className="px-4 py-3 text-[11px] font-bold uppercase tracking-wider text-muted-foreground">Key Prefix</TableHead>
									<TableHead className="text-center px-4 py-3 text-[11px] font-bold uppercase tracking-wider text-muted-foreground">Created At</TableHead>
									<TableHead className="text-right px-4 py-3 text-[11px] font-bold uppercase tracking-wider text-muted-foreground">Actions</TableHead>
								</TableRow>
							</TableHeader>
							<TableBody>
								{apiTokensData.map((keyItem: any) => (
									<TableRow key={keyItem.id} className="border-b border-border/40 hover:bg-muted/20 transition-colors">
										<TableCell className="px-4 py-3.5 font-semibold text-xs text-foreground">
											{keyItem.name}
										</TableCell>
										<TableCell className="px-4 py-3.5 text-xs font-mono text-muted-foreground">
											{keyItem.token_prefix || 'pat_'}...
										</TableCell>
										<TableCell className="text-center px-4 py-3.5 text-xs text-muted-foreground font-mono">
											{keyItem.created_at ? new Date(keyItem.created_at * 1000).toLocaleDateString() : 'N/A'}
										</TableCell>
										<TableCell className="text-right px-4 py-3.5">
											<div className="flex items-center justify-end gap-1">
												<Button
													variant="ghost"
													size="icon"
													onClick={() => handleDeleteApiKey(keyItem.id)}
													className="size-8 text-muted-foreground hover:text-rose-500 cursor-pointer"
													title="Revoke Token"
												>
													<Trash2 className="size-3.5" />
												</Button>
											</div>
										</TableCell>
									</TableRow>
								))}
							</TableBody>
						</Table>
					)}
				</CardContent>
			</Card>

			{/* Create API Key Dialog */}
			<Dialog open={isCreateKeyOpen} onOpenChange={open => !open && setIsCreateKeyOpen(false)}>
				<DialogContent className="sm:max-w-md bg-card border border-border shadow-2xl p-6 flex flex-col gap-4 rounded-2xl">
					<DialogHeader className="space-y-1">
						<DialogTitle className="text-base font-bold text-foreground">Create Personal Access Token</DialogTitle>
						<DialogDescription className="text-xs text-muted-foreground">
							Enter a descriptive name and confirm your password to generate a new API token.
						</DialogDescription>
					</DialogHeader>

					{newlyCreatedKey ? (
						<div className="flex flex-col gap-3 py-2">
							<div className="p-3 bg-muted/40 border border-border rounded-xl space-y-1.5">
								<span className="text-[11px] font-bold text-emerald-500 uppercase tracking-wider flex items-center gap-1">
									<CheckCircle2 className="size-3.5" /> Save your token now (won't be shown again)
								</span>
								<div className="flex items-center gap-2">
									<code className="text-xs font-mono bg-background p-2 rounded border border-border flex-1 truncate select-all">
										{newlyCreatedKey}
									</code>
									<Button
										size="sm"
										onClick={() => handleCopyKey(newlyCreatedKey, 'modal')}
										className="h-8 text-xs px-3"
									>
										Copy
									</Button>
								</div>
							</div>
							<div className="flex justify-end pt-2">
								<Button size="sm" onClick={() => setIsCreateKeyOpen(false)} className="h-8 text-xs px-4">
									Done
								</Button>
							</div>
						</div>
					) : (
						<form onSubmit={handleCreateApiKey} className="flex flex-col gap-4">
							<div className="flex flex-col gap-1.5">
								<label className="text-xs font-semibold text-foreground">Token Name <span className="text-destructive">*</span></label>
								<Input
									placeholder="e.g. CLI Access Token"
									value={keyName}
									onChange={e => setKeyName(e.target.value)}
									className="h-9 text-xs bg-muted/20 border-border/60"
									autoFocus
								/>
							</div>

							<div className="flex flex-col gap-1.5">
								<label className="text-xs font-semibold text-foreground">Account Password <span className="text-destructive">*</span></label>
								<Input
									type="password"
									placeholder="Confirm your password"
									value={keyConfirmPassword}
									onChange={e => setKeyConfirmPassword(e.target.value)}
									className="h-9 text-xs bg-muted/20 border-border/60"
								/>
							</div>

							<div className="flex items-center justify-end gap-2 pt-2 border-t border-border/40">
								<Button
									type="button"
									variant="ghost"
									onClick={() => setIsCreateKeyOpen(false)}
									className="h-8 text-xs font-semibold"
								>
									Cancel
								</Button>
								<Button type="submit" disabled={isCreatingKey} className="h-8 text-xs font-bold px-4 cursor-pointer">
									{isCreatingKey ? 'Creating...' : 'Create Token'}
								</Button>
							</div>
						</form>
					)}
				</DialogContent>
			</Dialog>
		</div>
	);
}
