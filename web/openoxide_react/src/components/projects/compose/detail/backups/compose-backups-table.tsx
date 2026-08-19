import {useState} from 'react';
import {
	Database,
	Play,
	Trash2,
	RefreshCw,
	HardDrive,
	RotateCcw,
} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Badge} from '#/components/ui/badge';
import {
	Table,
	TableHeader,
	TableBody,
	TableHead,
	TableRow,
	TableCell,
} from '#/components/ui/table';

interface ComposeBackupsTableProps {
	backups: any[];
	isLoading: boolean;
	onRun: (id: number) => Promise<void>;
	onRestore: (id: number) => Promise<void>;
	onDelete: (id: number) => Promise<void>;
}

export function ComposeBackupsTable({
	backups,
	isLoading,
	onRun,
	onRestore,
	onDelete,
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

	return (
		<div className="w-full">
			{isLoading && safeBackups.length === 0 ? (
				<div className="flex h-40 items-center justify-center gap-2 rounded-xl border border-dashed border-border/60 bg-card/20 text-xs text-muted-foreground">
					<RefreshCw className="size-4 animate-spin text-primary" />{' '}
					Loading volume backup rules...
				</div>
			) : safeBackups.length === 0 ? (
				<div className="flex flex-col items-center justify-center gap-2 rounded-xl border border-dashed border-border/60 bg-card/10 py-16 text-xs text-muted-foreground">
					<Database className="size-8 opacity-40" />
					<p className="font-semibold text-foreground">
						No compose volume backup rules configured
					</p>
					<p className="text-[11px] text-muted-foreground">
						Add backup rules to schedule volume snapshots to S3
					</p>
				</div>
			) : (
				<div className="overflow-hidden rounded-xl border border-border/60 bg-card shadow-xs">
					<Table>
						<TableHeader>
							<TableRow className="border-b border-border/60 bg-muted/30 hover:bg-muted/30">
								<TableHead className="px-4 py-3.5 text-xs font-bold tracking-wider text-foreground uppercase">
									Backup Name
								</TableHead>
								<TableHead className="px-4 py-3.5 text-xs font-bold tracking-wider text-foreground uppercase">
									Service
								</TableHead>
								<TableHead className="px-4 py-3.5 text-xs font-bold tracking-wider text-foreground uppercase">
									Cron Schedule
								</TableHead>
								<TableHead className="px-4 py-3.5 text-xs font-bold tracking-wider text-foreground uppercase">
									Volume & Storage
								</TableHead>
								<TableHead className="px-4 py-3.5 text-right text-xs font-bold tracking-wider text-foreground uppercase">
									Actions
								</TableHead>
							</TableRow>
						</TableHeader>
						<TableBody>
							{safeBackups.map((b: any) => (
								<TableRow
									key={b.id}
									className="border-b border-border/40 transition-colors hover:bg-muted/40">
									<TableCell className="px-4 py-3.5 font-mono text-xs font-bold text-foreground">
										<div className="flex items-center gap-2.5">
											<HardDrive className="size-4 shrink-0 text-primary" />
											<span>{b.name}</span>
										</div>
									</TableCell>
									<TableCell className="px-4 py-3.5 text-xs font-semibold text-foreground">
										<Badge
											variant="secondary"
											className="font-mono text-[10px]">
											{b.service_name || 'app'}
										</Badge>
									</TableCell>
									<TableCell className="px-4 py-3.5 font-mono text-xs text-muted-foreground">
										<Badge
											variant="outline"
											className="font-mono text-[10px]">
											{b.cron_expression || '0 0 * * *'}
										</Badge>
									</TableCell>
									<TableCell className="px-4 py-3.5 font-mono text-xs text-muted-foreground">
										{b.volume_name || 'volume'} ➔{' '}
										{b.prefix || 'volume-backups/'}
									</TableCell>
									<TableCell className="px-4 py-3.5 text-right">
										<div className="flex items-center justify-end gap-1.5">
											<Button
												variant="outline"
												size="sm"
												onClick={() => handleRun(b.id)}
												disabled={activeRunningId === b.id}
												className="flex h-7 items-center gap-1 border-border text-xs font-semibold hover:bg-muted">
												<Play
													className={`size-3 ${activeRunningId === b.id ? 'animate-spin' : ''}`}
												/>{' '}
												Run
											</Button>
											<Button
												variant="outline"
												size="sm"
												onClick={() => handleRestore(b.id)}
												disabled={activeRestoringId === b.id}
												className="flex h-7 items-center gap-1 border-border text-xs font-semibold hover:bg-muted">
												<RotateCcw
													className={`size-3 ${activeRestoringId === b.id ? 'animate-spin' : ''}`}
												/>{' '}
												Restore
											</Button>
											<Button
												variant="ghost"
												size="icon"
												onClick={() => handleDelete(b.id)}
												disabled={activeDeletingId === b.id}
												className="size-7 rounded-lg text-muted-foreground hover:bg-destructive/10 hover:text-destructive">
												<Trash2 className="size-3.5" />
											</Button>
										</div>
									</TableCell>
								</TableRow>
							))}
						</TableBody>
					</Table>
				</div>
			)}
		</div>
	);
}
