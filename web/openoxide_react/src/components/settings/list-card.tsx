import {
	type ComponentType,
	type ReactNode,
	type Ref,
	useState,
} from 'react';
import {LayoutGrid, List, Search} from 'lucide-react';
import {cn} from '#/api/utils';
import {Input} from '#/components/ui/input';
import {Skeleton} from '#/components/ui/skeleton';

/**
 * The house list kit — one grammar for every resource surface (SSH Keys,
 * Destinations, DNS, Vault, Schedules, Backups, Domains): a `SectionHeader`
 * carrying the title and the single primary action, a quiet `Toolbar` row
 * (search + filter + `ViewToggle`), then either `ViewGrid` cards or ONE
 * bordered `ListCard` of `ListRow`s, plus matching skeleton/empty states.
 *
 * Create/edit always opens a right sheet; deletes use the quiet AlertDialog.
 * Nothing here draws its own frame — the page shell owns the padding.
 */

/** Toolbar above a list: search/filters left, primary action right. */
export function Toolbar({
	children,
	action,
}: {
	children?: ReactNode;
	action?: ReactNode;
}) {
	return (
		<div className="flex w-full animate-in flex-col items-stretch gap-3 duration-200 fade-in sm:flex-row sm:items-center">
			<div className="flex min-w-0 flex-1 flex-wrap items-center gap-2">
				{children}
			</div>
			{action && (
				<div className="flex shrink-0 items-center gap-2">{action}</div>
			)}
		</div>
	);
}

/**
 * Section heading with a full-bleed rule under it: title (+ inline extras
 * like a status pill) and one quiet note below, actions on the right. The
 * `-mx-4`/`px-4` pair lets the rule touch the page edges while the text
 * stays aligned with the content column.
 */
export function SectionHeader({
	title,
	description,
	action,
	children,
}: {
	title: ReactNode;
	description?: ReactNode;
	action?: ReactNode;
	children?: ReactNode;
}) {
	return (
		<div className="-mx-4 flex items-end justify-between gap-4 border-b border-border px-4 pb-4">
			<div className="min-w-0">
				<div className="flex min-w-0 items-center gap-2.5">
					<h3 className="text-2xl font-semibold tracking-[-0.02em] text-foreground">
						{title}
					</h3>
					{children}
				</div>
				{description && (
					<p className="mt-1 text-xs text-muted-foreground">
						{description}
					</p>
				)}
			</div>
			{action && (
				<div className="flex shrink-0 items-center gap-2">{action}</div>
			)}
		</div>
	);
}

/** Search box in the toolbar's house treatment. */
export function ToolbarSearch({
	value,
	onChange,
	placeholder,
	className,
	inputRef,
}: {
	value: string;
	onChange: (next: string) => void;
	placeholder: string;
	className?: string;
	/** Pass a ref to wire a `/` focus shortcut. */
	inputRef?: Ref<HTMLInputElement>;
}) {
	return (
		<div className={cn('relative w-full sm:max-w-xs', className)}>
			<Search className="pointer-events-none absolute top-1/2 left-3 size-4 -translate-y-1/2 text-muted-foreground/70" />
			<Input
				ref={inputRef}
				placeholder={placeholder}
				value={value}
				onChange={e => onChange(e.target.value)}
				className="h-9 rounded-lg pl-9 text-sm transition-colors duration-150 hover:border-foreground/25"
			/>
		</div>
	);
}

/** Quiet label above a card — only for tabs that hold more than one list. */
export function SectionLabel({children}: {children: ReactNode}) {
	return (
		<p className="text-[11px] font-semibold tracking-wide text-muted-foreground uppercase">
			{children}
		</p>
	);
}

export type ListView = 'grid' | 'list';

/** Grid/list choice kept per surface, same as the SSH Keys page. */
export function useListView(key: string) {
	const [view, setView] = useState<ListView>(() =>
		typeof window !== 'undefined' && localStorage.getItem(key) === 'list'
			? 'list'
			: 'grid',
	);
	const change = (next: ListView) => {
		setView(next);
		localStorage.setItem(key, next);
	};
	return [view, change] as const;
}

/** Grid/list switch used by every resource surface's toolbar. */
export function ViewToggle({
	view,
	onChange,
}: {
	view: ListView;
	onChange: (next: ListView) => void;
}) {
	const cell =
		'flex size-7 cursor-pointer items-center justify-center rounded-md text-muted-foreground transition-colors hover:text-foreground';
	return (
		<div className="flex items-center rounded-lg border bg-card p-1">
			<button
				type="button"
				aria-label="Grid view"
				title="Grid view"
				onClick={() => onChange('grid')}
				className={cn(
					cell,
					view === 'grid' && 'bg-accent text-foreground',
				)}>
				<LayoutGrid className="size-3.5" />
			</button>
			<button
				type="button"
				aria-label="List view"
				title="List view"
				onClick={() => onChange('list')}
				className={cn(
					cell,
					view === 'list' && 'bg-accent text-foreground',
				)}>
				<List className="size-3.5" />
			</button>
		</div>
	);
}

