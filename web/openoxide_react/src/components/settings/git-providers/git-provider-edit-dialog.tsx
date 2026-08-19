import {ExternalLink, RefreshCw, Save} from 'lucide-react';
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
import {type GitProviderKind, type GitProviderFormState, GIT_PROVIDER_LABELS} from './git-provider-types';

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
			case 'github': return <GithubIcon className="size-4.5 text-foreground" />;
			case 'gitlab': return <GitlabIcon className="size-4.5 text-orange-500" />;
			case 'bitbucket': return <BitbucketIcon className="size-4.5 text-blue-500" />;
			case 'gitea': return <GiteaIcon className="size-4.5 text-emerald-500" />;
			default: return <GithubIcon className="size-4.5" />;
		}
	};

	const renderField = (key: string, label: string, secret = false, placeholder = '') => (
		<div className="flex flex-col gap-1">
			<label className="text-xs font-semibold text-muted-foreground">{label}</label>
			<Input
				type={secret ? 'password' : 'text'}
				value={(form as any)[key] || ''}
				placeholder={placeholder}
				onChange={(e) => onChangeField(key, e.target.value)}
				className="h-9 text-xs font-mono"
			/>
		</div>
	);

	return (
		<Dialog open={isOpen} onOpenChange={(open) => !open && onClose()}>
			<DialogContent className="max-h-[90vh] overflow-y-auto sm:max-w-xl">
				<DialogHeader>
					<DialogTitle className="flex items-center gap-2.5 text-base font-bold">
						<div className="size-8 rounded-lg bg-muted/60 border border-border/60 flex items-center justify-center">
							{renderIcon(kind)}
						</div>
						{editing ? 'Edit' : 'Add'} {GIT_PROVIDER_LABELS[kind]}
					</DialogTitle>
					<DialogDescription className="text-xs">
						{kind === 'github'
							? 'Configure and save your GitHub App credentials.'
							: kind === 'gitlab'
							? 'Configure an OAuth application in GitLab, then authorize it.'
							: kind === 'gitea'
							? 'Configure an OAuth2 application in Gitea, then authorize it.'
							: 'Use a Bitbucket API token for repository access.'}
					</DialogDescription>
				</DialogHeader>

				<div className="rounded-xl border border-primary/20 bg-primary/5 p-3.5 text-xs text-muted-foreground">
					{kind === 'github' ? (
						<>Manual configuration. For automatic setup, use the <strong>Create GitHub App</strong> button.</>
					) : kind === 'gitlab' ? (
						<>
							Create an application at{' '}
							<a className="text-primary font-semibold underline inline-flex items-center gap-0.5" href={`${form.url || 'https://gitlab.com'}/-/profile/applications`} target="_blank" rel="noreferrer">
								GitLab Applications <ExternalLink className="size-3" />
							</a>
							. Set Redirect URI to <code className="font-mono bg-muted/60 px-1 py-0.5 rounded text-[11px]">{form.redirect_uri}</code> with <code className="font-mono bg-muted/60 px-1 py-0.5 rounded text-[11px]">api read_user read_repository</code> scopes.
						</>
					) : kind === 'gitea' ? (
						<>
							Create an OAuth2 application at{' '}
							<a className="text-primary font-semibold underline inline-flex items-center gap-0.5" href={`${form.url || 'https://gitea.com'}/user/settings/applications`} target="_blank" rel="noreferrer">
								Gitea Applications <ExternalLink className="size-3" />
							</a>
							. Set Redirect URI to <code className="font-mono bg-muted/60 px-1 py-0.5 rounded text-[11px]">{form.redirect_uri}</code>.
						</>
					) : (
						<>
							Create an API token at{' '}
							<a className="text-primary font-semibold underline inline-flex items-center gap-0.5" href="https://id.atlassian.com/manage-profile/security/api-tokens" target="_blank" rel="noreferrer">
								Bitbucket API Tokens <ExternalLink className="size-3" />
							</a>
							.
						</>
					)}
				</div>

				<div className="flex flex-col gap-3.5 pt-1">
					{renderField('name', 'Provider Name', false, 'my-personal-account')}

					{kind === 'github' && (
						<div className="grid grid-cols-1 md:grid-cols-2 gap-3">
							{renderField('app_name', 'App Name')}
							{renderField('app_id', 'App ID')}
							{renderField('installation_id', 'Installation ID')}
							{renderField('client_id', 'Client ID')}
							{renderField('client_secret', 'Client Secret', true)}
							{renderField('private_key', 'Private Key', true)}
						</div>
					)}

					{(kind === 'gitlab' || kind === 'gitea') && (
						<>
							<div className="grid grid-cols-1 md:grid-cols-2 gap-3">
								{renderField('url', 'Public URL', false, kind === 'gitlab' ? 'https://gitlab.com' : 'https://gitea.com')}
								{renderField('internal_url', 'Internal URL')}
							</div>
							{renderField('redirect_uri', 'Redirect URI')}
							{kind === 'gitlab' ? (
								<div className="grid grid-cols-1 md:grid-cols-2 gap-3">
									{renderField('application_id', 'Application ID')}
									{renderField('secret', 'Application Secret', true)}
									<div className="md:col-span-2">{renderField('group_name', 'Group Name')}</div>
								</div>
							) : (
								<div className="grid grid-cols-1 md:grid-cols-2 gap-3">
									{renderField('client_id', 'Client ID')}
									{renderField('client_secret', 'Client Secret', true)}
								</div>
							)}
						</>
					)}

					{kind === 'bitbucket' && (
						<>
							<div className="grid grid-cols-1 md:grid-cols-2 gap-3">
								{renderField('username', 'Username')}
								{renderField('email', 'Email')}
								{renderField('api_token', 'API Token', true)}
								{renderField('workspace', 'Workspace Name')}
							</div>
							{renderField('app_password', 'App Password (Legacy)', true)}
						</>
					)}

					<label className="flex items-center gap-2 text-xs font-semibold text-foreground cursor-pointer pt-1">
						<input
							type="checkbox"
							checked={form.shared === 'true'}
							onChange={(e) => onChangeField('shared', String(e.target.checked))}
							className="rounded border-border size-4 accent-primary"
						/>
						Share this Git Provider with all organization members
					</label>

					<div className="flex justify-end gap-2 border-t border-border/40 pt-4">
						<Button variant="outline" size="sm" onClick={onClose} className="h-9 px-4 text-xs font-semibold">
							Cancel
						</Button>
						<Button onClick={onSave} disabled={busy} className="h-9 px-4 bg-primary text-primary-foreground font-semibold text-xs gap-1.5 rounded-lg">
							{busy ? <RefreshCw className="size-3.5 animate-spin" /> : <Save className="size-3.5" />}
							{editing ? 'Save Changes' : kind === 'github' ? 'Create Provider' : 'Save & Authorize'}
						</Button>
					</div>
				</div>
			</DialogContent>
		</Dialog>
	);
}
