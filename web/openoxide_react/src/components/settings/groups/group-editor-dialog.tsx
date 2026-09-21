import {toast} from 'sonner';
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
import {formatApiError} from '#/api/utils';
import {
	usePermissionGroupCreate,
	usePermissionGroupUpdate,
} from '#/api/live';

export type Draft = {id?: number; name: string; actions: string[]};

/** resource prefix for grouping the policy catalog ("app:create" → "app"). */
const resourceOf = (action: string) => action.split(':')[0] ?? action;
const operationOf = (action: string) => action.split(':')[1] ?? action;

/** Create/edit permission-group dialog with the policy action picker. */
export function GroupEditorDialog({
	draft,
	setDraft,
	policies,
}: {
	draft: Draft | null;
	setDraft: (d: Draft | null) => void;
	policies: {action: string}[];
}) {
	const createMutation = usePermissionGroupCreate();
	const updateMutation = usePermissionGroupUpdate();

	const grouped = new Map<string, string[]>();
	for (const policy of policies) {
		const list = grouped.get(resourceOf(policy.action)) ?? [];
		list.push(policy.action);
		grouped.set(resourceOf(policy.action), list);
	}

	const toggleAction = (action: string) =>
		setDraft(
			draft
				? {
						...draft,
						actions: draft.actions.includes(action)
							? draft.actions.filter(item => item !== action)
							: [...draft.actions, action],
					}
				: draft,
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

	return (
		<Dialog open={!!draft} onOpenChange={open => !open && setDraft(null)}>
			<DialogContent className="max-h-[85vh] overflow-y-auto sm:max-w-lg">
				<DialogHeader>
					<DialogTitle className="text-base font-bold">
						{draft?.id ? 'Edit Group' : 'New Group'}
					</DialogTitle>
					<DialogDescription className="text-xs">
						Name the role and tick the actions its members may perform in
						this organization.
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
															? 'rounded-md bg-primary/10 px-2 py-1 font-mono text-[10px] font-semibold text-primary'
															: 'rounded-md bg-muted/20 px-2 py-1 font-mono text-[10px] text-muted-foreground hover:text-foreground'
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
	);
}
