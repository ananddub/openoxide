import {Search} from 'lucide-react';
import {useEffect, useState} from 'react';
import {toast} from 'sonner';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {Switch} from '#/components/ui/switch';
import {
	Sheet,
	SheetContent,
	SheetDescription,
	SheetFooter,
	SheetHeader,
	SheetTitle,
} from '#/components/ui/sheet';
import {formatApiError} from '#/api/utils';
import {WizardField} from '#/components/shared/sheet-wizard';
import {
	usePermissionGroupCreate,
	usePermissionGroupUpdate,
} from '#/api/live';

export type Draft = {id?: number; name: string; actions: string[]};

/** "app:create" → "app" / "create"; anything without a colon is its own word. */
const split = (action: string) => {
	const [resource = action, operation = action] = action.split(':');
	return {resource, operation};
};

const titleCase = (word: string) =>
	word ? word[0].toUpperCase() + word.slice(1).replace(/_/g, ' ') : word;

/** "app:create" → "Create app" — the row label a human can read. */
const actionLabel = (action: string) => {
	const {resource, operation} = split(action);
	return resource === operation
		? titleCase(resource)
		: `${titleCase(operation)} ${titleCase(resource)}`;
};

/** Create/edit a permission group — house form-sheet with an action picker. */
export function GroupEditorSheet({
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
	const [query, setQuery] = useState('');

	// Every open starts with the full catalog shown.
	useEffect(() => {
		if (!draft) setQuery('');
	}, [draft]);

	const actions = policies
		.map(policy => (policy.action ?? '').trim())
		.filter(Boolean);

	const search = query.trim().toLowerCase();
	const visible = search
		? actions.filter(
				action =>
					action.toLowerCase().includes(search) ||
					actionLabel(action).toLowerCase().includes(search),
			)
		: actions;

	const grouped = new Map<string, string[]>();
	for (const action of visible) {
		const {resource} = split(action);
		const list = grouped.get(resource) ?? [];

		list.push(action);
		grouped.set(resource, list);
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

	const toggleResource = (resource: string) => {
		if (!draft) return;
		const group = grouped.get(resource) ?? [];
		const allOn = group.every(a => draft.actions.includes(a));
		setDraft({
			...draft,
			actions: allOn
				? draft.actions.filter(a => !group.includes(a))
				: [...new Set([...draft.actions, ...group])],
		});
	};

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

	const busy = createMutation.isPending || updateMutation.isPending;
	const saving = draft?.id ? 'Save changes' : 'Create group';
	const allSelected =
		actions.length > 0 && draft?.actions.length === actions.length;

	const toggleEverything = () =>
		setDraft(
			draft ? {...draft, actions: allSelected ? [] : actions} : draft,
		);

	return (
		<Sheet open={!!draft} onOpenChange={open => !open && setDraft(null)}>
			<SheetContent>
				<SheetHeader>
					<SheetTitle>
						{draft?.id ? 'Edit Group' : 'New Group'}
					</SheetTitle>
					<SheetDescription>
						Name the role and pick the actions its members may perform in
						this organization.
					</SheetDescription>
				</SheetHeader>

				{draft && (
					<>
						<div className="flex min-h-0 flex-1 flex-col gap-4 overflow-y-auto px-6 py-6">
							<WizardField label="Name" required>
								<Input
									value={draft.name}
									autoFocus
									onChange={event =>
										setDraft({...draft, name: event.target.value})
									}
									placeholder="Deploy Operators"
									className="bg-background"
								/>
							</WizardField>

							<div className="flex items-center justify-between gap-3">
								<p className="text-[13px] font-medium text-foreground">
									{draft.actions.length} of {actions.length} actions
									selected
								</p>
								<button
									type="button"
									onClick={toggleEverything}
									className="cursor-pointer text-[13px] font-medium text-primary hover:underline">
									{allSelected ? 'Clear all' : 'Select all'}
								</button>
							</div>

							{actions.length === 0 ? (
								<div className="w-full shrink-0 rounded-xl border bg-card px-4 py-8 text-center">
									<p className="text-sm font-medium text-foreground">
										The action catalog is empty
									</p>
									<p className="mt-1 text-[13px] text-muted-foreground">
										Apply the policy-catalog migration so the server
										returns the actions a group can grant.
									</p>
								</div>
							) : (
								<div className="relative w-full shrink-0">
									<Search className="pointer-events-none absolute top-1/2 left-3 size-4 -translate-y-1/2 text-muted-foreground/70" />
									<Input
										value={query}
										onChange={event => setQuery(event.target.value)}
										placeholder="Search actions — deploy, member, app:create"
										className="h-9 rounded-lg bg-background pl-9 text-sm transition-colors duration-150 hover:border-foreground/25"
									/>
								</div>
							)}

							{actions.length > 0 && visible.length === 0 && (
								<div className="w-full shrink-0 rounded-xl border bg-card px-4 py-8 text-center">
									<p className="text-sm font-medium text-foreground">
										No action matches “{query.trim()}”
									</p>
									<p className="mt-1 text-[13px] text-muted-foreground">
										Try a resource like app or database, or an operation
										like read.
									</p>
								</div>
							)}
							{actions.length > 0 && (
								<div className="w-full shrink-0 overflow-hidden rounded-xl border bg-card">
									{[...grouped.entries()].map(
										([resource, groupActions]) => {
											const allOn = groupActions.every(a =>
												draft.actions.includes(a),
											);
											return (
												<div key={resource}>
													<div className="flex items-center justify-between gap-3 border-b bg-muted/40 px-4 py-1.5">
														<p className="text-[11px] font-semibold tracking-wide text-muted-foreground uppercase">
															{titleCase(resource)}
														</p>
														<button
															type="button"
															onClick={() => toggleResource(resource)}
															className="cursor-pointer text-[11px] font-medium text-primary hover:underline">
															{allOn ? 'Clear' : 'All'}
														</button>
													</div>
													<div className="divide-y">
														{groupActions.map(action => (
															<label
																key={action}
																className="flex cursor-pointer items-center justify-between gap-4 px-4 py-2.5 transition-colors hover:bg-accent/30">
																<span className="min-w-0 flex-1">
																	<span className="block truncate text-sm text-foreground">
																		{actionLabel(action)}
																	</span>
																	<span className="block truncate text-[11px] text-muted-foreground">
																		{action}
																	</span>
																</span>
																<Switch
																	checked={draft.actions.includes(action)}
																	onCheckedChange={() =>
																		toggleAction(action)
																	}
																/>
															</label>
														))}
													</div>
												</div>
											);
										},
									)}
								</div>
							)}
						</div>

						<SheetFooter>
							<Button
								onClick={() => void save()}
								disabled={busy || !draft.name.trim()}
								className="h-10 gap-1.5 px-5">
								{saving}
							</Button>
						</SheetFooter>
					</>
				)}
			</SheetContent>
		</Sheet>
	);
}
