import {Titlebar} from '#/components/shared/titlebar';
import {Plus} from 'lucide-react';
import {Button} from '#/components/ui/button';

/**
 * Section page header — minimal grammar: the title registers as the top
 * bar crumb, only an optional primary "Add …" action renders in-page.
 */
export function PageHeader({
	title,
	actionLabel,
	onAction,
}: {
	title: string;
	actionLabel?: string;
	onAction?: () => void;
}) {
	return (
		<Titlebar
			breadcrumbs={[{label: title}]}
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
