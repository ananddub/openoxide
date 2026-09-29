import {useEffect, useState} from 'react';
import {Check} from 'lucide-react';
import {toast} from 'sonner';
import {Button} from '#/components/ui/button';
import {
	useApplicationSetBitbucketSource,
	useApplicationSetCustomGitSource,
	useApplicationSetDockerSource,
	useApplicationSetGiteaSource,
	useApplicationSetGithubSource,
	useApplicationSetGitlabSource,
} from '#/api/live';
import {cn, formatApiError} from '#/api/utils';
import {SheetGroup} from '#/components/shared/field';
import {SettingsSheet} from '#/components/shared/settings-sheet';
import {APP_SOURCE_PROVIDERS} from '#/components/projects/detail/provider-tabs';
import {DropSourceForm} from './drop-source-form';
import {DockerSourceForm} from './source-settings-card/docker-source-form';
import {GitSourceForm} from './source-settings-card/git-source-form';
import {RepoSourceForm} from './source-settings-card/repo-source-form';
import {
	sourceFormFromApp,
	type SourceForm,
} from './source-settings-card/source-form';

interface SourceSettingsCardProps {
	app: any;
	onUpdated: () => void;
	/** Header-icon trigger: the caller owns visibility, no strip is painted. */
	open?: boolean;
	onOpenChange?: (open: boolean) => void;
}

/** Titled config group per provider family. */
function groupFor(provider: string): {title: string; description: string} {
	if (provider === 'DOCKER')
		return {
			title: 'Docker image',
			description:
				'Deploy a prebuilt image, optionally from a private registry.',
		};
	if (provider === 'GIT')
		return {
			title: 'Git repository',
			description: 'Deploy from any Git host by clone URL.',
		};
	if (provider === 'DROP')
		return {
			title: 'Deploy drop',
			description:
				'Upload a build artifact instead of connecting a repository.',
		};
	return {
		title: 'Repository',
		description: 'Pick the repository, branch and build path.',
	};
}

/** One compact sheet: provider chips on top, the selected provider's config right below — same grammar as the compose Source & Provider sheet. */
export function SourceSettingsCard({
	app,
	onUpdated,
	open,
	onOpenChange,
}: SourceSettingsCardProps) {
	const [provider, setProvider] = useState(app.source_type || 'GIT');
	const [form, setForm] = useState<SourceForm>(() =>
		sourceFormFromApp(app),
	);
	const [savingSource, setSavingSource] = useState(false);

	const patchDocker = useApplicationSetDockerSource();
	const patchGit = useApplicationSetCustomGitSource();
	const patchGithub = useApplicationSetGithubSource();
	const patchGitlab = useApplicationSetGitlabSource();
	const patchGitea = useApplicationSetGiteaSource();
	const patchBitbucket = useApplicationSetBitbucketSource();

	useEffect(() => {
		if (app) {
			setProvider(app.source_type || 'GIT');
			setForm(sourceFormFromApp(app));
		}
	}, [app]);

	const onPatch = (patch: Partial<SourceForm>) =>
		setForm(f => ({...f, ...patch}));

	const handleSaveSource = async () => {
		setSavingSource(true);
		try {
			const {
				gitUrl,
				gitBranch,
				gitBuildPath,
				dockerImage,
				registryUrl,
				dockerUsername,
				dockerPassword,
				sshKeyId,
				repoOwner,
				repoName,
			} = form;
			if (provider === 'DOCKER') {
				await patchDocker.mutateAsync({
					id: app.id,
					image: dockerImage,
					username: dockerUsername || undefined,
					password: dockerPassword || undefined,
					registry_url: registryUrl || undefined,
				});
			} else if (provider === 'GIT') {
				await patchGit.mutateAsync({
					id: app.id,
					url: gitUrl,
					repository: repoName || gitUrl,
					owner: repoOwner || '',
					branch: gitBranch || 'main',
					build_path: gitBuildPath || '/',
					ssh_key_id: sshKeyId || undefined,
				});
			} else if (provider === 'GITHUB') {
				await patchGithub.mutateAsync({
					id: app.id,
					owner: repoOwner,
					repository: repoName,
					branch: gitBranch,
					build_path: gitBuildPath,
				});
			} else if (provider === 'GITLAB') {
				await patchGitlab.mutateAsync({
					id: app.id,
					owner: repoOwner,
					repository: repoName,
					branch: gitBranch,
					build_path: gitBuildPath,
				});
			} else if (provider === 'GITEA') {
				await patchGitea.mutateAsync({
					id: app.id,
					owner: repoOwner,
					repository: repoName,
					branch: gitBranch,
					build_path: gitBuildPath,
				});
			} else if (provider === 'BITBUCKET') {
				await patchBitbucket.mutateAsync({
					id: app.id,
					owner: repoOwner,
					repository: repoName,
					branch: gitBranch,
				});
			}
			toast.success('Source configuration saved');
			onUpdated();
		} catch (err: unknown) {
			toast.error(formatApiError(err));
		} finally {
			setSavingSource(false);
		}
	};

	const group = groupFor(provider);

	return (
		<SettingsSheet
			title="Source & Provider Configuration"
			description="Where this application pulls its code or image from."
			open={open}
			onOpenChange={onOpenChange}
			/* Form-heavy sheet — roomier than the default 560px rail. */
			contentClassName="sm:data-[side=right]:w-[820px]"
			footer={
				provider === 'DROP' ? undefined : (
					<Button
						onClick={() => void handleSaveSource()}
						disabled={savingSource}
						className="h-8 gap-1.5 px-4 text-xs">
						{savingSource ? 'Saving…' : 'Save changes'}
					</Button>
				)
			}>
			<SheetGroup title="Source">
				{/* Compact provider picker — a chip grid, not radio rows.
				    Drop is disabled until the artifact pipeline is ready. */}
				<div className="grid grid-cols-2 gap-2 sm:grid-cols-3">
					{APP_SOURCE_PROVIDERS.map(p => {
						const selected = provider === p.id;
						const disabled = p.id === 'DROP';
						return (
							<button
								key={p.id}
								type="button"
								onClick={() => !disabled && setProvider(p.id)}
								aria-pressed={selected}
								disabled={disabled}
								title={disabled ? 'Deploy drop is coming soon' : undefined}
								className={cn(
									'flex cursor-pointer items-center gap-2.5 rounded-lg border px-3 py-2.5 text-sm font-medium transition-colors',
									selected
										? 'border-primary/50 bg-primary/[0.08] text-foreground'
										: 'border-border/60 text-muted-foreground hover:bg-muted/30 hover:text-foreground',
									disabled &&
										'cursor-not-allowed border-border/40 opacity-50 hover:bg-transparent',
								)}>
								<span className="shrink-0">{p.icon}</span>
								<span className="truncate">{p.label}</span>
								{selected && (
									<Check className="ml-auto size-3.5 shrink-0 text-primary" />
								)}
							</button>
						);
					})}
				</div>
			</SheetGroup>

			<SheetGroup title={group.title} description={group.description}>
				{provider === 'DROP' ? (
					<DropSourceForm app={app} onUpdated={onUpdated} />
				) : provider === 'DOCKER' ? (
					<DockerSourceForm form={form} onPatch={onPatch} />
				) : provider === 'GIT' ? (
					<GitSourceForm form={form} onPatch={onPatch} />
				) : (
					<RepoSourceForm form={form} onPatch={onPatch} />
				)}
			</SheetGroup>
		</SettingsSheet>
	);
}
