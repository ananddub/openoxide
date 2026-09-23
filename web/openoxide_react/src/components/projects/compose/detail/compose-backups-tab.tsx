import {useBackupItems} from '#/api/live';
import {useState, useMemo, useEffect} from 'react';
import {Plus} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {
	SectionHeader,
	Toolbar,
	ToolbarSearch,
	ViewToggle,
	useListView,
} from '#/components/settings/list-card';
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
	const [query, setQuery] = useState('');
	const [view, setView] = useListView('compose-backups-view');
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

	const q = query.trim().toLowerCase();
	const visibleBackups = useMemo(
		() =>
			q
				? composeBackups.filter((b: any) =>
						[
							b?.name,
							b?.service_name,
							b?.cron_expression,
							b?.volume_name,
							b?.prefix,
						]
							.filter(Boolean)
							.join(' ')
							.toLowerCase()
							.includes(q),
					)
				: composeBackups,
		[composeBackups, q],
	);

	const {handleCreate, handleRun, handleRestore, handleDelete} =
		useVolumeBackupActions({
			compose,
			isDatabase,
			normalizedDatabaseKind,
			databaseId,
		});

	const createButton = (
		<Button onClick={() => setIsCreateOpen(true)} className="h-9 gap-1.5">
			<Plus className="size-3.5" />
			Create Backup Rule
		</Button>
	);

	return (
		<div className="flex w-full flex-col gap-5">
			<SectionHeader
				title="Backups"
				description="Rules that snapshot this stack's volumes to an S3 destination."
				action={createButton}
			/>

			<Toolbar action={<ViewToggle view={view} onChange={setView} />}>
				<ToolbarSearch
					value={query}
					onChange={setQuery}
					placeholder="Search backup rules…"
					className="min-w-[200px] flex-1 sm:max-w-none"
				/>
			</Toolbar>

			{/* Backups Table Component (< 200 lines) */}
			<ComposeBackupsTable
				backups={visibleBackups}
				isLoading={isLoading}
				hasQuery={q !== ''}
				view={view}
				onRun={handleRun}
				onRestore={handleRestore}
				onDelete={handleDelete}
				createAction={createButton}
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
