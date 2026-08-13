import {useState, useMemo} from 'react';
import {Database, Plus} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Badge} from '#/components/ui/badge';
import {toast} from 'sonner';
import {$api} from '#/api/query';
import {formatApiError} from '#/api/utils';
import {useBackupListVolumeBackups} from 'virtual:openoxide-live';
import {CreateBackupModal} from './backups/create-backup-modal';
import {ComposeBackupsTable} from './backups/compose-backups-table';

interface ComposeBackupsTabProps {
	compose: any;
	backups?: any[];
	isLoading?: boolean;
}

// Extract service names defined under 'services:' in docker-compose.yml content
const extractServicesFromYaml = (yamlStr?: string): string[] => {
	if (!yamlStr) return [];
	const lines = yamlStr.split('\n');
	const services: string[] = [];
	let inServicesBlock = false;
	let servicesIndent = 0;

	for (const line of lines) {
		const trimmed = line.trimEnd();
		if (!trimmed || trimmed.trimStart().startsWith('#')) continue;

		const indent = line.search(/\S/);
		const text = trimmed.trim();

		if (text === 'services:' || text.startsWith('services:')) {
			inServicesBlock = true;
			servicesIndent = indent;
			continue;
		}

		if (inServicesBlock) {
			if (indent <= servicesIndent && text.endsWith(':') && !text.startsWith('-')) {
				inServicesBlock = false;
			} else if (indent > servicesIndent && text.endsWith(':') && !text.includes(' ') && !text.includes('.')) {
				const serviceName = text.slice(0, -1).trim();
				if (serviceName && !services.includes(serviceName)) {
					services.push(serviceName);
				}
			}
		}
	}
	return services;
};

export function ComposeBackupsTab({compose, backups: passedBackups, isLoading: passedIsLoading}: ComposeBackupsTabProps) {
	const [isCreateOpen, setIsCreateOpen] = useState(false);

	const availableServices = useMemo(() => {
		return extractServicesFromYaml(compose?.compose_file);
	}, [compose?.compose_file]);

	const servicesList = availableServices.length > 0 ? availableServices : ['app'];

	// Real-time volume backups live hook (fallback if not passed from parent)
	const {data: rawBackups, loading: innerLoading} = useBackupListVolumeBackups();

	// Safe array normalization and filtering for current compose stack
	const composeBackups = useMemo(() => {
		if (passedBackups) return passedBackups;
		const list = rawBackups ?? [];
		return list.filter(
			(b: any) => b.compose_id === compose?.id || b.app_name === compose?.app_name
		);
	}, [passedBackups, rawBackups, compose]);
	const isLoading = passedIsLoading ?? innerLoading;

	// Mutations
	const createMutation = $api.useMutation('post', '/backups/volume');
	const runMutation = $api.useMutation('post', '/backups/volume/{id}/run');
	const restoreMutation = $api.useMutation('post', '/backups/volume/{id}/restore');
	const deleteMutation = $api.useMutation('delete', '/backups/volume/{id}');

	const handleCreate = async (data: {
		name: string;
		serviceName: string;
		volumeName: string;
		cronExpr: string;
		prefix: string;
		turnOff: boolean;
	}) => {
		try {
			await createMutation.mutateAsync({
				body: {
					name: data.name,
					compose_id: compose?.id,
					app_name: compose?.app_name,
					service_name: data.serviceName,
					volume_name: data.volumeName,
					cron_expression: data.cronExpr,
					prefix: data.prefix,
					turn_off: data.turnOff ? 1 : 0,
					destination_id: 0,
					organization_id: 1,
					service_type: 'COMPOSE',
				} as any,
			});
			toast.success('Compose volume backup rule created successfully');

		} catch (err: any) {
			toast.error(formatApiError(err));
		}
	};

	const handleRun = async (id: number) => {
		try {
			await runMutation.mutateAsync({params: {path: {id}}});
			toast.success('Volume snapshot triggered successfully');

		} catch (err: any) {
			toast.error(formatApiError(err));
		}
	};

	const handleRestore = async (id: number) => {
		try {
			await restoreMutation.mutateAsync({params: {path: {id}}, body: {backup_file: ''}});
			toast.success('Volume snapshot restore initiated');

		} catch (err: any) {
			toast.error(formatApiError(err));
		}
	};

	const handleDelete = async (id: number) => {
		try {
			await deleteMutation.mutateAsync({params: {path: {id}}});
			toast.success('Volume backup rule deleted');

		} catch (err: any) {
			toast.error(formatApiError(err));
		}
	};

	return (
		<div className="flex flex-col gap-6">
			{/* Top Header Card */}
			<section className="bg-card border border-border rounded-xl p-5 flex items-center justify-between flex-wrap gap-4 shadow-sm">
				<div>
					<h3 className="text-sm font-bold text-foreground flex items-center gap-2">
						<Database className="w-4 h-4 text-primary" /> Volume Backups
					</h3>
					<p className="text-xs text-muted-foreground mt-1">Configure volume backup rules to stream S3 snapshots of your compose container data</p>
				</div>
				<div className="flex items-center gap-3">
					<Badge variant="outline" className="text-xs font-mono px-3 py-1">
						Active Rules: {composeBackups.length}
					</Badge>
					<Button onClick={() => setIsCreateOpen(true)} size="sm" className="h-8 text-xs font-semibold flex items-center gap-1.5">
						<Plus className="w-4 h-4" /> Create Backup Rule
					</Button>
				</div>
			</section>

			{/* Backups Table Component (< 200 lines) */}
			<ComposeBackupsTable
				backups={composeBackups}
				isLoading={isLoading}
				onRun={handleRun}
				onRestore={handleRestore}
				onDelete={handleDelete}
			/>

			{/* Create Modal Component (< 200 lines) */}
			<CreateBackupModal
				isOpen={isCreateOpen}
				onClose={() => setIsCreateOpen(false)}
				servicesList={servicesList}
				defaultServiceName={compose?.name || compose?.app_name || 'database'}
				defaultVolumeName={compose?.volume_name || `${compose?.kind || 'db'}_data`}
				hideServiceAndVolumeSelect={!!compose?.kind || servicesList.length <= 1}
				onCreate={handleCreate}
			/>
		</div>
	);
}
