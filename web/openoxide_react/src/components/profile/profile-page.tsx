import {useState} from 'react';
import {
	BadgeCheck,
	ChevronDown,
	Mail,
	Palette,
	ShieldCheck,
} from 'lucide-react';
import {useProfile} from '#/api/live';
import {isSolidColorAvatar, getAvatarInitials} from '#/lib/avatar-utils';
import {useAuthSession} from '#/stores/session-context';
import {usePermissions} from '#/hooks/use-permissions';
import {InlineProfileEditor} from './inline-profile-editor';

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
			<div className="rounded-xl bg-card">
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
						<div className="flex size-16 shrink-0 items-center justify-center rounded-full bg-foreground/[0.08] text-xl font-semibold text-foreground uppercase">
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
						expanded
							? 'grid-rows-[1fr] opacity-100'
							: 'grid-rows-[0fr] opacity-0'
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
			<div className="overflow-hidden rounded-xl bg-card">
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
}: {
	label: string;
	value: string;
	icon: React.ReactNode;
}) {
	return (
		<div className="flex items-center gap-4 px-5 py-3.5">
			<span className="flex size-8 shrink-0 items-center justify-center rounded-lg bg-muted/50 text-muted-foreground">
				{icon}
			</span>
			<span className="min-w-0 flex-1 text-sm text-muted-foreground">
				{label}
			</span>
			<span className="shrink-0 truncate text-sm font-medium text-foreground">
				{value}
			</span>
		</div>
	);
}
