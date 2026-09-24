import {
	Globe,
	Clock,
	Database,
	Terminal,
	Rocket,
	SquareTerminal,
	Trash2,
	Pencil,
	type LucideIcon,
} from 'lucide-react';
import type {ComposeService} from './compose-visualizer';

export interface CanvasContextMenuProps {
	x: number;
	y: number;
	targetType: 'service' | 'domain' | 'schedule' | 'backup';
	targetData: any;
	onClose: () => void;
	onAddDomain?: (service: ComposeService) => void;
	onAddSchedule?: (service: ComposeService) => void;
	onAddBackup?: (service: ComposeService) => void;
	onOpenTerminal?: (service: ComposeService) => void;
	onViewLogs?: (service: ComposeService) => void;
	onViewDeployLogs?: (service: ComposeService) => void;
	onEditDomain?: (domainData: any) => void;
	onDeleteDomain?: (domainData: any) => void;
	onEditSchedule?: (scheduleData: any) => void;
	onDeleteSchedule?: (scheduleData: any) => void;
	onEditBackup?: (backupData: any) => void;
	onDeleteBackup?: (backupData: any) => void;
}

export interface MenuItem {
	icon: LucideIcon;
	label: string;
	color: string;
	hover: string;
	weight?: 'medium' | 'semibold' | 'bold';
	dividerBefore?: boolean;
	action?: (data: any) => void;
}

const TONE = {
	emerald: {c: 'text-foreground', h: 'hover:bg-muted'},
	emeraldBold: {c: 'text-foreground', h: 'hover:bg-muted'},
	purple: {c: 'text-foreground', h: 'hover:bg-muted'},
	sky: {c: 'text-foreground', h: 'hover:bg-muted'},
	amber: {c: 'text-foreground', h: 'hover:bg-muted'},
	primary: {c: 'text-foreground', h: 'hover:bg-muted'},
	destructive: {c: 'text-destructive', h: 'hover:bg-destructive/10'},
};

const item = (
	icon: LucideIcon,
	label: string,
	tone: keyof typeof TONE,
	action?: (data: any) => void,
	extra?: Partial<MenuItem>,
): MenuItem => ({
	icon,
	label,
	color: TONE[tone].c,
	hover: TONE[tone].h,
	action,
	...extra,
});

export function buildMenu(
	targetType: CanvasContextMenuProps['targetType'],
	p: CanvasContextMenuProps,
): MenuItem[] {
	if (targetType === 'service') {
		const items: MenuItem[] = [];
		if (p.onAddDomain)
			items.push(item(Globe, 'Add Domain', 'emerald', p.onAddDomain));
		if (p.onAddSchedule)
			items.push(item(Clock, 'Add Schedule', 'purple', p.onAddSchedule));
		items.push(item(Database, 'Add Volume Backup', 'sky', p.onAddBackup));
		items.push(
			item(SquareTerminal, 'Terminal', 'emeraldBold', p.onOpenTerminal, {
				weight: 'bold',
				dividerBefore: true,
			}),
		);
		items.push(
			item(Terminal, 'Live Container Logs', 'amber', p.onViewLogs),
		);
		if (p.onViewDeployLogs)
			items.push(
				item(Rocket, 'Deployment Logs', 'primary', p.onViewDeployLogs),
			);
		return items;
	}
	if (targetType === 'domain') {
		return [
			item(Pencil, 'Edit Domain', 'emeraldBold', p.onEditDomain, {
				weight: 'semibold',
			}),
			item(Trash2, 'Delete Domain', 'destructive', p.onDeleteDomain, {
				weight: 'bold',
			}),
		];
	}
	if (targetType === 'schedule') {
		return [
			item(Pencil, 'Edit Schedule', 'purple', p.onEditSchedule, {
				weight: 'semibold',
			}),
			item(Trash2, 'Delete Schedule', 'destructive', p.onDeleteSchedule, {
				weight: 'bold',
			}),
		];
	}
	return [
		item(Pencil, 'Edit Backup', 'sky', p.onEditBackup, {
			weight: 'semibold',
		}),
		item(Trash2, 'Delete Backup', 'destructive', p.onDeleteBackup, {
			weight: 'bold',
		}),
	];
}
