import React from 'react';
import {Titlebar} from '#/components/shared/titlebar';
import {Plus} from 'lucide-react';
import {Button} from '#/components/ui/button';

/**
 * Section page header: icon + title + description with an optional
 * primary "Add …" action. `variant="bordered"` matches the settings-style
 * pages, `variant="plain"` the dashboard-style pages.
 */
export function PageHeader({
	variant: _variant = 'bordered',
	icon,
	title,
	description,
	actionLabel,
	onAction,
}: {
	variant?: 'bordered' | 'plain';
	icon?: React.ReactNode;
	title: string;
	description: React.ReactNode;
	actionLabel?: string;
	onAction?: () => void;
}) {
	return (
		<Titlebar
			icon={icon}
			title={title}
			description={description}
			actions={
				onAction && (
					<Button onClick={onAction} size="sm" className="gap-1.5">
						<Plus className="size-3.5" /> {actionLabel}
					</Button>
				)
			}
		/>
	);
}
