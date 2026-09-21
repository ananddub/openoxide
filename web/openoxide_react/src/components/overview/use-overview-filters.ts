import {useMemo} from 'react';

type BackupFilters = {
	search: string;
	type: string;
	sort: string;
};

type DomainFilters = {
	search: string;
	ssl: string;
	sort: string;
};

/** Memoized search/type/sort filtering for the overview backups, domains
 *  and deployment-history tables. */
export function useOverviewFilters(
	rawBackups: any[] | undefined,
	rawDomains: any[] | undefined,
	rawDeployments: any[] | undefined,
	backups: BackupFilters,
	domains: DomainFilters,
	deploySearch: string,
	deployStatus: string,
) {
	const filteredBackups = useMemo(() => {
		let list = (rawBackups || []).filter((b: any) => {
			const matchesSearch =
				!backups.search ||
				(b.name || '')
					.toLowerCase()
					.includes(backups.search.toLowerCase()) ||
				(b.destination || '')
					.toLowerCase()
					.includes(backups.search.toLowerCase());

			const matchesType =
				backups.type === 'all' ||
				(backups.type === 'volume' &&
					(b.backup_type || '').toLowerCase().includes('volume')) ||
				(backups.type === 'database' &&
					(b.backup_type || '').toLowerCase().includes('database'));

			return matchesSearch && matchesType;
		});

		if (backups.sort === 'newest')
			list.sort(
				(a, b) => Number(b.created_at || 0) - Number(a.created_at || 0),
			);
		else if (backups.sort === 'oldest')
			list.sort(
				(a, b) => Number(a.created_at || 0) - Number(b.created_at || 0),
			);

		return list;
	}, [rawBackups, backups.search, backups.type, backups.sort]);

	const filteredDomains = useMemo(() => {
		let list = (rawDomains || []).filter((d: any) => {
			const hostStr = (d.domain || d.host || '').toLowerCase();
			const serviceStr = (d.service_name || '').toLowerCase();
			const projectStr = (d.project_name || '').toLowerCase();

			const matchesSearch =
				!domains.search ||
				hostStr.includes(domains.search.toLowerCase()) ||
				serviceStr.includes(domains.search.toLowerCase()) ||
				projectStr.includes(domains.search.toLowerCase());

			const matchesSsl =
				domains.ssl === 'all' ||
				(domains.ssl === 'https' && d.https) ||
				(domains.ssl === 'http' && !d.https);

			return matchesSearch && matchesSsl;
		});

		if (domains.sort === 'newest')
			list.sort((a, b) => Number(b.id || 0) - Number(a.id || 0));
		else if (domains.sort === 'oldest')
			list.sort((a, b) => Number(a.id || 0) - Number(b.id || 0));

		return list;
	}, [rawDomains, domains.search, domains.ssl, domains.sort]);

	const filteredDeploymentsHistory = useMemo(() => {
		return (rawDeployments || []).filter((d: any) => {
			const titleStr = (d.title || d.description || '').toLowerCase();
			const matchesSearch =
				!deploySearch || titleStr.includes(deploySearch.toLowerCase());

			return matchesSearch && matchesStatus(d, deployStatus);
		});
	}, [rawDeployments, deploySearch, deployStatus]);

	return {filteredBackups, filteredDomains, filteredDeploymentsHistory};
}

function matchesStatus(d: any, filter: string) {
	const statusStr = (d.status || '').toLowerCase();
	return (
		filter === 'all' ||
		(filter === 'success' &&
			(statusStr === 'done' ||
				statusStr === 'healthy' ||
				statusStr === 'success')) ||
		(filter === 'failed' &&
			(statusStr === 'error' || statusStr === 'failed')) ||
		(filter === 'running' &&
			(statusStr === 'running' || statusStr === 'queued'))
	);
}
