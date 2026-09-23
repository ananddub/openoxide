import {useState} from 'react';
import {ChevronDown, Mail, Palette, ShieldCheck} from 'lucide-react';
import {useProfile} from '#/api/live';
import {isSolidColorAvatar, getAvatarInitials} from '#/lib/avatar-utils';
import {useAuthSession} from '#/stores/session-context';
import {usePermissions} from '#/hooks/use-permissions';
import {Button} from '#/components/ui/button';
import {ListCard, RowTile} from '#/components/settings/list-card';
import {InlineProfileEditor} from './inline-profile-editor';

/**
 * Settings → Account — the house card grammar: one bordered card for the
 * identity (avatar, name, email) with the edit control in its action slot,
 * unfolding the inline editor under the row; a second card for the quiet
 * facts. No in-page heading — the crumb and the rail already say
 * "Account".
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
		<div className="flex w-full animate-in flex-col gap-5 duration-150 fade-in">
			<div className="w-full overflow-hidden rounded-xl border bg-card">
				<div className="flex flex-wrap items-center gap-4 px-4 py-3">
					{avatarValue ? (
						<img
							src={avatarValue}
							alt=""
							className="size-12 shrink-0 rounded-lg bg-muted object-cover"
							style={
								isSolidColorAvatar(avatarValue)
									? {backgroundColor: avatarValue}
									: undefined
							}
						/>
					) : (
						<div className="flex size-12 shrink-0 items-center justify-center rounded-lg bg-foreground/[0.05] text-base font-medium text-foreground uppercase">
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
						onClick={() => setExpanded(v => !v)}
						aria-expanded={expanded}
						className="h-8 shrink-0 gap-1.5 rounded-lg text-xs font-medium">
						{expanded ? 'Close' : 'Edit profile'}
						<ChevronDown
							className={`size-3.5 transition-transform duration-300 ease-out ${
								expanded ? 'rotate-180' : ''
							}`}
						/>
					</Button>
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

			<ListCard>
				<FactRow
					label="Email"
					value={email || '—'}
					icon={<Mail className="size-4" />}
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
