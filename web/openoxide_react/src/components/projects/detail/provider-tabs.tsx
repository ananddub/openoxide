import {UploadCloud, Code2} from 'lucide-react';
import {
	BitbucketIcon,
	DockerIcon,
	GiteaIcon,
	GithubIcon,
	GitIcon,
	GitlabIcon,
} from '#/components/icons/data-tools-icons';

type IconJsx = React.ReactElement;

export const APP_SOURCE_PROVIDERS: {
	id: string;
	label: string;
	icon: IconJsx;
}[] = [
	{
		id: 'GITHUB',
		label: 'GitHub',
		icon: <GithubIcon className="h-4 w-4" />,
	},
	{
		id: 'GITLAB',
		label: 'GitLab',
		icon: <GitlabIcon className="h-4.5 w-4.5" />,
	},
	{
		id: 'GITEA',
		label: 'Gitea',
		icon: <GiteaIcon className="h-4.5 w-4.5" />,
	},
	{
		id: 'BITBUCKET',
		label: 'Bitbucket',
		icon: <BitbucketIcon className="h-4 w-4" />,
	},
	{id: 'GIT', label: 'Git', icon: <GitIcon className="h-4.5 w-4.5" />},
	{
		id: 'DOCKER',
		label: 'Docker',
		icon: <DockerIcon className="h-4.5 w-4.5" />,
	},
	{id: 'DROP', label: 'Drop', icon: <UploadCloud className="h-4 w-4" />},
];

export const COMPOSE_SOURCE_PROVIDERS: {
	id: string;
	label: string;
	icon: IconJsx;
}[] = [
	{
		id: 'GITHUB',
		label: 'GitHub',
		icon: <GithubIcon className="h-4 w-4" />,
	},
	{
		id: 'GITLAB',
		label: 'GitLab',
		icon: <GitlabIcon className="h-4.5 w-4.5" />,
	},
	{
		id: 'GITEA',
		label: 'Gitea',
		icon: <GiteaIcon className="h-4.5 w-4.5" />,
	},
	{
		id: 'BITBUCKET',
		label: 'Bitbucket',
		icon: <BitbucketIcon className="h-4 w-4" />,
	},
	{id: 'GIT', label: 'Git', icon: <GitIcon className="h-4.5 w-4.5" />},
	{
		id: 'RAW',
		label: 'Raw YML',
		icon: <Code2 className="h-4 w-4 text-primary" />,
	},
];

/** Underline tab strip for picking a source provider. */
export function ProviderTabs({
	value,
	onChange,
	providers,
}: {
	value: string;
	onChange: (id: string) => void;
	providers: {id: string; label: string; icon: IconJsx}[];
}) {
	return (
		<div className="relative -mb-[1px] flex w-full flex-nowrap gap-0 overflow-x-auto border-b pb-1 [-ms-overflow-style:none] [scrollbar-width:none] [&::-webkit-scrollbar]:hidden">
			{providers.map(p => {
				const isActive = value === p.id;
				return (
					<button
						key={p.id}
						type="button"
						onClick={() => onChange(p.id)}
						className={`-mb-[1px] flex shrink-0 cursor-pointer items-center gap-1 whitespace-nowrap border-b-2 px-2.5 pt-2 pb-2 text-xs font-bold transition-all ${
							isActive
								? 'border-foreground font-bold text-foreground'
								: ' text-muted-foreground hover:text-foreground'
						}`}>
						{p.icon}
						{p.label}
					</button>
				);
			})}
		</div>
	);
}
