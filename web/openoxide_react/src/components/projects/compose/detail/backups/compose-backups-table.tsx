import { useState } from 'react';
import { Database, Play, Trash2, RefreshCw, HardDrive, RotateCcw } from 'lucide-react';
import { Button } from '#/components/ui/button';
import { Badge } from '#/components/ui/badge';
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

export function ComposeBackupsTable({ backups, isLoading, onRun, onRestore, onDelete }: ComposeBackupsTableProps) {
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
		<div className="w-full">
			{isLoading && safeBackups.length === 0 ? (
				<div className="flex items-center justify-center h-40 text-xs text-muted-foreground gap-2 border border-dashed border-border/60 rounded-xl bg-card/20">
					<RefreshCw className="size-4 animate-spin text-primary" /> Loading volume backup rules...
				</div>
			) : safeBackups.length === 0 ? (
				<div className="flex flex-col items-center justify-center py-16 text-muted-foreground gap-2 text-xs border border-dashed border-border/60 rounded-xl bg-card/10">
					<Database className="size-8 opacity-40" />
					<p className="font-semibold text-foreground">No compose volume backup rules configured</p>
					<p className="text-[11px] text-muted-foreground">Add backup rules to schedule volume snapshots to S3</p>
				</div>
			) : (
				<div className="rounded-xl border border-border/60 bg-card overflow-hidden shadow-xs">
					<Table>
						<TableHeader>
							<TableRow className="border-b border-border/60 bg-muted/30 hover:bg-muted/30">
								<TableHead className="py-3.5 px-4 font-bold text-foreground text-xs uppercase tracking-wider">Backup Name</TableHead>
								<TableHead className="py-3.5 px-4 font-bold text-foreground text-xs uppercase tracking-wider">Service</TableHead>
								<TableHead className="py-3.5 px-4 font-bold text-foreground text-xs uppercase tracking-wider">Cron Schedule</TableHead>
								<TableHead className="py-3.5 px-4 font-bold text-foreground text-xs uppercase tracking-wider">Volume & Storage</TableHead>
								<TableHead className="py-3.5 px-4 text-right font-bold text-foreground text-xs uppercase tracking-wider">Actions</TableHead>
							</TableRow>
						</TableHeader>
						<TableBody>
							{safeBackups.map((b: any) => (
								<TableRow key={b.id} className="border-b border-border/40 hover:bg-muted/40 transition-colors">
									<TableCell className="py-3.5 px-4 font-bold text-xs text-foreground font-mono">
										<div className="flex items-center gap-2.5">
											<HardDrive className="size-4 text-primary shrink-0" />
											<span>{b.name}</span>
										</div>
									</TableCell>
									<TableCell className="py-3.5 px-4 text-xs font-semibold text-foreground">
										<Badge variant="secondary" className="text-[10px] font-mono">
											{b.service_name || 'app'}
										</Badge>
									</TableCell>
									<TableCell className="py-3.5 px-4 text-xs text-muted-foreground font-mono">
										<Badge variant="outline" className="text-[10px] font-mono">
											{b.cron_expression || '0 0 * * *'}
										</Badge>
									</TableCell>
									<TableCell className="py-3.5 px-4 text-xs text-muted-foreground font-mono">
										{b.volume_name || 'volume'} ➔ {b.prefix || 'volume-backups/'}
									</TableCell>
									<TableCell className="py-3.5 px-4 text-right">
										<div className="flex items-center justify-end gap-1.5">
											<Button
												variant="outline"
												size="sm"
												onClick={() => handleRun(b.id)}
												disabled={activeRunningId === b.id}
												className="h-7 text-xs font-semibold border-border hover:bg-muted flex items-center gap-1"
											>
												<Play className={`size-3 ${activeRunningId === b.id ? 'animate-spin' : ''}`} /> Run
											</Button>
											<Button
												variant="outline"
												size="sm"
												onClick={() => handleRestore(b.id)}
												disabled={activeRestoringId === b.id}
												className="h-7 text-xs font-semibold border-border hover:bg-muted flex items-center gap-1"
											>
												<RotateCcw className={`size-3 ${activeRestoringId === b.id ? 'animate-spin' : ''}`} /> Restore
											</Button>
											<Button
												variant="ghost"
												size="icon"
												onClick={() => handleDelete(b.id)}
												disabled={activeDeletingId === b.id}
												className="size-7 text-muted-foreground hover:text-destructive hover:bg-destructive/10 rounded-lg"
											>
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
