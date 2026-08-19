import {Badge} from '#/components/ui/badge';
import {cn} from '#/api/utils';

interface TagBadgeProps {
	name: string;
	color?: string | null;
	className?: string;
	children?: React.ReactNode;
}

export function TagBadge({
	name,
	color,
	className,
	children,
}: TagBadgeProps) {
	const activeColor = color || '#3b82f6';
	return (
		<Badge
			style={{
				backgroundColor: `${activeColor}20`,
				color: activeColor,
				borderColor: `${activeColor}40`,
			}}
			className={cn(
				'inline-flex items-center gap-1.5 rounded-md border px-2.5 py-1 text-xs font-semibold shadow-2xs transition-all',
				className,
			)}>
			<span
				className="size-2 shrink-0 rounded-full shadow-2xs"
				style={{backgroundColor: activeColor}}
			/>
			<span>{name}</span>
			{children}
		</Badge>
	);
}
