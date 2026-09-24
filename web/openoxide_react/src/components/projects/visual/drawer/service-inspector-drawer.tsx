import {useState} from 'react';
import {
	Sheet,
	SheetContent,
	SheetHeader,
	SheetTitle,
} from '#/components/ui/sheet';
import {GeistSwitch} from '#/components/shared/geist-switch';
import {Badge} from '#/components/ui/badge';
import {Button} from '#/components/ui/button';
import {
	Box,
	Database,
	Sliders,
	Activity,
	Play,
	Square,
	RotateCw,
	Terminal,
} from 'lucide-react';
import {
	InspectorOverviewTab,
	InspectorConfigTab,
	InspectorLogsTab,
} from './inspector-tabs';

interface ServiceInspectorDrawerProps {
	isOpen: boolean;
	onClose: () => void;
	node?: {
		id: number;
		type: 'app' | 'database' | 'compose' | 'destination';
		name: string;
		status?: string;
		dbType?: string;
		url?: string;
	} | null;
}

export function ServiceInspectorDrawer({
	isOpen,
	onClose,
	node,
}: ServiceInspectorDrawerProps) {
	const [activeTab, setActiveTab] = useState<
		'overview' | 'config' | 'logs'
	>('overview');

	if (!node) return null;

	const isDb = node.type === 'database';
	const isCompose = node.type === 'compose';
	const isRunning =
		node.status?.toLowerCase() === 'running' ||
		node.status?.toLowerCase() === 'active';

	return (
		<Sheet open={isOpen} onOpenChange={open => !open && onClose()}>
			<SheetContent
				side="right"
				className="border- flex w-[450px] flex-col border-l bg-card p-0 font-sans sm:w-[540px]">
				{/* Drawer Header */}
				<SheetHeader className="flex flex-col gap-2 border-b  bg-muted/20 p-5">
					<div className="flex items-center justify-between gap-3">
						<div className="flex min-w-0 items-center gap-3">
							<div className="flex h-9 w-9 shrink-0 items-center justify-center rounded-xl bg-primary/10 text-primary">
								{isDb ? (
									<Database className="h-4.5 w-4.5 text-emerald-400" />
								) : isCompose ? (
									<Box className="h-4.5 w-4.5 text-violet-400" />
								) : (
									<Box className="h-4.5 w-4.5 text-blue-400" />
								)}
							</div>
							<div className="min-w-0 flex-1">
								<SheetTitle className="truncate text-base font-bold text-foreground">
									{node.name}
								</SheetTitle>
								<div className="mt-0.5 flex items-center gap-2">
									<Badge
										variant="outline"
										className="py-0 font-mono text-[10px] uppercase">
										{node.dbType || node.type}
									</Badge>
									<span className="flex items-center gap-1 font-mono text-[11px] text-muted-foreground">
										<span
											className={`h-2 w-2 rounded-full ${
												isRunning ? 'bg-emerald-500' : 'bg-amber-500'
											}`}
										/>
										{isRunning ? 'Running' : 'Stopped'}
									</span>
								</div>
							</div>
						</div>

						{/* Quick Actions */}
						<div className="flex shrink-0 items-center gap-1.5">
							{isRunning ? (
								<Button
									variant="outline"
									size="sm"
									className="h-8 cursor-pointer gap-1 border-amber-500/30 text-xs text-amber-400 hover:bg-amber-500/10">
									<Square className="h-3.5 w-3.5 fill-current" /> Stop
								</Button>
							) : (
								<Button
									variant="outline"
									size="sm"
									className="h-8 cursor-pointer gap-1 border-emerald-500/30 text-xs text-emerald-400 hover:bg-emerald-500/10">
									<Play className="h-3.5 w-3.5 fill-current" /> Start
								</Button>
							)}

							<Button
								variant="outline"
								size="sm"
								className="h-8 cursor-pointer gap-1.5 text-xs">
								<RotateCw className="h-3.5 w-3.5" /> Restart
							</Button>
						</div>
					</div>
				</SheetHeader>

				{/* Tabs Navigation: Overview, Config, Live Logs (Geist Switch) */}
				<div className="flex min-h-0 flex-1 flex-col">
					<div className="border-b bg-muted/10 px-5 py-2.5">
						<GeistSwitch
							options={[
								{key: 'overview', label: 'Overview', icon: Activity},
								{key: 'config', label: 'Config', icon: Sliders},
								{key: 'logs', label: 'Live Logs', icon: Terminal},
							]}
							value={activeTab}
							onChange={setActiveTab}
						/>
					</div>

					{/* Tab panels */}
					{activeTab === 'overview' && (
						<InspectorOverviewTab node={node} />
					)}
					{activeTab === 'config' && <InspectorConfigTab />}
					{activeTab === 'logs' && <InspectorLogsTab node={node} />}
				</div>
			</SheetContent>
		</Sheet>
	);
}
