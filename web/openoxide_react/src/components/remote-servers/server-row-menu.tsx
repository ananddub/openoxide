import {
	ArrowLeftRight,
	MoreVertical,
	Pencil,
	Plug,
	Power,
	ShieldCheck,
	Shield,
	Terminal,
	Trash2,
} from 'lucide-react';
import {
	DropdownMenu,
	DropdownMenuContent,
	DropdownMenuItem,
	DropdownMenuSeparator,
	DropdownMenuTrigger,
} from '#/components/ui/dropdown';
import type {RemoteServerResponse} from '#/types/api-helpers';
import type {ServerRowActions} from './server-row';

/** 3-dot action menu for a server row. */
export function ServerRowMenu({
	item,
	isServerActive,
	actions,
}: {
	item: RemoteServerResponse;
	isServerActive: boolean;
	actions: ServerRowActions;
}) {
	const rows: Array<{
		key: string;
		icon: React.ReactNode;
		label: string;
		show: boolean;
		separator?: boolean;
		danger?: boolean;
		onClick: () => void;
	}> = [
		{
			key: 'audit',
			icon: <ShieldCheck className="h-3.5 w-3.5" />,
			label: 'Audit Server',
			show: true,
			onClick: () => actions.onSetup(item),
		},
		{
			key: 'terminal',
			icon: <Terminal className="h-3.5 w-3.5" />,
			label: 'Open Terminal',
			show: !!actions.onOpenTerminal,
			onClick: () => actions.onOpenTerminal?.(item),
		},
		{
			key: 'test',
			icon: <Plug className="h-3.5 w-3.5" />,
			label: 'Test Connection',
			show: true,
			onClick: () => actions.onTest(item),
		},
		{
			key: 'migrate',
			icon: <ArrowLeftRight className="h-3.5 w-3.5" />,
			label: 'Migrate Services',
			show: !!actions.onMigrate,
			onClick: () => actions.onMigrate?.(item),
		},
		{
			key: 'network',
			icon: <Shield className="h-3.5 w-3.5" />,
			label: 'Private Network',
			show: true,
			onClick: () => actions.onNetwork(item),
		},
		{
			key: 'power',
			icon: (
				<Power
					className={`h-3.5 w-3.5 ${isServerActive ? 'text-amber-500' : 'text-emerald-500'}`}
				/>
			),
			label: isServerActive ? 'Deactivate' : 'Activate',
			show: true,
			separator: true,
			onClick: () => actions.onToggleStatus(item),
		},
		{
			key: 'edit',
			icon: <Pencil className="h-3.5 w-3.5" />,
			label: 'Edit Server',
			show: true,
			onClick: () => actions.onEdit(item),
		},
		{
			key: 'delete',
			icon: <Trash2 className="h-3.5 w-3.5" />,
			label: 'Delete Server',
			show: true,
			separator: true,
			danger: true,
			onClick: () => actions.onDelete(item),
		},
	];

	return (
		<DropdownMenu>
			<DropdownMenuTrigger
				render={
					<button className="flex size-6 shrink-0 cursor-pointer items-center justify-center text-muted-foreground transition-colors hover:text-foreground" />
				}>
				<MoreVertical className="size-4" />
			</DropdownMenuTrigger>
			<DropdownMenuContent align="end" className="w-44">
				{rows
					.filter(r => r.show)
					.map(r => (
						<FragmentRow key={r.key} row={r} />
					))}
			</DropdownMenuContent>
		</DropdownMenu>
	);
}

function FragmentRow({
	row,
}: {
	row: {
		icon: React.ReactNode;
		label: string;
		separator?: boolean;
		danger?: boolean;
		onClick: () => void;
	};
}) {
	return (
		<>
			{row.separator && <DropdownMenuSeparator />}
			<DropdownMenuItem
				className={`cursor-pointer gap-2 ${row.danger ? 'text-destructive focus:text-destructive' : ''}`}
				onClick={row.onClick}>
				{row.icon} {row.label}
			</DropdownMenuItem>
		</>
	);
}
