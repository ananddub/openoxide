import type {ReactNode} from 'react';
import {Pencil, Plus, Trash2} from 'lucide-react';
import {Button} from '#/components/ui/button';

/** Card wrapper for one Advanced-tab feature section. */
export function AdvancedSection({
	icon,
	title,
	description,
	action,
	children,
}: {
	icon?: ReactNode;
	title: string;
	description?: string;
	action?: ReactNode;
	children: ReactNode;
}) {
	return (
		<section className="flex flex-col gap-4 rounded-xl bg-card p-6 shadow-sm">
			<div className="flex items-start justify-between gap-3">
				<div>
					<h3 className="flex items-center gap-2 text-base font-bold tracking-tight text-foreground">
						{icon}
						{title}
					</h3>
					{description && (
						<p className="mt-0.5 text-xs text-muted-foreground">
							{description}
						</p>
					)}
				</div>
				{action}
			</div>
			{children}
		</section>
	);
}

/** One row in a resource list with edit/delete affordances. */
export function ResourceRow({
	title,
	subtitle,
	badges,
	onEdit,
	onDelete,
	deleting,
}: {
	title: ReactNode;
	subtitle?: ReactNode;
	badges?: ReactNode;
	onEdit?: () => void;
	onDelete?: () => void;
	deleting?: boolean;
}) {
	return (
		<div className="flex items-center justify-between gap-3 rounded-lg border border-border/40 bg-background/40 px-3 py-2.5">
			<div className="min-w-0">
				<div className="truncate font-mono text-xs font-semibold text-foreground">
					{title}
				</div>
				{subtitle && (
					<div className="truncate text-[11px] text-muted-foreground">
						{subtitle}
					</div>
				)}
			</div>
			<div className="flex shrink-0 items-center gap-1.5">
				{badges}
				{onEdit && (
					<Button
						variant="ghost"
						size="icon"
						title="Edit"
						onClick={onEdit}
						className="size-7 rounded-lg text-muted-foreground hover:text-foreground">
						<Pencil className="size-3.5" />
					</Button>
				)}
				{onDelete && (
					<Button
						variant="ghost"
						size="icon"
						title="Delete"
						disabled={deleting}
						onClick={onDelete}
						className="size-7 rounded-lg text-muted-foreground hover:bg-destructive/10 hover:text-destructive">
						<Trash2 className="size-3.5" />
					</Button>
				)}
			</div>
		</div>
	);
}

export function AddResourceButton({
	onClick,
	label,
}: {
	onClick: () => void;
	label: string;
}) {
	return (
		<Button
			variant="outline"
			size="sm"
			onClick={onClick}
			className="h-8 items-center gap-1.5 rounded-lg text-xs font-semibold">
			<Plus className="size-3.5" /> {label}
		</Button>
	);
}

export function EmptyRow({label}: {label: string}) {
	return (
		<p className="rounded-lg border border-dashed border-border/50 px-3 py-4 text-center text-xs text-muted-foreground">
			{label}
		</p>
	);
}

/** Danger-zone action button (label swaps to a working state while busy). */
export function DangerAction({
	label,
	busy,
	onConfirm,
}: {
	label: string;
	busy?: boolean;
	onConfirm: () => void;
}) {
	return (
		<Button
			variant="outline"
			disabled={busy}
			onClick={onConfirm}
			className="flex h-9 items-center gap-1.5 rounded-lg border-destructive/40 text-xs font-semibold text-destructive/80 hover:bg-destructive/5 hover:text-destructive">
			{busy ? 'Working…' : label}
		</Button>
	);
}