/** Card grid container shared by every grid view. */
export function ViewGrid({children}: {children: ReactNode}) {
	return (
		<div className="grid w-full grid-cols-1 gap-3 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 2xl:grid-cols-5">
			{children}
		</div>
	);
}

/** Grid placeholder cards matching `ViewGrid`. */
export function GridSkeleton({cards = 4}: {cards?: number}) {
	return (
		<ViewGrid>
			{[...Array(cards)].map((_, i) => (
				<div
					key={i}
					className="min-h-[124px] animate-pulse rounded-lg border bg-card"
				/>
			))}
		</ViewGrid>
	);
}

/**
 * Grid card shell — the card counterpart of a `ListRow`: flat hairline
 * card, identity row on top, meta pinned to the bottom.
 */
export function ViewCard({
	children,
	className,
}: {
	children: ReactNode;
	className?: string;
}) {
	return (
		<div
			className={cn(
				'group flex min-h-[124px] flex-col justify-between gap-3.5 rounded-lg border bg-card p-4 transition-colors hover:bg-accent/30',
				className,
			)}>
			{children}
		</div>
	);
}

/** One bordered list card. Children are the rows themselves. */
export function ListCard({
	children,
	className,
}: {
	children: ReactNode;
	className?: string;
}) {
	return (
		<div
			className={cn(
				'w-full divide-y overflow-hidden rounded-xl border bg-card',
				className,
			)}>
			{children}
		</div>
	);
}

/** Row inside a ListCard: tile, primary + secondary text, actions on hover. */
export function ListRow({
	icon,
	title,
	subtitle,
	badges,
	children,
	onClick,
	className,
}: {
	icon?: ReactNode;
	title: ReactNode;
	subtitle?: ReactNode;
	badges?: ReactNode;
	children?: ReactNode;
	onClick?: () => void;
	className?: string;
}) {
	const inner = (
		<>
			{icon}
			<div className="min-w-0 flex-1">
				<div className="flex min-w-0 flex-wrap items-center gap-2">
					<span className="truncate text-sm font-medium text-foreground">
						{title}
					</span>
					{badges}
				</div>
				{subtitle && (
					<p className="mt-0.5 truncate text-xs text-muted-foreground">
						{subtitle}
					</p>
				)}
			</div>
			{children && (
				<div className="flex shrink-0 items-center gap-1.5 opacity-0 transition-opacity group-hover:opacity-100 group-focus-within:opacity-100">
					{children}
				</div>
			)}
		</>
	);

	const cls = cn(
		'group flex w-full items-center gap-3.5 px-4 py-3 text-start transition-colors hover:bg-accent/30',
		onClick && 'cursor-pointer',
		className,
	);

	return onClick ? (
		<button type="button" onClick={onClick} className={cls}>
			{inner}
		</button>
	) : (
		<div className={cls}>{inner}</div>
	);
}

/** Icon tile for a row — fill, never a frame. */
export function RowTile({
	children,
	className,
}: {
	children: ReactNode;
	className?: string;
}) {
	return (
		<span
			className={cn(
				'flex size-9 shrink-0 items-center justify-center rounded-lg bg-foreground/[0.05] text-foreground',
				className,
			)}>
			{children}
		</span>
	);
}

/** Loading placeholder in the same card shape as the list it replaces. */
export function ListSkeleton({rows = 3}: {rows?: number}) {
	return (
		<div className="w-full divide-y overflow-hidden rounded-xl border bg-card">
			{[...Array(rows)].map((_, i) => (
				<div key={i} className="flex items-center gap-3.5 px-4 py-3">
					<Skeleton className="size-9 shrink-0 rounded-lg" />
					<div className="flex-1 space-y-2">
						<Skeleton className="h-3.5 w-40" />
						<Skeleton className="h-3 w-64" />
					</div>
					<Skeleton className="h-5 w-16 rounded-full" />
				</div>
			))}
		</div>
	);
}

/** Empty state — replaces the list card, it does not sit inside it. */
export function ListEmpty({
	icon: Icon,
	title,
	description,
	action,
}: {
	icon: ComponentType<{className?: string}>;
	title: string;
	description: string;
	action?: ReactNode;
}) {
	return (
		<div className="flex flex-col items-center justify-center rounded-xl border bg-card p-12 text-center">
			<Icon className="mb-4 size-10 text-muted-foreground/40" />
			<h3 className="mb-2 text-xl font-semibold tracking-[-0.01em] text-foreground">
				{title}
			</h3>
			<p className="mx-auto max-w-sm text-sm leading-relaxed text-muted-foreground/70">
				{description}
			</p>
			{action && <div className="mt-4">{action}</div>}
		</div>
	);
}
