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
import {useAppStore} from '#/stores/app-store';
import {useOrganizationStore} from '#/stores/organization-store';

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

	const activeOrg = useOrganizationStore(state => state.activeOrg);
	const projectsList = useAppStore(state => state.projects || []);

	// Find linked server details if any
	const linkedServer = servers.find(srv => srv.id === s.server_id);

	// Find linked application details if any
	const linkedApp = useMemo(() => {
		if (!s.application_id || !Array.isArray(projectsList)) return null;
		for (const item of projectsList) {
			const proj = item as Record<string, unknown>;
			if (Array.isArray(proj.applications)) {
				const found = (
					proj.applications as Record<string, unknown>[]
				).find(a => (a.id || a.application_id) === s.application_id);
				if (found) return found;
			}
			if (Array.isArray(proj.environments)) {
				for (const envObj of proj.environments as Record<
					string,
					unknown
				>[]) {
					if (Array.isArray(envObj.applications)) {
						const found = (
							envObj.applications as Record<string, unknown>[]
						).find(a => (a.id || a.application_id) === s.application_id);
						if (found) return found;
					}
				}
			}
		}
		return null;
	}, [s.application_id, projectsList]);

	const appName =
		linkedApp?.name ||
		linkedApp?.app_name ||
		(s.application_id ? `App #${s.application_id}` : null);
	const serverName =
		linkedServer?.name || (s.server_id ? `Server #${s.server_id}` : null);

	return (
		<div
			className={cn(
				'flex flex-col justify-between gap-3.5 rounded-xl border border-border bg-card p-4 shadow-2xs transition-all hover:border-border/80',
				!isEnabled && 'opacity-70',
			)}>
			{/* Header: Status Dot + Title + Actions */}
			<div className="flex items-center justify-between gap-2">
				<div className="flex min-w-0 items-center gap-2">
					<span
						className={cn(
							'size-2 shrink-0 rounded-full',
							isEnabled ? 'bg-emerald-500' : 'bg-muted-foreground/40',
						)}
					/>
					<h3
						className="truncate text-sm font-bold text-foreground"
						title={s.name}>
						{s.name}
					</h3>
					<span className="shrink-0 font-mono text-xs text-muted-foreground/60">
						#{s.id}
					</span>
				</div>

				<div className="flex shrink-0 items-center gap-0.5">
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
						className="h-7 w-7 text-muted-foreground hover:bg-destructive/10 hover:text-destructive">
						<Trash2 className="h-3.5 w-3.5" />
					</Button>
				</div>
			</div>

			{/* Description if present */}
			{s.description && (
				<p className="-mt-1 line-clamp-1 text-xs font-medium text-muted-foreground">
					{s.description}
				</p>
			)}

			{/* Command & Target Box */}
			<div className="flex items-center justify-between gap-2 rounded-lg border border-border/40 bg-muted/30 p-2.5 font-mono text-xs dark:bg-muted/20">
				<div className="flex min-w-0 items-center gap-2">
					<Terminal className="h-3.5 w-3.5 shrink-0 text-primary" />
					<code className="truncate font-semibold text-foreground">
						{s.command}
					</code>
				</div>
				{(serverName || appName) && (
					<span className="flex shrink-0 items-center gap-1 rounded border border-border/40 bg-muted/40 px-2 py-0.5 font-sans text-xs text-muted-foreground">
						{serverName ? (
							<Server className="h-3 w-3 text-muted-foreground" />
						) : (
							<Cpu className="h-3 w-3 text-muted-foreground" />
						)}
						<span className="max-w-[110px] truncate">
							{serverName || appName}
						</span>
					</span>
				)}
			</div>

			{/* Footer */}
			<div className="flex items-center justify-between pt-0.5">
				<div className="flex items-center gap-2">
					<span className="rounded border border-border/40 bg-muted/40 px-2 py-0.5 font-mono text-xs font-medium text-muted-foreground">
						{s.cron_expression}
					</span>
					<span className="font-mono text-[10px] font-bold text-muted-foreground/60 uppercase">
						{s.shell_type || 'bash'}
					</span>
				</div>

				<Button
					variant="outline"
					size="sm"
					onClick={() => onToggle(s)}
					className="h-6 border-border/40 bg-muted/20 px-2.5 text-xs font-semibold dark:bg-muted/15">
					{isEnabled ? (
						<>
							<Pause className="mr-1 h-3 w-3 text-muted-foreground" />
							Pause
						</>
					) : (
						<>
							<Play className="mr-1 h-3 w-3 text-muted-foreground" />
							Resume
						</>
					)}
				</Button>
			</div>
		</div>
	);
}
