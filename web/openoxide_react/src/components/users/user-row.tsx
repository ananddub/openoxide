import {MoreHorizontal, ShieldCheck, UserCheck} from 'lucide-react';
import {DialogAction} from '#/components/shared/dialog-action';
import {Badge} from '#/components/ui/badge';
import {Button} from '#/components/ui/button';
import {
	DropdownMenu,
	DropdownMenuContent,
	DropdownMenuItem,
	DropdownMenuLabel,
	DropdownMenuTrigger,
} from '#/components/ui/dropdown';
import {TableCell, TableRow} from '#/components/ui/table';
import type {UserMember} from './show-users';

const ROLE_BADGE_CLASS = (role: string) =>
	role === 'owner'
		? 'border-purple-500/30 bg-purple-500/10 text-purple-500'
		: role === 'admin'
			? 'border-blue-500/30 bg-blue-500/10 text-blue-500'
			: 'border-emerald-500/30 bg-emerald-500/10 text-emerald-500';

/** A single row of the users table: identity, role badge, status, 2FA, join
 * date and the manage kebab (change role / delete). */
export function UserRow({
	member,
	onEditRole,
	onDelete,
}: {
	member: UserMember;
	onEditRole: (member: UserMember) => void;
	onDelete: (id: string, email: string) => void;
}) {
	const isOwner = member.role === 'owner';
	const canManage = !isOwner && !member.isSelf;

	return (
		<TableRow className="transition-colors hover:bg-muted/20">
			<TableCell className="font-medium">
				<div className="flex items-center gap-3">
					<div className="flex size-8 shrink-0 items-center justify-center rounded-full bg-primary/10 text-xs font-bold text-primary">
						{member.email.substring(0, 2).toUpperCase()}
					</div>
					<span className="flex items-center gap-1.5 text-xs font-semibold text-foreground">
						{member.email}
						{member.isSelf && (
							<span className="rounded bg-primary/10 px-1.5 py-0.5 font-mono text-[10px] font-bold text-primary">
								You
							</span>
						)}
					</span>
				</div>
			</TableCell>
			<TableCell className="text-center">
				<Badge
					variant="outline"
					className={`px-2.5 py-0.5 text-[11px] font-semibold capitalize ${ROLE_BADGE_CLASS(member.role)}`}>
					{member.role}
				</Badge>
			</TableCell>
			<TableCell className="text-center">
				<span className="inline-flex items-center gap-1 text-xs font-medium text-emerald-500">
					<UserCheck className="h-3.5 w-3.5" />
					Active
				</span>
			</TableCell>
			<TableCell className="text-center">
				{member.twoFactorEnabled ? (
					<span className="inline-flex items-center gap-1 text-xs font-medium text-emerald-500">
						<ShieldCheck className="h-3.5 w-3.5" /> Enabled
					</span>
				) : (
					<span className="font-mono text-xs text-muted-foreground">
						Disabled
					</span>
				)}
			</TableCell>
			<TableCell className="text-center">
				<span className="font-mono text-xs text-muted-foreground">
					{new Date(member.createdAt).toLocaleDateString()}
				</span>
			</TableCell>
			<TableCell className="text-right">
				{canManage ? (
					<DropdownMenu>
						<DropdownMenuTrigger
							render={
								<Button
									variant="ghost"
									className="h-8 w-8 p-0 text-muted-foreground hover:text-foreground">
									<MoreHorizontal className="h-4 w-4" />
								</Button>
							}
						/>
						<DropdownMenuContent align="end" className="w-40">
							<DropdownMenuLabel className="text-xs">
								Actions
							</DropdownMenuLabel>
							<DropdownMenuItem
								onClick={() => onEditRole(member)}
								className="cursor-pointer text-xs">
								Change Role
							</DropdownMenuItem>
							<DialogAction
								title="Delete User"
								description={`Are you sure you want to delete user ${member.email}?`}
								type="destructive"
								onClick={() => onDelete(member.id, member.email)}>
								<DropdownMenuItem
									className="w-full cursor-pointer text-xs text-rose-500 hover:text-rose-600"
									onSelect={e => e?.preventDefault()}>
									Delete User
								</DropdownMenuItem>
							</DialogAction>
						</DropdownMenuContent>
					</DropdownMenu>
				) : (
					<span className="font-mono text-xs text-muted-foreground">
						—
					</span>
				)}
			</TableCell>
		</TableRow>
	);
}
