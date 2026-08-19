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

	const githubCallbackBaseUrl = getGithubCallbackBaseUrl();
	const manifest = JSON.stringify({
		name: `OpenOxide-${new Date().toISOString().slice(0, 10)}`,
		url: window.location.origin,
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

	return (
		<Dialog open={isOpen} onOpenChange={(open) => !open && onClose()}>
			<DialogContent>
				<DialogHeader>
					<DialogTitle>Create GitHub App</DialogTitle>
					<DialogDescription>
						GitHub will create the App and return here to save it in OpenOxide.
					</DialogDescription>
				</DialogHeader>

				<div className="grid gap-3">
					<Input
						value={githubUrl}
						onChange={(e) => setGithubUrl(e.target.value)}
						placeholder="https://github.com"
					/>

					<form
						action={`${githubUrl.replace(/\/$/, '')}/settings/apps/new`}
						method="post"
					>
						<input type="hidden" name="manifest" value={manifest} />
						<Button type="submit" className="w-full">
							Create GitHub App
						</Button>
					</form>
				</div>
			</DialogContent>
		</Dialog>
	);
}
