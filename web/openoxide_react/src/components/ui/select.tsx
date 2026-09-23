import * as React from 'react';
import {Select as SelectPrimitive} from '@base-ui/react/select';

import {cn} from '#/api/utils.ts';
import {ChevronDownIcon, CheckIcon, ChevronUpIcon} from 'lucide-react';

const SelectPrimitiveRoot = SelectPrimitive.Root;

/**
 * Wraps Base UI's Select.Root and auto-derives the `items` value→label map
 * from the rendered `<SelectItem>` descendants. Base UI's `<Select.Value>`
 * otherwise prints the raw value (a numeric id), which is the "shows 3
 * instead of the name" bug. Callers that pass their own `items` win.
 */
function Select<
	Value,
	Multiple extends boolean | undefined = false,
>({children, items, ...props}: SelectPrimitive.Root.Props<Value, Multiple>) {
	type Items = SelectPrimitive.Root.Props<Value, Multiple>['items'];
	const derivedItems = React.useMemo<Items>(() => {
		if (items !== undefined) return items;
		const collected: Array<{value: unknown; label: React.ReactNode}> = [];
		const seen = new Set<unknown>();
		const walk = (nodes: React.ReactNode) => {
			React.Children.forEach(nodes, child => {
				if (!React.isValidElement(child)) return;
				const cp = child.props as {
					value?: unknown;
					children?: React.ReactNode;
				};
				if (
					child.type === SelectItem &&
					cp.value !== undefined &&
					!seen.has(cp.value)
				) {
					seen.add(cp.value);
					collected.push({value: cp.value, label: cp.children});
				}
				if (cp.children) walk(cp.children);
			});
		};
		walk(children);
		return collected.length ? (collected as unknown as Items) : undefined;
	}, [children, items]);

	return (
		<SelectPrimitiveRoot items={derivedItems} {...props}>
			{children}
		</SelectPrimitiveRoot>
	);
}

function SelectGroup({className, ...props}: SelectPrimitive.Group.Props) {
	return (
		<SelectPrimitive.Group
			data-slot="select-group"
			className={cn('scroll-my-1 p-1', className)}
			{...props}
		/>
	);
}

function SelectValue({className, ...props}: SelectPrimitive.Value.Props) {
	return (
		<SelectPrimitive.Value
			data-slot="select-value"
			className={cn('flex flex-1 text-left', className)}
			{...props}
		/>
	);
}

function SelectTrigger({
	className,
	size = 'default',
	children,
	...props
}: SelectPrimitive.Trigger.Props & {
	size?: 'sm' | 'default' | 'lg';
}) {
	return (
		<SelectPrimitive.Trigger
			data-slot="select-trigger"
			data-size={size}
			{...props}
			render={
				<button
					className={cn(
						"flex h-10 w-full items-center justify-between gap-2 overflow-hidden rounded-md border border-input bg-card px-3 py-2 text-sm whitespace-nowrap shadow-[0_1px_1px_rgba(0,0,0,0.02)] transition-colors duration-150 outline-none select-none focus-visible:border-foreground/40 focus-visible:shadow-none focus-visible:ring-2 focus-visible:ring-ring/30 disabled:cursor-not-allowed disabled:opacity-50 aria-invalid:border-destructive aria-invalid:ring-3 aria-invalid:ring-destructive/20 data-placeholder:text-muted-foreground data-[size=lg]:h-12 data-[size=sm]:h-8 data-[size=sm]:rounded-md *:data-[slot=select-value]:line-clamp-1 *:data-[slot=select-value]:flex *:data-[slot=select-value]:items-center *:data-[slot=select-value]:gap-1.5 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4",
						className,
					)}>
					{children}
					<SelectPrimitive.Icon
						render={
							<ChevronDownIcon className="pointer-events-none size-4 text-muted-foreground opacity-70" />
						}
					/>
				</button>
			}
		/>
	);
}

