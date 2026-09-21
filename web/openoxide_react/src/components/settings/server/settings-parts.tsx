import type {ComponentType, ReactNode} from 'react';
import {Label} from '#/components/ui/label';
import {Switch} from '#/components/ui/switch';

export function SectionLabel({
	icon: Icon,
	children,
}: {
	icon: ComponentType<{className?: string}>;
	children: ReactNode;
}) {
	return (
		<p className="flex items-center gap-2 text-[11px] font-semibold tracking-wider text-muted-foreground uppercase">
			<Icon className="size-3.5" />
			{children}
		</p>
	);
}

export function Field(props: {
	label: string;
	hint?: string;
	children: ReactNode;
}) {
	return (
		<div className="flex flex-col gap-1.5">
			<Label className="text-xs font-semibold">{props.label}</Label>
			{props.children}
			{props.hint && (
				<p className="text-[10px] text-muted-foreground">{props.hint}</p>
			)}
		</div>
	);
}

/** Toggle row with title/description on the left and a render-prop switch. */
export function ToggleRow(props: {
	title: string;
	description: string;
	checked: boolean;
	onChecked: (v: boolean) => void;
}) {
	return (
		<div className="flex items-center justify-between gap-3 rounded-xl bg-muted/20 px-4 py-3 ring-1 ring-border/40">
			<div className="min-w-0">
				<p className="text-[13px] font-medium text-foreground">
					{props.title}
				</p>
				<p className="mt-0.5 text-xs text-muted-foreground">
					{props.description}
				</p>
			</div>
			<Switch checked={props.checked} onCheckedChange={props.onChecked} />
		</div>
	);
}
