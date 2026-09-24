import {useState, type ReactNode} from 'react';
import {ChevronRight} from 'lucide-react';
import {cn} from '#/api/utils';
import {
	Sheet,
	SheetContent,
	SheetHeader,
	SheetFooter,
	SheetTitle,
	SheetDescription,
} from '#/components/ui/sheet';

interface SettingsSheetProps {
	title: string;
	/** One quiet line under the title, shown in the row and the sheet header. */
	description?: ReactNode;
	/** Small lucide icon rendered left of the title. */
	icon?: ReactNode;
	defaultOpen?: boolean;
	/** Small content (an inline action button, ...) shown right of the title. */
	meta?: ReactNode;
	/** Primary action(s) pinned to the sheet footer (e.g. the Save button). */
	footer?: ReactNode;
	/** Replaces the default Cancel button on the footer's left (e.g. a Back action). */
	footerLeft?: ReactNode;
	/** Fires when the sheet opens/closes, so callers can reset sub-views. */
	onOpenChange?: (open: boolean) => void;
	/** Extra classes for the SheetContent shell (e.g. a wider panel for
	 * editor-heavy forms). */
	contentClassName?: string;
	children: ReactNode;
}

/** Geist settings strip whose body lives in a right-side sheet instead of collapsing inline. The row looks like `SettingsCollapse`; clicking it slides open a panel holding the form (title + description + children), with an optional sticky footer for the Save action. */
export function SettingsSheet({
	title,
	description,
	icon,
	defaultOpen = false,
	meta,
	footer,
	footerLeft,
	onOpenChange,
	contentClassName,
	children,
}: SettingsSheetProps) {
	const [open, setOpen] = useState(defaultOpen);
	const setSheetOpen = (next: boolean) => {
		setOpen(next);
		onOpenChange?.(next);
	};
	return (
		<>
			<div
				role="button"
				tabIndex={0}
				onClick={() => setSheetOpen(true)}
				onKeyDown={event => {
					if (event.key === 'Enter' || event.key === ' ') {
						event.preventDefault();
						setSheetOpen(true);
					}
				}}
				className={cn(
					'group flex w-full cursor-pointer items-center gap-3 border-t border-border px-5 py-3.5 text-left transition-colors first:border-t-0 hover:bg-muted/30 focus-visible:ring-1 focus-visible:ring-ring focus-visible:outline-none',
					open && 'bg-muted/30',
				)}>
				{icon && <span className="shrink-0 text-primary">{icon}</span>}
				<span className="flex min-w-0 flex-col">
					<span className="truncate text-sm font-medium text-foreground">
						{title}
					</span>
					{description && (
						<span className="truncate text-xs text-muted-foreground">
							{description}
						</span>
					)}
				</span>
				<span className="ml-auto flex shrink-0 items-center gap-2">
					{meta && (
						<span
							onClick={event => event.stopPropagation()}
							onKeyDown={event => event.stopPropagation()}>
							{meta}
						</span>
					)}
					<ChevronRight className="size-4 shrink-0 text-muted-foreground/50 transition-transform group-hover:translate-x-0.5" />
				</span>
			</div>

			<Sheet open={open} onOpenChange={setSheetOpen}>
				<SheetContent
					side="right"
					className={cn('gap-0 p-0', contentClassName)}>
					<SheetHeader className="pr-14">
						<SheetTitle>{title}</SheetTitle>
						{description && (
							<SheetDescription>{description}</SheetDescription>
						)}
					</SheetHeader>
					<div className="flex min-h-0 flex-1 flex-col gap-6 overflow-y-auto px-6 py-6">
						{children}
					</div>
					{footer && (
						<SheetFooter className="justify-between">
							<div>{footerLeft}</div>
							<div className="flex items-center gap-3">{footer}</div>
						</SheetFooter>
					)}
				</SheetContent>
			</Sheet>
		</>
	);
}
