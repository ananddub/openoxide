import {MoreHorizontal, Trash2, UserCog} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import {
	DropdownMenu,
	DropdownMenuContent,
	DropdownMenuItem,
	DropdownMenuTrigger,
} from '#/components/ui/dropdown';
import type {MemberItem} from '#/api/live';
import {getAvatarInitials} from '#/lib/avatar-utils';
import {ListRow, RowTile} from '#/components/settings/list-card';
import {ROLES} from './use-users-management';

/** A single team-member row: avatar tile, badges, inline role control, kebab. */
export function MemberRow({
	member,
	self,
	profileName,
	profileAvatar,
	manageable,
	deletable,
	onRole,
	onEditRole,
	onDelete,
}: {
	member: MemberItem;
	self: boolean;
	profileName?: string;
	profileAvatar?: string;
	manageable: boolean;
	deletable: boolean;
	onRole: (role: string) => void;
	onEditRole: () => void;
	onDelete: () => void;
}) {
	const displayName =
		self && profileName
			? profileName
			: member.name || member.email.split('@')[0];
	const displayAvatar =
		self && profileAvatar ? profileAvatar : member.avatar;

	return (
		<ListRow
			icon={
				displayAvatar ? (
					<img
						src={displayAvatar}
						alt={displayName}
						className="size-9 shrink-0 rounded-full bg-muted object-cover"
					/>
				) : (
					<RowTile>
						{getAvatarInitials(displayName || member.email)}
					</RowTile>
				)
			}
			title={displayName}
			badges={
				self ? (
					<span className="rounded-md bg-primary/10 px-1.5 py-0.5 text-[10px] font-medium text-primary">
						you
					</span>
				) : member.banned ? (
					<span className="rounded-md bg-rose-500/10 px-1.5 py-0.5 text-[10px] font-medium text-rose-400">
						Banned
					</span>
				) : undefined
			}
			subtitle={member.email}>
			{manageable ? (
				<Select
					value={member.role.toLowerCase()}
					onValueChange={v => v && onRole(v)}>
					<SelectTrigger className="h-7 w-28 bg-muted/40 text-xs capitalize">
						<SelectValue />
					</SelectTrigger>
					<SelectContent>
						{ROLES.filter(r => r !== 'owner').map(r => (
							<SelectItem key={r} value={r} className="text-xs capitalize">
								{r}
							</SelectItem>
						))}
					</SelectContent>
				</Select>
			) : (
				<span className="rounded-md bg-muted/50 px-2 py-1 text-xs font-medium text-muted-foreground capitalize">
					{member.role.toLowerCase()}
				</span>
			)}
			{deletable && !self && (
				<DropdownMenu>
					<DropdownMenuTrigger
						render={
							<Button
								variant="ghost"
								size="icon"
								className="size-7 rounded-lg text-muted-foreground hover:text-foreground"
							/>
						}>
						<MoreHorizontal className="size-4" />
					</DropdownMenuTrigger>
					<DropdownMenuContent align="end" className="w-44">
						<DropdownMenuItem
							onClick={onEditRole}
							className="gap-2 text-xs">
							<UserCog className="size-3.5" /> Change role
						</DropdownMenuItem>
						<DropdownMenuItem
							onClick={onDelete}
							className="gap-2 text-xs text-destructive focus:text-destructive">
							<Trash2 className="size-3.5" /> Remove from workspace
						</DropdownMenuItem>
					</DropdownMenuContent>
				</DropdownMenu>
			)}
		</ListRow>
	);
}
