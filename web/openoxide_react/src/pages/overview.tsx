import {createFileRoute} from '@tanstack/react-router';
import {useState} from 'react';
import {Box, Globe, Clock, Rocket, LayoutDashboard} from 'lucide-react';
import {Titlebar} from '#/components/shared/titlebar';
import {GeistSwitch} from '#/components/shared/geist-switch';
import {useOrganizations} from '#/stores/session-context';
import {
	useDomainItems,
	useBackupItems,
	useDeploymentRunning,
} from '#/api/live';
import {useDeployments} from '#/hooks/deployments/use-deployments';
import {OverviewServicesTab} from '#/components/overview/overview-services-tab';
import {OverviewBackupsTab} from '#/components/overview/overview-backups-tab';
import {OverviewDomainsTab} from '#/components/overview/overview-domains-tab';
import {OverviewDeploymentsTab} from '#/components/overview/overview-deployments-tab';
import {useOverviewFilters} from '#/components/overview/use-overview-filters';

export const Route = createFileRoute('/_app/overview')({
	component: OverviewPage,
});

function OverviewPage() {
	const [activeTab, setActiveTab] = useState<
		'services' | 'backups' | 'domains' | 'deployments'
	>('services');
	const [deploymentSubTab, setDeploymentSubTab] = useState<
		'history' | 'queue'
	>('history');

	// Backups Toolbar State
	const [backupSearch, setBackupSearch] = useState('');
	const [backupTypeFilter, setBackupTypeFilter] = useState('all');
	const [backupSort, setBackupSort] = useState('newest');

	// Domains Toolbar State
	const [domainSearch, setDomainSearch] = useState('');
	const [domainSslFilter, setDomainSslFilter] = useState('all');
	const [domainSort, setDomainSort] = useState('newest');

	// Deployments Toolbar State
	const [deploySearch, setDeploySearch] = useState('');
	const [deployStatusFilter, setDeployStatusFilter] = useState('all');

	const {activeOrg} = useOrganizations();
	const domainsQuery = useDomainItems(BigInt(activeOrg?.id || 1));
	const backupsQuery = useBackupItems(BigInt(activeOrg?.id || 1));
	// Live running deployments queue
	const {data: runningDeployments = []} = useDeploymentRunning();

	const {
		filteredAndSorted: rawDeploymentsList,
		handleRefresh,
		refreshing,
		handleClearAllDeployments,
	} = useDeployments();

	const {filteredBackups, filteredDomains, filteredDeploymentsHistory} =
		useOverviewFilters(
			backupsQuery.data,
			domainsQuery.data,
			rawDeploymentsList,
			{search: backupSearch, type: backupTypeFilter, sort: backupSort},
			{search: domainSearch, ssl: domainSslFilter, sort: domainSort},
			deploySearch,
			deployStatusFilter,
		);

	return (
		<div className="mx-auto flex w-full max-w-[1920px] animate-in flex-col gap-6 px-4 pt-4 pb-6 duration-200 fade-in">
			{/* Vercel-style titlebar */}
			<Titlebar
				breadcrumbs={[{label: 'Overview', icon: LayoutDashboard}]}
			/>

			{/* Main Overview Tabs (Services -> Backups -> Domains -> Deployments) */}
			<GeistSwitch
				options={[
					{key: 'services', label: 'Services', icon: Box},
					{
						key: 'backups',
						label: 'Backups',
						icon: Clock,
						count: filteredBackups.length,
					},
					{
						key: 'domains',
						label: 'Domains',
						icon: Globe,
						count: filteredDomains.length,
					},
					{
						key: 'deployments',
						label: 'Deployments',
						icon: Rocket,
						count: filteredDeploymentsHistory.length,
					},
				]}
				value={activeTab}
				onChange={setActiveTab}
			/>

			{activeTab === 'services' && <OverviewServicesTab />}

			{activeTab === 'backups' && (
				<OverviewBackupsTab
					items={filteredBackups}
					search={backupSearch}
					onSearch={setBackupSearch}
					type={backupTypeFilter}
					onType={setBackupTypeFilter}
					sort={backupSort}
					onSort={setBackupSort}
				/>
			)}

			{activeTab === 'domains' && (
				<OverviewDomainsTab
					items={filteredDomains}
					search={domainSearch}
					onSearch={setDomainSearch}
					ssl={domainSslFilter}
					onSsl={setDomainSslFilter}
					sort={domainSort}
					onSort={setDomainSort}
				/>
			)}

			{activeTab === 'deployments' && (
				<OverviewDeploymentsTab
					history={filteredDeploymentsHistory}
					queue={runningDeployments}
					subTab={deploymentSubTab}
					onSubTab={setDeploymentSubTab}
					search={deploySearch}
					onSearch={setDeploySearch}
					status={deployStatusFilter}
					onStatus={setDeployStatusFilter}
					onClear={handleClearAllDeployments}
					onRefresh={handleRefresh}
					refreshing={refreshing}
				/>
			)}
		</div>
	);
}
