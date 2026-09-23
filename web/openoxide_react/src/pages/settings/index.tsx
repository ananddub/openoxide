import {createFileRoute, useSearch} from '@tanstack/react-router';
import {Settings} from 'lucide-react';
import {usePermissions} from '#/hooks/use-permissions';
import {Titlebar} from '#/components/shared/titlebar';
import {
	SETTINGS_TABS,
	isSettingsTab,
	type SettingsTabId,
} from './settings-tabs';

/**
 * Settings shell — the service-detail grammar: the section name is the
 * top-bar crumb (`Settings / Team`), the section switcher is the sidebar
 * rail (`PageContextNav` renders SETTINGS_TABS on this route), and the
 * page itself paints nothing but full-width content.
 *
 * So no hero title, no in-page headings and no right-side rail — the
 * sidebar already IS that rail. Tabs are URL-driven (`?tab=`) which is
 * the same param the rail writes, so deep links, refresh and back/forward
 * all keep the active section.
 *
 * Resource-management surfaces (SSH Keys, Tags, Registry, Vault, S3,
 * Web Server) stay top-level in the sidebar — they are workspaces, not
 * settings.
 */

export const Route = createFileRoute('/_app/settings')({
	component: SettingsPage,
	validateSearch: (
		search: Record<string, unknown>,
	): {tab?: SettingsTabId} => ({
		tab: isSettingsTab(search.tab) ? search.tab : undefined,
	}),
});

function SettingsPage() {
	const {tab} = useSearch({from: '/_app/settings'});
	const {can} = usePermissions();

	const visibleTabs = SETTINGS_TABS.filter(
		t => !t.permission || can(t.permission),
	);
	const active = visibleTabs.find(t => t.id === tab) ?? visibleTabs[0];
	const Active = active.component;

	return (
		<div className="mx-auto flex w-full max-w-[1920px] animate-in flex-col gap-6 px-4 pt-4 pb-6 duration-200 fade-in">
			<Titlebar
				breadcrumbs={[
					{label: 'Settings', to: '/settings', icon: Settings},
					{label: active.label},
				]}
			/>

			<div className="min-w-0">
				<Active />
			</div>
		</div>
	);
}
