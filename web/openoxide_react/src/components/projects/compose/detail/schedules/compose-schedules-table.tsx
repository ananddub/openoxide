import {useState} from 'react';
import {
	Calendar,
	Play,
	Trash2,
	RefreshCw,
	Box,
	Terminal,
	CalendarDays,
} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Badge} from '#/components/ui/badge';

interface ComposeSchedulesTableProps {
	schedules: any[];
	isLoading: boolean;
	onRun: (id: number) => Promise<void>;
	onDelete: (id: number) => Promise<void>;
}

export function ComposeSchedulesTable({
	schedules,
	isLoading,
	onRun,
	onDelete,
}: ComposeSchedulesTableProps) {
	const [activeRunningId, setActiveRunningId] = useState<number | null>(
		null,
	);
	const [activeDeletingId, setActiveDeletingId] = useState<number | null>(
		null,
	);
	const safeSchedules = Array.isArray(schedules) ? schedules : [];

	const handleRun = async (id: number) => {
		setActiveRunningId(id);
		try {
			await onRun(id);
		} finally {
			setActiveRunningId(null);
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
		<section className="flex flex-col gap-4 rounded-xl border border-border bg-card p-5">
			{isLoading && safeSchedules.length === 0 ? (
				<div className="flex h-48 items-center justify-center gap-2 text-xs text-muted-foreground">
					<RefreshCw className="h-4 w-4 animate-spin text-primary" />{' '}
					Loading schedules...
				</div>
			) : safeSchedules.length === 0 ? (
				<div className="flex h-48 flex-col items-center justify-center gap-2 text-xs text-muted-foreground">
					<Calendar className="h-8 w-8 opacity-40" />
					<p>No compose schedules configured.</p>
				</div>
			) : (
				<div className="flex flex-col gap-3">
					{safeSchedules.map((s: any) => (
						<div
							key={s.id}
							className="flex flex-wrap items-center justify-between gap-4 rounded-xl border border-border bg-card p-4">
							<div className="flex min-w-0 items-start gap-3">
								<div className="flex h-8 w-8 shrink-0 items-center justify-center rounded-lg border border-border bg-muted text-foreground">
									<Calendar className="h-4 w-4 text-muted-foreground" />
								</div>
								<div className="flex min-w-0 flex-col gap-1.5">
									<div className="flex flex-wrap items-center gap-2">
										<span className="text-xs font-bold text-foreground">
											{s.name}
										</span>
										<Badge
											variant="secondary"
											className="gap-1 font-mono text-[10px]">
											<Box className="h-3 w-3" /> Service:{' '}
											{s.service_name || 'app'}
										</Badge>
										<Badge
											variant="outline"
											className="gap-1 font-mono text-[10px]">
											<CalendarDays className="h-3 w-3 text-muted-foreground" />
											{s.cron_expression || '0 * * * *'}
										</Badge>
									</div>
									<div className="flex max-w-xl items-center gap-2 rounded-md border border-border/60 bg-muted/50 px-2.5 py-1 font-mono text-xs text-foreground">
										<Terminal className="h-3.5 w-3.5 shrink-0 text-muted-foreground" />
										<code className="truncate">{s.command}</code>
									</div>
								</div>
							</div>

							<div className="flex shrink-0 items-center gap-2">
								<Button
									variant="outline"
									size="sm"
									onClick={() => handleRun(s.id)}
									disabled={activeRunningId === s.id}
									className="flex h-8 items-center gap-1.5 border-border text-xs font-semibold">
									<Play
										className={`h-3.5 w-3.5 ${activeRunningId === s.id ? 'animate-spin' : ''}`}
									/>{' '}
									Run Now
								</Button>

								<Button
									variant="ghost"
									size="icon"
									onClick={() => handleDelete(s.id)}
									disabled={activeDeletingId === s.id}
									className="h-8 w-8 text-muted-foreground hover:bg-destructive/10 hover:text-destructive">
									<Trash2 className="h-4 w-4" />
								</Button>
							</div>
						</div>
					))}
				</div>
			)}
		</section>
	);
}
