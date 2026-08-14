import {cn} from '#/api/utils';
import {Power} from 'lucide-react';

interface Props {
	className?: string;
	logoUrl?: string;
}

export const Logo = ({
	className = 'size-8',
	logoUrl,
}: Props) => {
	if (logoUrl && logoUrl !== '/gokploy.png') {
		return (
			<img
				src={logoUrl}
				alt="OpenOxide Logo"
				className={cn(className, 'rounded-sm object-contain')}
			/>
		);
	}
	return (
		<div className={cn('flex items-center justify-center rounded-lg bg-primary/10 p-1 text-primary shrink-0', className)}>
			<Power className="size-full stroke-[2.5]" />
		</div>
	);
};
