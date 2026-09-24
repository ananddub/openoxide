import {useState, type ReactNode} from 'react';
import {ChevronDown} from 'lucide-react';
import {cn} from '#/api/utils';
import {
	Collapsible,
	CollapsibleTrigger,
	CollapsibleContent,
} from '#/components/ui/collapsible';

interface SettingsCollapseProps {
	title: string;
	/** One quiet line under the title, shown in the collapsed header. */
	description?: ReactNode;
	/** Small lucide icon rendered left of the title. */
	icon?: ReactNode;
	defaultOpen?: boolean;
	/** Small content (duration, status icon, ...) shown right of the title. */
	meta?: ReactNode;
	onOpenChange?: (open: boolean) => void;
	children: ReactNode;
}

/** Geist-collapse strip for embedding inside the architecture card: full-width header line that expands into the settings body below, matching the Git Integration strip. */
export function SettingsCollapse({
	title,
	description,
	icon,
	defaultOpen = false,
	meta,
	onOpenChange,
	children,
}: SettingsCollapseProps) {
	const [open, setOpen] = useState(defaultOpen);
	const handleOpenChange = (next: boolean) => {
		setOpen(next);
		onOpenChange?.(next);
	};
	return (
		<Collapsible
			open={open}
			onOpenChange={handleOpenChange}
			className="border-t border-border first:border-t-0">
			<CollapsibleTrigger
				className={cn(
					'group flex w-full cursor-pointer items-center gap-3 px-5 py-3.5 text-left transition-colors hover:bg-muted/30',
					open && 'bg-muted/30',
				)}>
				<ChevronDown
					className={cn(
						'size-4 shrink-0 text-muted-foreground transition-transform duration-200',
						!open && '-rotate-90',
					)}
				/>
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
				{meta && (
					<span className="ml-auto flex items-center gap-2">{meta}</span>
				)}
			</CollapsibleTrigger>
			<CollapsibleContent>
				<div className="px-5 pt-1 pb-5">{children}</div>
			</CollapsibleContent>
		</Collapsible>
	);
}
