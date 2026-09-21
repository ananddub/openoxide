import {cn} from '#/api/utils.ts';

function Skeleton({className, ...props}: React.ComponentProps<'div'>) {
	return (
		<div
			data-slot="skeleton"
			className={cn(
				'animate-pulse rounded-md bg-foreground/[0.06]',
				className,
			)}
			{...props}
		/>
	);
}

export {Skeleton};
