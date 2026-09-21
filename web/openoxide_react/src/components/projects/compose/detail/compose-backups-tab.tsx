import {useBackupItems} from '#/api/live';
import {useState, useMemo, useEffect} from 'react';
import {Database, Plus} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Badge} from '#/components/ui/badge';
import {useOrganizations} from '#/stores/session-context';
import {CreateBackupModal} from './backups/create-backup-modal';
import {ComposeBackupsTable} from './backups/compose-backups-table';
import {useVolumeBackupActions} from './backups/use-volume-backup-actions';
import {filterResourceBackups} from './backups/backup-mappers';
import {
	buildRawGitUrl,
	getComposeServiceNames,
} from '#/utils/compose-services';

interface ComposeBackupsTabProps {
	compose: any;
	backups?: any[];
	databaseId?: number;
	databaseKind?: string;
}

export function ComposeBackupsTab({
	compose,
	backups: passedBackups,
	databaseId,
	databaseKind,
}: ComposeBackupsTabProps) {
	const isDatabase = databaseId !== undefined;
	const normalizedDatabaseKind = (
		databaseKind ||
		compose?.kind ||
		''
	).toLowerCase();
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

	// Live backups query; the table-watch stream (backups tables) keeps it
	// fresh, so no zustand slice involved.
	const {activeOrg} = useOrganizations();
	const {data: storeBackups = []} = useBackupItems(
		BigInt(activeOrg?.id || 1),
	);

	// Safe array normalization and filtering for current compose stack
	const composeBackups = useMemo(() => {
		if (passedBackups && passedBackups.length > 0) return passedBackups;
		return filterResourceBackups(storeBackups, {
			isDatabase,
			composeId: compose?.id,
			databaseId,
		});
	}, [passedBackups, storeBackups, compose, databaseId, isDatabase]);
	const isLoading = false;

	const {handleCreate, handleRun, handleRestore, handleDelete} =
		useVolumeBackupActions({
			compose,
			isDatabase,
			normalizedDatabaseKind,
			databaseId,
		});

	return (
		<div className="flex flex-col gap-6">
			{/* Top Header Card */}
			<section className="flex flex-wrap items-center justify-between gap-4 rounded-xl bg-card p-5 shadow-sm">
				<div>
					<h3 className="flex items-center gap-2 text-sm font-bold text-foreground">
						<Database className="h-4 w-4 text-primary" /> Volume Backups
					</h3>
					<p className="mt-1 text-xs text-muted-foreground">
						Configure volume backup rules to stream S3 snapshots of your
						{isDatabase ? ' database' : ' compose container'} data
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
