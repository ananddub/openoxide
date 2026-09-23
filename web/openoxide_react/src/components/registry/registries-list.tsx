import {useState} from 'react';
import {Button} from '#/components/ui/button';
import {
	DropdownMenu,
	DropdownMenuContent,
	DropdownMenuItem,
	DropdownMenuSeparator,
	DropdownMenuTrigger,
} from '#/components/ui/dropdown';
import {toast} from 'sonner';
import {formatApiError, cn} from '#/api/utils';
import {
	Database,
	Plug,
	RefreshCw,
	Check,
	X,
	MoreHorizontal,
	Search,
} from 'lucide-react';

import type {RegistryResponse} from '#/types/api-helpers';
import {useRegistryTest} from '#/api/live';
import type {SshKeysView} from '#/components/ssh-keys/ssh-keys-list';

interface RegistriesListProps {
	/** Already filtered by the page toolbar. */
	registries: RegistryResponse[];
	isLoading: boolean;
	hasFilters: boolean;
	view: SshKeysView;
	onClearFilters: () => void;
	onEdit: (item: RegistryResponse) => void;
	onDelete: (id: number) => void;
}

/** Shared connection-test state for one registry row/card. */
function useConnectionTest() {
	const [status, setStatus] = useState<
		'testing' | 'success' | 'failed' | undefined
	>();
	const testMutation = useRegistryTest();

	const test = async (id: number | string) => {
		setStatus('testing');
		try {
			await testMutation.mutateAsync(Number(id));
			setStatus('success');
			toast.success('Registry connection test passed');
			setTimeout(() => setStatus(undefined), 3000);
		} catch (err: unknown) {
			setStatus('failed');
			toast.error(formatApiError(err));
		}
	};

	return {status, test};
}

function TestButton({
	itemId,
	status,
	onTest,
}: {
	itemId: number | string;
	status: 'testing' | 'success' | 'failed' | undefined;
	onTest: () => void;
}) {
	const isTesting = status === 'testing';
	return (
		<Button
			variant="ghost"
			size="icon"
			disabled={isTesting}
			onClick={onTest}
			title="Test Registry Connection"
			className="size-7 shrink-0 rounded-md text-muted-foreground hover:text-foreground">
			{isTesting ? (
				<RefreshCw className="size-3.5 animate-spin text-amber-500" />
			) : status === 'success' ? (
				<Check className="size-3.5 text-emerald-500" />
			) : status === 'failed' ? (
				<X className="size-3.5 text-rose-500" />
			) : (
				<Plug className="size-3.5" />
			)}
			<span className="sr-only">Test {String(itemId)}</span>
		</Button>
	);
}

function KebabMenu({
	onEdit,
	onDelete,
}: {
	onEdit: () => void;
	onDelete: () => void;
}) {
	return (
		<DropdownMenu>
			<DropdownMenuTrigger
				render={
					<Button
						variant="ghost"
						size="icon"
						className="size-7 shrink-0 rounded-md text-muted-foreground hover:bg-muted/80 hover:text-foreground focus-visible:ring-0"
						aria-label="Registry actions">
						<MoreHorizontal className="size-4" />
					</Button>
				}
			/>
			<DropdownMenuContent align="end" className="w-44">
				<DropdownMenuItem
					className="cursor-pointer gap-2"
					onClick={onEdit}>
					Edit Registry
				</DropdownMenuItem>
				<DropdownMenuSeparator />
				<DropdownMenuItem
					className="cursor-pointer gap-2 text-destructive focus:text-destructive"
					onClick={onDelete}>
					Delete Registry
				</DropdownMenuItem>
			</DropdownMenuContent>
		</DropdownMenu>
	);
}

function StatusDot({
	status,
}: {
	status: 'testing' | 'success' | 'failed' | undefined;
}) {
	const cls =
		status === 'testing'
			? 'bg-amber-500 animate-pulse'
			: status === 'success'
				? 'bg-emerald-500'
				: status === 'failed'
					? 'bg-rose-500'
					: 'bg-zinc-400/80 dark:bg-zinc-500';
	return (
		<span aria-hidden className={`size-2 shrink-0 rounded-full ${cls}`} />
	);
}

/** Registry surface — Projects grammar: grid cards or one bordered list
 *  card, matching the SSH Keys page. */
