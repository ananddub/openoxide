import {useEffect, useMemo, useRef, useState} from 'react';
import {createFileRoute} from '@tanstack/react-router';
import {
	Cloud,
	HardDrive,
	LayoutGrid,
	List,
	Plus,
	Search,
} from 'lucide-react';
import {toast} from 'sonner';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import {
	AlertDialog,
	AlertDialogAction,
	AlertDialogCancel,
	AlertDialogContent,
	AlertDialogDescription,
	AlertDialogFooter,
	AlertDialogHeader,
	AlertDialogTitle,
} from '#/components/ui/alert-dialog';
import {DestinationsList} from '#/components/destinations/destinations-list';
import {SectionHeader} from '#/components/settings/list-card';
import type {SshKeysView} from '#/components/ssh-keys/ssh-keys-list';
import {CreateDestinationModal} from '#/components/destinations/create-destination-modal';
import {
	PROVIDER_FILTERS,
	PROVIDER_LABELS,
	providerLabel,
	type ProviderFilter,
} from '#/components/destinations/destinations-shared';
import {useDestinationList, useDestinationDelete} from '#/api/live';
import {formatApiError, cn} from '#/api/utils';

import type {DestinationResponse} from '#/types/api-helpers';

export const Route = createFileRoute('/_app/destinations')({
	component: DestinationsPage,
});

const VIEW_STORAGE_KEY = 'destinations-view';

/**
 * S3 Destinations — the SSH Keys / Projects-page grammar: a section title
 * (with the New button on its right), then a toolbar (search + provider
 * filter + grid/list toggle) and cards or one bordered list card below;
 * create/edit opens a Geist sheet, delete a quiet AlertDialog. No side rail.
 */