function SelectContent({
	className,
	children,
	side = 'bottom',
	sideOffset = 4,
	align = 'start',
	alignOffset = 0,
	alignItemWithTrigger = false,
	...props
}: SelectPrimitive.Popup.Props &
	Pick<
		SelectPrimitive.Positioner.Props,
		| 'align'
		| 'alignOffset'
		| 'side'
		| 'sideOffset'
		| 'alignItemWithTrigger'
	>) {
	return (
		<SelectPrimitive.Portal>
			<SelectPrimitive.Positioner
				side={side}
				sideOffset={sideOffset}
				align={align}
				alignOffset={alignOffset}
				alignItemWithTrigger={alignItemWithTrigger}
				className="isolate z-[1000002]">
				<SelectPrimitive.Popup
					data-slot="select-content"
					data-align-trigger={alignItemWithTrigger}
					className={cn(
						'relative isolate z-50 max-h-[var(--available-height)] w-[var(--anchor-width)] min-w-[165px] origin-[var(--transform-origin)] overflow-x-hidden overflow-y-auto rounded-lg border border-border bg-popover p-1 text-popover-foreground shadow-[0_1px_1px_rgba(0,0,0,0.02),0_8px_16px_-4px_rgba(0,0,0,0.06),0_24px_32px_-8px_rgba(0,0,0,0.08)] duration-100 data-[align-trigger=true]:animate-none data-[side=bottom]:slide-in-from-top-2 data-[side=inline-end]:slide-in-from-left-2 data-[side=inline-start]:slide-in-from-right-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 data-open:animate-in data-open:fade-in-0 data-open:zoom-in-95 data-closed:animate-out data-closed:fade-out-0 data-closed:zoom-out-95',
						className,
					)}
					{...props}>
					<SelectScrollUpButton />
					<SelectPrimitive.List>{children}</SelectPrimitive.List>
					<SelectScrollDownButton />
				</SelectPrimitive.Popup>
			</SelectPrimitive.Positioner>
		</SelectPrimitive.Portal>
	);
}

function SelectLabel({
	className,
	...props
}: SelectPrimitive.GroupLabel.Props) {
	return (
		<SelectPrimitive.GroupLabel
			data-slot="select-label"
			className={cn(
				'px-1.5 py-1 text-xs text-muted-foreground',
				className,
			)}
			{...props}
		/>
	);
}

function SelectItem({
	className,
	children,
	...props
}: SelectPrimitive.Item.Props) {
	return (
		<SelectPrimitive.Item
			data-slot="select-item"
			className={cn(
				"relative flex w-full cursor-pointer items-center gap-2 rounded-lg py-1.5 pr-8 pl-2.5 text-xs text-foreground outline-none select-none hover:text-foreground focus:bg-accent/70 focus:text-accent-foreground data-[state=checked]:font-semibold data-[state=checked]:text-primary data-disabled:pointer-events-none data-disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-3.5",
				className,
			)}
			{...props}>
			<SelectPrimitive.ItemText className="flex flex-1 shrink-0 gap-2 whitespace-nowrap">
				{children}
			</SelectPrimitive.ItemText>
			<SelectPrimitive.ItemIndicator
				render={
					<span className="pointer-events-none absolute right-2 flex size-4 items-center justify-center" />
				}>
				<CheckIcon className="pointer-events-none" />
			</SelectPrimitive.ItemIndicator>
		</SelectPrimitive.Item>
	);
}

function SelectSeparator({
	className,
	...props
}: SelectPrimitive.Separator.Props) {
	return (
		<SelectPrimitive.Separator
			data-slot="select-separator"
			className={cn(
				'pointer-events-none -mx-1 my-1 h-px bg-border',
				className,
			)}
			{...props}
		/>
	);
}

function SelectScrollUpButton({
	className,
	...props
}: React.ComponentProps<typeof SelectPrimitive.ScrollUpArrow>) {
	return (
		<SelectPrimitive.ScrollUpArrow
			data-slot="select-scroll-up-button"
			className={cn(
				"top-0 z-10 flex w-full cursor-default items-center justify-center bg-popover py-1 [&_svg:not([class*='size-'])]:size-4",
				className,
			)}
			{...props}>
			<ChevronUpIcon />
		</SelectPrimitive.ScrollUpArrow>
	);
}

function SelectScrollDownButton({
	className,
	...props
}: React.ComponentProps<typeof SelectPrimitive.ScrollDownArrow>) {
	return (
		<SelectPrimitive.ScrollDownArrow
			data-slot="select-scroll-down-button"
			className={cn(
				"bottom-0 z-10 flex w-full cursor-default items-center justify-center bg-popover py-1 [&_svg:not([class*='size-'])]:size-4",
				className,
			)}
			{...props}>
			<ChevronDownIcon />
		</SelectPrimitive.ScrollDownArrow>
	);
}

export {
	Select,
	SelectContent,
	SelectGroup,
	SelectItem,
	SelectLabel,
	SelectScrollDownButton,
	SelectScrollUpButton,
	SelectSeparator,
	SelectTrigger,
	SelectValue,
};
