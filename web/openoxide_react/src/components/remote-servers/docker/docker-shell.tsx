import {type ReactNode} from 'react';
import {RefreshCw, X} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Skeleton} from '#/components/ui/skeleton';
import {
	Dialog,
	DialogContent,
	DialogHeader,
	DialogTitle,
} from '#/components/ui/dialog';

interface DockerPanelShellProps {
	hint: string;
	count: number;
	isLoading: boolean;
	error: string | null;
	isRefreshing: boolean;
	onRefresh: () => void;
	empty: ReactNode;
	children: ReactNode;
}

/** Frame shared by every server Docker tab: caption, refresh, and one of loading / error / empty / content. */
export function DockerPanelShell({
	hint,
	count,
	isLoading,
	error,
	isRefreshing,
	onRefresh,
	empty,
	children,
}: DockerPanelShellProps) {
	return (
		<section className="overflow-hidden rounded-xl border border-border bg-card">
			<div className="flex items-center gap-3 border-b border-border/50 px-5 py-4">
				<div className="min-w-0 flex-1">
					<h3 className="text-[15px] font-semibold text-foreground">
						{hint}
					</h3>
					<p className="text-xs text-muted-foreground">
						{isLoading ? 'Reading the engine…' : `${count} resource(s)`}
					</p>
				</div>
				<Button
					variant="outline"
					size="sm"
					onClick={onRefresh}
					disabled={isRefreshing}
					className="h-8 shrink-0 cursor-pointer gap-1.5 text-xs">
					<RefreshCw
						className={`size-3.5 ${isRefreshing ? 'animate-spin' : ''}`}
					/>
					Refresh
				</Button>
			</div>
			{error ? (
				<p className="px-5 py-10 text-center text-xs text-rose-500">
					{error}
				</p>
			) : isLoading ? (
				<div className="space-y-2 p-5">
					<Skeleton className="h-8 w-full" />
					<Skeleton className="h-8 w-full" />
					<Skeleton className="h-8 w-full" />
				</div>
			) : count === 0 ? (
				empty
			) : (
				children
			)}
		</section>
	);
}

export type DockerColumn<T> = {
	label: string;
	cell: (row: T) => ReactNode;
	/** Right-aligns the cell for numeric columns. */
	align?: 'right';
	/** Extra classes on every body cell of this column. */
	className?: string;
};

export function DockerTable<T>({
	columns,
	rows,
	keyOf,
	onSelect,
}: {
	columns: DockerColumn<T>[];
	rows: T[];
	keyOf: (row: T) => string;
	onSelect?: (row: T) => void;
}) {
	return (
		<div className="max-h-[calc(100vh-300px)] scrollbar-thin scrollbar-thumb-border/40 overflow-y-auto">
			<table className="w-full text-xs">
				<thead className="sticky top-0 z-10 bg-card">
					<tr className="border-b border-border/50 text-left text-muted-foreground">
						{columns.map(column => (
							<th
								key={column.label}
								className={`px-5 py-2.5 font-medium ${
									column.align === 'right' ? 'text-right' : ''
								}`}>
								{column.label}
							</th>
						))}
					</tr>
				</thead>
				<tbody>
					{rows.map(row => (
						<tr
							key={keyOf(row)}
							onClick={onSelect ? () => onSelect(row) : undefined}
							className={`border-b border-border/40 last:border-0 ${
								onSelect ? 'cursor-pointer hover:bg-muted/40' : ''
							}`}>
							{columns.map(column => (
								<td
									key={column.label}
									className={`px-5 py-2.5 align-middle ${
										column.align === 'right'
											? 'text-right tabular-nums'
											: ''
									} ${column.className ?? ''}`}>
									{column.cell(row)}
								</td>
							))}
						</tr>
					))}
				</tbody>
			</table>
		</div>
	);
}

export function DockerEmptyState({label}: {label: string}) {
	return (
		<p className="px-5 py-12 text-center text-xs text-muted-foreground">
			{label}
		</p>
	);
}

export function DockerStatusPill({
	running,
	label,
}: {
	running: boolean;
	label: string;
}) {
	return (
		<span
			className={`rounded-full px-2 py-0.5 text-[11px] font-medium ${
				running
					? 'bg-emerald-500/10 text-emerald-500'
					: 'bg-muted text-muted-foreground'
			}`}>
			{label}
		</span>
	);
}

/** Raw `docker inspect` document for the picked resource; the engine output is shown verbatim. */
export function DockerInspectDialog({
	title,
	documentJson,
	isLoading,
	error,
	onClose,
}: {
	title: string;
	documentJson: string;
	isLoading: boolean;
	error: string | null;
	onClose: () => void;
}) {
	const pretty = (() => {
		if (!documentJson) return '';
		try {
			return JSON.stringify(JSON.parse(documentJson), null, 2);
		} catch {
			return documentJson;
		}
	})();
	return (
		<Dialog open onOpenChange={open => !open && onClose()}>
			<DialogContent className="max-w-3xl">
				<DialogHeader>
					<DialogTitle className="flex items-center gap-2 text-sm">
						Inspect — {title}
						<Button
							variant="ghost"
							size="sm"
							onClick={onClose}
							className="ml-auto size-7 cursor-pointer p-0">
							<X className="size-4" />
						</Button>
					</DialogTitle>
				</DialogHeader>
				{error ? (
					<p className="py-8 text-center text-xs text-rose-500">{error}</p>
				) : isLoading ? (
					<Skeleton className="h-64 w-full" />
				) : (
					<pre className="max-h-[60vh] scrollbar-thin scrollbar-thumb-border/40 overflow-auto rounded-lg bg-muted/40 p-4 font-mono text-[11px] leading-relaxed">
						{pretty || 'Empty inspect response.'}
					</pre>
				)}
			</DialogContent>
		</Dialog>
	);
}
