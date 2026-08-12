import {cn} from '#/api/utils';

interface Props {
	className?: string;
	logoUrl?: string;
}

export const Logo = ({
	className = 'size-20',
	logoUrl = '/gokploy.png',
}: Props) => {
	return (
		<img
			src={logoUrl}
			alt="Organization Logo"
			className={cn(className, 'rounded-sm object-contain')}
		/>
	);
};
