import {Loader2, Save} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {Switch} from '#/components/ui/switch';
import {
	Sheet,
	SheetContent,
	SheetDescription,
	SheetFooter,
	SheetHeader,
	SheetTitle,
} from '#/components/ui/sheet';
import {WizardField} from '#/components/shared/sheet-wizard';
import {
	type GitProviderKind,
	type GitProviderFormState,
	GIT_PROVIDER_LABELS,
} from './git-provider-types';
import {GitProviderIcon} from './git-provider-icon';
import {GitProviderKindHint} from './git-provider-kind-hint';

interface GitProviderEditSheetProps {
	isOpen: boolean;
	kind: GitProviderKind;
	editing: unknown;
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

/** Add / edit a Git provider — house form-sheet grammar. */
export function GitProviderEditSheet({
	isOpen,
	kind,
	editing,
	form,
	busy,
	onClose,
	onChangeField,
	onSave,
}: GitProviderEditSheetProps) {
	const field = (
		key: keyof GitProviderFormState & string,
		label: string,
		secret = false,
		placeholder = '',
	) => (
		<WizardField key={key} label={label}>
			<Input
				type={secret ? 'password' : 'text'}
				value={form[key] || ''}
				placeholder={placeholder}
				onChange={e => onChangeField(key, e.target.value)}
				className="bg-background font-mono"
			/>
		</WizardField>
	);

	return (
		<Sheet open={isOpen} onOpenChange={open => !open && onClose()}>
			<SheetContent className="p-0">
				<SheetHeader>
					<SheetTitle className="flex items-center gap-2">
						<GitProviderIcon kind={kind} className="size-4" />
						{editing ? 'Edit' : 'Add'} {GIT_PROVIDER_LABELS[kind]}
					</SheetTitle>
					<SheetDescription>{KIND_DESCRIPTIONS[kind]}</SheetDescription>
				</SheetHeader>

				<div className="flex min-h-0 flex-1 flex-col gap-5 overflow-y-auto px-6 py-6">
					<GitProviderKindHint kind={kind} form={form} />

					{field('name', 'Provider Name', false, 'my-personal-account')}

					{kind === 'github' && [
						field('app_name', 'App Name'),
						field('app_id', 'App ID'),
						field('installation_id', 'Installation ID'),
						field('client_id', 'Client ID'),
						field('client_secret', 'Client Secret', true),
						field('private_key', 'Private Key', true),
					]}

					{(kind === 'gitlab' || kind === 'gitea') && (
						<>
							{field(
								'url',
								'Public URL',
								false,
								kind === 'gitlab'
									? 'https://gitlab.com'
									: 'https://gitea.com',
							)}
							{field('internal_url', 'Internal URL')}
							{field('redirect_uri', 'Redirect URI')}
							{kind === 'gitlab'
								? [
										field('application_id', 'Application ID'),
										field('secret', 'Application Secret', true),
										field('group_name', 'Group Name'),
									]
								: [
										field('client_id', 'Client ID'),
										field('client_secret', 'Client Secret', true),
									]}
						</>
					)}

					{kind === 'bitbucket' && [
						field('username', 'Username'),
						field('email', 'Email'),
						field('api_token', 'API Token', true),
						field('workspace', 'Workspace Name'),
						field('app_password', 'App Password (Legacy)', true),
					]}

					<div className="flex items-center justify-between gap-4 rounded-xl border border-border/60 px-4 py-3.5">
						<div className="min-w-0">
							<p className="text-[13px] font-medium text-foreground">
								Share with the organization
							</p>
							<p className="mt-0.5 text-[13px] text-muted-foreground">
								Every member can pick this provider when creating services.
							</p>
						</div>
						<Switch
							checked={form.shared === 'true'}
							onCheckedChange={checked =>
								onChangeField('shared', String(checked))
							}
						/>
					</div>
				</div>

				<SheetFooter className="justify-between">
					<Button variant="outline" onClick={onClose}>
						Cancel
					</Button>
					<Button
						onClick={() => void onSave()}
						disabled={busy || !form.name}
						className="h-10 gap-1.5 px-5">
						{busy ? (
							<Loader2 className="size-3.5 animate-spin" />
						) : (
							<Save className="size-3.5" />
						)}
						{editing
							? 'Save changes'
							: kind === 'github'
								? 'Add provider'
								: 'Save & authorize'}
					</Button>
				</SheetFooter>
			</SheetContent>
		</Sheet>
	);
}
