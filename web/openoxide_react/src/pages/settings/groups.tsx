import {createFileRoute, Navigate} from '@tanstack/react-router';
import {Plus, Search, ShieldCheck} from 'lucide-react';
import {useMemo, useState} from 'react';
import {toast} from 'sonner';
import {
	usePermissionGroupDelete,
	usePermissionGroupList,
	usePermissionGroupSetActive,
	usePolicyList,
} from '#/api/live';
import {usePermissions} from '#/hooks/permissions/use-permissions';
import {formatApiError} from '#/api/utils';
import type {PermissionGroupPayload} from '#/api/live/permissions/types';
import {Button} from '#/components/ui/button';
import {
	GridSkeleton,
	ListEmpty,
	ListRegion,
	ListSkeleton,
	SectionHeader,
	Toolbar,
	ToolbarSearch,
	ViewToggle,
	useListView,
} from '#/components/settings/list-card';
import {GroupList} from '#/components/settings/groups/group-list';
import {
	GroupEditorSheet,
	type Draft,
} from '#/components/settings/groups/group-editor-sheet';
import {ConfirmDeleteDialog} from '#/components/settings/confirm-delete-dialog';

export const Route = createFileRoute('/_app/settings/groups')({
	component: () => (
		<Navigate to="/settings" search={{tab: 'groups'}} replace />
	),
});

export function GroupsPage() {
	const {can, isLoading: grantsLoading} = usePermissions();
	const groupsQuery = usePermissionGroupList();
	const policiesQuery = usePolicyList();
	const deleteMutation = usePermissionGroupDelete();
	const setActiveMutation = usePermissionGroupSetActive();

	const [draft, setDraft] = useState<Draft | null>(null);
	const [search, setSearch] = useState('');
	const [view, setView] = useListView('settings-groups-view');
	const [togglingId, setTogglingId] = useState<bigint | null>(null);
	const [deleting, setDeleting] = useState<{
		id: number;
		name: string;
	} | null>(null);

	const groups = groupsQuery.data ?? [];
	const policies = policiesQuery.data ?? [];
	const canManage =
		can('group:create') || can('group:update') || can('group:delete');
	const canToggle = can('group:update');

	const q = search.trim().toLowerCase();
	const visibleGroups = useMemo(
		() =>
			q
				? groups.filter(
						group =>
							group.name.toLowerCase().includes(q) ||
							group.actions.some(action =>
								action.toLowerCase().includes(q),
							),
					)
				: groups,
		[groups, q],
	);

	const remove = async () => {
		if (!deleting) return;
		try {
			await deleteMutation.mutateAsync(deleting.id);
			toast.success('Group deleted');
			setDeleting(null);
		} catch (err) {
			toast.error(formatApiError(err));
		}
	};

	const toggleActive = async (
		group: PermissionGroupPayload,
		next: boolean,
	) => {
		setTogglingId(group.id);
		try {
			await setActiveMutation.mutateAsync({id: group.id, is_active: next});
			toast.success(next ? 'Group enabled' : 'Group disabled');
		} catch (err) {
			toast.error(formatApiError(err));
		} finally {
			setTogglingId(null);
		}
	};

	return (
		<div className="flex h-full w-full animate-in flex-col gap-5 duration-200 fade-in">
			<SectionHeader
				title="Permission Groups"
				description="Bundle the actions a set of members may perform, then assign the group from the Team section."
				action={
					canManage && (
						<Button
							onClick={() => setDraft({name: '', actions: []})}
							className="h-9 gap-1.5">
							<Plus className="size-3.5" />
							New group
						</Button>
					)
				}
			/>

			<Toolbar action={<ViewToggle view={view} onChange={setView} />}>
				<ToolbarSearch
					value={search}
					onChange={setSearch}
					placeholder="Search groups by name or action"
					className="min-w-[200px] flex-1 sm:max-w-none"
				/>
			</Toolbar>

			<ListRegion>
				{groupsQuery.isLoading || grantsLoading ? (
					view === 'grid' ? (
						<GridSkeleton />
					) : (
						<ListSkeleton />
					)
				) : groups.length === 0 ? (
					<ListEmpty
						icon={ShieldCheck}
						title="No permission groups yet"
						description="A group bundles the actions a set of members is allowed to perform. Assign groups from the Team section."
						action={
							canManage && (
								<Button
									onClick={() => setDraft({name: '', actions: []})}
									className="h-9 gap-1.5 text-xs font-semibold">
									<Plus className="size-3.5" />
									New group
								</Button>
							)
						}
					/>
				) : visibleGroups.length === 0 ? (
					<ListEmpty
						icon={Search}
						title="No groups match your search"
						description="Try a different name, or an action like app:update."
						action={
							q !== '' && (
								<Button
									variant="ghost"
									onClick={() => setSearch('')}
									className="text-xs font-semibold text-primary">
									Clear Search
								</Button>
							)
						}
					/>
				) : (
					<GroupList
						groups={visibleGroups}
						view={view}
						canManage={canManage}
						canToggle={canToggle}
						togglingId={togglingId}
						onToggleActive={(group, next) =>
							void toggleActive(group, next)
						}
						onEdit={group =>
							setDraft({
								id: Number(group.id),
								name: group.name,
								actions: [...group.actions],
							})
						}
						onDelete={group =>
							setDeleting({
								id: Number(group.id),
								name: group.name,
							})
						}
					/>
				)}
			</ListRegion>

			<ConfirmDeleteDialog
				open={deleting !== null}
				onOpenChange={o => !o && setDeleting(null)}
				title="Delete permission group"
				description={
					<span>
						Members assigned{' '}
						<span className="font-medium text-foreground">
							{deleting?.name}
						</span>{' '}
						would lose the actions it grants. This cannot be undone.
					</span>
				}
				confirmLabel="Delete group"
				busy={deleteMutation.isPending}
				onConfirm={() => void remove()}
			/>

			<GroupEditorSheet
				draft={draft}
				setDraft={setDraft}
				policies={policies}
			/>
		</div>
	);
}
