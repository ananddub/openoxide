import {Badge} from '#/components/ui/badge';
import {cn} from '#/api/utils';

interface TagBadgeProps {
	name: string;
	color?: string | null;
	className?: string;
	children?: React.ReactNode;
}

export function TagBadge({name, color, className, children}: TagBadgeProps) {
	const activeColor = color || '#3b82f6';
	return (
		<Badge
			style={{
				backgroundColor: `${activeColor}20`,
				color: activeColor,
				borderColor: `${activeColor}40`,
			}}
			className={cn(
				'inline-flex items-center gap-1.5 px-2.5 py-1 text-xs font-semibold rounded-md border transition-all shadow-2xs',
				className
			)}
		>
			<span
				className="size-2 rounded-full shrink-0 shadow-2xs"
				style={{backgroundColor: activeColor}}
			/>
			<span>{name}</span>
			{children}
		</Badge>
	);
}
