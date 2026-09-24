import {HostConfigCard} from '#/components/settings/server/host-config-card';
import {DockerDiskCard} from '#/components/settings/server/docker-disk-card';

// Rendered as the "Web Server" tab of pages/infra.tsx — no standalone route.

export function WebServerSettingsPage() {
	return (
		<div className="flex w-full flex-col gap-5">
			<HostConfigCard />
			<DockerDiskCard />
		</div>
	);
}
