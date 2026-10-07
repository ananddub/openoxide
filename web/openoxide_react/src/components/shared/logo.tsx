import {cn} from '#/api/utils';

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
		<svg
			xmlns="http://www.w3.org/2000/svg"
			viewBox="0 0 24 24"
			fill="none"
			stroke="currentColor"
			strokeWidth="2"
			strokeLinecap="round"
			strokeLinejoin="round"
			aria-hidden="true"
			className={cn('shrink-0 text-primary', className)}>
			<path d="M12 3v6.5" />
			<path d="M18.4 6.6a9 9 0 1 1-12.77.04" />
		</svg>
	);
};
