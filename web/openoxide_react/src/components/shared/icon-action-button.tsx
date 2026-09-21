import React from 'react';
import {Pencil, Trash2} from 'lucide-react';
import {Button} from '#/components/ui/button';

type Tone = 'primary' | 'danger' | 'muted';

const toneClasses: Record<Tone, string> = {
	primary: 'hover:bg-primary/10 hover:text-primary',
	danger: 'hover:bg-destructive/10 hover:text-destructive',
	muted: 'hover:bg-accent hover:text-foreground',
};

/** Ghost square icon button used in card row action groups. */
export function IconActionButton({
	icon,
	title,
	onClick,
	tone = 'muted',
}: {
	icon: React.ReactNode;
	title: string;
	onClick: () => void;
	tone?: Tone;
}) {
	return (
		<Button
			variant="ghost"
			size="icon"
			onClick={onClick}
			title={title}
			className={`size-8 rounded-lg text-muted-foreground ${toneClasses[tone]}`}>
			{icon}
		</Button>
	);
}

export const EditIconButton = ({
	onClick,
	title = 'Edit provider',
}: {
	onClick: () => void;
	title?: string;
}) => (
	<IconActionButton
		onClick={onClick}
		icon={<Pencil className="size-4" />}
		title={title}
		tone="primary"
	/>
);

export const DeleteIconButton = ({
	onClick,
	title = 'Delete provider',
}: {
	onClick: () => void;
	title?: string;
}) => (
	<IconActionButton
		onClick={onClick}
		icon={<Trash2 className="size-4" />}
		title={title}
		tone="danger"
	/>
);
