import {createFileRoute, Navigate} from '@tanstack/react-router';
import {useState, useMemo} from 'react';
import {GitBranch, Search, Plug} from 'lucide-react';
import {Input} from '#/components/ui/input';
import {useGitProviders} from '#/components/settings/git-providers/use-git-providers';
import {GitProviderCreateButtons} from '#/components/settings/git-providers/git-provider-create-buttons';
import {GitProviderCard} from '#/components/settings/git-providers/git-provider-card';
import {GithubManifestDialog} from '#/components/settings/git-providers/github-manifest-dialog';
import {GitProviderEditDialog} from '#/components/settings/git-providers/git-provider-edit-dialog';
import {SettingsSection} from '#/components/settings/settings-section';

export const Route = createFileRoute('/_app/settings/git-providers')({
	component: () => (
		<Navigate to="/settings" search={{tab: 'git'}} replace />
	),
});

export function GitProvidersPage() {
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
		<div className="flex w-full animate-in flex-col gap-6 duration-200 fade-in">
			<SettingsSection
				icon={Plug}
				title="Connect New Provider"
				description="GitHub, GitLab, Bitbucket, or Gitea — for automated repository deployments and webhooks.">
				<GitProviderCreateButtons onSelectKind={openCreate} />
			</SettingsSection>

			<SettingsSection
				icon={GitBranch}
				title="Configured Git Providers"
				description={`${providers.length} ${providers.length === 1 ? 'provider' : 'providers'} connected to this organization.`}>
				<div className="mb-3 flex flex-wrap items-center justify-between gap-3">
					{providers.length > 0 && (
						<div className="relative w-full sm:w-64">
							<Search className="absolute top-1/2 left-3 size-3.5 -translate-y-1/2 text-muted-foreground" />
							<Input
								placeholder="Search providers..."
								value={search}
								onChange={e => setSearch(e.target.value)}
								className="h-8  bg-muted/20 pl-8 text-xs"
							/>
						</div>
					)}
				</div>
				{filteredProviders.length === 0 ? (
					<div className="flex flex-col items-center justify-center rounded-xl bg-muted/20 py-16 text-center">
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
			</SettingsSection>

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
