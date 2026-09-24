import * as React from 'react';
import {Input as InputPrimitive} from '@base-ui/react/input';

import {cn} from '#/api/utils.ts';

function Input({
	className,
	type,
	...props
}: React.ComponentProps<'input'>) {
	return (
		<InputPrimitive
			type={type}
			data-slot="input"
			className={cn(
				'h-9 w-full min-w-0 rounded-md border border-input bg-card px-3 py-2 text-xs text-foreground shadow-[0_1px_1px_rgba(0,0,0,0.02)] transition-colors duration-150 outline-none file:inline-flex file:h-6 file:border-0 file:bg-transparent file:text-sm file:font-medium file:text-foreground placeholder:text-muted-foreground focus-visible:border-foreground/40 focus-visible:shadow-none focus-visible:ring-2 focus-visible:ring-ring/30 disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50 aria-invalid:border-destructive aria-invalid:ring-3 aria-invalid:ring-destructive/20 dark:aria-invalid:border-destructive/50 dark:aria-invalid:ring-destructive/40',
				className,
			)}
			{...props}
		/>
	);
}

export {Input};
