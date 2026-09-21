import {
	HardDrive,
	Trash2,
	Pencil,
	RefreshCw,
	Check,
	X,
	Plug,
	MoreVertical,
} from 'lucide-react';
import {Badge} from '#/components/ui/badge';
import {Separator} from '#/components/ui/separator';
import {
	DropdownMenu,
	DropdownMenuContent,
	DropdownMenuItem,
	DropdownMenuSeparator,
	DropdownMenuTrigger,
} from '#/components/ui/dropdown';
import type {DestinationResponse} from '#/types/api-helpers';
import {providerLabel} from '#/components/destinations/destinations-shared';

export type TestStatus = 'testing' | 'success' | 'failed' | undefined;

interface DestinationRowProps {
	destination: DestinationResponse;
	status: TestStatus;
	isDeleting: boolean;
	onTest: (id: string | number) => void;
	onEdit: (item: DestinationResponse) => void;
	onDelete: (id: string | number) => void;
}

/** A single destination row with its test status and actions menu. */
export function DestinationRow({
	destination: d,
	status,
	isDeleting,
	onTest,
	onEdit,
	onDelete,
}: DestinationRowProps) {
	const isTesting = status === 'testing';
	const label = providerLabel(d.provider);
	const dotCls = isTesting
		? 'bg-amber-400 animate-pulse'
		: status === 'success'
			? 'bg-emerald-500'
			: status === 'failed'
				? 'bg-rose-500'
				: 'bg-zinc-500/60';

	return (
		<div className="group flex items-center gap-4 bg-card px-4 py-3.5 transition-colors hover:bg-accent/30">
			{/* Icon + test status dot */}
			<div className="relative shrink-0">
				<div className="flex h-8 w-8 items-center justify-center rounded-full bg-muted">
					<HardDrive className="h-4 w-4 text-foreground/70" />
				</div>
				<span
					className={`absolute -right-0.5 -bottom-0.5 h-2.5 w-2.5 rounded-full border-2 border-background ${dotCls}`}
				/>
			</div>

			{/* Info */}
			<div className="min-w-0 flex-1">
				<div className="mb-0.5 flex items-center gap-2">
					<span className="truncate text-sm font-medium text-foreground">
						{d.name}
					</span>
					<Badge
						variant="secondary"
						className="shrink-0 py-0 font-mono text-[10px]">
						{label}
					</Badge>
				</div>
				<p className="truncate font-mono text-[11px] text-muted-foreground">
					{d.bucket}
					{d.region ? ` · ${d.region}` : ''}
					{d.endpoint ? ` · ${d.endpoint}` : ''}
				</p>
			</div>

			{/* Hover: test button */}
			<div className="flex items-center gap-3 opacity-0 transition-opacity group-hover:opacity-100">
				<button
					onClick={() => onTest(d.id)}
					disabled={isTesting}
					title={isTesting ? 'Testing…' : 'Test connection'}
					className="cursor-pointer text-muted-foreground transition-colors hover:text-foreground disabled:cursor-not-allowed disabled:opacity-50">
					{isTesting ? (
						<RefreshCw className="h-4 w-4 animate-spin text-amber-500" />
					) : status === 'success' ? (
						<Check className="h-4 w-4 text-emerald-500" />
					) : status === 'failed' ? (
						<X className="h-4 w-4 text-rose-500" />
					) : (
						<Plug className="h-4 w-4" />
					)}
				</button>
			</div>

			<Separator
				orientation="vertical"
				className="h-5 opacity-0 transition-opacity group-hover:opacity-100"
			/>

			{/* 3-dot menu */}
			<DropdownMenu>
				<DropdownMenuTrigger
					render={
						<button className="cursor-pointer text-muted-foreground transition-colors hover:text-foreground" />
					}>
					<MoreVertical className="h-4 w-4" />
				</DropdownMenuTrigger>
				<DropdownMenuContent align="end" className="w-40">
					<DropdownMenuItem
						className="cursor-pointer gap-2"
						onClick={() => onTest(d.id)}>
						<Plug className="h-3.5 w-3.5" /> Test Connection
					</DropdownMenuItem>
					<DropdownMenuItem
						className="cursor-pointer gap-2"
						onClick={() => onEdit(d)}>
						<Pencil className="h-3.5 w-3.5" /> Edit
					</DropdownMenuItem>
					<DropdownMenuSeparator />
					<DropdownMenuItem
						className="cursor-pointer gap-2 text-destructive focus:text-destructive"
						onClick={() => onDelete(d.id)}
						disabled={isDeleting}>
						<Trash2 className="h-3.5 w-3.5" />
						{isDeleting ? 'Deleting…' : 'Delete'}
					</DropdownMenuItem>
				</DropdownMenuContent>
			</DropdownMenu>
		</div>
	);
}
