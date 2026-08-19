import {BitbucketIcon, GiteaIcon, GithubIcon, GitlabIcon} from '#/components/icons/data-tools-icons';
import {type GitProviderKind} from './git-provider-types';

interface GitProviderCreateButtonsProps {
	onSelectKind: (kind: GitProviderKind) => void;
}

export function GitProviderCreateButtons({onSelectKind}: GitProviderCreateButtonsProps) {
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
		<div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-3.5 mb-6">
			{providerCards.map((item) => (
				<button
					key={item.kind}
					type="button"
					onClick={() => onSelectKind(item.kind)}
					className={`group relative flex flex-col justify-between p-4 rounded-2xl border border-border/80 bg-card/60 hover:shadow-md transition-all duration-200 text-left cursor-pointer ${item.hoverClass}`}
				>
					<div className="flex items-start justify-between w-full mb-2">
						<div className="size-9 rounded-xl bg-muted/60 border border-border/60 flex items-center justify-center group-hover:scale-105 transition-transform">
							{item.icon}
						</div>
						<span className="text-[10px] font-semibold text-muted-foreground bg-muted/50 px-2 py-0.5 rounded-full border border-border/40">
							{item.badge}
						</span>
					</div>

					<div>
						<h4 className="text-sm font-bold text-foreground group-hover:text-primary transition-colors flex items-center gap-1.5">
							{item.name}
						</h4>
						<p className="text-xs text-muted-foreground mt-0.5 line-clamp-1">
							{item.description}
						</p>
					</div>
				</button>
			))}
		</div>
	);
}
