import {GeneralSection} from './advanced-tab/general-section';
import {PortsSection} from './advanced-tab/ports-section';
import {VolumesSection} from './advanced-tab/volumes-section';
import {RedirectsSection} from './advanced-tab/redirects-section';
import {ResourceSection} from './advanced-tab/resource-section';
import {DangerSection} from './advanced-tab/danger-section';
import type {AdvancedTabProps} from './advanced-tab/types';
import {SectionHeader} from '#/components/settings/list-card';

export function AdvancedTab({app, onUpdated}: AdvancedTabProps) {
	return (
		<div className="flex w-full flex-col gap-6 pb-6">
			<SectionHeader
				title="Advanced"
				description="Runtime command, ports, volumes, redirects, replicas and other low-level settings for this application."
			/>
			<div className="overflow-hidden rounded-xl border border-border bg-card">
				<GeneralSection app={app} onUpdated={onUpdated} />
				<PortsSection app={app} />
				<VolumesSection app={app} />
				<RedirectsSection app={app} />
				<ResourceSection app={app} onUpdated={onUpdated} />
				<DangerSection app={app} />
			</div>
		</div>
	);
}