export function RegistriesList({
	registries,
	isLoading,
	hasFilters,
	view,
	onClearFilters,
	onEdit,
	onDelete,
}: RegistriesListProps) {
	if (isLoading) {
		if (view === 'grid') {
			return (
				<div className="grid w-full grid-cols-1 gap-3 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 2xl:grid-cols-5">
					{[1, 2, 3, 4].map(i => (
						<div
							key={i}
							className="min-h-[88px] animate-pulse rounded-lg border bg-card"
						/>
					))}
				</div>
			);
		}
		return (
			<div className="w-full divide-y overflow-hidden rounded-xl border bg-card">
				{[1, 2, 3].map(i => (
					<div key={i} className="flex items-center gap-3.5 px-4 py-3">
						<div className="size-9 shrink-0 animate-pulse rounded-lg bg-muted/60" />
						<div className="flex-1 space-y-2">
							<SkeletonBar className="h-3.5 w-40" />
							<SkeletonBar className="h-3 w-64" />
						</div>
					</div>
				))}
			</div>
		);
	}

	if (registries.length === 0) {
		return (
			<div className="flex flex-col items-center justify-center gap-3 rounded-xl border bg-card p-12 text-center">
				<Search className="size-10 text-muted-foreground/40" />
				<h3 className="text-xl font-semibold tracking-[-0.01em] text-foreground">
					No matching registries found
				</h3>
				<p className="mx-auto max-w-sm text-sm leading-relaxed text-muted-foreground/70">
					Try adjusting your search keywords or clearing the filters.
				</p>
				{hasFilters && (
					<Button
						variant="ghost"
						onClick={onClearFilters}
						className="mt-4 text-xs font-semibold text-primary">
						Clear All Filters
					</Button>
				)}
			</div>
		);
	}

	if (view === 'grid') {
		return (
			<div className="grid w-full grid-cols-1 gap-3 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 2xl:grid-cols-5">
				{registries.map(item => (
					<RegistryGridCard
						key={String(item.id)}
						item={item}
						onEdit={onEdit}
						onDelete={onDelete}
					/>
				))}
			</div>
		);
	}

	return (
		<div className="w-full divide-y overflow-hidden rounded-xl border bg-card">
			{registries.map(item => (
				<RegistryListRow
					key={String(item.id)}
					item={item}
					onEdit={onEdit}
					onDelete={onDelete}
				/>
			))}
		</div>
	);
}

function SkeletonBar({className}: {className?: string}) {
	return (
		<div className={cn('animate-pulse rounded bg-muted/60', className)} />
	);
}

function registryIdentity(item: RegistryResponse) {
	const d = item as any;
	return {
		name: String(d.name || d.registry_name || ''),
		url: String(d.registry_url || 'docker.io'),
		prefix: d.image_prefix ? String(d.image_prefix) : '',
	};
}

function RegistryGridCard({
	item,
	onEdit,
	onDelete,
}: {
	item: RegistryResponse;
	onEdit: (item: RegistryResponse) => void;
	onDelete: (id: number) => void;
}) {
	const {status, test} = useConnectionTest();
	const {name, url, prefix} = registryIdentity(item);

	return (
		<div className="group flex min-h-[88px] flex-col justify-between gap-3 rounded-lg border bg-card p-4 transition-colors hover:bg-accent/30">
			<div className="flex min-w-0 items-start gap-3">
				<div className="flex size-9 shrink-0 items-center justify-center rounded-lg bg-foreground/[0.05] text-foreground">
					<Database className="size-4" />
				</div>
				<div className="min-w-0 flex-1">
					<div className="flex min-w-0 items-center gap-2">
						<span className="truncate text-sm font-medium text-foreground">
							{name}
						</span>
						<StatusDot status={status} />
					</div>
					<p className="mt-0.5 truncate font-mono text-[11px] text-muted-foreground">
						{url}
					</p>
				</div>
			</div>
			<div className="flex items-center justify-between gap-2">
				{prefix ? (
					<span className="truncate rounded bg-muted/50 px-1.5 py-0.5 font-mono text-[10px] font-medium text-muted-foreground">
						{prefix}
					</span>
				) : (
					<span />
				)}
				<div className="flex shrink-0 items-center gap-1">
					<TestButton
						itemId={item.id}
						status={status}
						onTest={() => void test(item.id)}
					/>
					<KebabMenu
						onEdit={() => onEdit(item)}
						onDelete={() => onDelete(Number(item.id))}
					/>
				</div>
			</div>
		</div>
	);
}

function RegistryListRow({
	item,
	onEdit,
	onDelete,
}: {
	item: RegistryResponse;
	onEdit: (item: RegistryResponse) => void;
	onDelete: (id: number) => void;
}) {
	const {status, test} = useConnectionTest();
	const {name, url, prefix} = registryIdentity(item);

	return (
		<div className="group flex items-center gap-3.5 px-4 py-3 transition-colors hover:bg-accent/30">
			<div className="flex size-9 shrink-0 items-center justify-center rounded-lg bg-foreground/[0.05] text-foreground">
				<Database className="size-4" />
			</div>
			<div className="min-w-0 flex-1">
				<div className="flex items-center gap-2">
					<span className="truncate text-sm font-medium text-foreground">
						{name}
					</span>
					{prefix ? (
						<span className="shrink-0 rounded bg-muted/50 px-1.5 py-0.5 font-mono text-[10px] font-medium text-muted-foreground">
							{prefix}
						</span>
					) : null}
					<StatusDot status={status} />
				</div>
				<p className="mt-0.5 truncate font-mono text-[11px] text-muted-foreground">
					{url}
				</p>
			</div>
			<div className="flex shrink-0 items-center gap-1">
				<TestButton
					itemId={item.id}
					status={status}
					onTest={() => void test(item.id)}
				/>
				<KebabMenu
					onEdit={() => onEdit(item)}
					onDelete={() => onDelete(Number(item.id))}
				/>
			</div>
		</div>
	);
}
