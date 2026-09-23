import {useEffect, useState} from 'react';
import {Save} from 'lucide-react';
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
import {DropSourceForm} from './drop-source-form';
import {DockerSourceForm} from './source-settings-card/docker-source-form';
import {GitSourceForm} from './source-settings-card/git-source-form';
import {
	APP_SOURCE_PROVIDERS,
	ProviderTabs,
} from '#/components/projects/detail/provider-tabs';
import {RepoSourceForm} from './source-settings-card/repo-source-form';
import {
	sourceFormFromApp,
	type SourceForm,
} from './source-settings-card/source-form';

interface SourceSettingsCardProps {
	app: any;
	onUpdated: () => void;
	embedded?: boolean;
}

export function SourceSettingsCard({
	app,
	onUpdated,
	embedded,
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
		} catch (err: any) {
			toast.error(formatApiError(err));
		} finally {
			setSavingSource(false);
		}
	};

	return (
		<section
			className={cn(
				'flex flex-col gap-4',
				!embedded && 'rounded-xl bg-card p-5',
			)}>
			{!embedded && (
				<div>
					<h3 className="text-sm font-bold text-foreground">
						Provider Source
					</h3>
					<p className="mt-1 text-xs text-muted-foreground">
						Select and configure the repository source code of your
						application
					</p>
				</div>
			)}

			<ProviderTabs
				value={provider}
				onChange={setProvider}
				providers={APP_SOURCE_PROVIDERS}
			/>

			{provider === 'DROP' ? (
				<DropSourceForm app={app} onUpdated={onUpdated} />
			) : (
				<>
					{provider === 'DOCKER' ? (
						<DockerSourceForm form={form} onPatch={onPatch} />
					) : provider === 'GIT' ? (
						<GitSourceForm form={form} onPatch={onPatch} />
					) : (
						<RepoSourceForm form={form} onPatch={onPatch} />
					)}

					<div className="mt-2 flex justify-end">
						<Button
							onClick={handleSaveSource}
							disabled={savingSource}
							className="flex h-9 items-center gap-1.5 rounded-lg bg-primary text-xs font-semibold text-primary-foreground hover:bg-primary/95">
							<Save className="h-3.5 w-3.5" />{' '}
							{savingSource ? 'Saving...' : 'Save Source'}
						</Button>
					</div>
				</>
			)}
		</section>
	);
}
