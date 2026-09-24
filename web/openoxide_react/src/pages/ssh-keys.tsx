import {useCallback, useEffect, useMemo, useRef, useState} from 'react';
import {createFileRoute} from '@tanstack/react-router';
import {FileKey2, LayoutGrid, List, Plus, Search} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {SectionHeader} from '#/components/settings/list-card';
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
import {
	SshKeysList,
	type SshKeysView,
} from '#/components/ssh-keys/ssh-keys-list';
import {SshKeyCreateSheet} from '#/components/ssh-keys/ssh-key-create-sheet';
import {ViewKeySheet} from '#/components/ssh-keys/view-key-sheet';
import {useSshKeyItems} from '#/api/live';
import {useDeleteSshKey} from '#/hooks/ssh-keys/use-ssh-keys';
import {
	getKeyType,
	KEY_TYPES,
	type KeyTypeFilter,
} from '#/components/ssh-keys/ssh-keys-shared';
import {cn} from '#/api/utils';

import type {SshKeyResponse} from '#/types/api-helpers';

export const Route = createFileRoute('/_app/ssh-keys')({
	component: SshKeysPage,
});

const VIEW_STORAGE_KEY = 'ssh-keys-view';

/** SSH Keys — the Projects-page grammar: a section title, then a toolbar (search + filter + New) and one bordered list card below; create opens a Geist sheet, delete a quiet AlertDialog. No side rail. */
function SshKeysPage() {
	const [search, setSearch] = useState('');
	const [typeFilter, setTypeFilter] = useState<KeyTypeFilter>('All');
	const [view, setView] = useState<SshKeysView>(() => {
		if (typeof window === 'undefined') return 'grid';
		return localStorage.getItem(VIEW_STORAGE_KEY) === 'list'
			? 'list'
			: 'grid';
	});
	const [isCreateOpen, setIsCreateOpen] = useState(false);

	const handleViewChange = (next: SshKeysView) => {
		setView(next);
		localStorage.setItem(VIEW_STORAGE_KEY, next);
	};
	const [selectedKeyForView, setSelectedKeyForView] =
		useState<SshKeyResponse | null>(null);
	const [keyForDelete, setKeyForDelete] = useState<SshKeyResponse | null>(
		null,
	);

	const sshKeysQuery = useSshKeyItems();
	const keys = useMemo(
		() => (sshKeysQuery.data ?? []) as unknown as SshKeyResponse[],
		[sshKeysQuery.data],
	);

	const filtered = useMemo(() => {
		return keys.filter(k => {
			const matchName = k.name
				.toLowerCase()
				.includes(search.toLowerCase());
			const matchType =
				typeFilter === 'All' || getKeyType(k.public_key) === typeFilter;
			return matchName && matchType;
		});
	}, [keys, search, typeFilter]);

	const hasFilters = search !== '' || typeFilter !== 'All';
	const clearFilters = () => {
		setSearch('');
		setTypeFilter('All');
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

	const handleViewKey = useCallback(
		(key: SshKeyResponse) => setSelectedKeyForView(key),
		[],
	);
	const handleCloseView = useCallback(
		() => setSelectedKeyForView(null),
		[],
	);

	return (
		<div className="mx-auto flex w-full max-w-[1920px] animate-in flex-col gap-6 px-4 pt-4 pb-6 duration-200 fade-in">
			<SectionHeader
				title="SSH Keys"
				description="Key pairs to authenticate with remote servers and Git repositories."
				action={
					<Button
						onClick={() => setIsCreateOpen(true)}
						className="h-9 gap-1.5">
						<Plus className="size-3.5" />
						New SSH Key
					</Button>
				}
			/>

			{/* Search + type filter + New SSH Key */}
			<div className="flex w-full animate-in flex-col items-stretch gap-3 duration-200 fade-in sm:flex-row sm:items-center">
				<div className="relative w-full sm:grow">
					<Search className="pointer-events-none absolute top-1/2 left-3 size-4 -translate-y-1/2 text-muted-foreground/70" />
					<Input
						ref={searchRef}
						placeholder="Search SSH Keys"
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
						value={typeFilter}
						onValueChange={v => setTypeFilter(v as KeyTypeFilter)}>
						<SelectTrigger className="h-9 w-auto rounded-lg bg-card">
							<SelectValue>
								{typeFilter === 'All' ? 'All Types' : typeFilter}
							</SelectValue>
						</SelectTrigger>
						<SelectContent className="bg-card">
							{KEY_TYPES.map(t => (
								<SelectItem key={t} value={t}>
									{t === 'All' ? 'All Types' : t}
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
			{!sshKeysQuery.isLoading && keys.length === 0 ? (
				<div className="flex flex-col items-center justify-center rounded-xl border bg-card p-12 text-center">
					<FileKey2 className="mb-4 size-10 text-muted-foreground/40" />
					<h3 className="mb-2 text-xl font-semibold tracking-[-0.01em] text-foreground">
						No SSH keys yet
					</h3>
					<p className="mx-auto max-w-sm text-sm leading-relaxed text-muted-foreground/70">
						Add a key pair to authenticate with remote servers and Git
						repositories.
					</p>
					<Button
						onClick={() => setIsCreateOpen(true)}
						className="mt-4 h-9 gap-1.5 text-xs font-semibold">
						<Plus className="size-3.5" />
						New SSH Key
					</Button>
				</div>
			) : (
				<SshKeysList
					keys={filtered}
					isLoading={sshKeysQuery.isLoading}
					hasFilters={hasFilters}
					view={view}
					onClearFilters={clearFilters}
					onViewKey={handleViewKey}
					onDeleteKey={setKeyForDelete}
				/>
			)}

			{/* Create SSH Key — Geist sheet, same as New Project */}
			<SshKeyCreateSheet
				isOpen={isCreateOpen}
				onOpenChange={setIsCreateOpen}
			/>

			{/* View SSH Key details — Geist side sheet */}
			{selectedKeyForView && (
				<ViewKeySheet
					isOpen={!!selectedKeyForView}
					sshKey={selectedKeyForView}
					onClose={handleCloseView}
				/>
			)}

			{keyForDelete && (
				<DeleteKeyDialog
					sshKey={keyForDelete}
					onClose={() => setKeyForDelete(null)}
				/>
			)}
		</div>
	);
}

/** Quiet AlertDialog confirm — the same grammar as Delete Project. */
function DeleteKeyDialog({
	sshKey,
	onClose,
}: {
	sshKey: SshKeyResponse;
	onClose: () => void;
}) {
	const {deleting, handleDelete} = useDeleteSshKey(
		Number(sshKey.id),
		onClose,
	);

	return (
		<AlertDialog open onOpenChange={open => !open && onClose()}>
			<AlertDialogContent>
				<AlertDialogHeader>
					<AlertDialogTitle>Delete SSH Key</AlertDialogTitle>
					<AlertDialogDescription>
						Are you sure you want to delete the key{' '}
						<strong className="text-foreground">{sshKey.name}</strong>? Any
						remote servers or deployments relying on it will lose automated
						access until updated.
					</AlertDialogDescription>
				</AlertDialogHeader>
				<AlertDialogFooter>
					<AlertDialogCancel onClick={onClose}>Cancel</AlertDialogCancel>
					<AlertDialogAction
						className="bg-danger text-white hover:bg-danger/90"
						disabled={deleting}
						onClick={e => {
							e.preventDefault();
							void handleDelete();
						}}>
						{deleting ? 'Deleting...' : 'Delete Key'}
					</AlertDialogAction>
				</AlertDialogFooter>
			</AlertDialogContent>
		</AlertDialog>
	);
}
