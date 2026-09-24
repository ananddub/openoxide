import {cn} from '#/api/utils';
import {describeCron} from '#/utils/cron';
import {IconTooltip} from '#/components/shared/icon-tooltip';

/** When a job actually runs, said out loud — "Every day at 02:00" rather than `0 2 * * *`. The raw expression stays available on hover, and an unusual shape that cannot be read out is shown as-is. */
export function CronChip({
	expression,
	className,
}: {
	expression?: string;
	className?: string;
}) {
	const raw = (expression ?? '').trim();
	const human = describeCron(raw);
	if (!raw) return null;

	return (
		<IconTooltip label={human ? `${human} · ${raw}` : raw}>
			<span
				className={cn(
					'shrink-0 cursor-default rounded-md bg-foreground/[0.05] px-1.5 py-0.5 text-[10px] font-medium text-muted-foreground',
					!human && 'font-mono',
					className,
				)}>
				{human ?? raw}
			</span>
		</IconTooltip>
	);
}
