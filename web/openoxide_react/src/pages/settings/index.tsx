import {createFileRoute} from '@tanstack/react-router';
import {Settings, Building2, Users, Shield} from 'lucide-react';
import {useOrganizationStore} from '#/stores/organization-store';
import {Badge} from '#/components/ui/badge';
import {Button} from '#/components/ui/button';

export const Route = createFileRoute('/_app/settings')({
	component: SettingsIndexPage,
});

function SettingsIndexPage() {
	const activeOrg = useOrganizationStore((state) => state.activeOrg);

	return (
		<div className="p-6 flex flex-col gap-6 max-w-7xl mx-auto w-full animate-in fade-in duration-200">
			<div className="flex flex-col gap-1">
				<h1 className="text-2xl font-bold text-foreground tracking-tight">Organization Settings</h1>
				<p className="text-xs text-muted-foreground">
					Manage workspace preferences, team organization, and security policies
				</p>
			</div>

			<div className="p-5 border rounded-xl bg-card flex flex-col gap-4">
				<div className="flex items-center justify-between">
					<div className="flex items-center gap-3">
						<div className="size-10 rounded-xl bg-primary/10 flex items-center justify-center text-primary">
							<Building2 className="size-6" />
						</div>
						<div>
							<h3 className="text-sm font-bold text-foreground">{activeOrg?.name || 'Default Organization'}</h3>
							<p className="text-[10px] text-muted-foreground">Organization ID: #{activeOrg?.id || 1}</p>
						</div>
					</div>
					<Badge variant="default" className="text-[10px]">ACTIVE WORKSPACE</Badge>
				</div>
			</div>
		</div>
	);
}
