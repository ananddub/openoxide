import {HostConfigCard} from '#/components/settings/server/host-config-card';
import {DockerDiskCard} from '#/components/settings/server/docker-disk-card';
import {HealthRail} from '#/components/settings/server/health-rail';

// Rendered as the "Web Server" tab of pages/infra.tsx — no standalone route.

export function WebServerSettingsPage() {
	return (
		<div className="flex w-full flex-col gap-6">
			{/* ── Content grid + right rail ── */}
			<div className="grid grid-cols-1 items-start gap-6 lg:grid-cols-[minmax(0,1fr)_340px]">
				<div className="flex min-w-0 flex-col gap-6">
					<HostConfigCard />
					<DockerDiskCard />
				</div>
				<HealthRail />
			</div>
		</div>
	);
}
