import {Users, Search} from 'lucide-react';
import {Input} from '#/components/ui/input';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import {ListCard, ListEmpty} from '#/components/settings/list-card';
import {MemberRow} from './member-row';
import {
	ROLES,
	type UsersManagementController as Ctl,
} from './use-users-management';

/** Members list card: search + role filter header, avatar rows with inline
 * role control and a kebab for change-role / remove. */
export function MembersCard({ctl}: {ctl: Ctl}) {
	const {
		members,
		filteredMembers,
		isLoading,
		searchQuery,
		setSearchQuery,
		roleFilter,
		setRoleFilter,
		userProfile,
		isSelf,
		canManageMember,
		can,
		changeRole,
		setEditingRoleUser,
		setDeleteTarget,
	} = ctl;

	return (
		<ListCard
			title={`Members (${members.length})`}
			description="Everyone with access to this workspace."
			action={
				<div className="flex items-center gap-2">
					<div className="relative">
						<Search className="absolute top-1/2 left-2.5 size-3.5 -translate-y-1/2 text-muted-foreground" />
						<Input
							placeholder="Search members..."
							value={searchQuery}
							onChange={e => setSearchQuery(e.target.value)}
							className="h-8 w-48 bg-muted/40 pl-8 text-xs"
						/>
					</div>
					<Select
						value={roleFilter}
						onValueChange={v => setRoleFilter(v ?? 'ALL')}>
						<SelectTrigger className="h-8 w-28 bg-muted/40 text-xs">
							<SelectValue />
						</SelectTrigger>
						<SelectContent>
							<SelectItem value="ALL" className="text-xs">
								All roles
							</SelectItem>
							{ROLES.map(r => (
								<SelectItem
									key={r}
									value={r}
									className="text-xs capitalize">
									{r}
								</SelectItem>
							))}
						</SelectContent>
					</Select>
				</div>
			}>
			{isLoading ? (
				<div className="space-y-2 p-5">
					{[...Array(3)].map((_, i) => (
						<div
							key={i}
							className="h-12 animate-pulse rounded-xl bg-muted/30"
						/>
					))}
				</div>
			) : filteredMembers.length === 0 ? (
				<ListEmpty
					icon={Users}
					title="No members found"
					description={
						searchQuery || roleFilter !== 'ALL'
							? 'Try adjusting your search query or role filter.'
							: 'Add team members to collaborate on projects.'
					}
				/>
			) : (
				<div className="divide-y divide-border/40">
					{filteredMembers.map(member => (
						<MemberRow
							key={String(member.id)}
							member={member}
							self={isSelf(member)}
							profileName={userProfile?.name}
							profileAvatar={userProfile?.avatar}
							manageable={canManageMember(member)}
							deletable={can('member:delete')}
							onRole={role => changeRole(member, role)}
							onEditRole={() => setEditingRoleUser(member)}
							onDelete={() => setDeleteTarget(member)}
						/>
					))}
				</div>
			)}
		</ListCard>
	);
}
