import type {ComponentType} from 'react';
import {cn} from '#/api/utils';

export type UnderlineTabDef<K extends string> = {
	key: K;
	label: string;
	icon: ComponentType<{className?: string}>;
	count?: number;
};

/**
 * OpenShip's shared tab bar (servers/[serverId] page): a quiet border-b
 * strip of icon+label rows; the active row gets a foreground text tone and
 * a primary underline inset by the row's own padding so it is exactly as
 * wide as its label. Overflow-x on narrow screens.
 */
export function UnderlineTabs<K extends string>({
	tabs,
	value,
	onChange,
	className,
}: {
	tabs: UnderlineTabDef<K>[];
	value: K;
	onChange: (key: K) => void;
	className?: string;
}) {
	return (
		<div
			className={cn(
				'flex items-center gap-1 overflow-x-auto border-b border-border/50',
				className,
			)}>
			{tabs.map(({key, label, icon: Icon, count}) => {
				const active = key === value;
				return (
					<button
						key={key}
						type="button"
						onClick={() => onChange(key)}
						className={cn(
							'relative inline-flex shrink-0 cursor-pointer items-center gap-2 px-4 py-2.5 text-sm font-medium whitespace-nowrap transition-colors',
							active
								? 'text-foreground'
								: 'text-muted-foreground hover:text-foreground/70',
						)}>
						<Icon className="size-4" />
						{label}
						{count !== undefined && (
							<span
								className={cn(
									'rounded-full px-1.5 py-0.5 text-[10px] font-medium tabular-nums',
									active
										? 'bg-muted text-foreground'
										: 'bg-muted/60 text-muted-foreground',
								)}>
								{count}
							</span>
						)}
						{active && (
							<span className="absolute inset-x-4 bottom-0 h-0.5 rounded-full bg-primary" />
						)}
					</button>
				);
			})}
		</div>
	);
}
