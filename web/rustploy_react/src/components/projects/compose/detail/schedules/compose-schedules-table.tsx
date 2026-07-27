import {useState} from 'react';
import {Calendar, Play, Trash2, RefreshCw, Box, Terminal, CalendarDays} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Badge} from '#/components/ui/badge';

interface ComposeSchedulesTableProps {
	schedules: any[];
	isLoading: boolean;
	onRun: (id: number) => Promise<void>;
	onDelete: (id: number) => Promise<void>;
}

export function ComposeSchedulesTable({schedules, isLoading, onRun, onDelete}: ComposeSchedulesTableProps) {
	const [activeRunningId, setActiveRunningId] = useState<number | null>(null);
	const [activeDeletingId, setActiveDeletingId] = useState<number | null>(null);
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
		<section className="bg-card border border-border rounded-xl p-5 flex flex-col gap-4">
			{isLoading && safeSchedules.length === 0 ? (
				<div className="flex items-center justify-center h-48 text-xs text-muted-foreground gap-2">
					<RefreshCw className="w-4 h-4 animate-spin text-primary" /> Loading schedules...
				</div>
			) : safeSchedules.length === 0 ? (
				<div className="flex flex-col items-center justify-center h-48 text-muted-foreground gap-2 text-xs">
					<Calendar className="w-8 h-8 opacity-40" />
					<p>No compose schedules configured.</p>
				</div>
			) : (
				<div className="flex flex-col gap-3">
					{safeSchedules.map((s: any) => (
						<div
							key={s.id}
							className="border border-border rounded-xl p-4 bg-card flex items-center justify-between gap-4 flex-wrap"
						>
							<div className="flex items-start gap-3 min-w-0">
								<div className="w-8 h-8 rounded-lg bg-muted flex items-center justify-center text-foreground shrink-0 border border-border">
									<Calendar className="w-4 h-4 text-muted-foreground" />
								</div>
								<div className="flex flex-col gap-1.5 min-w-0">
									<div className="flex items-center gap-2 flex-wrap">
										<span className="text-xs font-bold text-foreground">{s.name}</span>
										<Badge variant="secondary" className="text-[10px] font-mono gap-1">
											<Box className="w-3 h-3" /> Service: {s.service_name || 'app'}
										</Badge>
										<Badge variant="outline" className="text-[10px] font-mono gap-1">
											<CalendarDays className="w-3 h-3 text-muted-foreground" />
											{s.cron_expression || '0 * * * *'}
										</Badge>
									</div>
									<div className="font-mono text-xs bg-muted/50 border border-border/60 rounded-md px-2.5 py-1 text-foreground flex items-center gap-2 max-w-xl">
										<Terminal className="w-3.5 h-3.5 text-muted-foreground shrink-0" />
										<code className="truncate">{s.command}</code>
									</div>
								</div>
							</div>

							<div className="flex items-center gap-2 shrink-0">
								<Button
									variant="outline"
									size="sm"
									onClick={() => handleRun(s.id)}
									disabled={activeRunningId === s.id}
									className="h-8 text-xs font-semibold border-border flex items-center gap-1.5"
								>
									<Play className={`w-3.5 h-3.5 ${activeRunningId === s.id ? 'animate-spin' : ''}`} /> Run Now
								</Button>

								<Button
									variant="ghost"
									size="icon"
									onClick={() => handleDelete(s.id)}
									disabled={activeDeletingId === s.id}
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
