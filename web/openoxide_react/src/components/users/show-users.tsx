import {useState} from 'react';
import {Loader2, Users} from 'lucide-react';
import {toast} from 'sonner';
import {
	Card,
	CardContent,
	CardDescription,
	CardHeader,
	CardTitle,
} from '#/components/ui/card';
import {
	Table,
	TableBody,
	TableHead,
	TableHeader,
	TableRow,
} from '#/components/ui/table';
import {ChangeRoleModal} from './change-role-modal';
import {UserRow} from './user-row';

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

const HEADERS: {label: string; className: string}[] = [
	{label: 'User', className: 'text-xs'},
	{label: 'Role', className: 'text-center text-xs'},
	{label: 'Status', className: 'text-center text-xs'},
	{label: '2FA', className: 'text-center text-xs'},
	{label: 'Joined Date', className: 'text-center text-xs'},
	{label: 'Actions', className: 'text-right text-xs'},
];

export const ShowUsers = () => {
	const [users, setUsers] = useState<UserMember[]>(INITIAL_USERS);
	const [isLoading] = useState(false);
	const [editingRoleUser, setEditingRoleUser] =
		useState<UserMember | null>(null);

	const handleDeleteUser = (userId: string, email: string) => {
		setUsers(prev => prev.filter(u => u.id !== userId));
		toast.success(`User '${email}' deleted successfully`);
	};

	return (
		<div className="mx-auto w-full max-w-5xl">
			<Card className="overflow-hidden rounded-xl border bg-card">
				<CardHeader className="pb-4">
					<CardTitle className="flex items-center gap-2.5 text-lg font-bold">
						<div className="flex size-8 shrink-0 items-center justify-center rounded-xl bg-primary/10 text-primary">
							<Users className="size-4" />
						</div>
						<span>Users</span>
					</CardTitle>
					<CardDescription className="text-xs text-muted-foreground">
						Add and manage team members for your OpenOxide account.
					</CardDescription>
				</CardHeader>
				<CardContent className="space-y-4 border-t pt-4">
					{isLoading ? (
						<div className="flex min-h-[20vh] flex-row items-center justify-center gap-2 text-sm text-muted-foreground">
							<Loader2 className="size-4 animate-spin text-primary" />
							<span>Loading users...</span>
						</div>
					) : users.length === 0 ? (
						<div className="flex min-h-[20vh] flex-col items-center justify-center gap-2 rounded-xl bg-muted/10 p-6 text-center">
							<Users className="size-8 text-muted-foreground/40" />
							<span className="text-sm font-semibold text-foreground">
								No users found
							</span>
						</div>
					) : (
						<div className="overflow-x-auto rounded-xl">
							<Table>
								<TableHeader className="bg-muted/30">
									<TableRow>
										{HEADERS.map(h => (
											<TableHead
												key={h.label}
												className={`${h.className} font-bold tracking-wider text-muted-foreground uppercase`}>
												{h.label}
											</TableHead>
										))}
									</TableRow>
								</TableHeader>
								<TableBody>
									{users.map(member => (
										<UserRow
											key={member.id}
											member={member}
											onEditRole={setEditingRoleUser}
											onDelete={handleDeleteUser}
										/>
									))}
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
					isOpen
					onClose={() => setEditingRoleUser(null)}
					onSuccess={() => {
						// refresh role list
					}}
				/>
			)}
		</div>
	);
};
