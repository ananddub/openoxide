import {useMemo} from 'react';
import {
	Play,
	Pause,
	Trash2,
	Edit,
	Terminal,
	Server,
	Cpu,
} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {cn} from '#/api/utils';
import type {Schedule} from '#/hooks/use-schedules';
import {$api} from '#/api/query';

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

	// Query projects to resolve application names
	const {data: projectsList} = $api.useQuery('get', '/projects') as any;

	// Find linked server details if any
	const linkedServer = servers.find(srv => srv.id === s.server_id);

	// Find linked application details if any
	const linkedApp = useMemo(() => {
		if (!s.application_id || !Array.isArray(projectsList)) return null;
		for (const proj of projectsList) {
			if (proj.applications) {
				const found = proj.applications.find((a: any) => (a.id || a.application_id) === s.application_id);
				if (found) return found;
			}
			if (proj.environments) {
				for (const env of proj.environments) {
					if (env.applications) {
						const found = env.applications.find((a: any) => (a.id || a.application_id) === s.application_id);
						if (found) return found;
					}
				}
			}
		}
		return null;
	}, [s.application_id, projectsList]);

	const appName = linkedApp?.name || linkedApp?.app_name || (s.application_id ? `App #${s.application_id}` : null);
	const serverName = linkedServer?.name || (s.server_id ? `Server #${s.server_id}` : null);

	return (
		<div className={cn(
			'bg-card border border-border/80 hover:border-border rounded-xl p-3.5 flex flex-col justify-between gap-3 transition-colors',
			!isEnabled && 'opacity-65'
		)}>
			{/* Header: Status Dot + Title + Actions */}
			<div className="flex items-center justify-between gap-2">
				<div className="flex items-center gap-2 min-w-0">
					<span
						className={cn(
							'h-2 w-2 rounded-full shrink-0',
							isEnabled ? 'bg-emerald-500' : 'bg-muted-foreground/40'
						)}
					/>
					<h3 className="text-sm font-semibold text-foreground truncate" title={s.name}>
						{s.name}
					</h3>
					<span className="text-xs font-mono text-muted-foreground/60 shrink-0">
						#{s.id}
					</span>
				</div>

				<div className="flex items-center gap-0.5 shrink-0">
					<Button
						variant="ghost"
						size="icon"
						onClick={() => onRun(s.id!)}
						disabled={!isEnabled}
						title="Trigger run"
						className="h-7 w-7 text-muted-foreground hover:text-foreground">
						<Play className="h-3.5 w-3.5" />
					</Button>
					<Button
						variant="ghost"
						size="icon"
						onClick={() => onEdit(s)}
						title="Edit"
						className="h-7 w-7 text-muted-foreground hover:text-foreground">
						<Edit className="h-3.5 w-3.5" />
					</Button>
					<Button
						variant="ghost"
						size="icon"
						onClick={() => onDelete(s.id!)}
						title="Delete"
						className="h-7 w-7 text-muted-foreground hover:text-destructive hover:bg-destructive/10">
						<Trash2 className="h-3.5 w-3.5" />
					</Button>
				</div>
			</div>

			{/* Description if present */}
			{s.description && (
				<p className="text-xs text-muted-foreground line-clamp-1 -mt-1">
					{s.description}
				</p>
			)}

			{/* Command & Target Box */}
			<div className="flex items-center justify-between gap-2 bg-muted/30 border border-border/50 rounded-lg p-2 text-xs font-mono">
				<div className="flex items-center gap-2 min-w-0">
					<Terminal className="h-3.5 w-3.5 text-muted-foreground shrink-0" />
					<code className="truncate text-foreground">{s.command}</code>
				</div>
				{(serverName || appName) && (
					<span className="text-xs font-sans text-muted-foreground bg-background border border-border/60 px-2 py-0.5 rounded shrink-0 flex items-center gap-1">
						{serverName ? <Server className="h-3 w-3" /> : <Cpu className="h-3 w-3" />}
						<span className="truncate max-w-[110px]">{serverName || appName}</span>
					</span>
				)}
			</div>

			{/* Footer */}
			<div className="flex items-center justify-between pt-0.5">
				<div className="flex items-center gap-2">
					<span className="text-xs font-mono text-muted-foreground bg-muted/40 border border-border/40 px-2 py-0.5 rounded">
						{s.cron_expression}
					</span>
					<span className="text-[10px] text-muted-foreground/60 uppercase font-mono">
						{s.shell_type || 'bash'}
					</span>
				</div>

				<Button
					variant="outline"
					size="sm"
					onClick={() => onToggle(s)}
					className="h-6 text-xs font-medium px-2 border-border/80">
					{isEnabled ? (
						<>
							<Pause className="h-3 w-3 mr-1 text-muted-foreground" />
							Pause
						</>
					) : (
						<>
							<Play className="h-3 w-3 mr-1 text-muted-foreground" />
							Resume
						</>
					)}
				</Button>
			</div>
		</div>
	);
}
