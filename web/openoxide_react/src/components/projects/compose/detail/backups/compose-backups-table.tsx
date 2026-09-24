import {useState} from 'react';
import type {ReactNode} from 'react';
import {Database, Play, Trash2, RotateCcw, Search} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {
	GridSkeleton,
	ListCard,
	ListRow,
	RowTile,
	ListEmpty,
	ListSkeleton,
	ViewCard,
	ViewGrid,
	type ListView,
} from '#/components/settings/list-card';
import {CronChip} from '#/components/shared/cron-chip';

interface ComposeBackupsTableProps {
	backups: any[];
	isLoading: boolean;
	hasQuery: boolean;
	view: ListView;
	onRun: (id: number) => Promise<void>;
	onRestore: (id: number) => Promise<void>;
	onDelete: (id: number) => Promise<void>;
	createAction?: ReactNode;
}

function Chip({children, mono}: {children: ReactNode; mono?: boolean}) {
	return (
		<span
			className={`shrink-0 rounded-md bg-foreground/[0.05] px-1.5 py-0.5 text-[10px] font-medium text-muted-foreground ${
				mono ? 'font-mono' : ''
			}`}>
			{children}
		</span>
	);
}

function BackupTile() {
	return (
		<RowTile>
			<Database className="size-4" />
		</RowTile>
	);
}

/** Compose / database volume-backup rules — grid cards or one bordered list card, matching the SSH Keys page. */
export function ComposeBackupsTable({
	backups,
	isLoading,
	hasQuery,
	view,
	onRun,
	onRestore,
	onDelete,
	createAction,
}: ComposeBackupsTableProps) {
	const [activeRunningId, setActiveRunningId] = useState<number | null>(
		null,
	);
	const [activeRestoringId, setActiveRestoringId] = useState<
		number | null
	>(null);
	const [activeDeletingId, setActiveDeletingId] = useState<number | null>(
		null,
	);
	const safeBackups = Array.isArray(backups) ? backups : [];

	const handleRun = async (id: number) => {
		setActiveRunningId(id);
		try {
			await onRun(id);
		} finally {
			setActiveRunningId(null);
		}
	};

	const handleRestore = async (id: number) => {
		setActiveRestoringId(id);
		try {
			await onRestore(id);
		} finally {
			setActiveRestoringId(null);
		}
	};

	const handleDelete = async (id: number) => {
		setActiveDeletingId(id);
		try {
			await onDelete(id);
		} finally {
			setActiveDeletingId(null);
		}
	};

	const actions = (b: any, compact?: boolean) => (
		<>
			<Button
				variant="ghost"
				size={compact ? 'icon' : 'sm'}
				onClick={() => handleRun(b.id)}
				disabled={activeRunningId === b.id}
				title="Run backup now"
				className={
					compact
						? 'size-8 text-muted-foreground hover:text-foreground'
						: 'h-8 gap-1.5 text-xs text-muted-foreground hover:text-foreground'
				}>
				<Play
					className={`${compact ? 'size-4' : 'size-3.5'} ${activeRunningId === b.id ? 'animate-spin' : ''}`}
				/>
				{!compact && 'Run'}
			</Button>
			<Button
				variant="ghost"
				size={compact ? 'icon' : 'sm'}
				onClick={() => handleRestore(b.id)}
				disabled={activeRestoringId === b.id}
				title="Restore from the latest snapshot"
				className={
					compact
						? 'size-8 text-muted-foreground hover:text-foreground'
						: 'h-8 gap-1.5 text-xs text-muted-foreground hover:text-foreground'
				}>
				<RotateCcw
					className={`${compact ? 'size-4' : 'size-3.5'} ${activeRestoringId === b.id ? 'animate-spin' : ''}`}
				/>
				{!compact && 'Restore'}
			</Button>
			<Button
				variant="ghost"
				size="icon"
				onClick={() => handleDelete(b.id)}
				disabled={activeDeletingId === b.id}
				title="Delete backup rule"
				className="size-8 text-muted-foreground hover:bg-destructive/10 hover:text-destructive">
				<Trash2 className="size-4" />
			</Button>
		</>
	);

	const path = (b: any) => (
		<span className="font-mono">
			{b.volume_name || 'volume'} → {b.prefix || 'volume-backups/'}
		</span>
	);

	if (isLoading && safeBackups.length === 0)
		return view === 'grid' ? <GridSkeleton /> : <ListSkeleton />;

	if (safeBackups.length === 0)
		return hasQuery ? (
			<ListEmpty
				icon={Search}
				title="No backup rules match your search"
				description="Try a different name, service, volume or cron expression."
			/>
		) : (
			<ListEmpty
				icon={Database}
				title="No volume backup rules yet"
				description="Add backup rules to schedule volume snapshots to S3."
				action={createAction}
			/>
		);

	if (view === 'grid')
		return (
			<ViewGrid>
				{safeBackups.map((b: any) => (
					<ViewCard key={b.id}>
						<div className="flex items-start justify-between gap-2.5">
							<div className="flex min-w-0 items-center gap-3">
								<BackupTile />
								<div className="flex min-w-0 flex-col">
									<h3 className="truncate text-sm font-semibold text-foreground">
										{b.name}
									</h3>
									<span className="mt-0.5 truncate text-[11px] text-muted-foreground">
										{b.service_name || 'app'}
									</span>
								</div>
							</div>
							{actions(b, true)}
						</div>
						<div className="flex flex-col gap-2">
							<code className="truncate rounded-md bg-foreground/[0.04] px-2 py-1 text-[11px] text-primary">
								{path(b)}
							</code>
							<div className="flex flex-wrap items-center gap-1.5">
								<Chip>{b.service_name || 'app'}</Chip>
								<CronChip expression={b.cron_expression || '0 0 * * *'} />
							</div>
						</div>
					</ViewCard>
				))}
			</ViewGrid>
		);

	return (
		<ListCard>
			{safeBackups.map((b: any) => (
				<ListRow
					key={b.id}
					icon={<BackupTile />}
					title={b.name}
					subtitle={path(b)}
					badges={
						<>
							<Chip>{b.service_name || 'app'}</Chip>
							<CronChip expression={b.cron_expression || '0 0 * * *'} />
						</>
					}>
					{actions(b)}
				</ListRow>
			))}
		</ListCard>
	);
}
