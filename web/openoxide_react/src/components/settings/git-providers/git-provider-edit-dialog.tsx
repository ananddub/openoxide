import {RefreshCw, Save} from 'lucide-react';
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
import {GitProviderIcon} from './git-provider-icon';
import {GitProviderKindHint} from './git-provider-kind-hint';

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

const KIND_DESCRIPTIONS: Record<GitProviderKind, string> = {
	github: 'Configure and save your GitHub App credentials.',
	gitlab: 'Configure an OAuth application in GitLab, then authorize it.',
	gitea: 'Configure an OAuth2 application in Gitea, then authorize it.',
	bitbucket: 'Use a Bitbucket API token for repository access.',
};

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
	const renderField = (
		key: string,
		label: string,
		secret = false,
		placeholder = '',
	) => (
		<div className="flex flex-col gap-1">
			<label className="text-xs font-semibold text-muted-foreground">
				{label}
			</label>
			<Input
				type={secret ? 'password' : 'text'}
				value={(form as any)[key] || ''}
				placeholder={placeholder}
				onChange={e => onChangeField(key, e.target.value)}
				className="h-9 font-mono text-xs"
			/>
		</div>
	);

	return (
		<Dialog open={isOpen} onOpenChange={open => !open && onClose()}>
			<DialogContent className="max-h-[90vh] overflow-y-auto sm:max-w-xl">
				<DialogHeader>
					<DialogTitle className="flex items-center gap-2.5 text-base font-bold">
						<div className="flex size-8 items-center justify-center rounded-lg bg-muted/60">
							<GitProviderIcon kind={kind} />
						</div>
						{editing ? 'Edit' : 'Add'} {GIT_PROVIDER_LABELS[kind]}
					</DialogTitle>
					<DialogDescription className="text-xs">
						{KIND_DESCRIPTIONS[kind]}
					</DialogDescription>
				</DialogHeader>

				<GitProviderKindHint kind={kind} form={form} />

				<div className="flex flex-col gap-3.5 pt-1">
					{renderField(
						'name',
						'Provider Name',
						false,
						'my-personal-account',
					)}

					{kind === 'github' && (
						<div className="grid grid-cols-1 gap-3 md:grid-cols-2">
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
							<div className="grid grid-cols-1 gap-3 md:grid-cols-2">
								{renderField(
									'url',
									'Public URL',
									false,
									kind === 'gitlab'
										? 'https://gitlab.com'
										: 'https://gitea.com',
								)}
								{renderField('internal_url', 'Internal URL')}
							</div>
							{renderField('redirect_uri', 'Redirect URI')}
							{kind === 'gitlab' ? (
								<div className="grid grid-cols-1 gap-3 md:grid-cols-2">
									{renderField('application_id', 'Application ID')}
									{renderField('secret', 'Application Secret', true)}
									<div className="md:col-span-2">
										{renderField('group_name', 'Group Name')}
									</div>
								</div>
							) : (
								<div className="grid grid-cols-1 gap-3 md:grid-cols-2">
									{renderField('client_id', 'Client ID')}
									{renderField('client_secret', 'Client Secret', true)}
								</div>
							)}
						</>
					)}

					{kind === 'bitbucket' && (
						<>
							<div className="grid grid-cols-1 gap-3 md:grid-cols-2">
								{renderField('username', 'Username')}
								{renderField('email', 'Email')}
								{renderField('api_token', 'API Token', true)}
								{renderField('workspace', 'Workspace Name')}
							</div>
							{renderField('app_password', 'App Password (Legacy)', true)}
						</>
					)}

					<label className="flex cursor-pointer items-center gap-2 pt-1 text-xs font-semibold text-foreground">
						<input
							type="checkbox"
							checked={form.shared === 'true'}
							onChange={e =>
								onChangeField('shared', String(e.target.checked))
							}
							className="size-4 rounded  accent-primary"
						/>
						Share this Git Provider with all organization members
					</label>

					<div className="flex justify-end gap-2 border-t  pt-4">
						<Button
							variant="outline"
							size="sm"
							onClick={onClose}
							className="h-9 px-4 text-xs font-semibold">
							Cancel
						</Button>
						<Button
							onClick={onSave}
							disabled={busy}
							className="h-9 gap-1.5 rounded-lg bg-primary px-4 text-xs font-semibold text-primary-foreground">
							{busy ? (
								<RefreshCw className="size-3.5 animate-spin" />
							) : (
								<Save className="size-3.5" />
							)}
							{editing
								? 'Save Changes'
								: kind === 'github'
									? 'Create Provider'
									: 'Save & Authorize'}
						</Button>
					</div>
				</div>
			</DialogContent>
		</Dialog>
	);
}
