import {Search} from 'lucide-react';
import {useEffect, useMemo, useState} from 'react';
import {toast} from 'sonner';
import {Button} from '#/components/ui/button';
import {Checkbox} from '#/components/ui/checkbox';
import {Input} from '#/components/ui/input';
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

/** Column order a reader expects; anything new lands at the end, alphabetically. */
const OP_ORDER = [
	'read',
	'create',
	'update',
	'delete',
	'deploy',
	'write',
	'monitor',
];

const opRank = (op: string) => {
	const i = OP_ORDER.indexOf(op);
	return i === -1 ? OP_ORDER.length : i;
};

/** Resource + operation of an action, ready for the matrix. */
type Row = {resource: string; cells: Map<string, string>};

/** Create/edit a permission group — a resource × operation checkbox grid. */
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

	const catalog = useMemo(
		() =>
			policies.map(policy => (policy.action ?? '').trim()).filter(Boolean),
		[policies],
	);

	const {rows, ops} = useMemo(() => {
		const byResource = new Map<string, Map<string, string>>();
		const operations = new Set<string>();
		for (const action of catalog) {
			const {resource, operation} = split(action);
			const cells = byResource.get(resource) ?? new Map<string, string>();
			cells.set(operation, action);
			byResource.set(resource, cells);
			operations.add(operation);
		}
		return {
			rows: [...byResource.entries()]
				.map(([resource, cells]): Row => ({resource, cells}))
				.sort((a, b) => a.resource.localeCompare(b.resource)),
			ops: [...operations].sort(
				(a, b) => opRank(a) - opRank(b) || a.localeCompare(b),
			),
		};
	}, [catalog]);

	const search = query.trim().toLowerCase();
	const visibleRows = search
		? rows.filter(
				row =>
					row.resource.includes(search) ||
					[...row.cells.keys()].some(op => op.includes(search)) ||
					[...row.cells.values()].some(action => action.includes(search)),
			)
		: rows;

	// Actions outside the catalog (a retired or not-yet-synced code) are kept verbatim so editing a group never silently drops them.
	const unknown = catalog.length
		? (draft?.actions ?? []).filter(a => !catalog.includes(a))
		: [];

	const replace = (target: string[], on: boolean, next: string[]) => {
		if (!draft) return;
		const targetSet = new Set(target);
		setDraft({
			...draft,
			actions: [
				...unknown,
				...draft.actions.filter(a => !targetSet.has(a)),
				...(on ? next : []),
			],
		});
	};

	const selected = new Set(draft?.actions ?? []);

	const toggleAction = (action: string) =>
		replace([action], !selected.has(action), [action]);

	const toggleRow = (row: Row) => {
		const list = [...row.cells.values()];
		replace(list, !list.every(a => selected.has(a)), list);
	};

	const toggleColumn = (op: string) => {
		const list = visibleRows
			.map(row => row.cells.get(op))
			.filter((a): a is string => !!a);
		replace(list, !list.every(a => selected.has(a)), list);
	};

	const toggleEverything = () => {
		if (!draft) return;
		setDraft({
			...draft,
			actions: allSelected ? unknown : [...unknown, ...catalog],
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
	const selectedCount = (draft?.actions ?? []).filter(a =>
		catalog.includes(a),
	).length;
	const allSelected =
		catalog.length > 0 && selectedCount === catalog.length;

	const grid = {
		gridTemplateColumns: `minmax(120px,1.3fr) repeat(${ops.length}, minmax(56px,1fr))`,
		minWidth: '440px',
	};

	return (
		<Sheet open={!!draft} onOpenChange={open => !open && setDraft(null)}>
			<SheetContent className="max-w-[calc(100vw_-_1.5rem)] sm:data-[side=right]:w-[720px]">
				<SheetHeader>
					<SheetTitle>{draft?.id ? 'Edit Group' : 'New Group'}</SheetTitle>
					<SheetDescription>
						Name the role, then tick what its members may do. Rows are
						things, columns are actions.
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
									{selectedCount} of {catalog.length} allowed
								</p>
								<button
									type="button"
									onClick={toggleEverything}
									className="cursor-pointer text-[13px] font-medium text-primary hover:underline">
									{allSelected ? 'Clear all' : 'Allow everything'}
								</button>
							</div>

							{catalog.length === 0 ? (
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
										placeholder="Filter — app, read, deploy"
										className="h-9 rounded-lg bg-background pl-9 text-sm transition-colors duration-150 hover:border-foreground/25"
									/>
								</div>
							)}

							{catalog.length > 0 && visibleRows.length === 0 && (
								<div className="w-full shrink-0 rounded-xl border bg-card px-4 py-8 text-center">
									<p className="text-sm font-medium text-foreground">
										Nothing matches “{query.trim()}”
									</p>
									<p className="mt-1 text-[13px] text-muted-foreground">
										Try a resource like app or database, or an action like
										read.
									</p>
								</div>
							)}

							{catalog.length > 0 && visibleRows.length > 0 && (
								<div className="w-full shrink-0 overflow-hidden rounded-xl border bg-card">
									<div className="max-h-[52vh] overflow-auto">
										{/* Column headers — tick one to take the whole column. */}
										<div
											style={grid}
											className="sticky top-0 z-10 grid items-center border-b bg-muted/50 backdrop-blur-sm">
											<div className="px-3 py-2 text-[11px] font-semibold tracking-wide text-muted-foreground uppercase">
												Access
											</div>
											{ops.map(op => {
												const column = visibleRows
													.map(row => row.cells.get(op))
													.filter((a): a is string => !!a);
												const on =
													column.length > 0 &&
													column.every(a => selected.has(a));
												return (
													<button
														key={op}
														type="button"
														onClick={() => toggleColumn(op)}
														title={`${on ? 'Clear' : 'Select all'} ${titleCase(op)}`}
														className="flex cursor-pointer flex-col items-center gap-1 py-2 transition-colors hover:bg-accent/40">
														<Checkbox
															checked={on}
															tabIndex={-1}
															className="pointer-events-none"
														/>
														<span className="text-[10px] leading-none text-muted-foreground">
															{titleCase(op)}
														</span>
													</button>
												);
											})}
										</div>

										{visibleRows.map(row => {
											const list = [...row.cells.values()];
											const allOn = list.every(a => selected.has(a));
											return (
												<div
													key={row.resource}
													style={grid}
													className="grid items-center border-b border-border/60 last:border-b-0">
													<button
														type="button"
														onClick={() => toggleRow(row)}
														title={
															allOn
																? `Clear ${titleCase(row.resource)}`
																: `Allow everything on ${titleCase(row.resource)}`
														}
														className="flex cursor-pointer items-center gap-2 px-3 py-2 text-left transition-colors hover:bg-accent/40">
														<Checkbox
															checked={allOn}
															tabIndex={-1}
															className="pointer-events-none"
														/>
														<span className="truncate text-[13px] font-medium text-foreground">
															{titleCase(row.resource)}
														</span>
													</button>
													{ops.map(op => {
														const action = row.cells.get(op);
														if (!action) {
															return (
																<span
																	key={op}
																	title="Not available"
																	className="text-center text-[13px] text-muted-foreground/30">
																	—
																</span>
															);
														}
														return (
															<button
																key={op}
																type="button"
																onClick={() => toggleAction(action)}
																title={`${titleCase(op)} ${titleCase(row.resource)} · ${action}`}
																aria-pressed={selected.has(action)}
																className="grid cursor-pointer place-items-center py-2 transition-colors hover:bg-accent/40">
																<Checkbox
																	checked={selected.has(action)}
																	tabIndex={-1}
																	className="pointer-events-none"
																/>
															</button>
														);
													})}
												</div>
											);
										})}
									</div>
								</div>
							)}

							<p className="text-[12px] leading-relaxed text-muted-foreground/80">
								Unticked means denied — a member in this group cannot do
								that action. Members can belong to several groups; their
								permissions add up.
							</p>
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
