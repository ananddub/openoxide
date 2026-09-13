import {createFileRoute} from '@tanstack/react-router';
import {Loader2, Plus, ShieldCheck, Trash2, Users} from 'lucide-react';
import {useState} from 'react';
import {toast} from 'sonner';
import {
	usePermissionGroupCreate,
	usePermissionGroupDelete,
	usePermissionGroupList,
	usePermissionGroupUpdate,
	usePolicyList,
} from '#/api/live';
import {usePermissions} from '#/hooks/use-permissions';
import {formatApiError} from '#/api/utils';
import {Badge} from '#/components/ui/badge';
import {Button} from '#/components/ui/button';
import {
	Dialog,
	DialogContent,
	DialogDescription,
	DialogFooter,
	DialogHeader,
	DialogTitle,
} from '#/components/ui/dialog';
import {Input} from '#/components/ui/input';

export const Route = createFileRoute('/_app/settings/groups')({
	component: GroupsPage,
});

type Draft = {
	id?: number;
	name: string;
	actions: string[];
};

/** resource prefix for grouping the policy catalog ("app:create" → "app"). */
const resourceOf = (action: string) => action.split(':')[0] ?? action;
const operationOf = (action: string) => action.split(':')[1] ?? action;

function GroupsPage() {
	const {can, isLoading: grantsLoading} = usePermissions();
	const groupsQuery = usePermissionGroupList();
	const policiesQuery = usePolicyList();
	const createMutation = usePermissionGroupCreate();
	const updateMutation = usePermissionGroupUpdate();
	const deleteMutation = usePermissionGroupDelete();

	const [draft, setDraft] = useState<Draft | null>(null);

	const groups = groupsQuery.data ?? [];
	const policies = policiesQuery.data ?? [];
	const canManage = can('group:create') || can('group:update') || can('group:delete');

	const grouped = new Map<string, string[]>();
	for (const policy of policies) {
		const list = grouped.get(resourceOf(policy.action)) ?? [];
		list.push(policy.action);
		grouped.set(resourceOf(policy.action), list);
	}

	const toggleAction = (action: string) =>
		setDraft(current =>
			current
				? {
						...current,
						actions: current.actions.includes(action)
							? current.actions.filter(item => item !== action)
							: [...current.actions, action],
					}
				: current,
		);

	const save = async () => {
		if (!draft) return;
		if (!draft.name.trim()) {
			toast.error('Group name is required');
			return;
		}
		try {
			if (draft.id) {
				await updateMutation.mutateAsync({
					id: draft.id,
					name: draft.name.trim(),
					actions: draft.actions,
				});
				toast.success('Group updated');
			} else {
				await createMutation.mutateAsync({
					name: draft.name.trim(),
					actions: draft.actions,
				});
				toast.success('Group created');
			}
			setDraft(null);
		} catch (err) {
			toast.error(formatApiError(err));
		}
	};

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
		<div className="mx-auto flex w-full max-w-5xl animate-in flex-col gap-5 p-6 duration-200 fade-in">
			<div className="flex items-end justify-between gap-4">
				<div className="flex flex-col gap-1">
					<h1 className="flex items-center gap-2 text-2xl font-bold tracking-tight text-foreground">
						<ShieldCheck className="size-5 shrink-0 text-primary" />
						Permission Groups
					</h1>
					<p className="text-xs text-muted-foreground">
						Roles for this organization: each group grants a set of
						resource actions to every member it is assigned to.
					</p>
				</div>
				{canManage && (
					<Button
						size="sm"
						className="h-9 gap-1.5 text-xs"
						onClick={() => setDraft({name: '', actions: []})}>
						<Plus className="size-3.5" /> New Group
					</Button>
				)}
			</div>

			{groups.length === 0 && (
				<p className="text-sm text-muted-foreground">
					No groups yet in this organization.
				</p>
			)}

			<div className="grid gap-3">
				{groups.map(group => (
					<div
						key={String(group.id)}
						className="rounded-xl border bg-card p-4">
						<div className="flex items-center justify-between gap-3">
							<div className="flex min-w-0 items-center gap-2">
								<Users className="size-4 shrink-0 text-muted-foreground" />
								<h3 className="truncate text-sm font-bold text-foreground">
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

			<Dialog
				open={!!draft}
				onOpenChange={open => !open && setDraft(null)}>
				<DialogContent className="max-h-[85vh] overflow-y-auto sm:max-w-lg">
					<DialogHeader>
						<DialogTitle className="text-base font-bold">
							{draft?.id ? 'Edit Group' : 'New Group'}
						</DialogTitle>
						<DialogDescription className="text-xs">
							Name the role and tick the actions its members may
							perform in this organization.
						</DialogDescription>
					</DialogHeader>
					{draft && (
						<div className="space-y-4 py-2">
							<div className="space-y-1">
								<label className="text-[11px] font-bold tracking-wider text-muted-foreground uppercase">
									Name
								</label>
								<Input
									value={draft.name}
									autoFocus
									onChange={event =>
										setDraft({...draft, name: event.target.value})
									}
									placeholder="Deploy Operators"
									className="h-10 bg-muted/20 text-xs"
								/>
							</div>
							<div className="space-y-3">
								{[...grouped.entries()].map(([resource, actions]) => (
									<div key={resource}>
										<p className="mb-1 text-[10px] font-bold tracking-wider text-muted-foreground uppercase">
											{resource}
										</p>
										<div className="flex flex-wrap gap-2">
											{actions.map(action => {
												const active = draft.actions.includes(action);
												return (
													<button
														key={action}
														type="button"
														onClick={() => toggleAction(action)}
														className={
															active
																? 'rounded-md border border-primary/40 bg-primary/10 px-2 py-1 font-mono text-[10px] font-semibold text-primary'
																: 'rounded-md border border-border/50 bg-muted/20 px-2 py-1 font-mono text-[10px] text-muted-foreground hover:text-foreground'
														}>
														{operationOf(action)}
													</button>
												);
											})}
										</div>
									</div>
								))}
							</div>
						</div>
					)}
					<DialogFooter>
						<Button
							type="button"
							variant="secondary"
							size="sm"
							className="h-9 text-xs"
							onClick={() => setDraft(null)}>
							Cancel
						</Button>
						<Button
							type="button"
							size="sm"
							className="h-9 text-xs"
							onClick={() => void save()}>
							{draft?.id ? 'Save' : 'Create'}
						</Button>
					</DialogFooter>
				</DialogContent>
			</Dialog>
		</div>
	);
}
