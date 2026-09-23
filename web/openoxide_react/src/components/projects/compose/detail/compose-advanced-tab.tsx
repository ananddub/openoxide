import type {ComposeAdvancedTabProps} from './compose-advanced-tab/types';
import {CommandSection} from './compose-advanced-tab/command-section';
import {VolumesSection} from './compose-advanced-tab/volumes-section';
import {DangerSection} from './compose-advanced-tab/danger-section';
import {SectionHeader} from '#/components/settings/list-card';

export function ComposeAdvancedTab({
	compose,
	projectId,
	onUpdated,
}: ComposeAdvancedTabProps) {
	if (!compose?.id) return null;
	return (
		<div className="flex w-full flex-col gap-6 pb-6">
			<SectionHeader
				title="Advanced"
				description="Start command, volumes and other low-level settings for this compose stack."
			/>
			<div className="overflow-hidden rounded-xl border border-border bg-card">
				<CommandSection compose={compose} onUpdated={onUpdated} />
				<VolumesSection compose={compose} />
				<DangerSection compose={compose} projectId={projectId} />
			</div>
		</div>
	);
}
