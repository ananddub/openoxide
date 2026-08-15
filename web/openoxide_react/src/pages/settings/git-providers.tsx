import {createFileRoute} from '@tanstack/react-router';
import {GitBranch, GitFork} from 'lucide-react';
import {GithubIcon} from '#/components/icons/data-tools-icons';
import {Button} from '#/components/ui/button';
import {Badge} from '#/components/ui/badge';

export const Route = createFileRoute('/_app/settings/git-providers')({
	component: GitProvidersPage,
});

function GitProvidersPage() {
	return (
		<div className="p-6 flex flex-col gap-6 max-w-7xl mx-auto w-full animate-in fade-in duration-200">
			<div className="flex flex-col gap-1">
				<h1 className="text-2xl font-bold text-foreground tracking-tight">Git Providers</h1>
				<p className="text-xs text-muted-foreground">
					Connect GitHub, GitLab, and Bitbucket for automatic deployments and push webhooks
				</p>
			</div>

			<div className="grid grid-cols-1 md:grid-cols-2 gap-4">
				<div className="p-5 border rounded-xl bg-card flex flex-col justify-between gap-4">
					<div className="flex items-center justify-between">
						<div className="flex items-center gap-3">
							<div className="size-10 rounded-xl bg-muted/60 flex items-center justify-center">
								<GithubIcon className="size-6 text-foreground" />
							</div>
							<div>
								<h3 className="text-sm font-bold text-foreground">GitHub App</h3>
								<p className="text-[10px] text-muted-foreground">Repository & Webhook Integration</p>
							</div>
						</div>
						<Badge variant="outline" className="text-[10px]">CONNECTED</Badge>
					</div>
					<Button variant="outline" size="sm" className="w-full text-xs h-8">
						Configure Repositories
					</Button>
				</div>

				<div className="p-5 border rounded-xl bg-card flex flex-col justify-between gap-4">
					<div className="flex items-center justify-between">
						<div className="flex items-center gap-3">
							<div className="size-10 rounded-xl bg-muted/60 flex items-center justify-center">
								<GitBranch className="size-6 text-foreground" />
							</div>
							<div>
								<h3 className="text-sm font-bold text-foreground">GitLab</h3>
								<p className="text-[10px] text-muted-foreground">Self-Hosted or SaaS GitLab</p>
							</div>
						</div>
						<Badge variant="secondary" className="text-[10px]">AVAILABLE</Badge>
					</div>
					<Button variant="secondary" size="sm" className="w-full text-xs h-8">
						Connect GitLab
					</Button>
				</div>
			</div>
		</div>
	);
}
