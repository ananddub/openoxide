import {
	Play,
	Pause,
	Trash2,
	Edit,
	CalendarDays,
	Terminal,
	Cpu,
	Server,
	ServerCrash,
} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Card, CardContent, CardDescription, CardHeader, CardTitle} from '#/components/ui/card';
import {cn} from '#/api/utils';
import type {Schedule} from '#/hooks/use-schedules';

interface ScheduleCardProps {
	schedule: Schedule;
	onEdit: (s: Schedule) => void;
	onDelete: (id: number) => void;
	onToggle: (s: Schedule) => void;
	onRun: (id: number) => void;
	servers: any[];
}

export function ScheduleCard({
	schedule,
	onEdit,
	onDelete,
	onToggle,
	onRun,
	servers,
}: ScheduleCardProps) {
	const s = schedule;
	const isEnabled = s.enabled === 1;

	// Find linked server details if any
	const linkedServer = servers.find(srv => srv.id === s.server_id);

	return (
		<Card className={cn(
			'relative border-border/80 bg-card/30 hover:bg-card/65 transition-all duration-200 shadow-sm overflow-hidden group',
			!isEnabled && 'opacity-70'
		)}>
			<CardHeader className="pb-3">
				<div className="flex items-start justify-between gap-4">
					<div className="space-y-1 min-w-0">
						<div className="flex items-center gap-2.5 flex-wrap">
							<CardTitle className="text-base font-bold tracking-tight text-foreground truncate max-w-[200px]">
								{s.name}
							</CardTitle>
							<span className="text-[10px] font-mono bg-muted/65 text-muted-foreground px-1.5 py-0.5 rounded border border-border/40">
								#{s.id}
							</span>
							<span className={cn(
								'text-[9px] font-extrabold tracking-wider uppercase px-2 py-0.5 rounded-full border',
								isEnabled 
									? 'bg-emerald-500/5 text-emerald-500 border-emerald-500/20' 
									: 'bg-zinc-500/5 text-zinc-400 border-zinc-500/20'
							)}>
								{isEnabled ? 'Active' : 'Paused'}
							</span>
						</div>
						{s.description && (
							<CardDescription className="text-xs text-muted-foreground line-clamp-1 leading-relaxed mt-1">
								{s.description}
							</CardDescription>
						)}
					</div>

					<div className="flex items-center gap-1 shrink-0">
						<Button
							variant="ghost"
							size="icon"
							onClick={() => onRun(s.id!)}
							disabled={!isEnabled}
							title="Trigger manual run"
							className="size-8 rounded-lg hover:bg-primary/5 hover:text-primary text-muted-foreground/60">
							<Play className="size-4.5" />
						</Button>
						<Button
							variant="ghost"
							size="icon"
							onClick={() => onEdit(s)}
							title="Edit schedule"
							className="size-8 rounded-lg hover:bg-muted text-muted-foreground/60">
							<Edit className="size-4.5" />
						</Button>
						<Button
							variant="ghost"
							size="icon"
							onClick={() => onDelete(s.id!)}
							title="Delete schedule"
							className="size-8 rounded-lg hover:bg-rose-500/5 hover:text-rose-500 text-muted-foreground/60">
							<Trash2 className="size-4.5" />
						</Button>
					</div>
				</div>
			</CardHeader>

			<CardContent className="space-y-4">
				{/* Cron & Target Grid */}
				<div className="grid grid-cols-2 gap-3 text-xs border-t border-border/20 pt-3">
					<div className="space-y-1">
						<span className="text-[10px] text-muted-foreground font-semibold uppercase tracking-wider block">Cron Expression</span>
						<span className="font-mono text-zinc-100 bg-zinc-900/60 border border-zinc-800/80 px-2 py-0.5 rounded flex items-center gap-1.5 w-fit">
							<CalendarDays className="size-3.5 text-primary" />
							{s.cron_expression}
						</span>
					</div>
					<div className="space-y-1">
						<span className="text-[10px] text-muted-foreground font-semibold uppercase tracking-wider block">Target Destination</span>
						<span className="font-medium text-foreground flex items-center gap-1.5">
							{linkedServer ? (
								<>
									<Server className="size-3.5 text-zinc-400" />
									<span className="truncate max-w-[120px]">{linkedServer.name}</span>
								</>
							) : s.application_id ? (
								<>
									<Cpu className="size-3.5 text-blue-400" />
									<span>App #{s.application_id}</span>
								</>
							) : (
								<>
									<ServerCrash className="size-3.5 text-zinc-500" />
									<span className="text-muted-foreground">None</span>
								</>
							)}
						</span>
					</div>
				</div>

				{/* Script Command Preview */}
				<div className="space-y-1.5">
					<span className="text-[10px] text-muted-foreground font-semibold uppercase tracking-wider block">Command</span>
					<div className="relative font-mono text-[10.5px] bg-[#0c0c0e] border border-zinc-800/80 rounded-lg p-2.5 flex items-center gap-2 overflow-x-auto text-zinc-200">
						<Terminal className="size-3.5 text-zinc-500 shrink-0" />
						<code className="whitespace-nowrap truncate">{s.command}</code>
					</div>
				</div>

				{/* Pause/Resume Switcher Footer */}
				<div className="flex justify-between items-center border-t border-border/10 pt-3.5">
					<span className="text-[10px] text-muted-foreground/80 font-medium">
						Shell: {s.shell_type || 'bash'}
					</span>
					<Button
						variant="outline"
						size="sm"
						onClick={() => onToggle(s)}
						className={cn(
							'h-7 text-xs font-semibold px-2.5 rounded-lg border-border',
							isEnabled 
								? 'bg-zinc-900/40 text-amber-500 hover:text-amber-600 hover:bg-zinc-900/60' 
								: 'bg-primary/5 text-primary hover:bg-primary/10'
						)}>
						{isEnabled ? (
							<>
								<Pause className="size-3 mr-1" />
								Pause Schedule
							</>
						) : (
							<>
								<Play className="size-3 mr-1" />
								Resume Schedule
							</>
						)}
					</Button>
				</div>
			</CardContent>
		</Card>
	);
}
