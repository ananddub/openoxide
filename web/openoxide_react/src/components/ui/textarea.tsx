import * as React from 'react';

import {cn} from '#/api/utils.ts';

function Textarea({
	className,
	...props
}: React.ComponentProps<'textarea'>) {
	return (
		<textarea
			data-slot="textarea"
			className={cn(
				'flex field-sizing-content min-h-24 w-full rounded-md border border-input bg-card px-3 py-2 text-sm text-foreground shadow-[0_1px_1px_rgba(0,0,0,0.02)] transition-colors duration-150 outline-none placeholder:text-muted-foreground focus-visible:border-foreground/40 focus-visible:shadow-none focus-visible:ring-2 focus-visible:ring-ring/30 disabled:cursor-not-allowed disabled:opacity-50 aria-invalid:border-destructive aria-invalid:ring-3 aria-invalid:ring-destructive/20 dark:aria-invalid:border-destructive/50 dark:aria-invalid:ring-destructive/40',
				className,
			)}
			{...props}
		/>
	);
}

export {Textarea};
