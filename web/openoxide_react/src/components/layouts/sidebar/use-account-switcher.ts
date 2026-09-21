import * as React from 'react';
import {toast} from 'sonner';
import {useQueryClient} from '@tanstack/react-query';
import {useOrganizations, useAuthSession} from '#/stores/session-context';
import {persistActiveOrganizationId} from '#/stores/organization-context';
import {
	useOrganizationCreate,
	useOrganizationDelete,
	useOrganizationLeave,
	useOrganizationListByOwner,
	useOrganizationUpdate,
} from '#/api/live/organization';
import {formatApiError} from '#/api/utils';

/** State, mutations, derived values and handlers for {@link AccountSwitcher}. */
export function useAccountSwitcher() {
	const {data: organizations = []} = useOrganizationListByOwner();
	const {activeOrg, setOrganizations, setActiveOrg} = useOrganizations();
	const {user} = useAuthSession();
	const queryClient = useQueryClient();

	const createMutation = useOrganizationCreate();
	const updateMutation = useOrganizationUpdate();
	const leaveMutation = useOrganizationLeave();
	const deleteMutation = useOrganizationDelete();

	const [open, setOpen] = React.useState(false);
	const [isFormOpen, setIsFormOpen] = React.useState(false);
	const [editingId, setEditingId] = React.useState<string | null>(null);
	const [formName, setFormName] = React.useState('');
	const [isSubmitting, setIsSubmitting] = React.useState(false);
	const [confirmDelete, setConfirmDelete] = React.useState(false);

	// Keep the session store's org list in sync with the server list.
	React.useEffect(() => {
		if (organizations.length) {
			setOrganizations(
				organizations.map(org => ({
					id: Number(org.id),
					name: org.name,
					logo: org.logo,
					slug: org.slug,
					owner_id: Number(org.owner_id),
					role: org.role,
				})),
			);
		}
	}, [organizations, setOrganizations]);

	const activeId = activeOrg?.id ?? null;
	const activeEntry = organizations.find(
		org => String(org.id) === String(activeId),
	);
	const activeRole = (activeEntry?.role ?? 'MEMBER').toUpperCase();
	// Plan chip carries the PLAN name (Vercel grammar), never the org
	// name — that lives on the left half of the pill.
	const planLabel = organizations.length > 1 ? 'Team' : 'Hobby';

	const switchTo = (id: number) => {
		if (id === activeId) {
			setOpen(false);
			return;
		}
		persistActiveOrganizationId(id);
		const target = organizations.find(org => Number(org.id) === id);
		if (target) {
			setActiveOrg({
				id: Number(target.id),
				name: target.name,
				slug: target.slug,
				owner_id: Number(target.owner_id),
				role: target.role,
			});
		}
		// Every query is organization-scoped server-side; drop the cache so
		// nothing from the previous workspace survives the switch.
		queryClient.invalidateQueries();
		setOpen(false);
	};

	const openCreate = () => {
		setEditingId(null);
		setFormName('');
		setConfirmDelete(false);
		setIsFormOpen(true);
	};

	const openEdit = (id: string, name: string) => {
		setEditingId(id);
		setFormName(name);
		setConfirmDelete(false);
		setIsFormOpen(true);
	};

	const submitForm = async () => {
		const name = formName.trim();
		if (!name) {
			toast.error('Organization name is required');
			return;
		}
		setIsSubmitting(true);
		try {
			if (editingId) {
				await updateMutation.mutateAsync({id: Number(editingId), name});
				toast.success('Organization updated');
			} else {
				const created = await createMutation.mutateAsync({name});
				toast.success('Organization created');
				// Move straight into the new workspace.
				switchTo(Number(created.id));
			}
			setIsFormOpen(false);
		} catch (err) {
			toast.error(formatApiError(err));
		} finally {
			setIsSubmitting(false);
		}
	};

	const leave = async (id: number) => {
		try {
			await leaveMutation.mutateAsync(id);
			toast.success('You left the organization');
			if (activeId === id) {
				const next = organizations.find(org => Number(org.id) !== id);
				persistActiveOrganizationId(next ? Number(next.id) : null);
				queryClient.invalidateQueries();
			}
		} catch (err) {
			toast.error(formatApiError(err));
		}
	};

	const remove = async (id: number) => {
		if (!confirmDelete) {
			setConfirmDelete(true);
			return;
		}
		setIsSubmitting(true);
		try {
			await deleteMutation.mutateAsync(id);
			toast.success('Organization deleted');
			setIsFormOpen(false);
			if (activeId === id) {
				const next = organizations.find(org => Number(org.id) !== id);
				persistActiveOrganizationId(next ? Number(next.id) : null);
				queryClient.invalidateQueries();
			}
		} catch (err) {
			toast.error(formatApiError(err));
		} finally {
			setIsSubmitting(false);
			setConfirmDelete(false);
		}
	};

	return {
		organizations,
		activeOrg,
		user,
		open,
		setOpen,
		isFormOpen,
		setIsFormOpen,
		editingId,
		formName,
		setFormName,
		isSubmitting,
		confirmDelete,
		activeId,
		activeEntry,
		activeRole,
		planLabel,
		switchTo,
		openCreate,
		openEdit,
		submitForm,
		leave,
		remove,
	};
}

export type AccountSwitcherController = ReturnType<
	typeof useAccountSwitcher
>;
