import type {ComposeAdvancedTabProps} from './compose-advanced-tab/types';
import {CommandSection} from './compose-advanced-tab/command-section';
import {VolumesSection} from './compose-advanced-tab/volumes-section';
import {PatchesSection} from './compose-advanced-tab/patches-section';
import {WebhookSection} from './compose-advanced-tab/webhook-section';
import {DangerSection} from './compose-advanced-tab/danger-section';

export function ComposeAdvancedTab({
	compose,
	projectId,
	onUpdated,
}: ComposeAdvancedTabProps) {
	if (!compose?.id) return null;
	return (
		<div className="flex flex-col gap-6 pb-6">
			<CommandSection compose={compose} onUpdated={onUpdated} />
			<VolumesSection compose={compose} />
			<PatchesSection compose={compose} />
			<WebhookSection compose={compose} />
			<DangerSection compose={compose} projectId={projectId} />
		</div>
	);
}
