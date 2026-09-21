import {useState, useEffect, useRef} from 'react';
import {
	BadgeCheck,
	Camera,
	ChevronDown,
	Loader2,
	Mail,
	Palette,
	ShieldCheck,
	Upload,
} from 'lucide-react';
import {useUserUpdate, useProfile} from '#/api/live';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {isSolidColorAvatar, getAvatarInitials} from '#/lib/avatar-utils';
import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';
import {useAuthSession} from '#/stores/session-context';
import {usePermissions} from '#/hooks/use-permissions';

const PRESET_AVATARS = [
	'https://api.dicebear.com/7.x/bottts/svg?seed=OpenOxide1',
	'https://api.dicebear.com/7.x/bottts/svg?seed=OpenOxide2',
	'https://api.dicebear.com/7.x/bottts/svg?seed=OpenOxide3',
	'https://api.dicebear.com/7.x/bottts/svg?seed=OpenOxide4',
	'https://api.dicebear.com/7.x/avataaars/svg?seed=Alex',
	'https://api.dicebear.com/7.x/avataaars/svg?seed=Sarah',
	'https://api.dicebear.com/7.x/avataaars/svg?seed=Elena',
	'https://api.dicebear.com/7.x/avataaars/svg?seed=David',
	'https://api.dicebear.com/7.x/shapes/svg?seed=Shape1',
	'https://api.dicebear.com/7.x/shapes/svg?seed=Shape2',
];

/**
 * Settings → Account. The identity card carries an expand chevron instead
 * of an Edit button: clicking it unfolds the editor (name fields + avatar
 * picker) under the identity row with a smooth height + fade animation.
 * Collapsing discards unsaved edits.
 */
export function ProfilePage() {
	const {data: storeProfile} = useProfile();
	const {user: authUser} = useAuthSession();
	const {role} = usePermissions();
	const roleLabel = role && role !== 'NONE' ? role : null;

	const fullName = storeProfile?.name || authUser?.firstName || '';
	const email = storeProfile?.email || authUser?.email || '';
	const avatarValue = storeProfile?.avatar || '';
	const initials = getAvatarInitials(fullName || email);

	const [expanded, setExpanded] = useState(false);

	return (
		<div className="flex w-full animate-in flex-col gap-6 duration-150 fade-in">
			{/* Identity card — chevron expands the editor beneath the row */}
			<div className="rounded-2xl bg-card">
				<div className="flex flex-wrap items-center gap-5 p-6 pb-4">
					{avatarValue ? (
						<img
							src={avatarValue}
							alt=""
							className="size-16 shrink-0 rounded-full bg-muted object-cover"
							style={
								isSolidColorAvatar(avatarValue)
									? {backgroundColor: avatarValue}
									: undefined
							}
						/>
					) : (
						<div className="flex size-16 shrink-0 items-center justify-center rounded-full bg-foreground/[0.08] text-xl font-semibold uppercase text-foreground">
							{initials}
						</div>
					)}
					<div className="min-w-0 flex-1">
						<div className="flex flex-wrap items-center gap-2">
							<h2 className="truncate text-lg font-semibold text-foreground">
								{fullName || 'Unnamed'}
							</h2>
							{roleLabel && (
								<span className="inline-flex items-center gap-1 rounded-full bg-primary/10 px-2 py-0.5 text-[11px] font-medium text-primary">
									<ShieldCheck className="size-3" />
									{roleLabel}
								</span>
							)}
						</div>
						<p className="mt-0.5 flex items-center gap-1.5 truncate text-sm text-muted-foreground">
							<Mail className="size-3.5 shrink-0" />
							{email}
						</p>
					</div>
					<button
						type="button"
						onClick={() => setExpanded(v => !v)}
						aria-expanded={expanded}
						title={expanded ? 'Collapse' : 'Expand to edit profile'}
						className="flex size-9 shrink-0 items-center justify-center rounded-xl text-muted-foreground transition-colors hover:bg-input hover:text-foreground">
						<span className="sr-only">
							{expanded ? 'Collapse' : 'Expand to edit profile'}
						</span>
						<ChevronDown
							className={`size-4 transition-transform duration-300 ease-out ${
								expanded ? 'rotate-180' : ''
							}`}
						/>
					</button>
				</div>

				{/* Animated expand region: grid-rows 0fr → 1fr + fade */}
				<div
					className={`grid transition-[grid-template-rows,opacity] duration-300 ease-out ${
						expanded ? 'grid-rows-[1fr] opacity-100' : 'grid-rows-[0fr] opacity-0'
					}`}>
					<div className="overflow-hidden">
						<InlineProfileEditor
							open={expanded}
							initialFirstName={fullName.split(' ')[0] ?? ''}
							initialLastName={fullName.split(' ').slice(1).join(' ')}
							initialAvatar={avatarValue}
							email={email}
							onDone={() => setExpanded(false)}
						/>
					</div>
				</div>
			</div>

			{/* Quiet account facts */}
			<div className="overflow-hidden rounded-2xl bg-card">
				<div className="flex items-center gap-4 border-b border-border/40 px-5 py-4">
					<BadgeCheck className="size-4 shrink-0 text-muted-foreground" />
					<div className="min-w-0 flex-1">
						<p className="text-sm font-medium text-foreground">
							Sign-in identity
						</p>
						<p className="text-xs text-muted-foreground">
							Name and avatar are visible to every member of your
							workspaces.
						</p>
					</div>
				</div>
				<div className="divide-y divide-border/40">
					<FactRow
						label="Email"
						value={email || '—'}
						icon={<Mail className="size-3.5" />}
					/>
					<FactRow
						label="Avatar"
						value={
							!avatarValue
								? 'Initials'
								: isSolidColorAvatar(avatarValue)
									? 'Solid color'
									: avatarValue.startsWith('data:')
										? 'Uploaded image'
										: 'Preset'
						}
						icon={<Palette className="size-3.5" />}
					/>
				</div>
			</div>
		</div>
	);
}

