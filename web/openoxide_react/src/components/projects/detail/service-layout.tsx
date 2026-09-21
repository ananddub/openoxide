import type {ReactNode} from 'react';
import {
	Archive,
	Boxes,
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
import {Titlebar} from '#/components/shared/titlebar';
import {UnderlineTabs} from '#/components/shared/underline-tabs';

/**
 * Service detail shell — the Vercel project-page header: crumb trail on
 * top (`Projects / Project Details / <name>`), icon tile + title LEFT and
 * status actions RIGHT on the same row, then the service tabs LEFT on the
 * header's closing hairline (UnderlineTabs' own border-b IS the separator,
 * bleeding edge-to-edge). Content spans the full width below.
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
	/** Status chip / refresh / delete — right side of the title row. */
	actions?: ReactNode;
	tabs: readonly string[];
	activeTab: string;
	onTabChange: (tab: string) => void;
	children: ReactNode;
}) {
	return (
		<div className="mx-auto flex w-full max-w-[1600px] animate-in flex-col gap-6 px-4 py-6 duration-200 fade-in sm:px-6 lg:px-8">
			<Titlebar
				breadcrumbs={[
					{label: 'Projects', to: '/projects', icon: FolderOpen},
					{label: 'Project Details', to: '/projects/$id', params: {id}},
					{label: name},
				]}
				icon={icon}
				title={name}
				description={subtitle}
				actions={actions}
				separator={false}>
				{/* Tabs LEFT on the line; their border-b is the full-bleed
				    separator (Vercel project header / Geist Tabs). */}
				<div className="-mx-4 sm:-mx-6 lg:-mx-8">
					<UnderlineTabs
						tabs={tabs.map(tab => ({
							key: tab,
							label: tab,
							icon: SERVICE_TAB_ICONS[tab] ?? Boxes,
						}))}
						value={activeTab}
						onChange={onTabChange}
					/>
				</div>
			</Titlebar>

			{/* Full-width content */}
			<div className="min-w-0">{children}</div>
		</div>
	);
}
