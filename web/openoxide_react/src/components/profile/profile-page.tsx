import {useState} from 'react';
import {Mail, Palette, Pencil, ShieldCheck, User} from 'lucide-react';
import {useProfile} from '#/api/live';
import {isSolidColorAvatar, getAvatarInitials} from '#/lib/avatar-utils';
import {useAuthSession} from '#/stores/session-context';
import {usePermissions} from '#/hooks/permissions/use-permissions';
import {Button} from '#/components/ui/button';
import {ListCard, RowTile} from '#/components/settings/list-card';
import {ProfileEditorSheet} from './profile-editor-sheet';

/** Settings → Account — a quiet identity card up top, the facts below, and editing that opens the shared right-side sheet. */
export function ProfilePage() {
	const {data: storeProfile} = useProfile();
	const {user: authUser} = useAuthSession();
	const {role} = usePermissions();
	const roleLabel = role && role !== 'NONE' ? role : null;

	const fullName = storeProfile?.name || authUser?.firstName || '';
	const email = storeProfile?.email || authUser?.email || '';
	const avatarValue = storeProfile?.avatar || '';
	const initials = getAvatarInitials(fullName || email);

	const [editOpen, setEditOpen] = useState(false);

	return (
		<div className="mx-auto flex w-full max-w-3xl animate-in flex-col gap-5 duration-150 fade-in">
			{/* Identity */}
			<div className="flex flex-wrap items-center gap-4 rounded-xl border bg-card px-5 py-4">
				{avatarValue ? (
					<img
						src={avatarValue}
						alt=""
						className="size-14 shrink-0 rounded-xl bg-muted object-cover"
						style={
							isSolidColorAvatar(avatarValue)
								? {backgroundColor: avatarValue}
								: undefined
						}
					/>
				) : (
					<div className="flex size-14 shrink-0 items-center justify-center rounded-xl bg-foreground/[0.05] text-lg font-medium text-foreground uppercase">
						{initials}
					</div>
				)}

				<div className="min-w-0 flex-1">
					<div className="flex min-w-0 flex-wrap items-center gap-2">
						<span className="truncate text-sm font-medium text-foreground">
							{fullName || 'Unnamed'}
						</span>
						{roleLabel && (
							<span className="flex items-center gap-1 rounded-full bg-primary/10 px-2 py-0.5 text-[10px] font-medium text-primary">
								<ShieldCheck className="size-3" />
								{roleLabel}
							</span>
						)}
					</div>
					<p className="mt-0.5 flex items-center gap-1.5 truncate text-xs text-muted-foreground">
						<Mail className="size-3 shrink-0" />
						{email}
					</p>
				</div>

				<Button
					variant="outline"
					size="sm"
					type="button"
					onClick={() => setEditOpen(true)}
					className="h-8 shrink-0 gap-1.5 rounded-lg text-xs font-medium">
					<Pencil className="size-3.5" />
					Edit profile
				</Button>
			</div>

			{/* Quiet facts */}
			<ListCard>
				<FactRow
					label="Name"
					value={fullName || '—'}
					icon={<User className="size-4" />}
				/>
				<FactRow
					label="Email"
					value={email || '—'}
					icon={<Mail className="size-4" />}
				/>
				<FactRow
					label="Role"
					value={roleLabel || '—'}
					icon={<ShieldCheck className="size-4" />}
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
					icon={<Palette className="size-4" />}
				/>
			</ListCard>

			<ProfileEditorSheet
				open={editOpen}
				onOpenChange={setEditOpen}
				initialFirstName={fullName.split(' ')[0] ?? ''}
				initialLastName={fullName.split(' ').slice(1).join(' ')}
				initialAvatar={avatarValue}
				email={email}
			/>
		</div>
	);
}

function FactRow({
	label,
	value,
	icon,
}: {
	label: string;
	value: string;
	icon: React.ReactNode;
}) {
	return (
		<div className="flex items-center gap-3.5 px-4 py-3">
			<RowTile>{icon}</RowTile>
			<span className="min-w-0 flex-1 text-sm text-muted-foreground">
				{label}
			</span>
			<span className="shrink-0 truncate text-sm font-medium text-foreground">
				{value}
			</span>
		</div>
	);
}
