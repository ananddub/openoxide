import {
	BitbucketIcon,
	GiteaIcon,
	GithubIcon,
	GitlabIcon,
} from '#/components/icons/data-tools-icons';
import {type GitProviderKind} from './git-provider-types';

interface GitProviderCreateButtonsProps {
	onSelectKind: (kind: GitProviderKind) => void;
}

export function GitProviderCreateButtons({
	onSelectKind,
}: GitProviderCreateButtonsProps) {
	const providerCards = [
		{
			kind: 'github' as GitProviderKind,
			name: 'GitHub',
			description: 'Automatic App setup with webhooks',
			icon: <GithubIcon className="size-5 text-foreground" />,
			badge: 'Recommended',
			hoverClass: 'hover:border-zinc-500/50 hover:bg-zinc-500/5',
		},
		{
			kind: 'gitlab' as GitProviderKind,
			name: 'GitLab',
			description: 'OAuth2 app & token integration',
			icon: <GitlabIcon className="size-5 text-orange-500" />,
			badge: 'Cloud / Self-hosted',
			hoverClass: 'hover:border-orange-500/50 hover:bg-orange-500/5',
		},
		{
			kind: 'bitbucket' as GitProviderKind,
			name: 'Bitbucket',
			description: 'Workspace & API token access',
			icon: <BitbucketIcon className="size-5 text-blue-500" />,
			badge: 'Atlassian',
			hoverClass: 'hover:border-blue-500/50 hover:bg-blue-500/5',
		},
		{
			kind: 'gitea' as GitProviderKind,
			name: 'Gitea / Forgejo',
			description: 'Self-hosted lightweight Git server',
			icon: <GiteaIcon className="size-5 text-emerald-500" />,
			badge: 'Self-hosted',
			hoverClass: 'hover:border-emerald-500/50 hover:bg-emerald-500/5',
		},
	];

	return (
		<div className="mb-6 grid grid-cols-1 gap-3.5 sm:grid-cols-2 lg:grid-cols-4">
			{providerCards.map(item => (
				<button
					key={item.kind}
					type="button"
					onClick={() => onSelectKind(item.kind)}
					className={`group relative flex cursor-pointer flex-col justify-between rounded-2xl border border-border/80 bg-card/60 p-4 text-left transition-all duration-200 hover:shadow-md ${item.hoverClass}`}>
					<div className="mb-2 flex w-full items-start justify-between">
						<div className="flex size-9 items-center justify-center rounded-xl border border-border/60 bg-muted/60 transition-transform group-hover:scale-105">
							{item.icon}
						</div>
						<span className="rounded-full border border-border/40 bg-muted/50 px-2 py-0.5 text-[10px] font-semibold text-muted-foreground">
							{item.badge}
						</span>
					</div>

					<div>
						<h4 className="flex items-center gap-1.5 text-sm font-bold text-foreground transition-colors group-hover:text-primary">
							{item.name}
						</h4>
						<p className="mt-0.5 line-clamp-1 text-xs text-muted-foreground">
							{item.description}
						</p>
					</div>
				</button>
			))}
		</div>
	);
}
