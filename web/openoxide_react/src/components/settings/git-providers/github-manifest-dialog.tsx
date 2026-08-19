import {useState} from 'react';
import {GithubIcon} from '#/components/icons/data-tools-icons';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {
	Dialog,
	DialogContent,
	DialogDescription,
	DialogHeader,
	DialogTitle,
} from '#/components/ui/dialog';
import {getGithubCallbackBaseUrl} from './git-provider-types';

interface GithubManifestDialogProps {
	isOpen: boolean;
	onClose: () => void;
}

export function GithubManifestDialog({
	isOpen,
	onClose,
}: GithubManifestDialogProps) {
	const [githubUrl, setGithubUrl] = useState('https://github.com');
	const [orgName, setOrgName] = useState('');

	const githubCallbackBaseUrl = getGithubCallbackBaseUrl();
	const randomSuffix = Math.random().toString(36).slice(2, 6);
	const appName = `OpenOxide-${new Date().toISOString().slice(0, 10)}-${randomSuffix}`;

	const manifest = JSON.stringify({
		name: appName,
		url: window.location.origin,
		hook_attributes: {
			url: `${githubCallbackBaseUrl}/public/webhooks/github`,
		},
		redirect_url: `${githubCallbackBaseUrl}/git-providers/github/manifest/callback`,
		callback_urls: [
			`${githubCallbackBaseUrl}/git-providers/github/manifest/callback`,
		],
		public: false,
		request_oauth_on_install: true,
		default_permissions: {
			contents: 'read',
			metadata: 'read',
			emails: 'read',
			pull_requests: 'write',
		},
		default_events: ['push', 'pull_request'],
	});

	const cleanBase = githubUrl.replace(/\/$/, '');
	const targetAction = orgName.trim()
		? `${cleanBase}/organizations/${orgName.trim()}/settings/apps/new`
		: `${cleanBase}/settings/apps/new`;

	return (
		<Dialog open={isOpen} onOpenChange={open => !open && onClose()}>
			<DialogContent className="sm:max-w-md">
				<DialogHeader>
					<DialogTitle className="flex items-center gap-2.5 text-base font-bold">
						<div className="flex size-8 items-center justify-center rounded-lg bg-zinc-800 text-white">
							<GithubIcon className="size-4" />
						</div>
						Create GitHub App
					</DialogTitle>
					<DialogDescription className="text-xs">
						GitHub will automatically configure the App manifest,
						permissions, and webhook callback in OpenOxide.
					</DialogDescription>
				</DialogHeader>

				<div className="flex flex-col gap-4 pt-2">
					<div className="space-y-1.5">
						<label className="text-xs font-semibold text-muted-foreground">
							GitHub Base URL
						</label>
						<Input
							value={githubUrl}
							onChange={e => setGithubUrl(e.target.value)}
							placeholder="https://github.com"
							className="h-9 font-mono text-xs"
						/>
					</div>

					<div className="space-y-1.5">
						<label className="text-xs font-semibold text-muted-foreground">
							GitHub Organization{' '}
							<span className="text-[11px] font-normal text-muted-foreground/80">
								(Optional)
							</span>
						</label>
						<Input
							value={orgName}
							onChange={e => setOrgName(e.target.value)}
							placeholder="e.g. my-company-org"
							className="h-9 text-xs"
						/>
					</div>

					<form action={targetAction} method="post" className="pt-2">
						<input type="hidden" name="manifest" value={manifest} />
						<Button
							type="submit"
							className="h-9 w-full cursor-pointer gap-2 rounded-lg bg-primary text-xs font-semibold text-primary-foreground">
							<GithubIcon className="size-4" /> Continue on GitHub
						</Button>
					</form>
				</div>
			</DialogContent>
		</Dialog>
	);
}
