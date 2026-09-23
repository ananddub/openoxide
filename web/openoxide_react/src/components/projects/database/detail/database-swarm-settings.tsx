import {useState} from 'react';
import {ChevronRight, ServerCog} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {SettingsSheet} from '#/components/shared/settings-sheet';
import {SheetGroup, SheetNote} from '#/components/shared/field';
import {Duration} from './swarm-settings/field';
import {SwarmSettingsForm} from './swarm-settings/forms';
import {
	swarmSections,
	useDatabaseSwarmSettingsState,
} from './swarm-settings/state';

export function DatabaseSwarmSettings({
	database,
	onUpdated,
}: {
	database: any;
	onUpdated: () => void;
}) {
	const state = useDatabaseSwarmSettingsState(database, onUpdated);
	const [view, setView] = useState<'menu' | 'form'>('menu');

	const openSection = (id: string) => {
		state.setActive(id as any);
		setView('form');
	};
	const backToMenu = () => setView('menu');

	const activeLabel =
		swarmSections.find(([id]) => id === state.active)?.[1] || 'Settings';

	return (
		<SettingsSheet
			title={view === 'form' ? activeLabel : 'Cluster Settings'}
			description={
				view === 'form'
					? swarmDescription(state.active)
					: 'Docker Swarm service behavior'
			}
			icon={<ServerCog className="size-4" />}
			onOpenChange={next => {
				if (!next) setView('menu');
			}}
			footerLeft={
				view === 'form' ? (
					<Button
						type="button"
						variant="ghost"
						onClick={backToMenu}
						className="h-9 px-4 text-sm">
						Back
					</Button>
				) : undefined
			}
			footer={
				view === 'form' ? (
					<div className="flex items-center gap-3">
						<Button
							type="button"
							variant="ghost"
							onClick={state.clearActive}
							disabled={state.saving}
							className="h-9 px-4 text-sm">
							Clear
						</Button>
						<Button
							onClick={state.saveAll}
							disabled={state.saving}
							className="h-9 px-4 text-sm">
							{state.saving ? 'Saving...' : `Save ${activeLabel}`}
						</Button>
					</div>
				) : undefined
			}>
			{view === 'menu' ? (
				<SheetGroup
					title="Service behavior"
					description="Health checks, restart policy, placement and update strategy.">
					<SheetNote tone="warn">
						Changing placements may make logs/monitoring, backups and other
						features unavailable. Redeploy after editing to apply.
					</SheetNote>
					<div className="flex flex-col gap-2">
						{swarmSections.map(([id, label]) => (
							<button
								key={id}
								type="button"
								onClick={() => openSection(id)}
								className="group flex w-full items-center gap-3 rounded-xl border border-border/50 px-4 py-3.5 text-start transition-all duration-150 hover:border-border hover:bg-muted/30">
								<span className="min-w-0 flex-1">
									<span className="block text-sm font-medium text-foreground">
										{label}
									</span>
									<span className="block truncate text-[13px] text-muted-foreground">
										{swarmDescription(id)}
									</span>
								</span>
								<ChevronRight className="size-4 shrink-0 text-muted-foreground/50 transition-transform group-hover:translate-x-0.5" />
							</button>
						))}
					</div>
				</SheetGroup>
			) : (
				<div className="flex flex-col gap-5">
					{state.active === 'stop_grace_period_swarm' ? (
						<Duration
							label="Stop Grace Period (nanoseconds)"
							value={state.grace}
							set={v => state.setGrace(v == null ? '' : String(v))}
						/>
					) : (
						<SwarmSettingsForm
							section={state.active}
							value={state.values[state.active] || {}}
							set={state.set}
						/>
					)}
				</div>
			)}
		</SettingsSheet>
	);
}

function swarmDescription(key: string) {
	const descriptions: Record<string, string> = {
		health_check_swarm: 'Configure health check settings',
		restart_policy_swarm: 'Configure restart policy',
		placement_swarm: 'Configure placement constraints',
		update_config_swarm: 'Configure update strategy',
		rollback_config_swarm: 'Configure rollback strategy',
		mode_swarm: 'Configure service mode',
		network_swarm: 'Configure network attachments',
		labels_swarm: 'Configure service labels',
		stop_grace_period_swarm: 'Configure stop grace period',
		endpoint_spec_swarm: 'Configure endpoint specification',
	};
	return descriptions[key] || '';
}
