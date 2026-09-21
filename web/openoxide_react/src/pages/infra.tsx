import {
	createFileRoute,
	useNavigate,
	useSearch,
} from '@tanstack/react-router';
import type {ComponentType} from 'react';
import {
	Activity,
	Forward,
	GalleryVerticalEnd,
	PieChart,
} from 'lucide-react';
import {UnderlineTabs} from '#/components/shared/underline-tabs';
import {Titlebar} from '#/components/shared/titlebar';
import {WebServerSettingsPage} from './settings/server';
import {TraefikPanel} from './traefik';
import {SwarmPanel} from './swarm';
import {RequestsPanel} from './requests';

/**
 * Web Server / Traefik / Swarm / Requests — one page, OpenShip-style
 * underline tabs (the same bar the server detail uses). The tab is URL
 * driven (`?tab=`) so every section deep-links; the sidebar carries a
 * single "Web Server" entry instead of four.
 *
 * The first two panels are scrollable content; Traefik/Requests are
 * fixed-height workbenches (file tree + editor / live table), so the
 * content area switches between "page scroll" and "fill remaining
 * height" per tab.
 */

type InfraTabId = 'web-server' | 'traefik' | 'swarm' | 'requests';

type InfraTab = {
	id: InfraTabId;
	label: string;
	icon: ComponentType<{className?: string}>;
	component: ComponentType;
	/** Page header shown above the panel; swaps with the tab. */
	title: string;
	description: string;
	/** Workbench tabs fill the viewport instead of scrolling the page. */
	fixedHeight?: boolean;
};

export const Route = createFileRoute('/_app/infra')({
	component: InfraPage,
	validateSearch: (
		search: Record<string, unknown>,
	): {tab?: InfraTabId} => ({
		tab: isInfraTab(search.tab) ? search.tab : undefined,
	}),
});

const INFRA_TABS: InfraTab[] = [
	{
		id: 'web-server',
		label: 'Web Server',
		icon: Activity,
		component: WebServerSettingsPage,
		title: 'Web Server',
		description:
			'Reverse proxy, TLS certificates, and host maintenance for this panel.',
	},
	{
		id: 'traefik',
		label: 'Traefik',
		icon: GalleryVerticalEnd,
		component: TraefikPanel,
		title: 'Traefik Configuration',
		description:
			'Edit the live static and dynamic config files Traefik runs with.',
		fixedHeight: true,
	},
	{
		id: 'swarm',
		label: 'Swarm',
		icon: PieChart,
		component: SwarmPanel,
		title: 'Docker Swarm',
		description:
			'Cluster status, nodes, and join tokens across your engines.',
	},
	{
		id: 'requests',
		label: 'Requests',
		icon: Forward,
		component: RequestsPanel,
		title: 'Request Log',
		description: 'Live Traefik access log for routed traffic.',
		fixedHeight: true,
	},
];

function isInfraTab(value: unknown): value is InfraTabId {
	return (
		typeof value === 'string' && INFRA_TABS.some(tab => tab.id === value)
	);
}

function InfraPage() {
	const {tab} = useSearch({from: '/_app/infra'});
	const navigate = useNavigate();
	const active = INFRA_TABS.find(t => t.id === tab) ?? INFRA_TABS[0];
	const Active = active.component;

	const select = (id: InfraTabId) =>
		void navigate({
			to: '/infra',
			search: {tab: id === INFRA_TABS[0].id ? undefined : id},
			replace: true,
		});
	return (
		<div className="flex h-full min-h-0 w-full flex-col">
			{/* Titlebar (title swaps with tab) + tab strip — fixed; content scrolls below */}
			<div className="mx-auto flex w-full max-w-[1600px] shrink-0 flex-col gap-4 px-4 pt-6 sm:px-6 lg:px-8">
				<Titlebar
					breadcrumbs={[{label: active.title}]}
					title={active.title}
					description={active.description}
					separator={false}>
					{/* UnderlineTabs carries its own full-bleed border-b as the
					    separator; the active underline merges with it. */}
					<div className="-mx-4 sm:-mx-6 lg:-mx-8">
						<UnderlineTabs
							tabs={INFRA_TABS.map(({id, label, icon}) => ({
								key: id,
								label,
								icon,
							}))}
							value={active.id}
							onChange={select}
						/>
					</div>
				</Titlebar>
			</div>

			{active.fixedHeight ? (
				<div className="mx-auto flex min-h-0 w-full max-w-[1600px] flex-1 flex-col px-4 pb-4 sm:px-6 lg:px-8">
					<Active />
				</div>
			) : (
				<div className="min-h-0 flex-1 overflow-y-auto">
					<div className="mx-auto flex w-full max-w-[1600px] animate-in flex-col gap-6 px-4 py-6 duration-200 fade-in sm:px-6 lg:px-8">
						<Active />
					</div>
				</div>
			)}
		</div>
	);
}
