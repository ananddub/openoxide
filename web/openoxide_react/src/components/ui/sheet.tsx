import * as React from 'react';
import {Dialog as SheetPrimitive} from '@base-ui/react/dialog';
import {cn} from '#/api/utils.ts';
import {Button} from '#/components/ui/button.tsx';
import {XIcon} from 'lucide-react';

/**
 * Geist-style side panel (Vercel dashboard "Add New" pattern):
 * full-height drawer from the right over a dark scrim, hairline edge,
 * roomy header/footer. Create/edit flows live here — modals stay for
 * confirmations and quick prompts.
 */

function Sheet({...props}: SheetPrimitive.Root.Props) {
	return <SheetPrimitive.Root data-slot="sheet" {...props} />;
}

function SheetTrigger({...props}: SheetPrimitive.Trigger.Props) {
	return <SheetPrimitive.Trigger data-slot="sheet-trigger" {...props} />;
}

function SheetClose({...props}: SheetPrimitive.Close.Props) {
	return <SheetPrimitive.Close data-slot="sheet-close" {...props} />;
}

function SheetPortal({...props}: SheetPrimitive.Portal.Props) {
	return <SheetPrimitive.Portal data-slot="sheet-portal" {...props} />;
}

function SheetOverlay({
	className,
	...props
}: SheetPrimitive.Backdrop.Props) {
	return (
		<SheetPrimitive.Backdrop
			data-slot="sheet-overlay"
			className={cn(
				'fixed inset-0 z-50 bg-black/50 transition-opacity duration-200 data-ending-style:opacity-0 data-starting-style:opacity-0 supports-backdrop-filter:backdrop-blur-[2px]',
				className,
			)}
			{...props}
		/>
	);
}

function SheetContent({
	className,
	children,
	side = 'right',
	showCloseButton = true,
	...props
}: SheetPrimitive.Popup.Props & {
	side?: 'top' | 'right' | 'bottom' | 'left';
	showCloseButton?: boolean;
}) {
	return (
		<SheetPortal>
			<SheetOverlay />
			<SheetPrimitive.Popup
				data-slot="sheet-content"
				data-side={side}
				className={cn(
					// Base chrome: popover surface, hairline edge, layered
					// shadow matching the Geist modal recipe.
					'fixed z-50 flex flex-col gap-0 bg-popover bg-clip-padding text-sm text-popover-foreground shadow-[0_1px_1px_rgba(0,0,0,0.02),0_8px_24px_rgba(0,0,0,0.12),0_24px_64px_rgba(0,0,0,0.24)] transition duration-200 ease-in-out data-ending-style:opacity-0 data-starting-style:opacity-0',
					// Side placement + slide.
					// Right panel floats: inset from the top/right/bottom edges,
					// fully bordered, with a FIXED px radius so the global
					// --radius token can never flatten it back to a stuck drawer.
					'data-[side=right]:inset-y-3 data-[side=right]:right-3 data-[side=right]:max-h-[calc(100dvh-1.5rem)] data-[side=right]:w-full data-[side=right]:rounded-[14px] data-[side=right]:border data-[side=right]:border-border data-[side=right]:data-ending-style:translate-x-24 data-[side=right]:data-starting-style:translate-x-24',
					'data-[side=left]:inset-y-0 data-[side=left]:left-0 data-[side=left]:h-full data-[side=left]:w-full data-[side=left]:border-r data-[side=left]:border-border data-[side=left]:data-ending-style:-translate-x-24 data-[side=left]:data-starting-style:-translate-x-24',
					'data-[side=bottom]:inset-x-4 data-[side=bottom]:bottom-4 data-[side=bottom]:left-4 data-[side=bottom]:right-4 data-[side=bottom]:max-h-[calc(100dvh-2rem)] data-[side=bottom]:h-auto data-[side=bottom]:w-auto data-[side=bottom]:rounded-2xl data-[side=bottom]:border data-[side=bottom]:border-border data-[side=bottom]:data-ending-style:translate-y-24 data-[side=bottom]:data-starting-style:translate-y-24',
					'data-[side=top]:inset-x-0 data-[side=top]:top-0 data-[side=top]:h-auto data-[side=top]:max-h-[85vh] data-[side=top]:border-b data-[side=top]:data-ending-style:-translate-y-12 data-[side=top]:data-starting-style:-translate-y-12',
					// Vercel side-panel width on desktop.
					'sm:data-[side=right]:w-[560px] sm:data-[side=left]:w-[560px]',
					className,
				)}
				{...props}>
				{children}
				{showCloseButton && (
					<SheetPrimitive.Close
						data-slot="sheet-close"
						render={
							<Button
								variant="ghost"
								className="absolute top-3.5 right-3.5"
								size="icon-sm"
							/>
						}>
						<XIcon />
						<span className="sr-only">Close</span>
					</SheetPrimitive.Close>
				)}
			</SheetPrimitive.Popup>
		</SheetPortal>
	);
}

function SheetHeader({className, ...props}: React.ComponentProps<'div'>) {
	return (
		<div
			data-slot="sheet-header"
			className={cn(
				'flex flex-col gap-1 border-b border-border px-6 py-5',
				className,
			)}
			{...props}
		/>
	);
}

function SheetFooter({className, ...props}: React.ComponentProps<'div'>) {
	return (
		<div
			data-slot="sheet-footer"
			className={cn(
				'mt-auto flex items-center justify-end gap-3 border-t border-border bg-card/40 px-6 py-4',
				className,
			)}
			{...props}
		/>
	);
}

function SheetTitle({className, ...props}: SheetPrimitive.Title.Props) {
	return (
		<SheetPrimitive.Title
			data-slot="sheet-title"
			className={cn(
				'text-[17px] leading-none font-semibold tracking-[-0.01em] text-foreground',
				className,
			)}
			{...props}
		/>
	);
}

function SheetDescription({
	className,
	...props
}: SheetPrimitive.Description.Props) {
	return (
		<SheetPrimitive.Description
			data-slot="sheet-description"
			className={cn('text-[13px] text-muted-foreground', className)}
			{...props}
		/>
	);
}

export {
	Sheet,
	SheetTrigger,
	SheetClose,
	SheetContent,
	SheetHeader,
	SheetFooter,
	SheetTitle,
	SheetDescription,
};
