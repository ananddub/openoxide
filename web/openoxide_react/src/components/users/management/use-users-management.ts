import {useState, useMemo} from 'react';
import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';
import {
	useInviteCancel,
	useMemberAdd,
	useMemberRemove,
	useMemberUpdateRole,
	useInviteCreate,
	usePermissionGroupMembers,
	usePermissionGroupInvites,
	useProfile,
	type MemberItem,
	type InviteItem,
} from '#/api/live';
import {usePermissions} from '#/hooks/use-permissions';

export const ROLES = ['owner', 'admin', 'developer', 'viewer'] as const;

/** What each role can do — shown wherever a role is being picked. */
export const ROLE_DESC: Record<string, string> = {
	owner: 'Full access, including billing and removing members',
	admin: 'Manage projects, deployments and team members',
	developer: 'Create and deploy resources, cannot manage the team',
	viewer: 'Read-only access to the organization',
};

/** Owner first, then alphabetically — the list never reshuffles on refresh. */
const ROLE_ORDER: Record<string, number> = {
	owner: 0,
	admin: 1,
	developer: 2,
	viewer: 3,
};

/**
 * Controller for Settings → Team: live member/invite queries (invalidated by
 * the table-watch ORGANIZATION_* stream), the role/delete/invite mutations,
 * filter state, and the dialog open targets. Kept out of the page component so
 * the render layer stays a thin composition of cards + dialogs.
 */
export function useUsersManagement() {
	const membersQuery = usePermissionGroupMembers();
	const invitesQuery = usePermissionGroupInvites();
	const members = (membersQuery.data ?? []) as unknown as MemberItem[];
	const invites = useMemo(
		() =>
			((invitesQuery.data ?? []) as unknown as InviteItem[])
				.slice()
				.sort(
					(a, b) => Number(b.created_at ?? 0) - Number(a.created_at ?? 0),
				),
		[invitesQuery.data],
	);
	const userProfile = useProfile().data;
	const {can, canAny} = usePermissions();

	const addMemberMutation = useMemberAdd();
	const inviteMutation = useInviteCreate();
	const cancelInviteMutation = useInviteCancel();
	const updateRoleMutation = useMemberUpdateRole();
	const removeMemberMutation = useMemberRemove();

	const isLoading = membersQuery.isLoading;

	const [searchQuery, setSearchQuery] = useState('');
	const [roleFilter, setRoleFilter] = useState('ALL');

	const [isCreateOpen, setIsCreateOpen] = useState(false);
	const [actionType, setActionType] = useState<'direct' | 'invite'>(
		'direct',
	);
	const [inputEmail, setInputEmail] = useState('');
	const [selectedRole, setSelectedRole] = useState('developer');
	const [isSubmitting, setIsSubmitting] = useState(false);

	const [editingRoleUser, setEditingRoleUser] =
		useState<MemberItem | null>(null);
	const [deleteTarget, setDeleteTarget] = useState<MemberItem | null>(
		null,
	);
	const [isDeleting, setIsDeleting] = useState(false);

	const filteredMembers = useMemo(() => {
		const q = searchQuery.trim().toLowerCase();
		return members
			.filter(m => {
				const email = (m.email ?? '').toLowerCase();
				const name = (m.name ?? '').toLowerCase();
				const matchesSearch = !q || email.includes(q) || name.includes(q);
				const matchesRole =
					roleFilter === 'ALL' ||
					(m.role ?? '').toLowerCase() === roleFilter.toLowerCase();
				return matchesSearch && matchesRole;
			})
			.sort((a, b) => {
				const ra = ROLE_ORDER[(a.role ?? '').toLowerCase()] ?? 9;
				const rb = ROLE_ORDER[(b.role ?? '').toLowerCase()] ?? 9;
				if (ra !== rb) return ra - rb;
				return (a.name || a.email || '').localeCompare(
					b.name || b.email || '',
				);
			});
	}, [members, searchQuery, roleFilter]);

	const openCreate = (mode: 'direct' | 'invite') => {
		setActionType(mode);
		setInputEmail('');
		setSelectedRole('developer');
		setIsCreateOpen(true);
	};

	const handleCreateOrInvite = async (e: React.FormEvent) => {
		e.preventDefault();
		const email = inputEmail.trim().toLowerCase();
		if (!email) return;
		setIsSubmitting(true);
		try {
			if (actionType === 'direct') {
				await addMemberMutation.mutateAsync({
					email,
					role: selectedRole.toUpperCase(),
				});
				toast.success(`${email} added to the workspace`);
			} else {
				await inviteMutation.mutateAsync({
					email,
					role: selectedRole.toUpperCase(),
				});
				toast.success(`Invitation sent to ${email}`);
			}
			setIsCreateOpen(false);
			setInputEmail('');
		} catch (error) {
			toast.error(formatApiError(error));
		} finally {
			setIsSubmitting(false);
		}
	};

	const handleUpdateRole = async (newRole: string) => {
		if (!editingRoleUser) return;
		try {
			await updateRoleMutation.mutateAsync({
				id: Number(editingRoleUser.id),
				role: newRole.toUpperCase(),
			});
			toast.success('User role updated');
			setEditingRoleUser(null);
		} catch (error) {
			toast.error(formatApiError(error));
		}
	};

	const confirmDeleteUser = async () => {
		if (!deleteTarget) return;
		setIsDeleting(true);
		try {
			await removeMemberMutation.mutateAsync(Number(deleteTarget.id));
			toast.success('User removed from organization');
			setDeleteTarget(null);
		} catch (error) {
			toast.error(formatApiError(error));
		} finally {
			setIsDeleting(false);
		}
	};

	const handleRevokeInvitation = async (id: string) => {
		try {
			await cancelInviteMutation.mutateAsync(Number(id));
			toast.success('Invitation revoked successfully');
		} catch (error) {
			toast.error(formatApiError(error));
		}
	};

	const isSelf = (m: MemberItem) =>
		Boolean(
			userProfile &&
			((userProfile.email &&
				userProfile.email.toLowerCase() === m.email.toLowerCase()) ||
				(userProfile.id &&
					m.user_id &&
					Number(userProfile.id) === Number(m.user_id))),
		);

	const canManageMember = (m: MemberItem) =>
		can('member:update') && !isSelf(m) && m.role.toLowerCase() !== 'owner';

	return {
		members,
		invites,
		userProfile,
		can,
		canAny,
		isLoading,
		searchQuery,
		setSearchQuery,
		roleFilter,
		setRoleFilter,
		isCreateOpen,
		setIsCreateOpen,
		actionType,
		setActionType,
		inputEmail,
		setInputEmail,
		selectedRole,
		setSelectedRole,
		isSubmitting,
		editingRoleUser,
		setEditingRoleUser,
		deleteTarget,
		setDeleteTarget,
		isDeleting,
		filteredMembers,
		openCreate,
		handleCreateOrInvite,
		handleUpdateRole,
		confirmDeleteUser,
		handleRevokeInvitation,
		isSelf,
		canManageMember,
	};
}

export type UsersManagementController = ReturnType<
	typeof useUsersManagement
>;
