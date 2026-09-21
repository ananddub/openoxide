import {GeneralSection} from './advanced-tab/general-section';
import {PortsSection} from './advanced-tab/ports-section';
import {VolumesSection} from './advanced-tab/volumes-section';
import {RedirectsSection} from './advanced-tab/redirects-section';
import {SecuritySection} from './advanced-tab/security-section';
import {WebhookSection} from './advanced-tab/webhook-section';
import {ResourceSection} from './advanced-tab/resource-section';
import {DangerSection} from './advanced-tab/danger-section';
import type {AdvancedTabProps} from './advanced-tab/types';

export function AdvancedTab({app, onUpdated}: AdvancedTabProps) {
	return (
		<div className="flex flex-col gap-6 pb-6">
			<GeneralSection app={app} onUpdated={onUpdated} />
			<PortsSection app={app} />
			<VolumesSection app={app} />
			<RedirectsSection app={app} />
			<SecuritySection app={app} />
			<WebhookSection app={app} />
			<ResourceSection app={app} onUpdated={onUpdated} />
			<DangerSection app={app} />
		</div>
	);
}
