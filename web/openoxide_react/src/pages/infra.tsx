import {createFileRoute, useSearch} from '@tanstack/react-router';
import type {ComponentType} from 'react';
import {Titlebar} from '#/components/shared/titlebar';
import {SectionHeader} from '#/components/settings/list-card';
import {WebServerSettingsPage} from './settings/server';
import {TraefikPanel} from './traefik';
import {SwarmPanel} from './swarm';
import {RequestsPanel} from './requests';

/**
 * Web Server / Traefik / Swarm / Requests — settings grammar: no in-page
 * tab strip. The section switcher is the sidebar rail (`InfraContextNav`
 * renders these rows on this route), the top bar carries the crumb, and
 * the page paints nothing but the active section's content. The tab is
 * URL driven (`?tab=`) — the same param the rail writes — so deep links,
 * refresh and back/forward all keep the active section.
 *
 * Web Server/Swarm are scrollable content; Traefik/Requests are
 * fixed-height workbenches (file tree + editor / live table), so the
 * content area switches between "page scroll" and "fill remaining
 * height" per tab.
 */

type InfraTabId = 'web-server' | 'traefik' | 'swarm' | 'requests';

type InfraTab = {
	id: InfraTabId;
	label: string;
	component: ComponentType;
	/** Section header title + description shown above the panel. */
	title: string;
	description?: string;
	/** Workbench tabs fill the viewport instead of scrolling the page. */
	fixedHeight?: boolean;
	/** Panel paints its own `SectionHeader` because its primary action needs panel state. */
	ownsHeader?: boolean;
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
		component: WebServerSettingsPage,
		title: 'Web Server',
		description:
			'Reverse proxy, TLS certificates, and host maintenance for this panel.',
	},
	{
		id: 'traefik',
		label: 'Traefik',
		component: TraefikPanel,
		title: 'Traefik Configuration',
		description:
			'Edit the live static and dynamic config files Traefik runs with.',
		fixedHeight: true,
	},
	{
		id: 'swarm',
		label: 'Swarm',
		component: SwarmPanel,
		title: 'Docker Swarm',
		ownsHeader: true,
	},
	{
		id: 'requests',
		label: 'Requests',
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
	const active = INFRA_TABS.find(t => t.id === tab) ?? INFRA_TABS[0];
	const Active = active.component;

	const crumbs =
		active.id === 'web-server'
			? [{label: active.title}]
			: [{label: 'Web Server', to: '/infra'}, {label: active.title}];

	if (active.fixedHeight) {
		return (
			<div className="flex h-full min-h-0 w-full flex-col">
				<div className="mx-auto flex w-full max-w-[1920px] shrink-0 flex-col px-4 pt-4">
					<Titlebar breadcrumbs={crumbs} />
					<SectionHeader
						title={active.title}
						description={active.description}
					/>
				</div>
				<div className="mx-auto flex min-h-0 w-full max-w-[1920px] flex-1 flex-col px-4 py-4">
					<Active />
				</div>
			</div>
		);
	}

	return (
		<div className="mx-auto flex w-full max-w-[1920px] animate-in flex-col gap-6 px-4 pt-4 pb-6 duration-200 fade-in">
			<Titlebar breadcrumbs={crumbs} />
			{!active.ownsHeader && (
				<SectionHeader
					title={active.title}
					description={active.description}
				/>
			)}
			<div className="min-w-0">
				<Active />
			</div>
		</div>
	);
}
