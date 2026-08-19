import {useState} from 'react';
import {
	Loader2,
	MoreHorizontal,
	Users,
	ShieldCheck,
	UserCheck,
} from 'lucide-react';
import {toast} from 'sonner';
import {DialogAction} from '#/components/shared/dialog-action';
import {Badge} from '#/components/ui/badge';
import {Button} from '#/components/ui/button';
import {
	Card,
	CardContent,
	CardDescription,
	CardHeader,
	CardTitle,
} from '#/components/ui/card';
import {
	DropdownMenu,
	DropdownMenuContent,
	DropdownMenuItem,
	DropdownMenuLabel,
	DropdownMenuTrigger,
} from '#/components/ui/dropdown';
import {
	Table,
	TableBody,
	TableCell,
	TableHead,
	TableHeader,
	TableRow,
} from '#/components/ui/table';
import {ChangeRoleModal} from './change-role-modal';

export interface UserMember {
	id: string;
	email: string;
	role: string;
	banned: boolean;
	twoFactorEnabled: boolean;
	createdAt: string;
	isSelf?: boolean;
}

const INITIAL_USERS: UserMember[] = [
	{
		id: '1',
		email: 'admin@openoxide.com',
		role: 'owner',
		banned: false,
		twoFactorEnabled: true,
		createdAt: new Date().toISOString(),
		isSelf: true,
	},
];

export const ShowUsers = () => {
	const [users, setUsers] = useState<UserMember[]>(INITIAL_USERS);
	const [isLoading] = useState(false);
	const [editingRoleUser, setEditingRoleUser] =
		useState<UserMember | null>(null);

	const handleDeleteUser = async (userId: string, email: string) => {
		try {
			setUsers(prev => prev.filter(u => u.id !== userId));
			toast.success(`User '${email}' deleted successfully`);
		} catch {
			toast.error('Error deleting user');
		}
	};

	return (
		<div className="mx-auto w-full max-w-5xl">
			<Card className="overflow-hidden rounded-2xl border border-border/80 bg-card shadow-xs">
				<CardHeader className="pb-4">
					<CardTitle className="flex items-center gap-2.5 text-lg font-bold">
						<div className="flex size-8 shrink-0 items-center justify-center rounded-xl border border-primary/20 bg-primary/10 text-primary">
							<Users className="size-4" />
						</div>
						<span>Users</span>
					</CardTitle>
					<CardDescription className="text-xs text-muted-foreground">
						Add and manage team members for your OpenOxide account.
					</CardDescription>
				</CardHeader>
				<CardContent className="space-y-4 border-t border-border/60 pt-4">
					{isLoading ? (
						<div className="flex min-h-[20vh] flex-row items-center justify-center gap-2 text-sm text-muted-foreground">
							<Loader2 className="size-4 animate-spin text-primary" />
							<span>Loading users...</span>
						</div>
					) : !users || users.length === 0 ? (
						<div className="flex min-h-[20vh] flex-col items-center justify-center gap-2 rounded-xl border border-dashed border-border bg-muted/10 p-6 text-center">
							<Users className="size-8 text-muted-foreground/40" />
							<span className="text-sm font-semibold text-foreground">
								No users found
							</span>
						</div>
					) : (
						<div className="overflow-x-auto rounded-xl border border-border/60">
							<Table>
								<TableHeader className="bg-muted/30">
									<TableRow className="border-border/60">
										<TableHead className="text-xs font-bold tracking-wider text-muted-foreground uppercase">
											User
										</TableHead>
										<TableHead className="text-center text-xs font-bold tracking-wider text-muted-foreground uppercase">
											Role
										</TableHead>
										<TableHead className="text-center text-xs font-bold tracking-wider text-muted-foreground uppercase">
											Status
										</TableHead>
										<TableHead className="text-center text-xs font-bold tracking-wider text-muted-foreground uppercase">
											2FA
										</TableHead>
										<TableHead className="text-center text-xs font-bold tracking-wider text-muted-foreground uppercase">
											Joined Date
										</TableHead>
										<TableHead className="text-right text-xs font-bold tracking-wider text-muted-foreground uppercase">
											Actions
										</TableHead>
									</TableRow>
								</TableHeader>
								<TableBody>
									{users.map(member => {
										const isOwner = member.role === 'owner';
										const canManage = !isOwner && !member.isSelf;

										return (
											<TableRow
												key={member.id}
												className="border-border/60 transition-colors hover:bg-muted/20">
												<TableCell className="font-medium">
													<div className="flex items-center gap-3">
														<div className="flex size-8 shrink-0 items-center justify-center rounded-full border border-primary/20 bg-primary/10 text-xs font-bold text-primary">
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
														className={`px-2.5 py-0.5 text-[11px] font-semibold capitalize ${
															member.role === 'owner'
																? 'border-purple-500/30 bg-purple-500/10 text-purple-500'
																: member.role === 'admin'
																	? 'border-blue-500/30 bg-blue-500/10 text-blue-500'
																	: 'border-emerald-500/30 bg-emerald-500/10 text-emerald-500'
														}`}>
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
															<ShieldCheck className="h-3.5 w-3.5" />{' '}
															Enabled
														</span>
													) : (
														<span className="font-mono text-xs text-muted-foreground">
															Disabled
														</span>
													)}
												</TableCell>
												<TableCell className="text-center">
													<span className="font-mono text-xs text-muted-foreground">
														{new Date(
															member.createdAt,
														).toLocaleDateString()}
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
															<DropdownMenuContent
																align="end"
																className="w-40">
																<DropdownMenuLabel className="text-xs">
																	Actions
																</DropdownMenuLabel>
																<DropdownMenuItem
																	onClick={() =>
																		setEditingRoleUser(member)
																	}
																	className="cursor-pointer text-xs">
																	Change Role
																</DropdownMenuItem>

																<DialogAction
																	title="Delete User"
																	description={`Are you sure you want to delete user ${member.email}?`}
																	type="destructive"
																	onClick={() =>
																		handleDeleteUser(
																			member.id,
																			member.email,
																		)
																	}>
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
									})}
								</TableBody>
							</Table>
						</div>
					)}
				</CardContent>
			</Card>

			{editingRoleUser && (
				<ChangeRoleModal
					userEmail={editingRoleUser.email}
					currentRole={editingRoleUser.role}
					isOpen={editingRoleUser !== null}
					onClose={() => setEditingRoleUser(null)}
					onSuccess={() => {
						// refresh role list
					}}
				/>
			)}
		</div>
	);
};
