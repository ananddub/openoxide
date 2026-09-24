import React from 'react';

/** Small uppercase muted label used above form inputs across provider dialogs and settings cards. Replaces the repeated `text-[11px] font-bold tracking-wider ... uppercase` inline label. */
export function FieldLabel({
	children,
	htmlFor,
	className = '',
}: {
	children: React.ReactNode;
	htmlFor?: string;
	className?: string;
}) {
	return (
		<label
			htmlFor={htmlFor}
			className={`text-[11px] font-bold tracking-wider text-muted-foreground uppercase ${className}`}>
			{children}
		</label>
	);
}
