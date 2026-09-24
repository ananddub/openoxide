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
