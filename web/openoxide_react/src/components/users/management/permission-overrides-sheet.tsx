import {Check, Search, X} from 'lucide-react';
import {useEffect, useMemo, useRef, useState} from 'react';
import {toast} from 'sonner';
import {cn, formatApiError} from '#/api/utils';
import {
	usePermissionGroupList,
	usePolicyList,
	useUserPolicies,
	useUserPoliciesReplace,
} from '#/api/live';
import type {MemberItem} from '#/api/live';
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

type Choice = 'GRANT' | 'DENY';
type Policy = {id: number; action: string};
/** Overridden cells are solid, inherited ones washed out — the grid must show both at once. */
type Face = 'force-allow' | 'role-allow' | 'force-deny' | 'role-deny';

const titleCase = (word: string) =>
	word ? word[0].toUpperCase() + word.slice(1).replace(/_/g, ' ') : word;

const split = (action: string) => {
	const [resource = action, operation = action] = action.split(':');
	return {resource, operation};
};

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

/** Resource × operation grid, same shape the group editor uses. */
function buildMatrix(policies: Policy[]) {
	const byResource = new Map<string, Map<string, Policy>>();
	const operations = new Set<string>();
	for (const policy of policies) {
		const {resource, operation} = split(policy.action);
		const cells = byResource.get(resource) ?? new Map<string, Policy>();
		cells.set(operation, policy);
		byResource.set(resource, cells);
		operations.add(operation);
	}
	return {
		rows: [...byResource.entries()]
			.map(([resource, cells]) => ({resource, cells}))
			.sort((a, b) => a.resource.localeCompare(b.resource)),
		ops: [...operations].sort(
			(a, b) => opRank(a) - opRank(b) || a.localeCompare(b),
		),
	};
}

const FACE: Record<
	Face,
	{box: string; icon: typeof Check | null; label: string}
> = {
	'force-allow': {
		box: 'border-primary bg-primary text-primary-foreground',
		icon: Check,
		label: 'Forced allow',
	},
	'role-allow': {
		box: 'border-primary/30 bg-primary/10 text-primary/60',
		icon: Check,
		label: 'Allowed by role',
	},
	'force-deny': {
		box: 'border-destructive/50 bg-destructive/10 text-destructive',
		icon: X,
		label: 'Forced deny',
	},
	'role-deny': {
		box: 'border-input bg-input/50 text-transparent',
		icon: null,
		label: 'Denied by role',
	},
};

const LEGEND: Face[] = [
	'force-allow',
	'role-allow',
	'force-deny',
	'role-deny',
];

/** Same geometry as the Checkbox primitive, so overridden and inherited cells read as one grid. */
function FaceMark({face}: {face: Face}) {
	const {box, icon: Icon} = FACE[face];
	return (
		<span
			className={cn(
				'flex size-4 shrink-0 items-center justify-center rounded-[4px] border transition-colors',
				box,
			)}>
			{Icon && <Icon className="size-3.5" strokeWidth={3} />}
		</span>
	);
}

