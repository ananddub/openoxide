import {SectionHeader} from '#/components/settings/list-card';
import {Button} from '#/components/ui/button';
import {Plus} from 'lucide-react';

interface SchedulesHeaderProps {
	onOpenCreate: () => void;
	onRefresh: () => void;
	isRefreshing: boolean;
	disabled?: boolean;
}

export function SchedulesHeader({
	onOpenCreate,
	disabled = false,
}: SchedulesHeaderProps) {
	return (
		<SectionHeader
			title="Schedules"
			description="Automated tasks, cron jobs, database backups and maintenance scripts."
			action={
				<Button
					onClick={onOpenCreate}
					disabled={disabled}
					className="h-9 gap-1.5">
					<Plus className="size-3.5" />
					Create Schedule
				</Button>
			}
		/>
	);
}
