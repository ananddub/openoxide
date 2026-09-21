import {createFileRoute, Navigate} from '@tanstack/react-router';
import {Loader2, Plus, ShieldCheck, Trash2, Users} from 'lucide-react';
import {useState} from 'react';
import {toast} from 'sonner';
import {
	usePermissionGroupDelete,
	usePermissionGroupList,
	usePolicyList,
} from '#/api/live';
import {usePermissions} from '#/hooks/use-permissions';
import {formatApiError} from '#/api/utils';
import {Badge} from '#/components/ui/badge';
import {Button} from '#/components/ui/button';
import {SettingsSection} from '#/components/settings/settings-section';
import {
	GroupEditorDialog,
	type Draft,
} from '#/components/settings/groups/group-editor-dialog';

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

	const [draft, setDraft] = useState<Draft | null>(null);

	const groups = groupsQuery.data ?? [];
	const policies = policiesQuery.data ?? [];
	const canManage =
		can('group:create') || can('group:update') || can('group:delete');

	const remove = async (id: number) => {
		try {
			await deleteMutation.mutateAsync(id);
			toast.success('Group deleted');
		} catch (err) {
			toast.error(formatApiError(err));
		}
	};

	if (grantsLoading || groupsQuery.isLoading) {
		return (
			<div className="flex min-h-[40vh] items-center justify-center">
				<Loader2 className="size-5 animate-spin text-muted-foreground" />
			</div>
		);
	}

	return (
		<div className="flex w-full animate-in flex-col gap-5 duration-200 fade-in">
			<SettingsSection
				icon={ShieldCheck}
				title="Permission Groups"
				description="Roles for this organization: each group grants a set of resource actions to every member it is assigned to."
				action={
					canManage && (
						<Button
							size="sm"
							className="h-8 gap-1.5 text-xs"
							onClick={() => setDraft({name: '', actions: []})}>
							<Plus className="size-3.5" /> New Group
						</Button>
					)
				}>
				{groups.length === 0 ? (
					<p className="text-sm text-muted-foreground">
						No groups yet in this organization.
					</p>
				) : (
					<div className="grid gap-3">
						{groups.map(group => (
							<div
								key={String(group.id)}
								className="rounded-xl bg-muted/30 p-4">
								<div className="flex items-center justify-between gap-3">
									<div className="flex min-w-0 items-center gap-2">
										<Users className="size-4 shrink-0 text-muted-foreground" />
										<h3 className="truncate text-sm font-medium text-foreground">
											{group.name}
										</h3>
										{group.is_system && (
											<Badge variant="secondary" className="text-[9px]">
												SYSTEM
											</Badge>
										)}
										<span className="text-[10px] text-muted-foreground">
											{group.actions.length} actions
										</span>
									</div>
									{canManage && (
										<div className="flex shrink-0 gap-1">
											<Button
												variant="ghost"
												size="sm"
												className="h-7 text-xs"
												onClick={() =>
													setDraft({
														id: Number(group.id),
														name: group.name,
														actions: [...group.actions],
													})
												}>
												Edit
											</Button>
											{!group.is_system && (
												<Button
													variant="ghost"
													size="icon"
													className="h-7 w-7 text-rose-500"
													title="Delete group"
													onClick={() => void remove(Number(group.id))}>
													<Trash2 className="size-3.5" />
												</Button>
											)}
										</div>
									)}
								</div>
								<div className="mt-2 flex flex-wrap gap-1">
									{group.actions.slice(0, 12).map(action => (
										<Badge
											key={action}
											variant="outline"
											className="font-mono text-[9px]">
											{action}
										</Badge>
									))}
									{group.actions.length > 12 && (
										<span className="text-[10px] text-muted-foreground">
											+{group.actions.length - 12} more
										</span>
									)}
								</div>
							</div>
						))}
					</div>
				)}
			</SettingsSection>

			<GroupEditorDialog
				draft={draft}
				setDraft={setDraft}
				policies={policies}
			/>
		</div>
	);
}
