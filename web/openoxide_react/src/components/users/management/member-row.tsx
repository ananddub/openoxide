import {MoreHorizontal, Trash2, UserCog} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {
	DropdownMenu,
	DropdownMenuContent,
	DropdownMenuItem,
	DropdownMenuTrigger,
} from '#/components/ui/dropdown';
import type {MemberItem} from '#/api/live';
import {getAvatarInitials} from '#/lib/avatar-utils';
import {ListRow, RowTile, ViewCard} from '#/components/settings/list-card';

type MemberProps = {
	member: MemberItem;
	self: boolean;
	profileName?: string;
	profileAvatar?: string;
	canEditRole: boolean;
	canRemove: boolean;
	onEditRole: () => void;
	onDelete: () => void;
};

function memberView({
	member,
	self,
	profileName,
	profileAvatar,
}: MemberProps) {
	const displayName =
		self && profileName
			? profileName
			: member.name || member.email.split('@')[0];
	const displayAvatar =
		self && profileAvatar ? profileAvatar : member.avatar;
	const role = (member.role ?? '').toLowerCase();
	return {displayName, displayAvatar, role};
}

function Avatar({
	displayName,
	displayAvatar,
	member,
}: {
	displayName: string;
	displayAvatar?: string;
	member: MemberItem;
}) {
	return displayAvatar ? (
		<img
			src={displayAvatar}
			alt={displayName}
			className="size-9 shrink-0 rounded-full bg-muted object-cover"
		/>
	) : (
		<RowTile>{getAvatarInitials(displayName || member.email)}</RowTile>
	);
}

function Chips({
	role,
	self,
	member,
}: {
	role: string;
	self: boolean;
	member: MemberItem;
}) {
	return (
		<>
			{role && (
				<span className="rounded-md bg-foreground/[0.05] px-1.5 py-0.5 text-[10px] font-medium text-muted-foreground capitalize">
					{role}
				</span>
			)}
			{self && (
				<span className="rounded-md bg-primary/10 px-1.5 py-0.5 text-[10px] font-medium text-primary">
					you
				</span>
			)}
			{member.banned && (
				<span className="rounded-md bg-rose-500/10 px-1.5 py-0.5 text-[10px] font-medium text-rose-400">
					Banned
				</span>
			)}
		</>
	);
}

function Menu({
	role,
	canEditRole,
	canRemove,
	onEditRole,
	onDelete,
}: {
	role: string;
	canEditRole: boolean;
	canRemove: boolean;
	onEditRole: () => void;
	onDelete: () => void;
}) {
	if (!(canEditRole || canRemove)) return null;
	return (
		<DropdownMenu>
			<DropdownMenuTrigger
				render={
					<Button
						variant="ghost"
						size="icon"
						aria-label="Member actions"
						className="size-7 rounded-lg text-muted-foreground hover:text-foreground focus-visible:ring-0"
					/>
				}>
				<MoreHorizontal className="size-4" />
			</DropdownMenuTrigger>
			<DropdownMenuContent align="end" className="w-44">
				{canEditRole && role !== 'owner' && (
					<DropdownMenuItem
						onClick={onEditRole}
						className="cursor-pointer gap-2 text-xs">
						<UserCog className="size-3.5" /> Change role
					</DropdownMenuItem>
				)}
				{canRemove && (
					<DropdownMenuItem
						onClick={onDelete}
						className="cursor-pointer gap-2 text-xs text-destructive focus:text-destructive">
						<Trash2 className="size-3.5" /> Remove from workspace
					</DropdownMenuItem>
				)}
			</DropdownMenuContent>
		</DropdownMenu>
	);
}

/** A single team-member row. The role is always visible as a quiet chip — the actions that change it live in the hover kebab, so a row never looks different just because someone can edit it. */
export function MemberRow(props: MemberProps) {
	const {member, self} = props;
	const {displayName, displayAvatar, role} = memberView(props);
	const canAct = (props.canEditRole || props.canRemove) && !self;

	return (
		<ListRow
			icon={
				<Avatar
					displayName={displayName}
					displayAvatar={displayAvatar}
					member={member}
				/>
			}
			title={displayName}
			badges={<Chips role={role} self={self} member={member} />}
			subtitle={member.email}>
			{canAct && (
				<Menu
					role={role}
					canEditRole={props.canEditRole}
					canRemove={props.canRemove}
					onEditRole={props.onEditRole}
					onDelete={props.onDelete}
				/>
			)}
		</ListRow>
	);
}

/** Grid counterpart of {@link MemberRow}. */
export function MemberCard(props: MemberProps) {
	const {member, self} = props;
	const {displayName, displayAvatar, role} = memberView(props);
	const canAct = (props.canEditRole || props.canRemove) && !self;

	return (
		<ViewCard>
			<div className="flex min-w-0 items-start gap-3">
				<Avatar
					displayName={displayName}
					displayAvatar={displayAvatar}
					member={member}
				/>
				<div className="min-w-0 flex-1">
					<span className="block truncate text-sm font-medium text-foreground">
						{displayName}
					</span>
					<p className="mt-0.5 truncate text-[11px] text-muted-foreground">
						{member.email}
					</p>
				</div>
			</div>
			<div className="flex items-end justify-between gap-2">
				<div className="flex min-w-0 flex-wrap items-center gap-1.5">
					<Chips role={role} self={self} member={member} />
				</div>
				{canAct && (
					<div className="shrink-0 opacity-0 transition-opacity group-focus-within:opacity-100 group-hover:opacity-100">
						<Menu
							role={role}
							canEditRole={props.canEditRole}
							canRemove={props.canRemove}
							onEditRole={props.onEditRole}
							onDelete={props.onDelete}
						/>
					</div>
				)}
			</div>
		</ViewCard>
	);
}