function FactRow({
	label,
	value,
	icon,
	mono,
}: {
	label: string;
	value: string;
	icon: React.ReactNode;
	mono?: boolean;
}) {
	return (
		<div className="flex items-center gap-4 px-5 py-3.5">
			<span className="flex size-8 shrink-0 items-center justify-center rounded-lg bg-muted/50 text-muted-foreground">
				{icon}
			</span>
			<span className="min-w-0 flex-1 text-sm text-muted-foreground">
				{label}
			</span>
			<span
				className={`shrink-0 truncate text-sm text-foreground ${
					mono ? 'font-mono text-xs' : 'font-medium'
				}`}>
				{value}
			</span>
		</div>
	);
}

/** The editor body that unfolds under the identity row. */
function InlineProfileEditor({
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

	// Re-seed from the live profile every time the card opens, so a collapse
	// (Cancel) discards unsaved edits.
	useEffect(() => {
		if (open) {
			setFirstName(initialFirstName);
			setLastName(initialLastName);
			setAvatarValue(initialAvatar);
		}
	}, [open, initialFirstName, initialLastName, initialAvatar]);

	const colorInputRef = useRef<HTMLInputElement>(null);
	const fileInputRef = useRef<HTMLInputElement>(null);

	const userInitials = getAvatarInitials(
		`${firstName} ${lastName}`.trim() || email,
	);

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
			setAvatarValue(event.target?.result as string);
			toast.success('Avatar image loaded');
		};
		reader.readAsDataURL(file);
	};

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
		<form
			onSubmit={handleSubmit}
			className="border-t border-border/40 px-6 pb-6 pt-5">
			<div className="flex flex-col gap-5">
				{/* Name fields */}
				<div className="grid grid-cols-1 gap-3 sm:grid-cols-2">
					<label className="flex flex-col gap-1.5">
						<span className="text-xs font-medium text-foreground/80">
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
						<span className="text-xs font-medium text-foreground/80">
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

				{/* Avatar picker */}
				<div className="rounded-xl bg-muted/20 p-4">
					<div className="mb-3 flex items-center justify-between gap-3">
						<p className="text-[11px] font-semibold uppercase tracking-wide text-muted-foreground">
							Choose avatar
						</p>
						<p className="flex items-center gap-1.5 text-[11px] text-muted-foreground">
							<Camera className="size-3" />
							Upload up to 2 MB, or pick a color.
						</p>
					</div>
					<div className="flex flex-wrap items-center gap-2.5">
						<button
							type="button"
							onClick={() => setAvatarValue('')}
							aria-label="Use initials"
							className={`flex size-10 cursor-pointer items-center justify-center rounded-full text-[11px] font-bold uppercase transition-all ${
								!avatarValue
									? 'bg-foreground/[0.08] text-foreground ring-2 ring-primary'
									: 'bg-foreground/[0.05] text-muted-foreground hover:scale-105'
							}`}>
							{userInitials}
						</button>

						<button
							type="button"
							onClick={() => fileInputRef.current?.click()}
							aria-label="Upload image"
							className={`relative flex size-10 cursor-pointer items-center justify-center overflow-hidden rounded-full bg-muted/60 transition-all hover:scale-105 ${
								(avatarValue || '').startsWith('data:')
									? 'ring-2 ring-primary'
									: ''
							}`}>
							{(avatarValue || '').startsWith('data:') ? (
								<img
									src={avatarValue}
									alt="Custom avatar"
									className="size-full rounded-full object-cover"
								/>
							) : (
								<Upload className="size-4 text-muted-foreground" />
							)}
						</button>
						<input
							ref={fileInputRef}
							type="file"
							accept="image/*"
							className="hidden"
							onChange={handleImageFileUpload}
						/>

						<button
							type="button"
							onClick={() => colorInputRef.current?.click()}
							aria-label="Pick color"
							className={`relative flex size-10 cursor-pointer items-center justify-center overflow-hidden rounded-full bg-muted/60 transition-all hover:scale-105 ${
								isSolidColorAvatar(avatarValue) ? 'ring-2 ring-primary' : ''
							}`}
							style={{
								backgroundColor: isSolidColorAvatar(avatarValue)
									? avatarValue
									: undefined,
							}}>
							{!isSolidColorAvatar(avatarValue) && (
								<Palette className="size-4 text-muted-foreground" />
							)}
						</button>
						<input
							ref={colorInputRef}
							type="color"
							className="pointer-events-none absolute size-10 opacity-0"
							value={
								isSolidColorAvatar(avatarValue) ? avatarValue : '#3b82f6'
							}
							onChange={e => setAvatarValue(e.target.value)}
						/>

						<span className="mx-1 h-5 w-px bg-border/60" aria-hidden />

						{PRESET_AVATARS.map((imgUrl, idx) => (
							<button
								type="button"
								key={idx}
								onClick={() => setAvatarValue(imgUrl)}
								aria-label={`Preset avatar ${idx + 1}`}
								className={`size-10 cursor-pointer overflow-hidden rounded-full transition-all hover:scale-105 ${
									avatarValue === imgUrl ? 'ring-2 ring-primary' : ''
								}`}>
								<img
									src={imgUrl}
									alt=""
									className="size-full rounded-full object-cover"
								/>
							</button>
						))}
					</div>
				</div>

				{/* Actions */}
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
