import {Link} from '@tanstack/react-router';
import type {ComponentType, ReactNode} from 'react';
import {
	Archive,
	Boxes,
	ChevronRight,
	Clock,
	Container,
	FolderCog,
	FolderOpen,
	GitBranch,
	Globe,
	HardDrive,
	Network,
	Rocket,
	ScrollText,
	Settings2,
	SlidersHorizontal,
	Wrench,
	type LucideIcon,
} from 'lucide-react';
import {cn} from '#/api/utils';

/**
 * Service detail shell — the same layout language as Settings: primary
 * content on the left, a sticky tab rail on the right
 * (`lg:grid-cols-[1fr_280px]`). The rail carries an identity card (icon
 * tile, name, status/actions) above a quiet card of rounded pill rows —
 * inline icon, label only, active row a soft foreground wash. Below `lg`
 * the rail becomes a horizontal scroll strip under the breadcrumb.
 */

export const SERVICE_TAB_ICONS: Record<string, LucideIcon> = {
	General: Settings2,
	Architecture: Network,
	Environment: FolderCog,
	Domains: Globe,
	Deployments: Rocket,
	'Preview Deployments': GitBranch,
	Containers: Container,
	Schedules: Clock,
	Backups: Archive,
	'Volume Backups': HardDrive,
	Logs: ScrollText,
	Monitoring: SlidersHorizontal,
	Advanced: Wrench,
};

export function ServiceLayout({
	id,
	name,
	subtitle,
	icon,
	actions,
	tabs,
	activeTab,
	onTabChange,
	children,
}: {
	/** Project id — the breadcrumb's parent link. */
	id: string;
	name: string;
	subtitle?: string;
	/** Identity tile content (provider icon). */
	icon: ReactNode;
	/** Status chip / refresh / delete — under the name in the identity card. */
	actions?: ReactNode;
	tabs: readonly string[];
	activeTab: string;
	onTabChange: (tab: string) => void;
	children: ReactNode;
}) {
	return (
		<div className="mx-auto flex w-full max-w-[1600px] animate-in flex-col gap-6 px-4 py-6 duration-200 fade-in sm:px-6 lg:px-8">
			{/* Breadcrumb */}
			<div className="flex items-center gap-1.5 text-xs font-medium text-muted-foreground">
				<Link
					to="/projects"
					className="flex items-center gap-1 transition-colors hover:text-foreground">
					<FolderOpen className="size-3.5" /> Projects
				</Link>
				<ChevronRight className="size-3 opacity-40" />
				<Link
					to="/projects/$id"
					params={{id}}
					className="transition-colors hover:text-foreground">
					Project Details
				</Link>
				<span className="truncate font-semibold text-foreground">{name}</span>
			</div>

			{/* Mobile: pill strip */}
			<div className="overflow-x-auto lg:hidden">
				<div className="inline-flex items-center gap-1 rounded-xl bg-card p-1">
					{tabs.map(tab => {
						const Icon = SERVICE_TAB_ICONS[tab] ?? Boxes;
						const isActive = activeTab === tab;
						return (
							<button
								key={tab}
								type="button"
								onClick={() => onTabChange(tab)}
								className={cn(
									'inline-flex items-center gap-2 whitespace-nowrap rounded-lg px-3 py-2 text-sm font-medium transition-colors',
									isActive
										? 'bg-foreground/[0.07] text-foreground'
										: 'text-muted-foreground hover:bg-foreground/[0.04] hover:text-foreground',
								)}>
								<Icon className="size-[15px]" strokeWidth={1.7} />
								{tab}
							</button>
						);
					})}
				</div>
			</div>

			<div className="grid min-w-0 grid-cols-1 gap-6 lg:grid-cols-[1fr_280px]">
				{/* Active tab content (left, primary) */}
				<div className="min-w-0">{children}</div>

				{/* Rail (right, sticky) */}
				<aside className="hidden space-y-3 lg:sticky lg:top-6 lg:block lg:self-start">
					{/* Identity card */}
					<div className="rounded-2xl bg-card p-4">
						<div className="flex items-start gap-3">
							<div className="flex size-9 shrink-0 items-center justify-center rounded-xl bg-muted">
								{icon}
							</div>
							<div className="min-w-0 flex-1">
								<p className="truncate text-sm font-medium text-foreground">
									{name}
								</p>
								{subtitle && (
									<p className="truncate font-mono text-xs text-muted-foreground">
										{subtitle}
									</p>
								)}
							</div>
						</div>
						{actions && (
							<div className="mt-3 flex flex-wrap items-center gap-2">
								{actions}
							</div>
						)}
					</div>

					{/* Tab nav card */}
					<div className="rounded-2xl bg-card p-3">
						<div className="space-y-1">
							{tabs.map(tab => {
								const Icon: ComponentType<{
									className?: string;
									strokeWidth?: number;
								}> = SERVICE_TAB_ICONS[tab] ?? Boxes;
								const isActive = activeTab === tab;
								return (
									<button
										key={tab}
										type="button"
										onClick={() => onTabChange(tab)}
										className={cn(
											'flex w-full items-center gap-3 rounded-xl px-3 py-2.5 text-[14px] font-medium transition-colors',
											isActive
												? 'bg-foreground/[0.07] text-foreground'
												: 'text-muted-foreground hover:bg-foreground/[0.04] hover:text-foreground',
										)}>
										<Icon className="size-[17px] shrink-0" strokeWidth={1.7} />
										{tab}
									</button>
								);
							})}
						</div>
					</div>
				</aside>
			</div>
		</div>
	);
}
