import {CalendarDays, Plus, RefreshCw} from 'lucide-react';
import {Button} from '#/components/ui/button';

interface SchedulesHeaderProps {
	onOpenCreate: () => void;
	onRefresh: () => void;
	isRefreshing: boolean;
	disabled?: boolean;
}

export function SchedulesHeader({
	onOpenCreate,
	onRefresh,
	isRefreshing,
	disabled = false,
}: SchedulesHeaderProps) {
	return (
		<div className="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-3">
			<div className="flex items-center gap-3">
				<div className="w-9 h-9 rounded-lg bg-primary/10 border border-primary/20 flex items-center justify-center shrink-0">
					<CalendarDays className="w-4 h-4 text-primary" />
				</div>
				<div>
					<h1 className="text-base font-semibold text-foreground leading-none">Schedules</h1>
					<p className="text-xs text-muted-foreground mt-1">
						Manage automated tasks, cron jobs, database backups, and maintenance scripts
					</p>
				</div>
			</div>

			<div className="flex items-center gap-2 sm:ml-auto">
				<Button
					variant="outline"
					size="sm"
					onClick={onRefresh}
					disabled={isRefreshing}
					className="h-8 text-xs gap-1.5 cursor-pointer"
				>
					<RefreshCw className={`w-3.5 h-3.5 ${isRefreshing ? 'animate-spin' : ''}`} />
					Refresh
				</Button>
				<Button
					size="sm"
					onClick={onOpenCreate}
					disabled={disabled}
					className="h-8 text-xs gap-1.5 cursor-pointer"
				>
					<Plus className="w-3.5 h-3.5" />
					Create Schedule
				</Button>
			</div>
		</div>
	);
}
