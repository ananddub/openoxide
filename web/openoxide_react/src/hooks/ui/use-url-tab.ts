import {useCallback} from 'react';
import {useLocation, useNavigate} from '@tanstack/react-router';

/**
 * Tab state that lives in the URL (`?tab=Architecture`) instead of a
 * local useState. The sidebar's project-context nav drives the same
 * param, so rail and page content stay in sync — and deep links,
 * refresh, and back/forward all preserve the active tab.
 *
 * `replace` keeps the history clean: switching tabs is not navigation.
 * Other search params (e.g. the database page's `kind`) are preserved.
 */
export function useUrlTab(fallback = 'General') {
	const location = useLocation();
	const navigate = useNavigate();

	const search = (location.search ?? {}) as Record<string, unknown>;
	const activeTab =
		typeof search.tab === 'string' && search.tab ? search.tab : fallback;

	const setActiveTab = useCallback(
		(tab: string) => {
			void navigate({
				to: '.',
				search: (prev: Record<string, unknown>) => ({...prev, tab}),
				replace: true,
			});
		},
		[navigate],
	);

	return [activeTab, setActiveTab] as const;
}
