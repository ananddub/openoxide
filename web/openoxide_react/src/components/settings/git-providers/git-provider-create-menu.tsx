import {ChevronDown, Plus} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {
	DropdownMenu,
	DropdownMenuContent,
	DropdownMenuItem,
	DropdownMenuTrigger,
} from '#/components/ui/dropdown';
import {
	BitbucketIcon,
	GiteaIcon,
	GithubIcon,
	GitlabIcon,
} from '#/components/icons/data-tools-icons';
import {type GitProviderKind} from './git-provider-types';

const KINDS: {
	kind: GitProviderKind;
	name: string;
	description: string;
	icon: React.ReactNode;
}[] = [
	{
		kind: 'github',
		name: 'GitHub',
		description: 'Automatic App setup with webhooks',
		icon: <GithubIcon className="size-4 text-foreground" />,
	},
	{
		kind: 'gitlab',
		name: 'GitLab',
		description: 'OAuth2 app & token integration',
		icon: <GitlabIcon className="size-4 text-orange-500" />,
	},
	{
		kind: 'bitbucket',
		name: 'Bitbucket',
		description: 'Workspace & API token access',
		icon: <BitbucketIcon className="size-4 text-blue-500" />,
	},
	{
		kind: 'gitea',
		name: 'Gitea / Forgejo',
		description: 'Self-hosted lightweight Git server',
		icon: <GiteaIcon className="size-4 text-emerald-500" />,
	},
];

/** "New provider" — the Add New dropdown grammar: icon, name, one line. */
export function GitProviderCreateMenu({
	onSelectKind,
}: {
	onSelectKind: (kind: GitProviderKind) => void;
}) {
	return (
		<DropdownMenu>
			<DropdownMenuTrigger
				render={
					<Button className="h-9 gap-1.5 px-3 text-sm font-medium">
						<Plus className="size-3.5" />
						New provider
						<ChevronDown className="size-3.5 opacity-60" />
					</Button>
				}
			/>
			<DropdownMenuContent align="end" className="min-w-[300px] p-1.5">
				<div className="grid grid-cols-1 gap-1">
					{KINDS.map(item => (
						<DropdownMenuItem
							key={item.kind}
							onClick={() => onSelectKind(item.kind)}
							className="flex-col items-start gap-0.5 rounded-lg px-2.5 py-2">
							<span className="flex items-center gap-1.5">
								{item.icon}
								<span className="text-[13px] font-medium text-foreground">
									{item.name}
								</span>
							</span>
							<span className="ps-[22px] text-[11px] leading-snug text-muted-foreground">
								{item.description}
							</span>
						</DropdownMenuItem>
					))}
				</div>
			</DropdownMenuContent>
		</DropdownMenu>
	);
}
