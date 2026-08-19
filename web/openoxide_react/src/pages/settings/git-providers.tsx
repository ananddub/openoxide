import {createFileRoute} from '@tanstack/react-router';
import {useState, useMemo} from 'react';
import {GitBranch, Search, ShieldCheck} from 'lucide-react';
import {Input} from '#/components/ui/input';
import {useGitProviders} from '#/components/settings/git-providers/use-git-providers';
import {GitProviderCreateButtons} from '#/components/settings/git-providers/git-provider-create-buttons';
import {GitProviderCard} from '#/components/settings/git-providers/git-provider-card';
import {GithubManifestDialog} from '#/components/settings/git-providers/github-manifest-dialog';
import {GitProviderEditDialog} from '#/components/settings/git-providers/git-provider-edit-dialog';

export const Route = createFileRoute('/_app/settings/git-providers')({
	component: GitProvidersPage,
});

function GitProvidersPage() {
	const {
		providers,
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

	const filteredProviders = useMemo(() => {
		if (!search.trim()) return providers;
		const q = search.toLowerCase();
		return providers.filter(
			p =>
				p.name?.toLowerCase().includes(q) ||
				p.provider_type?.toLowerCase().includes(q) ||
				p.config?.url?.toLowerCase().includes(q),
		);
	}, [providers, search]);

	return (
		<div className="mx-auto flex max-w-5xl animate-in flex-col gap-6 p-6 duration-200 fade-in">
			{/* Header Banner */}
			<div className="flex flex-col justify-between gap-4 sm:flex-row sm:items-center">
				<div>
					<h1 className="flex items-center gap-2.5 text-2xl font-bold tracking-tight text-foreground">
						<div className="flex size-9 items-center justify-center rounded-xl border border-primary/20 bg-primary/10 text-primary">
							<GitBranch className="size-5" />
						</div>
						Git Providers
					</h1>
					<p className="mt-1 text-xs text-muted-foreground">
						Connect your Git accounts (GitHub, GitLab, Bitbucket, Gitea)
						for automated repository deployments & webhooks.
					</p>
				</div>

				<div className="flex items-center gap-2 self-start rounded-xl border border-border/60 bg-muted/40 px-3 py-1.5 text-xs font-semibold text-muted-foreground sm:self-center">
					<ShieldCheck className="size-4 text-emerald-500" />
					<span>
						{providers.length}{' '}
						{providers.length === 1 ? 'Provider' : 'Providers'} Configured
					</span>
				</div>
			</div>

			{/* Interactive Provider Connect Cards */}
			<section>
				<h3 className="mb-3 text-xs font-bold tracking-wider text-muted-foreground uppercase">
					Connect New Provider
				</h3>
				<GitProviderCreateButtons onSelectKind={openCreate} />
			</section>

			{/* Existing Providers List */}
			<section className="flex flex-col gap-3">
				<div className="flex flex-wrap items-center justify-between gap-3">
					<h3 className="text-xs font-bold tracking-wider text-muted-foreground uppercase">
						Configured Git Providers
					</h3>

					{providers.length > 0 && (
						<div className="relative w-full sm:w-64">
							<Search className="absolute top-1/2 left-3 size-3.5 -translate-y-1/2 text-muted-foreground" />
							<Input
								placeholder="Search providers..."
								value={search}
								onChange={e => setSearch(e.target.value)}
								className="h-8 border-border/80 bg-muted/20 pl-8 text-xs"
							/>
						</div>
					)}
				</div>

				{filteredProviders.length === 0 ? (
					<div className="flex flex-col items-center justify-center rounded-2xl border border-dashed border-border/80 bg-muted/5 p-12 text-center">
						<GitBranch className="mb-2.5 size-10 text-muted-foreground/30" />
						<p className="text-sm font-semibold text-foreground">
							No Git Providers found
						</p>
						<p className="mt-1 max-w-sm text-xs text-muted-foreground">
							{search.trim()
								? 'No providers matched your search query.'
								: 'Click on one of the provider cards above to connect your first Git repository account.'}
						</p>
					</div>
				) : (
					<div className="flex flex-col gap-3">
						{filteredProviders.map(provider => (
							<GitProviderCard
								key={provider.id}
								provider={provider}
								onAuthorize={handleAuthorize}
								onTest={handleTest}
								onEdit={openEdit}
								onDelete={handleDelete}
							/>
						))}
					</div>
				)}
			</section>

			{/* Dialogs */}
			<GithubManifestDialog
				isOpen={manifestOpen}
				onClose={() => setManifestOpen(false)}
			/>

			<GitProviderEditDialog
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
