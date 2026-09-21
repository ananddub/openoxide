import {useState} from 'react';
import {Settings, ServerCog} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {
	Dialog,
	DialogContent,
	DialogDescription,
	DialogFooter,
	DialogHeader,
	DialogTitle,
} from '#/components/ui/dialog';
import {SwarmSettingsForm} from './swarm-settings/forms';
import {AlertBlock} from '#/components/shared/alert-block';
import {
	Tooltip,
	TooltipContent,
	TooltipProvider,
	TooltipTrigger,
} from '#/components/ui/tooltip';
import {cn} from '#/api/utils';
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
	const [open, setOpen] = useState(false);
	const menuItems = swarmSections.map(([id, label]) => ({
		id,
		label,
		description: swarmDescription(id),
	}));
	return (
		<section className="flex flex-col gap-5 rounded-xl bg-card p-6 shadow-sm">
			<div className="flex flex-wrap items-start justify-between gap-4">
				<div>
					<h3 className="flex items-center gap-2 text-base font-bold">
						<ServerCog className="size-4 text-primary" /> Cluster Settings
					</h3>
					<p className="mt-1 text-xs text-muted-foreground">
						Configure Docker Swarm service behavior.
					</p>
				</div>
				<Button
					variant="secondary"
					onClick={() => setOpen(true)}
					className="h-9 gap-1.5 text-xs">
					<Settings className="size-4" /> Swarm Settings
				</Button>
			</div>
			<div className="rounded-lg bg-blue-500/5 p-3 text-xs text-muted-foreground">
				Redeploy after changing cluster settings to apply them.
			</div>
			<Dialog open={open} onOpenChange={setOpen}>
				<DialogContent className="flex h-[min(720px,85vh)] max-h-[85vh] flex-col sm:max-w-6xl">
					<DialogHeader>
						<DialogTitle>Swarm Settings</DialogTitle>
						<DialogDescription>
							Configure swarm settings for your service.
						</DialogDescription>
					</DialogHeader>
					<div className="mb-2">
						<AlertBlock type="info">
							Changing settings such as placements may cause the
							logs/monitoring, backups and other features to be
							unavailable.
						</AlertBlock>
					</div>
					<div className="flex min-h-0 flex-1 gap-4 py-2">
						<nav className="flex w-64 min-w-64 shrink-0 flex-col overflow-y-auto border-r pr-4">
							<div className="flex w-full flex-col gap-1">
								<TooltipProvider>
									{menuItems.map(item => (
										<Tooltip key={item.id}>
											<TooltipTrigger
												render={
													<button
														type="button"
														onClick={() => state.setActive(item.id)}
														className={cn(
															'!flex min-h-14 !w-full min-w-0 flex-col self-stretch rounded-md px-3 py-2 text-left text-sm transition-colors',
															state.active === item.id
																? 'bg-white text-foreground shadow-sm dark:bg-white/10'
																: 'text-foreground hover:bg-muted',
														)}>
														<div className="w-full truncate font-medium">
															{item.label}
														</div>
														<div className="w-full truncate text-xs text-muted-foreground">
															{item.description}
														</div>
													</button>
												}
											/>
											<TooltipContent side="right" className="max-w-xs">
												<p className="text-xs">{item.description}</p>
											</TooltipContent>
										</Tooltip>
									))}
								</TooltipProvider>
							</div>
						</nav>
						<div className="min-h-0 flex-1 overflow-y-auto p-1">
							{state.active === 'stop_grace_period_swarm' ? (
								<label className="flex max-w-2xl flex-col gap-1.5 text-sm font-medium">
									Stop Grace Period (nanoseconds)
									<span className="text-xs font-normal text-muted-foreground">
										Time to wait before forcefully stopping a task.
									</span>
									<Input
										type="number"
										value={state.grace}
										onChange={e => state.setGrace(e.target.value)}
										placeholder="30000000000"
									/>
								</label>
							) : (
								<SwarmSettingsForm
									section={state.active}
									value={state.values[state.active] || {}}
									set={state.set}
								/>
							)}
						</div>
					</div>
					<DialogFooter className="flex-row justify-end gap-2 border-t pt-4">
						<Button
							type="button"
							variant="outline"
							onClick={state.clearActive}
							disabled={state.saving}
							className="h-9 px-4 text-sm">
							Clear
						</Button>
						<Button
							onClick={async () => {
								if (await state.saveAll()) setOpen(false);
							}}
							disabled={state.saving}
							className="h-9 px-4 text-sm">
							{state.saving
								? 'Saving...'
								: `Save ${menuItems.find(item => item.id === state.active)?.label || 'Settings'}`}
						</Button>
					</DialogFooter>
				</DialogContent>
			</Dialog>
		</section>
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
