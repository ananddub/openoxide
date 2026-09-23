import {useState} from 'react';
import {
	HardDrive,
	Plug,
	RefreshCw,
	Check,
	X,
	MoreHorizontal,
	Search,
} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {
	DropdownMenu,
	DropdownMenuContent,
	DropdownMenuItem,
	DropdownMenuSeparator,
	DropdownMenuTrigger,
} from '#/components/ui/dropdown';
import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';
import {useDestinationTest} from '#/api/live';
import {providerLabel} from '#/components/destinations/destinations-shared';
import type {SshKeysView} from '#/components/ssh-keys/ssh-keys-list';

import type {DestinationResponse} from '#/types/api-helpers';

type TestStatus = 'testing' | 'success' | 'failed' | undefined;

interface DestinationsListProps {
	/** Already filtered by the page toolbar. */
	destinations: DestinationResponse[];
	isLoading: boolean;
	hasFilters: boolean;
	view: SshKeysView;
	onClearFilters: () => void;
	onEdit: (item: DestinationResponse) => void;
	onDelete: (item: DestinationResponse) => void;
}

function useConnectionTest() {
	const [status, setStatus] = useState<TestStatus>(undefined);
	const testMutation = useDestinationTest();

	const test = async (d: DestinationResponse) => {
		setStatus('testing');
		try {
			await Promise.race([
				testMutation.mutateAsync({
					provider: d.provider,
					bucket: d.bucket,
					region: d.region,
					endpoint: d.endpoint,
					access_key: d.access_key,
					secret_access_key: d.secret_access_key,
				}),
				new Promise<never>((_, reject) =>
					setTimeout(
						() => reject(new Error('S3 connection test timed out')),
						15_000,
					),
				),
			]);
			setStatus('success');
			toast.success('S3 destination connection test passed!');
			setTimeout(() => setStatus(undefined), 3000);
		} catch (err: unknown) {
			setStatus('failed');
			toast.error(formatApiError(err));
		}
	};

	return {status, test};
}

function TestButton({
	status,
	onTest,
}: {
	status: TestStatus;
	onTest: () => void;
}) {
	const isTesting = status === 'testing';
	return (
		<Button
			variant="ghost"
			size="icon"
			disabled={isTesting}
			onClick={onTest}
			title={isTesting ? 'Testing…' : 'Test connection'}
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
		</Button>
	);
}

function KebabMenu({
	onTest,
	onEdit,
	onDelete,
}: {
	onTest: () => void;
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
						aria-label="Destination actions">
						<MoreHorizontal className="size-4" />
					</Button>
				}
			/>
			<DropdownMenuContent align="end" className="w-44">
				<DropdownMenuItem
					className="cursor-pointer gap-2"
					onClick={onTest}>
					<Plug className="size-3.5" /> Test Connection
				</DropdownMenuItem>
				<DropdownMenuItem
					className="cursor-pointer gap-2"
					onClick={onEdit}>
					Edit Destination
				</DropdownMenuItem>
				<DropdownMenuSeparator />
				<DropdownMenuItem
					className="cursor-pointer gap-2 text-destructive focus:text-destructive"
					onClick={onDelete}>
					Delete Destination
				</DropdownMenuItem>
			</DropdownMenuContent>
		</DropdownMenu>
	);
}

function statusDotCls(status: TestStatus) {
	return status === 'testing'
		? 'bg-amber-500 animate-pulse'
		: status === 'success'
			? 'bg-emerald-500'
			: status === 'failed'
				? 'bg-rose-500'
				: 'bg-zinc-400/80 dark:bg-zinc-500';
}

function IconTile({status}: {status: TestStatus}) {
	return (
		<div className="relative shrink-0">
			<div className="flex size-9 items-center justify-center rounded-lg bg-foreground/[0.05] text-foreground">
				<HardDrive className="size-4" />
			</div>
			<span
				aria-hidden
				className={`absolute -right-0.5 -bottom-0.5 size-2.5 rounded-full border-2 border-background ${statusDotCls(status)}`}
			/>
		</div>
	);
}

function destinationLine(d: DestinationResponse) {
	return [d.bucket, d.region, d.endpoint].filter(Boolean).join(' · ');
}

