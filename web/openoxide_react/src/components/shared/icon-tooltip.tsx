import type {ReactElement} from 'react';
import {
	Tooltip,
	TooltipContent,
	TooltipTrigger,
} from '#/components/ui/tooltip';

/**
 * House tooltip for icon-only controls — the dark bubble instead of the
 * slow, unstyled browser `title`. Wrap a single element; keep its
 * `aria-label` for screen readers and drop the `title`.
 */
export function IconTooltip({
	label,
	side = 'top',
	children,
}: {
	label: string;
	side?: 'top' | 'bottom' | 'left' | 'right';
	children: ReactElement;
}) {
	return (
		<Tooltip>
			<TooltipTrigger render={children} />
			<TooltipContent side={side} className="px-2 py-1 text-[11px]">
				{label}
			</TooltipContent>
		</Tooltip>
	);
}
