import {
	Box,
	Layers,
	MoreVertical,
	Terminal,
	Server,
	HardDrive,
	Network,
	Play,
	Square,
	RotateCw,
} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {
	DropdownMenu,
	DropdownMenuTrigger,
	DropdownMenuContent,
	DropdownMenuItem,
	DropdownMenuSeparator,
	DropdownMenuLabel,
	DropdownMenuGroup,
} from '#/components/ui/dropdown';
import type {ContainerItem} from './container-inspect-modal';
import type {InspectType} from './container-inspect-body';

type ContainerAction = 'start' | 'stop' | 'restart' | 'kill';

const VIEW_OPTIONS: {
	type: InspectType;
	label: string;
	icon: any;
	color: string;
}[] = [
	{
		type: 'logs',
		label: 'View Logs',
		icon: Terminal,
		color: 'text-amber-400',
	},
	{
		type: 'config',
		label: 'View Config',
		icon: Server,
		color: 'text-primary',
	},
	{
		type: 'mount',
		label: 'View Mounts',
		icon: HardDrive,
		color: 'text-sky-400',
	},
	{
		type: 'network',
		label: 'View Network',
		icon: Network,
		color: 'text-emerald-400',
	},
];

const LIFECYCLE_ACTIONS: {
	action: ContainerAction;
	label: string;
	icon: any;
	color: string;
}[] = [
	{action: 'start', label: 'Start', icon: Play, color: 'text-emerald-400'},
	{action: 'stop', label: 'Stop', icon: Square, color: 'text-rose-400'},
	{
		action: 'restart',
		label: 'Restart',
		icon: RotateCw,
		color: 'text-sky-400',
	},
];

export function ContainerRow({
	container: c,
	onOpenModal,
	onAction,
}: {
	container: ContainerItem;
	onOpenModal: (container: ContainerItem, type: InspectType) => void;
	onAction: (container: ContainerItem, action: ContainerAction) => void;
}) {
	return (
		<tr className="transition-colors hover:bg-muted/20">
			<td className="px-4 py-3.5">
				<div className="flex items-center gap-2.5">
					<div className="flex h-8 w-8 shrink-0 items-center justify-center rounded-lg bg-secondary text-primary">
						<Box className="h-4 w-4" />
					</div>
					<div className="flex flex-col gap-0.5">
						<span className="font-mono font-bold text-foreground">
							{c.name}
						</span>
						<span className="flex items-center gap-1 font-mono text-[10px] text-muted-foreground">
							<Layers className="h-3 w-3 text-primary" /> Service:{' '}
							<b className="text-foreground">{c.service}</b>
						</span>
					</div>
				</div>
			</td>
			<td className="px-4 py-3.5 font-mono text-muted-foreground">
				<span className="rounded bg-muted/40 px-2 py-0.5 text-foreground">
					{c.id}
				</span>
			</td>
			<td className="max-w-[180px] truncate px-4 py-3.5 font-mono text-muted-foreground">
				{c.image}
			</td>
			<td className="px-4 py-3.5">
				<div className="flex items-center gap-1.5">
					<span
						className={`h-2 w-2 rounded-full ${c.status === 'running' ? 'animate-pulse bg-emerald-500' : 'bg-rose-500'}`}
					/>
					<span
						className={`text-[11px] font-semibold capitalize ${c.status === 'running' ? 'text-emerald-400' : 'text-rose-400'}`}>
						{c.statusText || c.status}
					</span>
				</div>
			</td>
			<td className="px-4 py-3.5 text-right">
				<DropdownMenu>
					<DropdownMenuTrigger
						render={
							<Button
								variant="ghost"
								size="icon"
								className="h-8 w-8 text-muted-foreground hover:bg-muted hover:text-foreground">
								<MoreVertical className="h-4 w-4" />
							</Button>
						}
					/>
					<DropdownMenuContent
						align="end"
						className="border- w-44 bg-card shadow-md">
						<DropdownMenuGroup>
							<DropdownMenuLabel className="px-2 py-1 text-[10px] font-bold text-muted-foreground uppercase">
								View Options
							</DropdownMenuLabel>
							{VIEW_OPTIONS.map(({type, label, icon: Icon, color}) => (
								<DropdownMenuItem
									key={type}
									onClick={() => onOpenModal(c, type)}
									className="flex cursor-pointer items-center gap-2 text-xs text-foreground">
									<Icon className={`h-3.5 w-3.5 ${color}`} /> {label}
								</DropdownMenuItem>
							))}
						</DropdownMenuGroup>
						<DropdownMenuSeparator />
						<DropdownMenuGroup>
							<DropdownMenuLabel className="px-2 py-1 text-[10px] font-bold text-muted-foreground uppercase">
								Actions
							</DropdownMenuLabel>
							{LIFECYCLE_ACTIONS.map(
								({action, label, icon: Icon, color}) => (
									<DropdownMenuItem
										key={action}
										onClick={() => onAction(c, action)}
										className="flex cursor-pointer items-center gap-2 text-xs text-foreground">
										<Icon className={`h-3.5 w-3.5 ${color}`} /> {label}
									</DropdownMenuItem>
								),
							)}
							<DropdownMenuItem
								onClick={() => onAction(c, 'kill')}
								className="cursor-pointer text-xs text-rose-500 hover:text-rose-400">
								Kill
							</DropdownMenuItem>
						</DropdownMenuGroup>
					</DropdownMenuContent>
				</DropdownMenu>
			</td>
		</tr>
	);
}