function DestinationsPage() {
	const [search, setSearch] = useState('');
	const [providerFilter, setProviderFilter] =
		useState<ProviderFilter>('All');
	const [view, setView] = useState<SshKeysView>(() => {
		if (typeof window === 'undefined') return 'grid';
		return localStorage.getItem(VIEW_STORAGE_KEY) === 'list'
			? 'list'
			: 'grid';
	});
	const [isCreateOpen, setIsCreateOpen] = useState(false);
	const [isEditOpen, setIsEditOpen] = useState(false);
	const [editingDestination, setEditingDestination] =
		useState<DestinationResponse | null>(null);
	const [deleteTarget, setDeleteTarget] =
		useState<DestinationResponse | null>(null);

	const handleViewChange = (next: SshKeysView) => {
		setView(next);
		localStorage.setItem(VIEW_STORAGE_KEY, next);
	};

	// Live destinations query; the table-watch stream keeps it fresh.
	const destinationsQuery = useDestinationList();
	const destinations = (destinationsQuery.data ??
		[]) as unknown as DestinationResponse[];

	const deleteMutation = useDestinationDelete();

	const filtered = useMemo(() => {
		return destinations.filter(d => {
			const q = search.toLowerCase();
			const matchName =
				d.name?.toLowerCase().includes(q) ||
				d.bucket?.toLowerCase().includes(q);
			const label = providerLabel(d.provider);
			const matchProvider =
				providerFilter === 'All' ||
				label === providerFilter ||
				(providerFilter === 'S3 Compatible' &&
					!Object.values(PROVIDER_LABELS).includes(label));
			return matchName && matchProvider;
		});
	}, [destinations, search, providerFilter]);

	const hasFilters = search.trim() !== '' || providerFilter !== 'All';
	const clearFilters = () => {
		setSearch('');
		setProviderFilter('All');
	};

	const searchRef = useRef<HTMLInputElement>(null);

	// Same `/` shortcut as the Projects toolbar.
	useEffect(() => {
		const onKey = (e: KeyboardEvent) => {
			if (e.key !== '/' || e.metaKey || e.ctrlKey || e.altKey) return;
			const el = document.activeElement as HTMLElement | null;
			if (
				el &&
				(el.tagName === 'INPUT' ||
					el.tagName === 'TEXTAREA' ||
					el.tagName === 'SELECT' ||
					el.isContentEditable)
			) {
				return;
			}
			e.preventDefault();
			searchRef.current?.focus();
		};
		window.addEventListener('keydown', onKey);
		return () => window.removeEventListener('keydown', onKey);
	}, []);

	const confirmDelete = async () => {
		if (!deleteTarget) return;
		try {
			await deleteMutation.mutateAsync(String(deleteTarget.id));
			toast.success('S3 Storage Destination deleted');
		} catch (err: unknown) {
			toast.error(formatApiError(err));
		} finally {
			setDeleteTarget(null);
		}
	};

	return (
		<div className="mx-auto flex w-full max-w-[1920px] animate-in flex-col gap-6 px-4 pt-4 pb-6 duration-200 fade-in">
			<SectionHeader
				title="S3 Destinations"
				description="S3-compatible buckets used for application backups and volume snapshots."
				action={
					<Button
						onClick={() => setIsCreateOpen(true)}
						className="h-9 gap-1.5">
						<Plus className="size-3.5" />
						New Destination
					</Button>
				}
			/>

			{/* Search + provider filter + grid/list toggle */}
			<div className="flex w-full animate-in flex-col items-stretch gap-3 duration-200 fade-in sm:flex-row sm:items-center">
				<div className="relative w-full sm:grow">
					<Search className="pointer-events-none absolute top-1/2 left-3 size-4 -translate-y-1/2 text-muted-foreground/70" />
					<Input
						ref={searchRef}
						placeholder="Search Destinations"
						value={search}
						onChange={e => setSearch(e.target.value)}
						className="h-9 rounded-lg pr-11 pl-9 text-sm transition-colors duration-150 hover:border-foreground/25"
					/>
					<kbd
						aria-hidden
						className="pointer-events-none absolute top-1/2 right-3 flex size-5 -translate-y-1/2 items-center justify-center rounded-md border border-border bg-foreground/[0.04] font-mono text-[11px] text-muted-foreground select-none">
						/
					</kbd>
				</div>

				<div className="flex shrink-0 items-center gap-2">
					<Select
						value={providerFilter}
						onValueChange={v => setProviderFilter(v as ProviderFilter)}>
						<SelectTrigger className="h-9 w-auto rounded-lg bg-card">
							<SelectValue>
								{providerFilter === 'All'
									? 'All Providers'
									: providerFilter}
							</SelectValue>
						</SelectTrigger>
						<SelectContent className="bg-card">
							{PROVIDER_FILTERS.map(f => (
								<SelectItem key={f} value={f}>
									{f === 'All' ? 'All Providers' : f}
								</SelectItem>
							))}
						</SelectContent>
					</Select>

					{/* Grid / list toggle */}
					<div className="flex items-center rounded-lg border bg-card p-1">
						<button
							type="button"
							aria-label="Grid view"
							title="Grid view"
							onClick={() => handleViewChange('grid')}
							className={cn(
								'flex size-7 cursor-pointer items-center justify-center rounded-md text-muted-foreground transition-colors hover:text-foreground',
								view === 'grid' && 'bg-accent text-foreground',
							)}>
							<LayoutGrid className="size-3.5" />
						</button>
						<button
							type="button"
							aria-label="List view"
							title="List view"
							onClick={() => handleViewChange('list')}
							className={cn(
								'flex size-7 cursor-pointer items-center justify-center rounded-md text-muted-foreground transition-colors hover:text-foreground',
								view === 'list' && 'bg-accent text-foreground',
							)}>
							<List className="size-3.5" />
						</button>
					</div>
				</div>
			</div>

			{/* Content: grid cards or compact list */}
			{!destinationsQuery.isLoading && destinations.length === 0 ? (
				<div className="flex flex-col items-center justify-center rounded-xl border bg-card p-12 text-center">
					<HardDrive className="mb-4 size-10 text-muted-foreground/40" />
					<h3 className="mb-2 text-xl font-semibold tracking-[-0.01em] text-foreground">
						No S3 destinations yet
					</h3>
					<p className="mx-auto max-w-sm text-sm leading-relaxed text-muted-foreground/70">
						Connect an S3-compatible bucket — AWS, R2, MinIO, Spaces — to
						enable backups and volume snapshots.
					</p>
					<Button
						onClick={() => setIsCreateOpen(true)}
						className="mt-4 h-9 gap-1.5 text-xs font-semibold">
						<Plus className="size-3.5" />
						New Destination
					</Button>
				</div>
			) : (
				<DestinationsList
					destinations={filtered}
					isLoading={destinationsQuery.isLoading}
					hasFilters={hasFilters}
					view={view}
					onClearFilters={clearFilters}
					onEdit={item => {
						setEditingDestination(item);
						setIsEditOpen(true);
					}}
					onDelete={setDeleteTarget}
				/>
			)}

			{/* Create — Geist sheet (same sheet handles edit) */}
			<CreateDestinationModal
				isOpen={isCreateOpen}
				onClose={() => setIsCreateOpen(false)}
				onSuccess={() => {}}
				editingDestination={null}
			/>

			<CreateDestinationModal
				isOpen={isEditOpen}
				onClose={() => {
					setIsEditOpen(false);
					setEditingDestination(null);
				}}
				onSuccess={() => {}}
				editingDestination={editingDestination}
			/>

			{/* Delete — quiet AlertDialog, same grammar as Delete Project */}
			<AlertDialog
				open={deleteTarget !== null}
				onOpenChange={open => !open && setDeleteTarget(null)}>
				<AlertDialogContent>
					<AlertDialogHeader>
						<AlertDialogTitle>Delete S3 Destination</AlertDialogTitle>
						<AlertDialogDescription>
							Are you sure you want to delete{' '}
							<strong className="text-foreground">
								{deleteTarget?.name}
							</strong>{' '}
							<Cloud className="inline size-3.5 text-muted-foreground" />?
							Backups and volume snapshots pointing at this bucket will
							stop until reconfigured.
						</AlertDialogDescription>
					</AlertDialogHeader>
					<AlertDialogFooter>
						<AlertDialogCancel onClick={() => setDeleteTarget(null)}>
							Cancel
						</AlertDialogCancel>
						<AlertDialogAction
							className="bg-destructive text-destructive-foreground hover:bg-destructive/90"
							onClick={e => {
								e.preventDefault();
								void confirmDelete();
							}}>
							Delete
						</AlertDialogAction>
					</AlertDialogFooter>
				</AlertDialogContent>
			</AlertDialog>
		</div>
	);
}
