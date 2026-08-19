import {ExternalLink, RefreshCw} from 'lucide-react';
import {BitbucketIcon, GiteaIcon, GithubIcon, GitlabIcon} from '#/components/icons/data-tools-icons';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {
	Dialog,
	DialogContent,
	DialogDescription,
	DialogHeader,
	DialogTitle,
} from '#/components/ui/dialog';
import {
	type GitProviderKind,
	type GitProviderFormState,
	GIT_PROVIDER_LABELS,
} from './git-provider-types';

interface GitProviderEditDialogProps {
	isOpen: boolean;
	kind: GitProviderKind;
	editing: any;
	form: GitProviderFormState;
	busy: boolean;
	onClose: () => void;
	onChangeField: (key: string, value: string) => void;
	onSave: () => Promise<void>;
}

export function GitProviderEditDialog({
	isOpen,
	kind,
	editing,
	form,
	busy,
	onClose,
	onChangeField,
	onSave,
}: GitProviderEditDialogProps) {
	const renderIcon = (type: GitProviderKind) => {
		switch (type) {
			case 'github':
				return <GithubIcon className="size-5" />;
			case 'gitlab':
				return <GitlabIcon className="size-5" />;
			case 'bitbucket':
				return <BitbucketIcon className="size-5" />;
			case 'gitea':
				return <GiteaIcon className="size-5" />;
			default:
				return <GithubIcon className="size-5" />;
		}
	};

	const renderField = (key: string, label: string, secret = false, placeholder = '') => (
		<div className="space-y-1">
			<label className="text-xs font-medium">{label}</label>
			<Input
				type={secret ? 'password' : 'text'}
				value={(form as any)[key] || ''}
				placeholder={placeholder}
				onChange={(e) => onChangeField(key, e.target.value)}
			/>
		</div>
	);

	return (
		<Dialog open={isOpen} onOpenChange={(open) => !open && onClose()}>
			<DialogContent className="max-h-[90vh] overflow-y-auto sm:max-w-xl">
				<DialogHeader>
					<DialogTitle className="flex items-center gap-2">
						{renderIcon(kind)} {editing ? 'Edit' : 'Add'} {GIT_PROVIDER_LABELS[kind]}
					</DialogTitle>
					<DialogDescription>
						{kind === 'github'
							? 'Create and install a GitHub App.'
							: kind === 'gitlab'
							? 'Create an OAuth application in GitLab, then authorize it.'
							: kind === 'gitea'
							? 'Create an OAuth2 application in Gitea, then authorize it.'
							: 'Use a Bitbucket API token for repository access.'}
					</DialogDescription>
				</DialogHeader>

				<div className="mb-2 rounded-lg border bg-muted/30 p-3 text-xs text-muted-foreground">
					{kind === 'github' ? (
						<>Use GitHub App creation, then install the app for the required account.</>
					) : kind === 'gitlab' ? (
						<>
							Create an application at{' '}
							<a
								className="text-primary underline"
								href={`${form.url || 'https://gitlab.com'}/-/profile/applications`}
								target="_blank"
								rel="noreferrer"
							>
								GitLab Applications <ExternalLink className="inline size-3" />
							</a>
							. Redirect URI: <code>{form.redirect_uri}</code>. Scopes:{' '}
							<code>api read_user read_repository</code>.
						</>
					) : kind === 'gitea' ? (
						<>
							Create an OAuth2 application at{' '}
							<a
								className="text-primary underline"
								href={`${form.url || 'https://gitea.com'}/user/settings/applications`}
								target="_blank"
								rel="noreferrer"
							>
								Gitea Applications <ExternalLink className="inline size-3" />
							</a>
							. Redirect URI: <code>{form.redirect_uri}</code>.
						</>
					) : (
						<>
							Bitbucket App Passwords are deprecated for new providers. Create an API token at{' '}
							<a
								className="text-primary underline"
								href="https://id.atlassian.com/manage-profile/security/api-tokens"
								target="_blank"
								rel="noreferrer"
							>
								Bitbucket settings <ExternalLink className="inline size-3" />
							</a>
							.
						</>
					)}
				</div>

				<div className="grid gap-3">
					{renderField('name', 'Provider name', false, 'my-personal-account')}

					{kind === 'github' && (
						<>
							{renderField('app_name', 'App name')}
							{renderField('app_id', 'App ID')}
							{renderField('installation_id', 'Installation ID')}
							{renderField('client_id', 'Client ID')}
							{renderField('client_secret', 'Client secret', true)}
							{renderField('private_key', 'Private key', true)}
						</>
					)}

					{(kind === 'gitlab' || kind === 'gitea') && (
						<>
							<div className="grid grid-cols-2 gap-3">
								{renderField('url', 'Public URL', false, kind === 'gitlab' ? 'https://gitlab.com' : 'https://gitea.com')}
								{renderField('internal_url', 'Internal URL')}
							</div>
							{renderField('redirect_uri', 'Redirect URI')}
							{kind === 'gitlab' ? (
								<>
									{renderField('application_id', 'Application ID')}
									{renderField('secret', 'Application Secret', true)}
									{renderField('group_name', 'Group name')}
								</>
							) : (
								<>
									{renderField('client_id', 'Client ID')}
									{renderField('client_secret', 'Client Secret', true)}
								</>
							)}
						</>
					)}

					{kind === 'bitbucket' && (
						<>
							<div className="grid grid-cols-2 gap-3">
								{renderField('username', 'Username')}
								{renderField('email', 'Email')}
							</div>
							{renderField('api_token', 'API token', true)}
							{renderField('workspace', 'Workspace name')}
							{renderField('app_password', 'App password (legacy)', true)}
						</>
					)}

					<label className="flex items-center gap-2 text-sm">
						<input
							type="checkbox"
							checked={form.shared === 'true'}
							onChange={(e) => onChangeField('shared', String(e.target.checked))}
						/>{' '}
						Share with organization
					</label>

					<div className="flex justify-between border-t pt-4">
						<Button variant="secondary" onClick={onClose}>
							Cancel
						</Button>
						<Button onClick={onSave} disabled={busy}>
							{busy && <RefreshCw className="mr-1.5 size-4 animate-spin" />}
							{editing ? 'Save changes' : kind === 'github' ? 'Create provider' : 'Save and authorize'}
						</Button>
					</div>
				</div>
			</DialogContent>
		</Dialog>
	);
}
