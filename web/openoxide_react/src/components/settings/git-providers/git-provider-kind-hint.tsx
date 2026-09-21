import {ExternalLink} from 'lucide-react';
import type {
	GitProviderKind,
	GitProviderFormState,
} from './git-provider-types';

/** Per-kind "where to create the app/token" instruction box. */
export function GitProviderKindHint({
	kind,
	form,
}: {
	kind: GitProviderKind;
	form: GitProviderFormState;
}) {
	const link = (href: string, text: string) => (
		<a
			className="inline-flex items-center gap-0.5 font-semibold text-primary underline"
			href={href}
			target="_blank"
			rel="noreferrer">
			{text} <ExternalLink className="size-3" />
		</a>
	);
	const code = (text: string) => (
		<code className="rounded bg-muted/60 px-1 py-0.5 font-mono text-[11px]">
			{text}
		</code>
	);

	return (
		<div className="rounded-xl bg-primary/5 p-3.5 text-xs text-muted-foreground">
			{kind === 'github' ? (
				<>
					Manual configuration. For automatic setup, use the{' '}
					<strong>Create GitHub App</strong> button.
				</>
			) : kind === 'gitlab' ? (
				<>
					Create an application at{' '}
					{link(
						`${form.url || 'https://gitlab.com'}/-/profile/applications`,
						'GitLab Applications',
					)}
					. Set Redirect URI to {code(form.redirect_uri)} with{' '}
					{code('api read_user read_repository')} scopes.
				</>
			) : kind === 'gitea' ? (
				<>
					Create an OAuth2 application at{' '}
					{link(
						`${form.url || 'https://gitea.com'}/user/settings/applications`,
						'Gitea Applications',
					)}
					. Set Redirect URI to {code(form.redirect_uri)}.
				</>
			) : (
				<>
					Create an API token at{' '}
					{link(
						'https://id.atlassian.com/manage-profile/security/api-tokens',
						'Bitbucket API Tokens',
					)}
					.
				</>
			)}
		</div>
	);
}