/** Force individual actions on or off for one member, on top of their role. */
export function PermissionOverridesSheet({
	member,
	onClose,
}: {
	member: MemberItem | null;
	onClose: () => void;
}) {
	const policiesQuery = usePolicyList();
	const groupsQuery = usePermissionGroupList();
	const policies: Policy[] = policiesQuery.data ?? [];
	const groups = groupsQuery.data ?? [];
	const overridesQuery = useUserPolicies(member?.user_id);
	const replaceMutation = useUserPoliciesReplace();

	const [choices, setChoices] = useState<Record<number, Choice>>({});
	const [query, setQuery] = useState('');

	// Seed once per member, after the server state has loaded — reseeding
	// later would wipe unsaved clicks when the query invalidates.
	const seededFor = useRef<number | null>(null);
	useEffect(() => {
		const uid = member?.user_id == null ? null : Number(member.user_id);
		if (uid === null) {
			seededFor.current = null;
			setChoices({});
			setQuery('');
			return;
		}
		if (seededFor.current === uid || !overridesQuery.data) return;
		const seeded: Record<number, Choice> = {};
		for (const row of overridesQuery.data)
			seeded[row.policy_id] = row.effect;
		setChoices(seeded);
		seededFor.current = uid;
	}, [member, overridesQuery.data]);

	const memberGroup = useMemo(
		() =>
			groups.find(
				group =>
					String(group.id) === String(member?.group_id ?? '') &&
					member?.group_id,
			),
		[groups, member],
	);
	const roleActions = useMemo(
		() => new Set(memberGroup?.actions ?? []),
		[memberGroup],
	);
	const roleActive = Boolean(memberGroup?.is_active);

	const {rows, ops} = useMemo(() => buildMatrix(policies), [policies]);
	const search = query.trim().toLowerCase();
	const visibleRows = search
		? rows.filter(
				row =>
					row.resource.includes(search) ||
					[...row.cells.keys()].some(op => op.includes(search)) ||
					[...row.cells.values()].some(p => p.action.includes(search)),
			)
		: rows;

	const faceOf = (policy: Policy): Face => {
		const current = choices[policy.id];
		if (current) return current === 'DENY' ? 'force-deny' : 'force-allow';
		return roleActions.has(policy.action) ? 'role-allow' : 'role-deny';
	};

	/** Click cycles inherit → allow → deny → inherit. */
	const cycle = (policy: Policy) => {
		const current = choices[policy.id];
		const next: Choice | null = !current
			? 'GRANT'
			: current === 'GRANT'
				? 'DENY'
				: null;
		setChoices(state => {
			const updated = {...state};
			if (next === null) delete updated[policy.id];
			else updated[policy.id] = next;
			return updated;
		});
	};

	// Tick a whole row or column to force-allow it; clicking again clears those overrides.
	const toggleGroup = (list: Policy[]) => {
		const on =
			list.length > 0 &&
			list.every(policy => choices[policy.id] === 'GRANT');
		setChoices(state => {
			const updated = {...state};
			for (const policy of list) {
				if (on) delete updated[policy.id];
				else updated[policy.id] = 'GRANT';
			}
			return updated;
		});
	};

	const overrideCount = Object.keys(choices).length;

	const save = async () => {
		if (!member) return;
		try {
			await replaceMutation.mutateAsync({
				user_id: Number(member.user_id),
				policies: Object.entries(choices).map(([id, effect]) => ({
					policy_id: Number(id),
					effect,
				})),
			});
			toast.success('Permission overrides saved');
			onClose();
		} catch (err) {
			toast.error(formatApiError(err));
		}
	};

	const grid = {
		gridTemplateColumns: `minmax(120px,1.3fr) repeat(${ops.length}, minmax(56px,1fr))`,
		minWidth: '440px',
	};

	return (
		<Sheet
			open={Boolean(member)}
			onOpenChange={open => !open && onClose()}>
			<SheetContent className="max-w-[calc(100vw_-_1.5rem)] sm:data-[side=right]:w-[720px]">
				<SheetHeader>
					<SheetTitle>Permission overrides</SheetTitle>
					<SheetDescription>
						Force actions on or off for{' '}
						<span className="font-medium text-foreground">
							{member?.email}
						</span>{' '}
						— an override wins over the role, everything else stays
						inherited.
					</SheetDescription>
				</SheetHeader>

				<div className="flex min-h-0 flex-1 flex-col gap-4 overflow-y-auto px-6 py-6">
					{memberGroup && (
						<div className="flex shrink-0 items-center justify-between gap-3 rounded-xl border bg-card px-3 py-2.5">
							<div className="min-w-0">
								<p className="truncate text-[13px] font-semibold text-foreground">
									{memberGroup.name}
								</p>
								<p className="text-[12px] text-muted-foreground">
									{roleActions.size} actions inherited
									{!roleActive && ' · role disabled'}
								</p>
							</div>
							<span
								className={cn(
									'shrink-0 rounded-md px-1.5 py-0.5 text-[10px] font-medium',
									roleActive
										? 'bg-foreground/[0.05] text-muted-foreground'
										: 'bg-destructive/10 text-destructive',
								)}>
								{roleActive ? 'Base role' : 'Inactive'}
							</span>
						</div>
					)}

					<div className="flex shrink-0 items-center justify-between gap-3">
						<p className="text-[13px] font-medium text-foreground">
							{overrideCount === 0
								? 'No overrides — follows the role'
								: `${overrideCount} override${overrideCount === 1 ? '' : 's'} set`}
						</p>
						{overrideCount > 0 && (
							<button
								type="button"
								onClick={() => setChoices({})}
								className="cursor-pointer text-[13px] font-medium text-primary hover:underline">
								Clear all
							</button>
						)}
					</div>

					<div className="relative w-full shrink-0">
						<Search className="pointer-events-none absolute top-1/2 left-3 size-4 -translate-y-1/2 text-muted-foreground/70" />
						<Input
							value={query}
							onChange={event => setQuery(event.target.value)}
							placeholder="Filter — app, read, update"
							className="h-9 rounded-lg pl-9 text-sm transition-colors duration-150 hover:border-foreground/25"
						/>
					</div>

					{policiesQuery.isLoading ? (
						<div className="w-full shrink-0 rounded-xl border bg-card px-4 py-8 text-center">
							<p className="text-sm font-medium text-foreground">
								Loading actions…
							</p>
							<p className="mt-1 text-[13px] text-muted-foreground">
								Fetching the permission catalog.
							</p>
						</div>
					) : policies.length === 0 ? (
						<div className="w-full shrink-0 rounded-xl border bg-card px-4 py-8 text-center">
							<p className="text-sm font-medium text-foreground">
								The action catalog is empty
							</p>
							<p className="mt-1 text-[13px] text-muted-foreground">
								Apply the policy-catalog migration so the server returns
								the actions a member can override.
							</p>
						</div>
					) : visibleRows.length === 0 ? (
						<div className="w-full shrink-0 rounded-xl border bg-card px-4 py-8 text-center">
							<p className="text-sm font-medium text-foreground">
								Nothing matches “{query.trim()}”
							</p>
							<p className="mt-1 text-[13px] text-muted-foreground">
								Try a resource like app or database, or an action like
								read.
							</p>
						</div>
					) : (
						<div className="w-full shrink-0 overflow-hidden rounded-xl border bg-card">
							<div className="max-h-[52vh] overflow-auto">
								{/* Column headers — tick one to force-allow that action everywhere. */}
								<div
									style={grid}
									className="sticky top-0 z-10 grid items-center border-b bg-muted/50 backdrop-blur-sm">
									<div className="px-3 py-2 text-[11px] font-semibold tracking-wide text-muted-foreground uppercase">
										Access
									</div>
									{ops.map(op => {
										const column = visibleRows
											.map(row => row.cells.get(op))
											.filter((p): p is Policy => !!p);
										const on =
											column.length > 0 &&
											column.every(
												policy => choices[policy.id] === 'GRANT',
											);
										return (
											<button
												key={op}
												type="button"
												disabled={column.length === 0}
												onClick={() => toggleGroup(column)}
												title={
													column.length === 0
														? `No ${titleCase(op)} action`
														: on
															? `Clear overrides on ${titleCase(op)}`
															: `Force allow ${titleCase(op)} everywhere`
												}
												className="flex cursor-pointer flex-col items-center gap-1 py-2 transition-colors hover:bg-accent/40 disabled:cursor-default disabled:hover:bg-transparent">
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
									const on = list.every(p => choices[p.id] === 'GRANT');
									return (
										<div
											key={row.resource}
											style={grid}
											className="grid items-center border-b border-border/60 last:border-b-0">
											<button
												type="button"
												onClick={() => toggleGroup(list)}
												title={
													on
														? `Clear overrides on ${titleCase(row.resource)}`
														: `Force allow every ${titleCase(row.resource)} action`
												}
												className="flex cursor-pointer items-center gap-2 px-3 py-2 text-left transition-colors hover:bg-accent/40">
												<Checkbox
													checked={on}
													tabIndex={-1}
													className="pointer-events-none"
												/>
												<span className="truncate text-[13px] font-medium text-foreground">
													{titleCase(row.resource)}
												</span>
											</button>
											{ops.map(op => {
												const policy = row.cells.get(op);
												if (!policy) {
													return (
														<span
															key={op}
															title="Not available"
															className="text-center text-[13px] text-muted-foreground/30">
															—
														</span>
													);
												}
												const face = faceOf(policy);
												return (
													<button
														key={op}
														type="button"
														onClick={() => cycle(policy)}
														aria-label={`${policy.action} — ${FACE[face].label}`}
														title={`${policy.action} — ${FACE[face].label} · click to change`}
														className="grid h-full cursor-pointer place-items-center py-2 transition-colors hover:bg-accent/40">
														<FaceMark face={face} />
													</button>
												);
											})}
										</div>
									);
								})}
							</div>
						</div>
					)}

					<div className="flex shrink-0 flex-col gap-2.5">
						<div className="flex flex-wrap items-center gap-x-3.5 gap-y-2">
							{LEGEND.map(face => (
								<span
									key={face}
									className="flex items-center gap-1.5 text-[11px] text-muted-foreground">
									<FaceMark face={face} />
									{FACE[face].label}
								</span>
							))}
						</div>
						<p className="text-[12px] leading-relaxed text-muted-foreground/80">
							Click a cell to cycle it: inherited → forced allow → forced
							deny. Overrides apply to this member only and win over the
							role.
						</p>
					</div>
				</div>

				<SheetFooter>
					<Button
						onClick={() => void save()}
						disabled={replaceMutation.isPending || !member}
						className="h-10 gap-1.5 px-5">
						Save overrides
					</Button>
				</SheetFooter>
			</SheetContent>
		</Sheet>
	);
}
