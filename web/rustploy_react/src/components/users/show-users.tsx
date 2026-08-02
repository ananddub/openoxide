import {useState} from 'react';
import {Loader2, MoreHorizontal, Users, ShieldCheck, UserCheck} from 'lucide-react';
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
		email: 'admin@rustploy.com',
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
	const [editingRoleUser, setEditingRoleUser] = useState<UserMember | null>(null);

	const handleDeleteUser = async (userId: string, email: string) => {
		try {
			setUsers(prev => prev.filter(u => u.id !== userId));
			toast.success(`User '${email}' deleted successfully`);
		} catch {
			toast.error('Error deleting user');
		}
	};

	return (
		<div className="w-full max-w-5xl mx-auto">
			<Card className="bg-card border border-border/80 rounded-2xl shadow-xs overflow-hidden">
				<CardHeader className="pb-4">
					<CardTitle className="text-lg font-bold flex items-center gap-2.5">
						<div className="size-8 rounded-xl bg-primary/10 border border-primary/20 flex items-center justify-center text-primary shrink-0">
							<Users className="size-4" />
						</div>
						<span>Users</span>
					</CardTitle>
					<CardDescription className="text-xs text-muted-foreground">
						Add and manage team members for your Rustploy account.
					</CardDescription>
				</CardHeader>
				<CardContent className="space-y-4 pt-4 border-t border-border/60">
					{isLoading ? (
						<div className="flex flex-row gap-2 items-center justify-center text-sm text-muted-foreground min-h-[20vh]">
							<Loader2 className="animate-spin size-4 text-primary" />
							<span>Loading users...</span>
						</div>
					) : !users || users.length === 0 ? (
						<div className="flex flex-col items-center gap-2 min-h-[20vh] justify-center text-center p-6 border border-dashed border-border rounded-xl bg-muted/10">
							<Users className="size-8 text-muted-foreground/40" />
							<span className="text-sm font-semibold text-foreground">No users found</span>
						</div>
					) : (
						<div className="overflow-x-auto rounded-xl border border-border/60">
							<Table>
								<TableHeader className="bg-muted/30">
									<TableRow className="border-border/60">
										<TableHead className="text-xs font-bold uppercase tracking-wider text-muted-foreground">User</TableHead>
										<TableHead className="text-center text-xs font-bold uppercase tracking-wider text-muted-foreground">Role</TableHead>
										<TableHead className="text-center text-xs font-bold uppercase tracking-wider text-muted-foreground">Status</TableHead>
										<TableHead className="text-center text-xs font-bold uppercase tracking-wider text-muted-foreground">2FA</TableHead>
										<TableHead className="text-center text-xs font-bold uppercase tracking-wider text-muted-foreground">Joined Date</TableHead>
										<TableHead className="text-right text-xs font-bold uppercase tracking-wider text-muted-foreground">Actions</TableHead>
									</TableRow>
								</TableHeader>
								<TableBody>
									{users.map(member => {
										const isOwner = member.role === 'owner';
										const canManage = !isOwner && !member.isSelf;

										return (
											<TableRow key={member.id} className="border-border/60 hover:bg-muted/20 transition-colors">
												<TableCell className="font-medium">
													<div className="flex items-center gap-3">
														<div className="size-8 rounded-full bg-primary/10 text-primary font-bold text-xs flex items-center justify-center shrink-0 border border-primary/20">
															{member.email.substring(0, 2).toUpperCase()}
														</div>
														<span className="text-xs text-foreground font-semibold flex items-center gap-1.5">
															{member.email}
															{member.isSelf && (
																<span className="text-[10px] bg-primary/10 text-primary px-1.5 py-0.5 rounded font-mono font-bold">
																	You
																</span>
															)}
														</span>
													</div>
												</TableCell>
												<TableCell className="text-center">
													<Badge
														variant="outline"
														className={`text-[11px] font-semibold capitalize px-2.5 py-0.5 ${
															member.role === 'owner'
																? 'bg-purple-500/10 text-purple-500 border-purple-500/30'
																: member.role === 'admin'
																? 'bg-blue-500/10 text-blue-500 border-blue-500/30'
																: 'bg-emerald-500/10 text-emerald-500 border-emerald-500/30'
														}`}
													>
														{member.role}
													</Badge>
												</TableCell>
												<TableCell className="text-center">
													<span className="inline-flex items-center gap-1 text-xs font-medium text-emerald-500">
														<UserCheck className="w-3.5 h-3.5" />
														Active
													</span>
												</TableCell>
												<TableCell className="text-center">
													{member.twoFactorEnabled ? (
														<span className="inline-flex items-center gap-1 text-xs text-emerald-500 font-medium">
															<ShieldCheck className="w-3.5 h-3.5" /> Enabled
														</span>
													) : (
														<span className="text-xs text-muted-foreground font-mono">
															Disabled
														</span>
													)}
												</TableCell>
												<TableCell className="text-center">
													<span className="text-xs text-muted-foreground font-mono">
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
																		className="h-8 w-8 p-0 text-muted-foreground hover:text-foreground"
																	>
																		<MoreHorizontal className="h-4 w-4" />
																	</Button>
																}
															/>
															<DropdownMenuContent align="end" className="w-40">
																<DropdownMenuLabel className="text-xs">
																	Actions
																</DropdownMenuLabel>
																<DropdownMenuItem
																	onClick={() => setEditingRoleUser(member)}
																	className="cursor-pointer text-xs"
																>
																	Change Role
																</DropdownMenuItem>

																<DialogAction
																	title="Delete User"
																	description={`Are you sure you want to delete user ${member.email}?`}
																	type="destructive"
																	onClick={() => handleDeleteUser(member.id, member.email)}
																>
																	<DropdownMenuItem
																		className="w-full cursor-pointer text-rose-500 hover:text-rose-600 text-xs"
																		onSelect={e => e?.preventDefault()}
																	>
																		Delete User
																	</DropdownMenuItem>
																</DialogAction>
															</DropdownMenuContent>
														</DropdownMenu>
													) : (
														<span className="text-xs text-muted-foreground font-mono">—</span>
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
