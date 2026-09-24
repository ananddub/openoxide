import {useCallback, useEffect, useMemo, useRef, useState} from 'react';
import {createFileRoute} from '@tanstack/react-router';
import {Database, LayoutGrid, List, Plus, Search} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
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
import {RegistriesList} from '#/components/registry/registries-list';
import type {SshKeysView} from '#/components/ssh-keys/ssh-keys-list';
import {CreateRegistryModal} from '#/components/registry/create-registry-modal';
import {SectionHeader} from '#/components/settings/list-card';
import {useRegistryDelete, useRegistryList} from '#/api/live';
import {toast} from 'sonner';
import {formatApiError, cn} from '#/api/utils';

import type {RegistryResponse} from '#/types/api-helpers';

export const Route = createFileRoute('/_app/registry')({
	component: RegistryPage,
});

const VIEW_STORAGE_KEY = 'registry-view';

/** Container Registries — the SSH Keys / Projects-page grammar: a section title, then a toolbar (search + grid/list toggle + New) and cards or one bordered list card below; create/edit opens a Geist sheet, delete a quiet AlertDialog. No side rail. */
function RegistryPage() {
	const [search, setSearch] = useState('');
	const [view, setView] = useState<SshKeysView>(() => {
		if (typeof window === 'undefined') return 'grid';
		return localStorage.getItem(VIEW_STORAGE_KEY) === 'list'
			? 'list'
			: 'grid';
	});
	const [isCreateOpen, setIsCreateOpen] = useState(false);
	const [isEditOpen, setIsEditOpen] = useState(false);
	const [editingRegistry, setEditingRegistry] =
		useState<RegistryResponse | null>(null);
	const [deletingId, setDeletingId] = useState<number | null>(null);

	const handleViewChange = (next: SshKeysView) => {
		setView(next);
		localStorage.setItem(VIEW_STORAGE_KEY, next);
	};

	const {data: rawRegistries, isLoading, refetch} = useRegistryList();

	const registries = Array.isArray(rawRegistries ?? [])
		? ((rawRegistries ?? []) as unknown as RegistryResponse[])
		: [];
	const deleteMutation = useRegistryDelete();

	const filtered = useMemo(() => {
		if (!search.trim()) return registries;
		const q = search.toLowerCase();
		return registries.filter(r => {
			const d = r as any;
			const name = String(d.name || d.registry_name || '').toLowerCase();
			const url = String(d.registry_url || '').toLowerCase();
			return name.includes(q) || url.includes(q);
		});
	}, [registries, search]);

	const hasFilters = search.trim() !== '';
	const clearFilters = () => setSearch('');

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

	const handleOpenEdit = useCallback((item: RegistryResponse) => {
		setEditingRegistry(item);
		setIsEditOpen(true);
	}, []);

	const confirmDelete = async () => {
		if (deletingId === null) return;
		try {
			await deleteMutation.mutateAsync(deletingId);
			toast.success('Registry deleted');
		} catch (err: unknown) {
			toast.error(formatApiError(err));
		} finally {
			setDeletingId(null);
		}
	};

	return (
		<div className="mx-auto flex w-full max-w-[1920px] animate-in flex-col gap-6 px-4 pt-4 pb-6 duration-200 fade-in">
			<SectionHeader
				title="Registries"
				description="Container registries for pulling and pushing images with stored credentials."
				action={
					<Button
						onClick={() => setIsCreateOpen(true)}
						className="h-9 gap-1.5">
						<Plus className="size-3.5" />
						New Registry
					</Button>
				}
			/>

			{/* Search + grid/list toggle */}
			<div className="flex w-full animate-in flex-col items-stretch gap-3 duration-200 fade-in sm:flex-row sm:items-center">
				<div className="relative w-full sm:grow">
					<Search className="pointer-events-none absolute top-1/2 left-3 size-4 -translate-y-1/2 text-muted-foreground/70" />
					<Input
						ref={searchRef}
						placeholder="Search Registries"
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
			{!isLoading && registries.length === 0 ? (
				<div className="flex flex-col items-center justify-center rounded-xl border bg-card p-12 text-center">
					<Database className="mb-4 size-10 text-muted-foreground/40" />
					<h3 className="mb-2 text-xl font-semibold tracking-[-0.01em] text-foreground">
						No registries yet
					</h3>
					<p className="mx-auto max-w-sm text-sm leading-relaxed text-muted-foreground/70">
						Add a Docker registry to pull or push container images with
						stored credentials.
					</p>
					<Button
						onClick={() => setIsCreateOpen(true)}
						className="mt-4 h-9 gap-1.5 text-xs font-semibold">
						<Plus className="size-3.5" />
						New Registry
					</Button>
				</div>
			) : (
				<RegistriesList
					registries={filtered}
					isLoading={isLoading}
					hasFilters={hasFilters}
					view={view}
					onClearFilters={clearFilters}
					onEdit={handleOpenEdit}
					onDelete={id => setDeletingId(id)}
				/>
			)}

			{/* Create — Geist sheet (same sheet handles edit) */}
			<CreateRegistryModal
				isOpen={isCreateOpen}
				onClose={() => setIsCreateOpen(false)}
				onSuccess={() => void refetch()}
				initialData={null}
			/>

			<CreateRegistryModal
				isOpen={isEditOpen}
				onClose={() => {
					setIsEditOpen(false);
					setEditingRegistry(null);
				}}
				onSuccess={() => void refetch()}
				initialData={editingRegistry}
			/>

			{/* Delete — quiet AlertDialog, same grammar as Delete Project */}
			<AlertDialog
				open={deletingId !== null}
				onOpenChange={open => !open && setDeletingId(null)}>
				<AlertDialogContent>
					<AlertDialogHeader>
						<AlertDialogTitle>Delete Container Registry</AlertDialogTitle>
						<AlertDialogDescription>
							Are you sure you want to delete this container registry?
							Applications pinning it will lose stored credentials until
							updated.
						</AlertDialogDescription>
					</AlertDialogHeader>
					<AlertDialogFooter>
						<AlertDialogCancel onClick={() => setDeletingId(null)}>
							Cancel
						</AlertDialogCancel>
						<AlertDialogAction
							className="text-destructive-foreground bg-destructive hover:bg-destructive/90"
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
