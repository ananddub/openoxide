import {cn} from '#/api/utils';
import {Power} from 'lucide-react';

interface Props {
	className?: string;
	logoUrl?: string;
}

export const Logo = ({className = 'size-8', logoUrl}: Props) => {
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
		<Power
			className={cn('shrink-0 stroke-[2.5] text-primary', className)}
		/>
	);
};
