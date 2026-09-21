import {Titlebar} from '#/components/shared/titlebar';
import {Clock} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Plus, RefreshCw} from 'lucide-react';

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
		<Titlebar
			breadcrumbs={[{label: 'Schedules'}]}
			icon={<Clock className="text-foreground/70" />}
			title="Schedules"
			description="Manage automated tasks, cron jobs, database backups, and maintenance scripts"
			actions={
				<>
					<Button
						variant="outline"
						size="sm"
						onClick={onRefresh}
						disabled={isRefreshing}>
						<RefreshCw
							className={`size-3.5 text-muted-foreground ${isRefreshing ? 'animate-spin' : ''}`}
						/>
						Refresh
					</Button>
					<Button size="sm" onClick={onOpenCreate} disabled={disabled}>
						<Plus className="size-3.5" />
						Create Schedule
					</Button>
				</>
			}
		/>
	);
}
