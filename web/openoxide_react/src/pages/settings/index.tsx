import {
	createFileRoute,
	useNavigate,
	useSearch,
} from '@tanstack/react-router';
import {Settings as SettingsIcon} from 'lucide-react';
import {usePermissions} from '#/hooks/use-permissions';
import {useAuthSession} from '#/stores/session-context';
import {Titlebar} from '#/components/shared/titlebar';
import {
	SETTINGS_TABS,
	isSettingsTab,
	type SettingsTabId,
} from './settings-tabs';

/**
 * Settings — Openship's layout: primary content on the left, a sticky
 * tab rail on the right (`lg:grid-cols-[1fr_280px]`), URL-driven via
 * `?tab=` so every section deep-links. Below `lg` the rail becomes a
 * horizontal pill strip above the content.
 *
 * The rail is OpenShip's exact visual language: an identity card (icon
 * tile + title + account email) above a quiet card of rounded pill rows
 * — inline icon, label only, active row a soft foreground wash. No
 * dividers, no boxed icons, no per-row subtitles.
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
	const navigate = useNavigate();
	const {can} = usePermissions();
	const {user} = useAuthSession();

	const visibleTabs = SETTINGS_TABS.filter(
		t => !t.permission || can(t.permission),
	);
	const active = visibleTabs.find(t => t.id === tab) ?? visibleTabs[0];
	const Active = active.component;

	const select = (id: SettingsTabId) =>
		void navigate({
			to: '/settings',
			search: {tab: id === visibleTabs[0].id ? undefined : id},
			replace: true,
		});

	return (
		<div className="mx-auto flex w-full max-w-[1600px] animate-in flex-col gap-6 px-4 py-6 duration-200 fade-in sm:px-6 lg:px-8">			<Titlebar
				breadcrumbs={[{label: 'Settings'}]}
				separator>
				{/* Mobile: pill strip (inside the header, above the hairline) */}
				<div className="overflow-x-auto lg:hidden">
					<div className="inline-flex items-center gap-1 rounded-xl bg-card p-1">
						{visibleTabs.map(t => (
							<button
								key={t.id}
								type="button"
								onClick={() => select(t.id)}
								className={`inline-flex items-center gap-2 rounded-lg px-3 py-2 text-sm font-medium whitespace-nowrap transition-colors ${
									t.id === active.id
										? 'bg-foreground/[0.07] text-foreground'
										: 'text-muted-foreground hover:bg-foreground/[0.04] hover:text-foreground'
									}`}>
								<t.icon className="size-[15px]" strokeWidth={1.7} />
								{t.label}
							</button>
						))}
					</div>
				</div>
			</Titlebar>

			{/* Section title — centered below the hairline, plain text, no icon */}
			<div className="flex flex-col items-center gap-1 text-center">
				<h1 className="text-xl font-semibold tracking-[-0.02em] text-foreground">
					Settings
				</h1>
				<p className="max-w-md text-[13px] text-muted-foreground">
					Account, team, and platform preferences
				</p>
			</div>

			<div className="grid min-w-0 grid-cols-1 gap-6 lg:grid-cols-[1fr_280px]">
				{/* Active tab content (left, primary) */}
				<div className="min-w-0">
					<Active />
				</div>

				{/* Rail (right, sticky): identity card + pill nav card */}
				<aside className="hidden space-y-3 lg:sticky lg:top-6 lg:block lg:self-start">
					<div className="rounded-xl bg-card p-4">
						<div className="flex items-center gap-3">
							<div className="flex size-9 shrink-0 items-center justify-center rounded-xl bg-muted">
								<SettingsIcon
									className="size-4 text-foreground"
									strokeWidth={1.7}
								/>
							</div>
							<div className="min-w-0 flex-1">
								<p className="truncate text-sm font-medium text-foreground">
									Settings
								</p>
								{user?.email && (
									<p className="truncate text-xs text-muted-foreground">
										{user.email}
									</p>
								)}
							</div>
						</div>
					</div>

					<div className="rounded-xl bg-card p-3">
						<div className="space-y-1">
							{visibleTabs.map(t => {
								const isActive = t.id === active.id;
								return (
									<button
										key={t.id}
										type="button"
										onClick={() => select(t.id)}
										className={`flex w-full items-center gap-3 rounded-xl px-3 py-2.5 text-[14px] font-medium transition-colors ${
											isActive
												? 'bg-foreground/[0.07] text-foreground'
												: 'text-muted-foreground hover:bg-foreground/[0.04] hover:text-foreground'
										}`}>
										<t.icon
											className="size-[17px] shrink-0"
											strokeWidth={1.7}
										/>
										{t.label}
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
