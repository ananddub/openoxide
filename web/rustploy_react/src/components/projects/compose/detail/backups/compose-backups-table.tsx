import {useState} from 'react';
import {Database, Play, Trash2, RefreshCw, HardDrive, RotateCcw, Box} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Badge} from '#/components/ui/badge';
import {toast} from 'sonner';

interface ComposeBackupsTableProps {
	backups: any[];
	isLoading: boolean;
	onRun: (id: number) => Promise<void>;
	onRestore: (id: number) => Promise<void>;
	onDelete: (id: number) => Promise<void>;
}

export function ComposeBackupsTable({backups, isLoading, onRun, onRestore, onDelete}: ComposeBackupsTableProps) {
	const [activeRunningId, setActiveRunningId] = useState<number | null>(null);
	const [activeRestoringId, setActiveRestoringId] = useState<number | null>(null);
	const [activeDeletingId, setActiveDeletingId] = useState<number | null>(null);
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
		<section className="bg-card border border-border rounded-xl p-5 flex flex-col gap-4 shadow-sm">
			{isLoading && safeBackups.length === 0 ? (
				<div className="flex items-center justify-center h-48 text-xs text-muted-foreground gap-2">
					<RefreshCw className="w-4 h-4 animate-spin text-primary" /> Loading volume backup rules...
				</div>
			) : safeBackups.length === 0 ? (
				<div className="flex flex-col items-center justify-center h-48 text-muted-foreground gap-2 text-xs">
					<Database className="w-8 h-8 opacity-40" />
					<p>No compose volume backup rules configured.</p>
				</div>
			) : (
				<div className="flex flex-col gap-3">
					{safeBackups.map((b: any) => (
						<div
							key={b.id}
							className="border border-border/80 rounded-lg p-4 bg-muted/20 hover:bg-muted/40 transition-colors flex items-center justify-between gap-4 flex-wrap"
						>
							<div className="flex items-start gap-3">
								<div className="w-8 h-8 rounded-lg bg-secondary flex items-center justify-center text-primary shrink-0 border border-border/40">
									<HardDrive className="w-4 h-4" />
								</div>
								<div className="flex flex-col gap-1">
									<div className="flex items-center gap-2 flex-wrap">
										<span className="text-xs font-bold text-foreground">{b.name}</span>
										<Badge variant="outline" className="text-[10px] font-mono">
											<Box className="w-3 h-3 mr-1 text-primary" /> Service: {b.service_name || 'app'}
										</Badge>
										<Badge variant="secondary" className="text-[10px] font-mono">
											{b.cron_expression || '0 0 * * *'}
										</Badge>
									</div>
									<span className="text-xs text-muted-foreground font-mono">
										Volume: {b.volume_name} ➔ S3: {b.prefix || 'volume-backups/'}
									</span>
								</div>
							</div>

							<div className="flex items-center gap-2">
								<Button
									variant="outline"
									size="sm"
									onClick={() => handleRun(b.id)}
									disabled={activeRunningId === b.id}
									className="h-8 text-xs font-semibold border-border hover:bg-muted flex items-center gap-1.5"
								>
									<Play className={`w-3.5 h-3.5 ${activeRunningId === b.id ? 'animate-spin' : ''}`} /> Run Snapshot
								</Button>

								<Button
									variant="outline"
									size="sm"
									onClick={() => handleRestore(b.id)}
									disabled={activeRestoringId === b.id}
									className="h-8 text-xs font-semibold border-border hover:bg-muted flex items-center gap-1.5"
								>
									<RotateCcw className={`w-3.5 h-3.5 ${activeRestoringId === b.id ? 'animate-spin' : ''}`} /> Restore
								</Button>

								<Button
									variant="ghost"
									size="icon"
									onClick={() => handleDelete(b.id)}
									disabled={activeDeletingId === b.id}
									className="h-8 w-8 text-muted-foreground hover:text-destructive hover:bg-destructive/10"
								>
									<Trash2 className="w-4 h-4" />
								</Button>
							</div>
						</div>
					))}
				</div>
			)}
		</section>
	);
}
