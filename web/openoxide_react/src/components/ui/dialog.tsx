import * as React from 'react';
import {Dialog as DialogPrimitive} from '@base-ui/react/dialog';

import {cn} from '#/api/utils.ts';
import {Button} from '#/components/ui/button.tsx';
import {XIcon} from 'lucide-react';

function Dialog({...props}: DialogPrimitive.Root.Props) {
	return <DialogPrimitive.Root data-slot="dialog" {...props} />;
}

function DialogTrigger({...props}: DialogPrimitive.Trigger.Props) {
	return <DialogPrimitive.Trigger data-slot="dialog-trigger" {...props} />;
}

function DialogPortal({...props}: DialogPrimitive.Portal.Props) {
	return <DialogPrimitive.Portal data-slot="dialog-portal" {...props} />;
}

function DialogClose({...props}: DialogPrimitive.Close.Props) {
	return <DialogPrimitive.Close data-slot="dialog-close" {...props} />;
}

function DialogOverlay({
	className,
	...props
}: DialogPrimitive.Backdrop.Props) {
	return (
		<DialogPrimitive.Backdrop
			data-slot="dialog-overlay"
			className={cn(
				// Geist modal scrim: dark enough to seat the surface, slight blur keeps the page readable behind it.
				'fixed inset-0 isolate z-50 bg-black/50 duration-150 supports-backdrop-filter:backdrop-blur-[2px] data-open:animate-in data-open:fade-in-0 data-closed:animate-out data-closed:fade-out-0',
				className,
			)}
			{...props}
		/>
	);
}

function DialogContent({
	className,
	children,
	showCloseButton = true,
	...props
}: DialogPrimitive.Popup.Props & {
	showCloseButton?: boolean;
}) {
	return (
		<DialogPortal>
			<DialogOverlay />
			<DialogPrimitive.Popup
				data-slot="dialog-content"
				className={cn(
					// Geist modal surface: hairline edge (alpha ladder), the elevated two-layer shadow — tight contact layer + wide soft ambient — and roomy padding.
					'fixed top-1/2 left-1/2 z-50 grid w-full max-w-[calc(100%-2rem)] -translate-x-1/2 -translate-y-1/2 gap-4 rounded-xl border border-border bg-popover p-6 text-sm text-popover-foreground shadow-[0_1px_2px_rgba(0,0,0,0.04),0_16px_40px_-8px_rgba(0,0,0,0.18),0_32px_64px_-16px_rgba(0,0,0,0.12)] duration-150 outline-none sm:max-w-lg dark:shadow-[0_1px_2px_rgba(0,0,0,0.4),0_16px_40px_-8px_rgba(0,0,0,0.6),0_32px_64px_-16px_rgba(0,0,0,0.5)] data-open:animate-in data-open:fade-in-0 data-open:zoom-in-95 data-closed:animate-out data-closed:fade-out-0 data-closed:zoom-out-95',
					className,
				)}
				{...props}>
				{children}
				{showCloseButton && (
					<DialogPrimitive.Close
						data-slot="dialog-close"
						render={
							<Button
								variant="ghost"
								className="absolute top-3.5 right-3.5 text-muted-foreground hover:text-foreground"
								size="icon-sm"
							/>
						}>
						<XIcon />
						<span className="sr-only">Close</span>
					</DialogPrimitive.Close>
				)}
			</DialogPrimitive.Popup>
		</DialogPortal>
	);
}

function DialogHeader({className, ...props}: React.ComponentProps<'div'>) {
	return (
		<div
			data-slot="dialog-header"
			className={cn('flex flex-col gap-1.5', className)}
			{...props}
		/>
	);
}

function DialogFooter({
	className,
	showCloseButton = false,
	children,
	...props
}: React.ComponentProps<'div'> & {
	showCloseButton?: boolean;
}) {
	return (
		// Geist modal footer: actions right-aligned on the surface — no tinted band, just a hairline to separate from the body.
		<div
			data-slot="dialog-footer"
			className={cn(
				'-mx-6 -mb-6 flex flex-col-reverse gap-2 rounded-b-xl border-t px-6 py-4 sm:flex-row sm:justify-end',
				className,
			)}
			{...props}>
			{children}
			{showCloseButton && (
				<DialogPrimitive.Close render={<Button variant="outline" />}>
					Close
				</DialogPrimitive.Close>
			)}
		</div>
	);
}

function DialogTitle({className, ...props}: DialogPrimitive.Title.Props) {
	return (
		<DialogPrimitive.Title
			data-slot="dialog-title"
			className={cn(
				'text-[17px] leading-snug font-semibold tracking-[-0.01em]',
				className,
			)}
			{...props}
		/>
	);
}

function DialogDescription({
	className,
	...props
}: DialogPrimitive.Description.Props) {
	return (
		<DialogPrimitive.Description
			data-slot="dialog-description"
			className={cn(
				'text-sm leading-relaxed text-muted-foreground *:[a]:underline *:[a]:underline-offset-3 *:[a]:hover:text-foreground',
				className,
			)}
			{...props}
		/>
	);
}

export {
	Dialog,
	DialogClose,
	DialogContent,
	DialogDescription,
	DialogFooter,
	DialogHeader,
	DialogOverlay,
	DialogPortal,
	DialogTitle,
	DialogTrigger,
};