/** Destinations surface — Projects grammar: grid cards or one bordered
 *  list card, matching the SSH Keys page. */
export function DestinationsList({
	destinations,
	isLoading,
	hasFilters,
	view,
	onClearFilters,
	onEdit,
	onDelete,
}: DestinationsListProps) {
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
							<div className="h-3.5 w-40 animate-pulse rounded bg-muted/60" />
							<div className="h-3 w-64 animate-pulse rounded bg-muted/60" />
						</div>
					</div>
				))}
			</div>
		);
	}

	if (destinations.length === 0) {
		return (
			<div className="flex flex-col items-center justify-center gap-3 rounded-xl border bg-card p-12 text-center">
				<Search className="size-10 text-muted-foreground/40" />
				<h3 className="text-xl font-semibold tracking-[-0.01em] text-foreground">
					No matching destinations found
				</h3>
				<p className="mx-auto max-w-sm text-sm leading-relaxed text-muted-foreground/70">
					Try adjusting your search keywords or clearing the provider
					filter.
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
				{destinations.map(d => (
					<DestinationGridCard
						key={String(d.id)}
						destination={d}
						onEdit={onEdit}
						onDelete={onDelete}
					/>
				))}
			</div>
		);
	}

	return (
		<div className="w-full divide-y overflow-hidden rounded-xl border bg-card">
			{destinations.map(d => (
				<DestinationListRow
					key={String(d.id)}
					destination={d}
					onEdit={onEdit}
					onDelete={onDelete}
				/>
			))}
		</div>
	);
}

function DestinationGridCard({
	destination: d,
	onEdit,
	onDelete,
}: {
	destination: DestinationResponse;
	onEdit: (item: DestinationResponse) => void;
	onDelete: (item: DestinationResponse) => void;
}) {
	const {status, test} = useConnectionTest();

	return (
		<div className="group flex min-h-[88px] flex-col justify-between gap-3 rounded-lg border bg-card p-4 transition-colors hover:bg-accent/30">
			<div className="flex min-w-0 items-start gap-3">
				<IconTile status={status} />
				<div className="min-w-0 flex-1">
					<div className="flex min-w-0 items-center gap-2">
						<span className="truncate text-sm font-medium text-foreground">
							{d.name}
						</span>
					</div>
					<p className="mt-0.5 truncate font-mono text-[11px] text-muted-foreground">
						{destinationLine(d)}
					</p>
				</div>
			</div>
			<div className="flex items-center justify-between gap-2">
				<span className="shrink-0 rounded bg-muted/50 px-1.5 py-0.5 font-mono text-[10px] font-medium text-muted-foreground">
					{providerLabel(d.provider)}
				</span>
				<div className="flex shrink-0 items-center gap-1">
					<TestButton status={status} onTest={() => void test(d)} />
					<KebabMenu
						onTest={() => void test(d)}
						onEdit={() => onEdit(d)}
						onDelete={() => onDelete(d)}
					/>
				</div>
			</div>
		</div>
	);
}

function DestinationListRow({
	destination: d,
	onEdit,
	onDelete,
}: {
	destination: DestinationResponse;
	onEdit: (item: DestinationResponse) => void;
	onDelete: (item: DestinationResponse) => void;
}) {
	const {status, test} = useConnectionTest();

	return (
		<div className="group flex items-center gap-3.5 px-4 py-3 transition-colors hover:bg-accent/30">
			<IconTile status={status} />
			<div className="min-w-0 flex-1">
				<div className="flex items-center gap-2">
					<span className="truncate text-sm font-medium text-foreground">
						{d.name}
					</span>
					<span className="shrink-0 rounded bg-muted/50 px-1.5 py-0.5 font-mono text-[10px] font-medium text-muted-foreground">
						{providerLabel(d.provider)}
					</span>
				</div>
				<p className="mt-0.5 truncate font-mono text-[11px] text-muted-foreground">
					{destinationLine(d)}
				</p>
			</div>
			<div className="flex shrink-0 items-center gap-1">
				<TestButton status={status} onTest={() => void test(d)} />
				<KebabMenu
					onTest={() => void test(d)}
					onEdit={() => onEdit(d)}
					onDelete={() => onDelete(d)}
				/>
			</div>
		</div>
	);
}
