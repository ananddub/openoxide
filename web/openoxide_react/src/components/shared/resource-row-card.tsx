import React from 'react';
import {Card} from '#/components/ui/card';
import {Badge} from '#/components/ui/badge';

/**
 * Horizontal provider/resource card row: leading icon, bold name with an
 * optional outline badge, extra meta content and trailing actions.
 */
export function ResourceRowCard({
	icon,
	name,
	badge,
	children,
	actions,
}: {
	icon: React.ReactNode;
	name: string;
	badge?: string;
	children?: React.ReactNode;
	actions?: React.ReactNode;
}) {
	return (
		<Card className="flex flex-col justify-between gap-4 rounded-xl bg-card p-4 shadow-2xs transition-all sm:flex-row sm:items-center">
			<div className="flex min-w-0 items-center gap-3.5">
				{icon}
				<div className="flex min-w-0 flex-col gap-1">
					<div className="flex items-center gap-2.5">
						<span className="truncate text-sm font-bold text-foreground">
							{name}
						</span>
						{badge && (
							<Badge
								variant="outline"
								className="shrink-0 font-mono text-[10px]">
								{badge}
							</Badge>
						)}
					</div>
					{children}
				</div>
			</div>

			<div className="flex shrink-0 items-center gap-2 self-end sm:self-center">
				{actions}
			</div>
		</Card>
	);
}

/** "• Added <date>" micro copy shared by provider rows. */
export function AddedDate({created_at}: {created_at: number | string}) {
	return (
		<span className="text-[11px] text-muted-foreground">
			• Added {new Date(Number(created_at) * 1000).toLocaleDateString()}
		</span>
	);
}
