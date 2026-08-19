import {useState, useMemo, useEffect} from 'react';
import {Database, Plus} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Badge} from '#/components/ui/badge';
import {toast} from 'sonner';
import {$api} from '#/api/query';
import {formatApiError} from '#/api/utils';
import {useAppStore} from '#/stores/app-store';
import {CreateBackupModal} from './backups/create-backup-modal';
import {ComposeBackupsTable} from './backups/compose-backups-table';
import {
	buildRawGitUrl,
	getComposeServiceNames,
} from '#/utils/compose-services';

interface ComposeBackupsTabProps {
	compose: any;
	backups?: any[];
	isLoading?: boolean;
}

export function ComposeBackupsTab({
	compose,
	backups: passedBackups,
	isLoading: passedIsLoading,
}: ComposeBackupsTabProps) {
	const [isCreateOpen, setIsCreateOpen] = useState(false);
	const [fetchedYaml, setFetchedYaml] = useState<string>('');

	useEffect(() => {
		if (compose?.compose_file && compose.compose_file.trim()) {
			setFetchedYaml(compose.compose_file);
			return;
		}
		const rawUrl = buildRawGitUrl(compose);
		if (rawUrl) {
			let isMounted = true;
			fetch(rawUrl)
				.then(res => (res.ok ? res.text() : ''))
				.then(text => {
					if (isMounted && text && text.trim()) {
						setFetchedYaml(text);
					}
				})
				.catch(() => {});
			return () => {
				isMounted = false;
			};
		}
	}, [compose]);

	const servicesList = useMemo(() => {
		return getComposeServiceNames(compose, fetchedYaml);
	}, [compose, fetchedYaml]);

	// Read volume backups directly from Zustand RAM Store
	const storeBackups = useAppStore(state => state.backups || []);

	// Safe array normalization and filtering for current compose stack
	const composeBackups = useMemo(() => {
		if (passedBackups && passedBackups.length > 0) return passedBackups;
		return storeBackups.filter(
			(b: any) =>
				Number(b.compose_id) === Number(compose?.id) ||
				b.app_name === compose?.app_name,
		);
	}, [passedBackups, storeBackups, compose]);
	const isLoading = false;

	// Mutations
	const createMutation = $api.useMutation('post', '/backups/volume');
	const runMutation = $api.useMutation('post', '/backups/volume/{id}/run');
	const restoreMutation = $api.useMutation(
		'post',
		'/backups/volume/{id}/restore',
	);
	const deleteMutation = $api.useMutation(
		'delete',
		'/backups/volume/{id}',
	);

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
			await restoreMutation.mutateAsync({
				params: {path: {id}},
				body: {backup_file: ''},
			});
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
			<section className="flex flex-wrap items-center justify-between gap-4 rounded-xl border border-border bg-card p-5 shadow-sm">
				<div>
					<h3 className="flex items-center gap-2 text-sm font-bold text-foreground">
						<Database className="h-4 w-4 text-primary" /> Volume Backups
					</h3>
					<p className="mt-1 text-xs text-muted-foreground">
						Configure volume backup rules to stream S3 snapshots of your
						compose container data
					</p>
				</div>
				<div className="flex items-center gap-3">
					<Badge variant="outline" className="px-3 py-1 font-mono text-xs">
						Active Rules: {composeBackups.length}
					</Badge>
					<Button
						onClick={() => setIsCreateOpen(true)}
						size="sm"
						className="flex h-8 items-center gap-1.5 text-xs font-semibold">
						<Plus className="h-4 w-4" /> Create Backup Rule
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
				defaultServiceName={
					compose?.name || compose?.app_name || 'database'
				}
				defaultVolumeName={
					compose?.volume_name || `${compose?.kind || 'db'}_data`
				}
				hideServiceAndVolumeSelect={
					!!compose?.kind || servicesList.length <= 1
				}
				onCreate={handleCreate}
			/>
		</div>
	);
}
