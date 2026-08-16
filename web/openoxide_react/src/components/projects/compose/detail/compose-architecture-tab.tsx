import {useMemo, useState, useEffect} from 'react';
import {$api} from '#/api/query';
import {useQueryClient} from '@tanstack/react-query';
import {toast} from 'sonner';
import {ComposeVisualizer, type ComposeService} from '#/components/projects/compose/visualizer/compose-visualizer';
import {ComposeDomainModal} from './domains/compose-domain-modal';
import {CreateScheduleModal} from './schedules/create-schedule-modal';
import {CreateBackupModal} from './backups/create-backup-modal';
import {ComposeDirectContainerLogsModal} from './logs/compose-direct-container-logs-modal';
import {ComposeDirectDeployLogsModal} from './deployments/compose-direct-deploy-logs-modal';
import {TerminalModal} from '#/components/projects/common/terminal-modal';

function buildRawGitUrl(compose: any): string | null {
	if (!compose) return null;
	const repo = compose.repository || compose.custom_git_url || compose.gitlab_repository || compose.gitea_repository || compose.bitbucket_repository;
	if (!repo || typeof repo !== 'string') return null;

	let cleanRepo = repo.trim().replace(/\.git$/, '');
	const branch = compose.branch || compose.custom_git_branch || compose.gitlab_branch || compose.gitea_branch || compose.bitbucket_branch || 'main';
	const rawPath = (compose.compose_path || 'docker-compose.yml').replace(/^\.\//, '');

	if (cleanRepo.includes('github.com')) {
		let pathPart = cleanRepo;
		if (pathPart.startsWith('https://github.com/')) pathPart = pathPart.replace('https://github.com/', '');
		else if (pathPart.startsWith('http://github.com/')) pathPart = pathPart.replace('http://github.com/', '');
		else if (pathPart.startsWith('github.com/')) pathPart = pathPart.replace('github.com/', '');
		const parts = pathPart.split('/').filter(Boolean);
		if (parts.length >= 2) {
			return `https://raw.githubusercontent.com/${parts[0]}/${parts[1]}/${branch}/${rawPath}`;
		}
	} else if (cleanRepo.includes('gitlab.com')) {
		let pathPart = cleanRepo;
		if (pathPart.startsWith('https://gitlab.com/')) pathPart = pathPart.replace('https://gitlab.com/', '');
		else if (pathPart.startsWith('gitlab.com/')) pathPart = pathPart.replace('gitlab.com/', '');
		const parts = pathPart.split('/').filter(Boolean);
		if (parts.length >= 2) {
			return `https://gitlab.com/${parts[0]}/${parts[1]}/-/raw/${branch}/${rawPath}`;
		}
	} else if (cleanRepo.includes('bitbucket.org')) {
		let pathPart = cleanRepo;
		if (pathPart.startsWith('https://bitbucket.org/')) pathPart = pathPart.replace('https://bitbucket.org/', '');
		const parts = pathPart.split('/').filter(Boolean);
		if (parts.length >= 2) {
			return `https://bitbucket.org/${parts[0]}/${parts[1]}/raw/${branch}/${rawPath}`;
		}
	}
	return null;
}

interface ComposeArchitectureTabProps {
	compose: any;
	domains?: any[];
	schedules?: any[];
	backups?: any[];
	onNavigateTab?: (tab: string) => void;
	onRefresh?: () => void;
}

export function ComposeArchitectureTab({
	compose,
	domains: passedDomains,
	schedules: passedSchedules,
	backups: passedBackups,
	onRefresh,
}: ComposeArchitectureTabProps) {
	const queryClient = useQueryClient();
	const composeId = compose?.id;

	// Modals State
	const [activeModal, setActiveModal] = useState<'domain' | 'schedule' | 'backup' | 'terminal' | 'logs' | 'deployLogs' | null>(null);
	const [selectedService, setSelectedService] = useState<ComposeService | null>(null);
	const [fetchedComposeFile, setFetchedComposeFile] = useState<string>('');

	// Fetch raw docker-compose.yml directly over HTTPS from GitHub/GitLab if DB compose_file is empty
	useEffect(() => {
		if (compose?.compose_file && compose.compose_file.trim()) {
			setFetchedComposeFile(compose.compose_file);
			return;
		}
		const rawUrl = buildRawGitUrl(compose);
		if (rawUrl) {
			let isMounted = true;
			fetch(rawUrl)
				.then((res) => (res.ok ? res.text() : ''))
				.then((text) => {
					if (isMounted && text && text.trim()) {
						setFetchedComposeFile(text);
					}
				})
				.catch(() => {});
			return () => {
				isMounted = false;
			};
		}
	}, [compose]);

	const domains = Array.isArray(passedDomains) ? passedDomains : [];
	const schedules = Array.isArray(passedSchedules) ? passedSchedules : [];

	// Mutations
	const createDomainMutation = $api.useMutation('post', '/domains');
	const patchDomainMutation = $api.useMutation('patch', '/domains/{id}');
	const deleteDomainMutation = $api.useMutation('delete', '/domains/{id}');

	const createScheduleMutation = $api.useMutation('post', '/schedules');
	const patchScheduleMutation = $api.useMutation('patch', '/schedules/{id}');
	const deleteScheduleMutation = $api.useMutation('delete', '/schedules/{id}');

	const createBackupMutation = $api.useMutation('post', '/backups/volume');
	const patchBackupMutation = $api.useMutation('patch', '/backups/volume/{id}');
	const deleteBackupMutation = $api.useMutation('delete', '/backups/volume/{id}');

	const [editingDomainData, setEditingDomainData] = useState<any | null>(null);
	const [editingScheduleData, setEditingScheduleData] = useState<any | null>(null);
	const [editingBackupData, setEditingBackupData] = useState<any | null>(null);

	console.log('[ComposeArchitectureTab] active editing state:', {editingDomainData, editingScheduleData, editingBackupData});

	const composeBackups = useMemo(() => {
		const list = Array.isArray(passedBackups) ? passedBackups : [];
		return list.filter((b: any) => b.compose_id === composeId || b.app_name === compose?.app_name);
	}, [passedBackups, composeId, compose]);

	const isGit = compose?.source_type && compose?.source_type !== 'RAW';
	const gitBuildPath = compose?.build_path || compose?.custom_git_build_path || 'docker-compose.yml';

	const servicesList = useMemo(() => {
		if (selectedService) return [selectedService.name];
		return [compose?.app_name || 'app'];
	}, [selectedService, compose]);

	// Handlers for Context Menu Options
	const handleAddDomain = (service: ComposeService) => {
		console.log('[ComposeArchitectureTab] handleAddDomain FOR SERVICE:', service);
		setEditingDomainData(null);
		setSelectedService(service);
		setActiveModal('domain');
	};

	const handleEditDomain = (domainData: any) => {
		console.log('[ComposeArchitectureTab] handleEditDomain FOR DOMAIN:', domainData);
		setEditingDomainData(domainData);
		setActiveModal('domain');
	};

	const handleAddSchedule = (service: ComposeService) => {
		console.log('[ComposeArchitectureTab] handleAddSchedule FOR SERVICE:', service);
		setEditingScheduleData(null);
		setSelectedService(service);
		setActiveModal('schedule');
	};

	const handleEditSchedule = (scheduleData: any) => {
		console.log('[ComposeArchitectureTab] handleEditSchedule FOR SCHEDULE:', scheduleData);
		setEditingScheduleData(scheduleData);
		setActiveModal('schedule');
	};

	const handleAddBackup = (service: ComposeService) => {
		console.log('[ComposeArchitectureTab] handleAddBackup FOR SERVICE:', service);
		setEditingBackupData(null);
		setSelectedService(service);
		setActiveModal('backup');
	};

	const handleEditBackup = (backupData: any) => {
		console.log('[ComposeArchitectureTab] handleEditBackup FOR BACKUP:', backupData);
		setEditingBackupData(backupData);
		setActiveModal('backup');
	};

	const handleOpenTerminal = (service: ComposeService) => {
		console.log('[ComposeArchitectureTab] handleOpenTerminal FOR SERVICE:', service);
		setSelectedService(service);
		setActiveModal('terminal');
	};

	const handleViewLogs = (service: ComposeService) => {
		console.log('[ComposeArchitectureTab] handleViewLogs FOR SERVICE:', service);
		setSelectedService(service);
		setActiveModal('logs');
	};

	const handleViewDeployLogs = (service: ComposeService) => {
		console.log('[ComposeArchitectureTab] handleViewDeployLogs FOR SERVICE:', service);
		setSelectedService(service);
		setActiveModal('deployLogs');
	};

	// Deletion Handlers for Canvas Nodes
	const handleDeleteDomain = async (domainData: any) => {
		const domainId = domainData?.id;
		if (!domainId) return;
		try {
			await deleteDomainMutation.mutateAsync({params: {path: {id: domainId}}});
			toast.success('Domain removed');
			queryClient.invalidateQueries();
			onRefresh?.();
		} catch (e: any) {
			toast.error(e?.message || 'Failed to delete domain');
		}
	};

	const handleDeleteSchedule = async (scheduleData: any) => {
		const scheduleId = scheduleData?.id;
		if (!scheduleId) return;
		try {
			await deleteScheduleMutation.mutateAsync({params: {path: {id: scheduleId}}});
			toast.success('Schedule removed');
			queryClient.invalidateQueries();
			onRefresh?.();
		} catch (e: any) {
			toast.error(e?.message || 'Failed to delete schedule');
		}
	};

	const handleDeleteBackup = async (backupData: any) => {
		const backupId = backupData?.id;
		if (!backupId) return;
		try {
			await deleteBackupMutation.mutateAsync({params: {path: {id: backupId}}});
			toast.success('Backup rule removed');
			queryClient.invalidateQueries();
			onRefresh?.();
		} catch (e: any) {
			toast.error(e?.message || 'Failed to delete backup rule');
		}
	};

	// Save Handlers
	const handleSaveDomain = async (data: {
		domain: string;
		serviceName: string;
		containerPort: number;
		https: boolean;
		path: string;
	}) => {
		if (editingDomainData?.id) {
			await patchDomainMutation.mutateAsync({
				params: {path: {id: editingDomainData.id}},
				body: {
					host: data.domain,
					service_name: data.serviceName,
					port: data.containerPort,
					https: data.https,
					path: data.path,
				} as any,
			});
			toast.success(`Domain updated successfully`);
		} else {
			await createDomainMutation.mutateAsync({
				body: {
					compose_id: composeId,
					host: data.domain,
					service_name: data.serviceName,
					port: data.containerPort,
					https: data.https,
					path: data.path,
				} as any,
			});
			toast.success(`Domain attached to ${data.serviceName}`);
		}
		queryClient.invalidateQueries();
		setActiveModal(null);
		setEditingDomainData(null);
	};

	const handleSaveSchedule = async (data: {
		name: string;
		serviceName: string;
		command: string;
		cronExpr: string;
	}) => {
		try {
			if (editingScheduleData?.id) {
				await patchScheduleMutation.mutateAsync({
					params: {path: {id: editingScheduleData.id}},
					body: {
						name: data.name,
						cron_expression: data.cronExpr,
						command: data.command,
						service_name: data.serviceName,
					} as any,
				});
				toast.success('Schedule updated successfully');
			} else {
				await createScheduleMutation.mutateAsync({
					body: {
						name: data.name,
						cron_expression: data.cronExpr,
						command: data.command,
						compose_id: composeId,
						app_name: compose?.app_name || data.serviceName,
						service_name: data.serviceName,
						schedule_type: 'compose',
						schedule_action: 'exec',
						shell_type: 'sh',
						enabled: 1,
					} as any,
				});
				toast.success(`Schedule attached to ${data.serviceName}`);
			}
			queryClient.invalidateQueries();
			setActiveModal(null);
			setEditingScheduleData(null);
		} catch (err: any) {
			toast.error(err?.message || 'Failed to save schedule');
		}
	};

	const handleSaveBackup = async (data: {
		name: string;
		serviceName: string;
		volumeName: string;
		cronExpr: string;
		prefix: string;
		turnOff: boolean;
	}) => {
		try {
			if (editingBackupData?.id) {
				await patchBackupMutation.mutateAsync({
					params: {path: {id: editingBackupData.id}},
					body: {
						name: data.name,
						volume_name: data.volumeName,
						prefix: data.prefix || '',
						service_name: data.serviceName,
						turn_off: data.turnOff ? 1 : 0,
						cron_expression: data.cronExpr,
					} as any,
				});
				toast.success('Volume backup updated successfully');
			} else {
				await createBackupMutation.mutateAsync({
					body: {
						name: data.name,
						volume_name: data.volumeName,
						prefix: data.prefix || '',
						service_type: 'compose',
						app_name: compose?.app_name || compose?.name || data.serviceName || 'app',
						service_name: data.serviceName,
						turn_off: data.turnOff ? 1 : 0,
						cron_expression: data.cronExpr,
						compose_id: composeId,
					} as any,
				});
				toast.success(`Backup rule attached to ${data.serviceName}`);
			}
			queryClient.invalidateQueries();
			setActiveModal(null);
			setEditingBackupData(null);
		} catch (err: any) {
			toast.error(err?.message || 'Failed to save volume backup');
		}
	};

	const terminalAppData = useMemo(() => ({
		...compose,
		name: `${compose?.name || 'compose'}:${selectedService?.name || 'app'}`,
		compose_id: composeId,
		app_name: compose?.app_name,
		service_name: selectedService?.name,
	}), [compose, composeId, selectedService]);

	return (
		<div className="flex flex-col gap-4 w-full animate-in fade-in duration-200">
			<div className="flex items-center justify-between">
				<div>
					<h3 className="text-sm font-bold text-foreground">Architecture Topology & Interactive Graph</h3>
					<p className="text-xs text-muted-foreground">
						Click any node for context options (Attach domain, schedule, backup, open terminal, view logs, or delete items).
					</p>
				</div>
			</div>

			<ComposeVisualizer
				composeFile={fetchedComposeFile || compose?.compose_file || ''}
				stackName={compose?.name || compose?.app_name}
				gitBuildPath={gitBuildPath}
				isGitSource={isGit}
				backups={composeBackups as any}
				schedules={schedules as any}
				domains={domains as any}
				onAddDomain={handleAddDomain}
				onAddSchedule={handleAddSchedule}
				onAddBackup={handleAddBackup}
				onOpenTerminal={handleOpenTerminal}
				onViewLogs={handleViewLogs}
				onViewDeployLogs={handleViewDeployLogs}
				onEditDomain={handleEditDomain}
				onDeleteDomain={handleDeleteDomain}
				onEditSchedule={handleEditSchedule}
				onDeleteSchedule={handleDeleteSchedule}
				onEditBackup={handleEditBackup}
				onDeleteBackup={handleDeleteBackup}
			/>

			{/* Domain Modal */}
			{activeModal === 'domain' && (
				<ComposeDomainModal
					isOpen={true}
					onClose={() => {
						setActiveModal(null);
						setEditingDomainData(null);
					}}
					editingDomain={editingDomainData}
					servicesList={servicesList}
					onSave={handleSaveDomain}
				/>
			)}

			{/* Schedule Modal */}
			{activeModal === 'schedule' && (
				<CreateScheduleModal
					isOpen={true}
					onClose={() => {
						setActiveModal(null);
						setEditingScheduleData(null);
					}}
					editingSchedule={editingScheduleData}
					servicesList={servicesList}
					onCreate={handleSaveSchedule}
				/>
			)}

			{/* Backup Modal */}
			{activeModal === 'backup' && (
				<CreateBackupModal
					isOpen={true}
					onClose={() => {
						setActiveModal(null);
						setEditingBackupData(null);
					}}
					editingBackup={editingBackupData}
					servicesList={servicesList}
					defaultServiceName={selectedService?.name}
					onCreate={handleSaveBackup}
				/>
			)}

			{/* Terminal Shell Modal */}
			{activeModal === 'terminal' && (
				<TerminalModal
					app={terminalAppData}
					open={true}
					onClose={() => setActiveModal(null)}
				/>
			)}

			{/* Standalone Live Container Logs Modal */}
			{activeModal === 'logs' && (
				<ComposeDirectContainerLogsModal
					isOpen={true}
					onClose={() => setActiveModal(null)}
					compose={compose}
					serviceName={selectedService?.name}
				/>
			)}

			{/* Standalone Direct Deployment Logs Stream Modal */}
			{activeModal === 'deployLogs' && (
				<ComposeDirectDeployLogsModal
					isOpen={true}
					onClose={() => setActiveModal(null)}
					composeId={composeId}
					serviceName={selectedService?.name}
				/>
			)}
		</div>
	);
}
