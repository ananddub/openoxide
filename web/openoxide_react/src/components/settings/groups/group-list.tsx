import {Pencil, ShieldCheck, Trash2} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Switch} from '#/components/ui/switch';
import {
	ListCard,
	type ListView,
	RowTile,
	ViewCard,
	ViewGrid,
} from '#/components/settings/list-card';
import type {PermissionGroupPayload} from '#/api/live/permissions/types';

type Props = {
	groups: PermissionGroupPayload[];
	view: ListView;
	canManage: boolean;
	/** May flip the enable/disable switch (group:update). */
	canToggle: boolean;
	togglingId: bigint | null;
	onToggleActive: (group: PermissionGroupPayload, next: boolean) => void;
	onEdit: (group: PermissionGroupPayload) => void;
	onDelete: (group: PermissionGroupPayload) => void;
};

const rowCls =
	'group flex w-full items-start gap-3.5 px-4 py-3 text-start transition-colors hover:bg-accent/30';

function GroupTile() {
	return (
		<RowTile>
			<ShieldCheck className="size-4" />
		</RowTile>
	);
}

function SystemChip() {
	return (
		<span className="shrink-0 rounded-md bg-foreground/[0.05] px-1.5 py-0.5 text-[10px] font-medium text-muted-foreground">
			System
		</span>
	);
}

function DisabledChip() {
	return (
		<span className="shrink-0 rounded-md bg-destructive/10 px-1.5 py-0.5 text-[10px] font-medium text-destructive">
			Disabled
		</span>
	);
}

/** Enable/disable handle; system groups are always on and show nothing. */
function ActiveSwitch({
	group,
	canToggle,
	togglingId,
	onToggleActive,
}: {
	group: PermissionGroupPayload;
	canToggle: boolean;
	togglingId: bigint | null;
	onToggleActive: (group: PermissionGroupPayload, next: boolean) => void;
}) {
	if (group.is_system || !canToggle) return null;
	return (
		<Switch
			size="sm"
			aria-label={`${group.is_active ? 'Disable' : 'Enable'} ${group.name}`}
			checked={group.is_active}
			disabled={togglingId === group.id}
			onCheckedChange={next => onToggleActive(group, next)}
		/>
	);
}

function ActionChips({
	actions,
	limit,
}: {
	actions: string[];
	limit: number;
}) {
	return (
		<div className="flex flex-wrap gap-1">
			{actions.slice(0, limit).map(action => (
				<span
					key={action}
					className="shrink-0 rounded-md bg-foreground/[0.05] px-1.5 py-0.5 font-mono text-[10px] text-muted-foreground">
					{action}
				</span>
			))}
			{actions.length > limit && (
				<span className="self-center text-[11px] text-muted-foreground">
					+{actions.length - limit} more
				</span>
			)}
		</div>
	);
}

function Identity({
	group,
	withChips,
}: {
	group: PermissionGroupPayload;
	withChips?: boolean;
}) {
	return (
		<div className="min-w-0 flex-1">
			<div className="flex min-w-0 flex-wrap items-center gap-2">
				<span className="truncate text-sm font-medium text-foreground">
					{group.name}
				</span>
				{group.is_system && <SystemChip />}
				{!group.is_active && <DisabledChip />}
				<span className="text-xs text-muted-foreground">
					{group.actions.length} actions
				</span>
			</div>
			{withChips && (
				<div className="mt-1.5">
					<ActionChips actions={group.actions} limit={12} />
				</div>
			)}
		</div>
	);
}

function Actions({
	group,
	canManage,
	onEdit,
	onDelete,
}: {
	group: PermissionGroupPayload;
	canManage: boolean;
	onEdit: (group: PermissionGroupPayload) => void;
	onDelete: (group: PermissionGroupPayload) => void;
}) {
	if (!canManage) return null;
	return (
		<div className="flex shrink-0 items-center gap-1.5 opacity-0 transition-opacity group-focus-within:opacity-100 group-hover:opacity-100">
			<Button
				variant="ghost"
				size="icon"
				title="Edit group"
				className="size-7 rounded-lg text-muted-foreground hover:text-foreground"
				onClick={() => onEdit(group)}>
				<Pencil className="size-3.5" />
			</Button>
			{!group.is_system && (
				<Button
					variant="ghost"
					size="icon"
					title="Delete group"
					className="size-7 rounded-lg text-muted-foreground hover:bg-destructive/10 hover:text-destructive"
					onClick={() => onDelete(group)}>
					<Trash2 className="size-3.5" />
				</Button>
			)}
		</div>
	);
}

/** Permission groups — grid cards or one bordered list card, same grammar. */
export function GroupList({
	groups,
	view,
	canManage,
	canToggle,
	togglingId,
	onToggleActive,
	onEdit,
	onDelete,
}: Props) {
	const switchProps = (group: PermissionGroupPayload) => ({
		group,
		canToggle,
		togglingId,
		onToggleActive,
	});
	const shared = (group: PermissionGroupPayload) => ({
		group,
		canManage,
		onEdit,
		onDelete,
	});

	if (view === 'grid') {
		return (
			<ViewGrid>
				{groups.map(group => (
					<ViewCard key={String(group.id)}>
						<div
							className={
								'flex min-w-0 items-start gap-3' +
								(group.is_active ? '' : ' opacity-60')
							}>
							<GroupTile />
							<Identity group={group} />
						</div>
						<div className="flex items-end justify-between gap-2">
							<div className="min-w-0">
								<ActionChips actions={group.actions} limit={5} />
							</div>
							<div className="flex shrink-0 items-center gap-1.5">
								<ActiveSwitch {...switchProps(group)} />
								<Actions {...shared(group)} />
							</div>
						</div>
					</ViewCard>
				))}
			</ViewGrid>
		);
	}

	return (
		<ListCard>
			{groups.map(group => (
				<div key={String(group.id)} className={rowCls}>
					<div
						className={
							'flex min-w-0 flex-1 items-start gap-3.5' +
							(group.is_active ? '' : ' opacity-60')
						}>
						<GroupTile />
						<Identity group={group} withChips />
					</div>
					<ActiveSwitch {...switchProps(group)} />
					<Actions {...shared(group)} />
				</div>
			))}
		</ListCard>
	);
}
