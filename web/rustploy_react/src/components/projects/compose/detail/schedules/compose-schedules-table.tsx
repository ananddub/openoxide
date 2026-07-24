import {useState} from 'react';
import {Calendar, Play, Trash2, RefreshCw, Box, Terminal} from 'lucide-react';
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
		<section className="bg-card border border-border rounded-xl p-5 flex flex-col gap-4 shadow-sm">
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
							className="border border-border/80 rounded-lg p-4 bg-muted/20 hover:bg-muted/40 transition-colors flex items-center justify-between gap-4 flex-wrap"
						>
							<div className="flex items-start gap-3">
								<div className="w-8 h-8 rounded-lg bg-secondary flex items-center justify-center text-primary shrink-0 border border-border/40">
									<Calendar className="w-4 h-4" />
								</div>
								<div className="flex flex-col gap-1">
									<div className="flex items-center gap-2 flex-wrap">
										<span className="text-xs font-bold text-foreground">{s.name}</span>
										<Badge variant="outline" className="text-[10px] font-mono">
											<Box className="w-3 h-3 mr-1 text-primary" /> Service: {s.service_name || 'app'}
										</Badge>
										<Badge variant="secondary" className="text-[10px] font-mono">
											{s.cron_expression || '0 * * * *'}
										</Badge>
									</div>
									<span className="text-xs text-muted-foreground font-mono flex items-center gap-1">
										<Terminal className="w-3 h-3" /> {s.command}
									</span>
								</div>
							</div>

							<div className="flex items-center gap-2">
								<Button
									variant="outline"
									size="sm"
									onClick={() => handleRun(s.id)}
									disabled={activeRunningId === s.id}
									className="h-8 text-xs font-semibold border-border hover:bg-muted flex items-center gap-1.5"
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
