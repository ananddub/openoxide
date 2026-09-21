import {
	BitbucketIcon,
	GiteaIcon,
	GithubIcon,
	GitlabIcon,
} from '#/components/icons/data-tools-icons';
import type {GitProviderKind} from './git-provider-types';

/** Provider glyph used by the dialog title and cards. */
export function GitProviderIcon({
	kind,
	className,
}: {
	kind: GitProviderKind;
	className?: string;
}) {
	switch (kind) {
		case 'github':
			return (
				<GithubIcon className={className ?? 'size-4.5 text-foreground'} />
			);
		case 'gitlab':
			return (
				<GitlabIcon className={className ?? 'size-4.5 text-orange-500'} />
			);
		case 'bitbucket':
			return (
				<BitbucketIcon className={className ?? 'size-4.5 text-blue-500'} />
			);
		case 'gitea':
			return (
				<GiteaIcon className={className ?? 'size-4.5 text-emerald-500'} />
			);
		default:
			return <GithubIcon className={className ?? 'size-4.5'} />;
	}
}
