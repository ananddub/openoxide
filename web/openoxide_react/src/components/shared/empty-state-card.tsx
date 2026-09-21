import React from 'react';
import {Button} from '#/components/ui/button';
import {Card} from '#/components/ui/card';
import {Plus} from 'lucide-react';

/**
 * Centered "nothing here yet" card with icon, copy and a call-to-action
 * button. Used by provider list pages (vault, dns, ...) and sections.
 */
export function EmptyStateCard({
	icon,
	title,
	description,
	actionLabel = 'Configure First Provider',
	onAction,
}: {
	icon: React.ReactNode;
	title: string;
	description: React.ReactNode;
	actionLabel?: string;
	onAction: () => void;
}) {
	return (
		<Card className="flex flex-col items-center justify-center gap-3 rounded-xl bg-muted/10 p-12 text-center">
			<div className="flex size-12 items-center justify-center rounded-full bg-primary/10 text-primary">
				{icon}
			</div>
			<div className="max-w-sm space-y-1">
				<h3 className="text-sm font-semibold text-foreground">{title}</h3>
				<p className="text-xs text-muted-foreground">{description}</p>
			</div>
			<Button
				onClick={onAction}
				size="sm"
				className="mt-2 h-8.5 gap-1.5 text-xs font-semibold">
				<Plus className="size-3.5" /> {actionLabel}
			</Button>
		</Card>
	);
}

/** Simple centered muted loading line for list pages. */
export function LoadingText({children}: {children: React.ReactNode}) {
	return (
		<div className="p-12 text-center text-xs text-muted-foreground">
			{children}
		</div>
	);
}
