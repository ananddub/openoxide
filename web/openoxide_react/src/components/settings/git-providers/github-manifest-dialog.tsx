import {useState} from 'react';
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

export function GithubManifestDialog({isOpen, onClose}: GithubManifestDialogProps) {
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
		callback_urls: [`${githubCallbackBaseUrl}/git-providers/github/manifest/callback`],
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
		<Dialog open={isOpen} onOpenChange={(open) => !open && onClose()}>
			<DialogContent>
				<DialogHeader>
					<DialogTitle>Create GitHub App</DialogTitle>
					<DialogDescription>
						GitHub will automatically configure the App manifest and save it in OpenOxide.
					</DialogDescription>
				</DialogHeader>

				<div className="grid gap-3 pt-2">
					<div className="space-y-1">
						<label className="text-xs font-medium">GitHub Base URL</label>
						<Input
							value={githubUrl}
							onChange={(e) => setGithubUrl(e.target.value)}
							placeholder="https://github.com"
						/>
					</div>

					<div className="space-y-1">
						<label className="text-xs font-medium">GitHub Organization (Optional)</label>
						<Input
							value={orgName}
							onChange={(e) => setOrgName(e.target.value)}
							placeholder="Leave empty for personal account"
						/>
					</div>

					<form action={targetAction} method="post" className="pt-2">
						<input type="hidden" name="manifest" value={manifest} />
						<Button type="submit" className="w-full">
							Continue on GitHub
						</Button>
					</form>
				</div>
			</DialogContent>
		</Dialog>
	);
}
