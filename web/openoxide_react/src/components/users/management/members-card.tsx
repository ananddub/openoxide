import {UserPlus, Users} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {
	GridSkeleton,
	ListCard,
	ListEmpty,
	ListSkeleton,
	type ListView,
	ViewGrid,
} from '#/components/settings/list-card';
import {MemberCard, MemberRow} from './member-row';
import {type UsersManagementController as Ctl} from './use-users-management';

/** Member grid/list — the page toolbar owns search and the role filter. */
export function MembersCard({ctl, view}: {ctl: Ctl; view: ListView}) {
	const {
		filteredMembers,
		isLoading,
		searchQuery,
		roleFilter,
		userProfile,
		isSelf,
		canManageMember,
		can,
		openCreate,
		setEditingRoleUser,
		setDeleteTarget,
	} = ctl;

	if (isLoading)
		return view === 'grid' ? (
			<GridSkeleton cards={4} />
		) : (
			<ListSkeleton rows={4} />
		);

	if (filteredMembers.length === 0) {
		const filtering = Boolean(searchQuery) || roleFilter !== 'ALL';
		return (
			<ListEmpty
				icon={Users}
				title={filtering ? 'No members found' : 'No members yet'}
				description={
					filtering
						? 'Try adjusting the search text or the role filter above.'
						: 'Invite people to this organization so they can work on its projects.'
				}
				action={
					!filtering && can('member:create') ? (
						<Button
							onClick={() => openCreate('invite')}
							className="h-9 gap-1.5">
							<UserPlus className="size-3.5" /> Invite someone
						</Button>
					) : undefined
				}
			/>
		);
	}

	const memberProps = (member: (typeof filteredMembers)[number]) => ({
		member,
		self: isSelf(member),
		profileName: userProfile?.name,
		profileAvatar: userProfile?.avatar,
		canEditRole: canManageMember(member),
		canEditPermissions: canManageMember(member),
		canRemove:
			can('member:delete') && member.role.toLowerCase() !== 'owner',
		onEditRole: () => setEditingRoleUser(member),
		onEditPermissions: () => ctl.setOverridesTarget(member),
		onDelete: () => setDeleteTarget(member),
	});

	if (view === 'grid') {
		return (
			<ViewGrid>
				{filteredMembers.map(member => (
					<MemberCard key={String(member.id)} {...memberProps(member)} />
				))}
			</ViewGrid>
		);
	}

	return (
		<ListCard>
			{filteredMembers.map(member => (
				<MemberRow key={String(member.id)} {...memberProps(member)} />
			))}
		</ListCard>
	);
}
