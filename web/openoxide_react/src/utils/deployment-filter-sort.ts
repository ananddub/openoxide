import type {
	Deployment,
	SortKey,
} from '#/hooks/deployments/use-deployments';

export type DeploymentFilterState = {
	searchQuery: string;
	statusFilter: string;
	typeFilter: string;
	sortBy: SortKey;
	sortDir: 'asc' | 'desc';
};

function matchesSearch(d: Deployment, q: string): boolean {
	if (!q) return true;
	return (
		d.title.toLowerCase().includes(q) ||
		(d.description || '').toLowerCase().includes(q) ||
		(d.error_message || '').toLowerCase().includes(q)
	);
}

function matchesStatus(d: Deployment, filter: string): boolean {
	if (filter === 'all') return true;
	const status = d.status.toUpperCase();
	return (
		(filter === 'running' &&
			(status === 'RUNNING' || status === 'CANCELLING')) ||
		(filter === 'queued' && status === 'QUEUED') ||
		(filter === 'done' && status === 'DONE') ||
		(filter === 'error' && status === 'ERROR') ||
		(filter === 'cancelled' &&
			(status === 'CANCELLED' || status === 'CANCELLING'))
	);
}

function matchesType(d: Deployment, filter: string): boolean {
	if (filter === 'all') return true;
	return (
		(filter === 'application' && d.application_id != null) ||
		(filter === 'compose' && d.compose_id != null) ||
		(filter === 'database' && d.database_id != null)
	);
}

function compare(a: Deployment, b: Deployment, key: SortKey): number {
	if (key === 'title') return a.title.localeCompare(b.title);
	if (key === 'status') return a.status.localeCompare(b.status);
	return Number(a.created_at || 0) - Number(b.created_at || 0);
}

/** Apply the search/status/type filters, then the chosen sort. */
export function filterAndSortDeployments(
	deployments: Deployment[],
	f: DeploymentFilterState,
): Deployment[] {
	const q = f.searchQuery.toLowerCase();
	const filtered = deployments.filter(
		d =>
			matchesSearch(d, q) &&
			matchesStatus(d, f.statusFilter) &&
			matchesType(d, f.typeFilter),
	);
	const dir = f.sortDir === 'desc' ? -1 : 1;
	return [...filtered].sort((a, b) => dir * compare(a, b, f.sortBy));
}
