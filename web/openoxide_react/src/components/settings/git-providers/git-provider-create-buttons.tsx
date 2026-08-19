import {Button} from '#/components/ui/button';
import {BitbucketIcon, GiteaIcon, GithubIcon, GitlabIcon} from '#/components/icons/data-tools-icons';
import {type GitProviderKind, GIT_PROVIDER_LABELS} from './git-provider-types';

interface GitProviderCreateButtonsProps {
	onSelectKind: (kind: GitProviderKind) => void;
}

export function GitProviderCreateButtons({onSelectKind}: GitProviderCreateButtonsProps) {
	const providerButtons = [
		{
			kind: 'github' as GitProviderKind,
			icon: <GithubIcon className="size-4" />,
			className: 'hover:border-zinc-500',
		},
		{
			kind: 'gitlab' as GitProviderKind,
			icon: <GitlabIcon className="size-4" />,
			className: 'hover:border-orange-500',
		},
		{
			kind: 'bitbucket' as GitProviderKind,
			icon: <BitbucketIcon className="size-4" />,
			className: 'bg-blue-700 text-white hover:bg-blue-600',
		},
		{
			kind: 'gitea' as GitProviderKind,
			icon: <GiteaIcon className="size-4" />,
			className: 'hover:border-green-500',
		},
	];

	return (
		<div className="mb-6 flex flex-wrap gap-3">
			{providerButtons.map((button) => (
				<Button
					key={button.kind}
					variant="secondary"
					className={`flex-1 justify-center gap-2 ${button.className}`}
					onClick={() => onSelectKind(button.kind)}
				>
					{button.icon}
					{GIT_PROVIDER_LABELS[button.kind]}
				</Button>
			))}
		</div>
	);
}
