import {useState, useMemo} from 'react';
import {HardDrive, Plus, RefreshCw, Search} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {
	ListCard,
	ListEmpty,
	SectionHeader,
	Toolbar,
	ToolbarSearch,
	ViewGrid,
	ViewToggle,
	useListView,
} from '#/components/settings/list-card';
import {ConfirmDeleteDialog} from '#/components/settings/confirm-delete-dialog';
import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';
import {CreateVolumeBackupModal} from './create-volume-backup-modal';
import {
	useVolumeBackupDelete,
	useVolumeBackupRestore,
	useVolumeBackupRun,
} from '#/api/live';
import {
	VolumeBackupCard,
	VolumeBackupRow,
} from './volume-backups-tab/volume-backup-row';

interface VolumeBackupsTabProps {
	app: any;
	backups?: any[];
	onRefresh?: () => void;
}

/**
 * Service → Backups — the SSH Keys grammar: section title with the one
 * primary action, toolbar (search + grid/list) and rules as grid cards or
 * one bordered list card.
 */
export function VolumeBackupsTab({
	app,
	backups: passedBackups,
	onRefresh,
}: VolumeBackupsTabProps) {
	const [showCreateModal, setShowCreateModal] = useState(false);
	const [deleteTargetId, setDeleteTargetId] = useState<number | null>(
		null,
	);
	const [activeActionId, setActiveActionId] = useState<{
		id: number;
		action: 'run' | 'restore';
	} | null>(null);

	const [query, setQuery] = useState('');
	const [view, setView] = useListView('service-backups-view');

	const allBackups = Array.isArray(passedBackups) ? passedBackups : [];
	const appBackups = allBackups.filter(
		b =>
			String(b.application_id) === String(app?.id) ||
			(b.app_name && b.app_name === app?.app_name),
	);
	const q = query.trim().toLowerCase();
	const visibleBackups = useMemo(
		() =>
			q
				? appBackups.filter((b: any) =>
						[b?.name, b?.volume_name, b?.prefix, b?.cron_expression]
							.filter(Boolean)
							.join(' ')
							.toLowerCase()
							.includes(q),
					)
				: appBackups,
		[appBackups, q],
	);

	const runMutation = useVolumeBackupRun();
	const restoreMutation = useVolumeBackupRestore();
	const deleteMutation = useVolumeBackupDelete();

	const runAction = async (
		id: number,
		action: 'run' | 'restore',
		fn: () => Promise<unknown>,
		success: string,
	) => {
		setActiveActionId({id, action});
		try {
			await fn();
			toast.success(success);
			onRefresh?.();
		} catch (err: any) {
			toast.error(formatApiError(err));
		} finally {
			setActiveActionId(null);
		}
	};

	const handleDelete = async () => {
		if (!deleteTargetId) return;
		try {
			await deleteMutation.mutateAsync(deleteTargetId);
			toast.success('Volume backup configuration deleted');
			onRefresh?.();
		} catch (err: any) {
			toast.error(formatApiError(err));
		} finally {
			setDeleteTargetId(null);
		}
	};

	const createButton = (
		<Button
			onClick={() => setShowCreateModal(true)}
			className="h-9 gap-1.5">
			<Plus className="size-3.5" />
			Add Volume Backup
		</Button>
	);

	const itemProps = (b: any) => ({
		backup: b,
		activeAction:
			activeActionId && activeActionId.id === b.id
				? activeActionId.action
				: null,
		onRun: () =>
			void runAction(
				b.id,
				'run',
				() => runMutation.mutateAsync(b.id),
				'Volume backup execution started successfully',
			),
		onRestore: () =>
			void runAction(
				b.id,
				'restore',
				() => restoreMutation.mutateAsync({id: b.id, backup_file: ''}),
				'Volume restore process initiated successfully',
			),
		onDelete: () => setDeleteTargetId(b.id),
	});

	return (
		<div className="flex w-full flex-col gap-5">
			<SectionHeader
				title="Backups"
				description="Scheduled S3 snapshots of this service's volumes."
				action={
					<>
						<Button
							variant="ghost"
							size="icon"
							onClick={() => onRefresh?.()}
							title="Refresh"
							className="size-9 text-muted-foreground hover:text-foreground">
							<RefreshCw className="size-4" />
						</Button>
						{createButton}
					</>
				}
			/>

			<Toolbar action={<ViewToggle view={view} onChange={setView} />}>
				<ToolbarSearch
					value={query}
					onChange={setQuery}
					placeholder="Search backup rules…"
					className="min-w-[200px] flex-1 sm:max-w-none"
				/>
			</Toolbar>

			{appBackups.length === 0 ? (
				<ListEmpty
					icon={HardDrive}
					title="No volume backups yet"
					description="Create a volume backup rule to stream S3 snapshots of your container data."
					action={createButton}
				/>
			) : visibleBackups.length === 0 ? (
				<ListEmpty
					icon={Search}
					title="No backup rules match your search"
					description="Try a different name, volume, prefix or cron expression."
				/>
			) : view === 'grid' ? (
				<ViewGrid>
					{visibleBackups.map(b => (
						<VolumeBackupCard key={b.id} {...itemProps(b)} />
					))}
				</ViewGrid>
			) : (
				<ListCard>
					{visibleBackups.map(b => (
						<VolumeBackupRow key={b.id} {...itemProps(b)} />
					))}
				</ListCard>
			)}

			{/* Create Modal */}
			<CreateVolumeBackupModal
				open={showCreateModal}
				onOpenChange={setShowCreateModal}
				app={app}
				onSuccess={() => onRefresh?.()}
			/>

			<ConfirmDeleteDialog
				open={deleteTargetId !== null}
				onOpenChange={open => !open && setDeleteTargetId(null)}
				title="Delete volume backup rule"
				description={
					<span>
						Scheduled S3 snapshots for this volume stop immediately.
						Existing snapshots in the bucket are not removed. This cannot
						be undone.
					</span>
				}
				confirmLabel="Delete rule"
				busy={deleteMutation.isPending}
				onConfirm={() => void handleDelete()}
			/>
		</div>
	);
}
