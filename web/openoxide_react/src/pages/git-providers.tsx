import {createFileRoute} from '@tanstack/react-router';
import {GitBranch, Search} from 'lucide-react';
import {useState, useMemo} from 'react';
import {Button} from '#/components/ui/button';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import {useGitProviders} from '#/components/settings/git-providers/use-git-providers';
import {GitProviderCreateMenu} from '#/components/settings/git-providers/git-provider-create-menu';
import {GitProviderList} from '#/components/settings/git-providers/git-provider-list';
import {GithubManifestDialog} from '#/components/settings/git-providers/github-manifest-dialog';
import {GitProviderEditSheet} from '#/components/settings/git-providers/git-provider-edit-sheet';
import {ConfirmDeleteDialog} from '#/components/settings/confirm-delete-dialog';
import {
	GridSkeleton,
	ListEmpty,
	ListSkeleton,
	SectionHeader,
	Toolbar,
	ToolbarSearch,
	ViewToggle,
	useListView,
} from '#/components/settings/list-card';
import {GIT_PROVIDER_LABELS} from '#/components/settings/git-providers/git-provider-types';

export const Route = createFileRoute('/_app/git-providers')({
	component: GitProvidersPage,
});

const KINDS = Object.keys(GIT_PROVIDER_LABELS) as Array<
	keyof typeof GIT_PROVIDER_LABELS
>;

export function GitProvidersPage() {
	const {
		providers,
		isLoading,
		open,
		manifestOpen,
		editing,
		kind,
		busy,
		form,
		setOpen,
		setManifestOpen,
		setField,
		openCreate,
		openEdit,
		handleAuthorize,
		handleTest,
		handleDelete,
		save,
	} = useGitProviders();

	const [search, setSearch] = useState('');
	const [typeFilter, setTypeFilter] = useState('All');
	const [view, setView] = useListView('settings-git-view');
	const [deletingId, setDeletingId] = useState<number | null>(null);
	const deleting = providers.find(p => p.id === deletingId);

	const q = search.trim().toLowerCase();
	const filteredProviders = useMemo(
		() =>
			providers.filter(p => {
				if (typeFilter !== 'All' && p.provider_type !== typeFilter)
					return false;
				if (!q) return true;
				return [p.name, p.provider_type, p.config?.url]
					.filter(Boolean)
					.join(' ')
					.toLowerCase()
					.includes(q);
			}),
		[providers, q, typeFilter],
	);
	const hasFilters = q !== '' || typeFilter !== 'All';

	return (
		<div className="mx-auto flex w-full max-w-[1920px] animate-in flex-col gap-5 px-4 pt-4 pb-6 duration-200 fade-in">
			<SectionHeader
				title="Git Providers"
				description="Connect GitHub, GitLab, Bitbucket or Gitea once and every project reuses the authorization for repository deployments and webhooks."
				action={<GitProviderCreateMenu onSelectKind={openCreate} />}
			/>

			<Toolbar
				action={
					<>
						<Select
							value={typeFilter}
							onValueChange={v => v && setTypeFilter(v)}>
							<SelectTrigger className="h-9 w-auto rounded-lg bg-card">
								<SelectValue>
									{typeFilter === 'All'
										? 'All Providers'
										: GIT_PROVIDER_LABELS[
												typeFilter as keyof typeof GIT_PROVIDER_LABELS
											]}
								</SelectValue>
							</SelectTrigger>
							<SelectContent className="bg-card">
								<SelectItem value="All">All Providers</SelectItem>
								{KINDS.map(k => (
									<SelectItem key={k} value={k}>
										{GIT_PROVIDER_LABELS[k]}
									</SelectItem>
								))}
							</SelectContent>
						</Select>
						<ViewToggle view={view} onChange={setView} />
					</>
				}>
				<ToolbarSearch
					value={search}
					onChange={setSearch}
					placeholder="Search by name, host or type…"
					className="min-w-[200px] flex-1 sm:max-w-none"
				/>
			</Toolbar>

			{isLoading ? (
				view === 'grid' ? (
					<GridSkeleton />
				) : (
					<ListSkeleton />
				)
			) : filteredProviders.length === 0 ? (
				hasFilters && providers.length > 0 ? (
					<ListEmpty
						icon={Search}
						title="No providers match your search"
						description="Try a different name, host or provider type."
						action={
							<Button
								variant="ghost"
								onClick={() => {
									setSearch('');
									setTypeFilter('All');
								}}
								className="text-xs font-semibold text-primary">
								Clear All Filters
							</Button>
						}
					/>
				) : (
					<ListEmpty
						icon={GitBranch}
						title="No Git providers yet"
						description="Connect GitHub, GitLab, Bitbucket or Gitea once and every project reuses the authorization for repository deployments and webhooks."
					/>
				)
			) : (
				<GitProviderList
					providers={filteredProviders}
					view={view}
					onAuthorize={handleAuthorize}
					onTest={handleTest}
					onEdit={openEdit}
					onDelete={setDeletingId}
				/>
			)}

			<ConfirmDeleteDialog
				open={deletingId !== null}
				onOpenChange={o => !o && setDeletingId(null)}
				title="Delete Git provider"
				description={
					<span>
						Repositories using{' '}
						<span className="font-medium text-foreground">
							{deleting?.name}
						</span>{' '}
						will need to be reconnected. This cannot be undone.
					</span>
				}
				confirmLabel="Delete provider"
				onConfirm={() => {
					if (deletingId !== null) void handleDelete(deletingId);
					setDeletingId(null);
				}}
			/>

			{/* Dialogs */}
			<GithubManifestDialog
				isOpen={manifestOpen}
				onClose={() => setManifestOpen(false)}
			/>

			<GitProviderEditSheet
				isOpen={open}
				kind={kind}
				editing={editing}
				form={form}
				busy={busy}
				onClose={() => setOpen(false)}
				onChangeField={setField}
				onSave={save}
			/>
		</div>
	);
}
