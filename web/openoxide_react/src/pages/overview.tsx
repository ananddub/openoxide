import {createFileRoute} from '@tanstack/react-router';
import {useState} from 'react';
import {Box, Globe, Clock, Rocket, House} from 'lucide-react';
import {Titlebar} from '#/components/shared/titlebar';
import {
	Tabs,
	TabsList,
	TabsTrigger,
	TabsContent,
} from '#/components/ui/tabs';
import {Badge} from '#/components/ui/badge';
import {useOrganizations} from '#/stores/session-context';
import {
	useDomainItems,
	useBackupItems,
	useDeploymentRunning,
} from '#/api/live';
import {useDeployments} from '#/hooks/deployments/use-deployments';
import {DeploymentLogsDialog} from '#/components/deployments/deployment-logs-dialog';
import {OverviewServicesTab} from '#/components/overview/overview-services-tab';
import {OverviewBackupsTab} from '#/components/overview/overview-backups-tab';
import {OverviewDomainsTab} from '#/components/overview/overview-domains-tab';
import {OverviewDeploymentsTab} from '#/components/overview/overview-deployments-tab';
import {useOverviewFilters} from '#/components/overview/use-overview-filters';

export const Route = createFileRoute('/_app/overview')({
	component: OverviewPage,
});

function OverviewPage() {
	const [activeTab, setActiveTab] = useState('services');
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
		selectedDeployment,
		setSelectedDeployment,
		logs,
		copied,
		handleCopyLogs,
		handleCancelDeployment,
		handleDeleteDeployment,
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
		<div className="mx-auto flex w-full max-w-[1600px] animate-in flex-col gap-6 px-4 py-6 duration-200 fade-in sm:px-6 lg:px-8">
			{/* Vercel-style titlebar */}
			<Titlebar
				breadcrumbs={[{label: 'Overview', icon: House}]}
				title="Overview"
				description="Centralized platform overview of all applications, compose stacks, databases, volume backups, domains, and deployments"
			/>

			{/* Main Overview Tabs (Services -> Backups -> Domains -> Deployments) */}
			<Tabs
				value={activeTab}
				onValueChange={setActiveTab}
				className="w-full space-y-6">
				<TabsList
					variant="line"
					className="w-full justify-start gap-6 rounded-none border-b  pb-0">
					<TabsTrigger
						value="services"
						className="flex items-center gap-2 pb-2.5 text-xs font-semibold">
						<Box className="size-4" />
						Services
					</TabsTrigger>
					<TabsTrigger
						value="backups"
						className="flex items-center gap-2 pb-2.5 text-xs font-semibold">
						<Clock className="size-4" />
						Backups
						<Badge
							variant="secondary"
							className="px-1.5 py-0 font-mono text-[10px]">
							{filteredBackups.length}
						</Badge>
					</TabsTrigger>
					<TabsTrigger
						value="domains"
						className="flex items-center gap-2 pb-2.5 text-xs font-semibold">
						<Globe className="size-4" />
						Domains
						<Badge
							variant="secondary"
							className="px-1.5 py-0 font-mono text-[10px]">
							{filteredDomains.length}
						</Badge>
					</TabsTrigger>
					<TabsTrigger
						value="deployments"
						className="flex items-center gap-2 pb-2.5 text-xs font-semibold">
						<Rocket className="size-4" />
						Deployments
						<Badge
							variant="secondary"
							className="px-1.5 py-0 font-mono text-[10px]">
							{filteredDeploymentsHistory.length}
						</Badge>
					</TabsTrigger>
				</TabsList>

				<TabsContent value="services">
					<OverviewServicesTab />
				</TabsContent>

				<TabsContent value="backups">
					<OverviewBackupsTab
						items={filteredBackups}
						search={backupSearch}
						onSearch={setBackupSearch}
						type={backupTypeFilter}
						onType={setBackupTypeFilter}
						sort={backupSort}
						onSort={setBackupSort}
					/>
				</TabsContent>

				<TabsContent value="domains">
					<OverviewDomainsTab
						items={filteredDomains}
						search={domainSearch}
						onSearch={setDomainSearch}
						ssl={domainSslFilter}
						onSsl={setDomainSslFilter}
						sort={domainSort}
						onSort={setDomainSort}
					/>
				</TabsContent>

				<TabsContent value="deployments">
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
						onSelect={setSelectedDeployment}
						onCancel={handleCancelDeployment}
						onDelete={handleDeleteDeployment}
					/>
				</TabsContent>
			</Tabs>

			{/* Logs Dialog */}
			<DeploymentLogsDialog
				selectedDeployment={selectedDeployment}
				onClose={() => setSelectedDeployment(null)}
				logs={logs}
				isLogsLoading={false}
				copied={copied}
				onCopyLogs={handleCopyLogs}
			/>
		</div>
	);
}
