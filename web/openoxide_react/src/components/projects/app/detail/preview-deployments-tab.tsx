import {useState} from 'react';
import {Eye, Info} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';

interface PreviewDeploymentsTabProps {
	app: any;
}

export function PreviewDeploymentsTab({app}: PreviewDeploymentsTabProps) {
	const [enabled, setEnabled] = useState(false);
	const [wildcard, setWildcard] = useState('');
	const [port, setPort] = useState('3000');
	const [limit, setLimit] = useState('3');

	const isGitProvider = ['GITHUB', 'GITLAB', 'GITEA', 'BITBUCKET', 'GIT'].includes(app.source_type || '');

	return (
		<div className="flex flex-col gap-6">
			{/* Enable toggle */}
			<section className="bg-card border border-border rounded-xl p-5 flex items-start justify-between">
				<div>
					<h3 className="text-sm font-bold text-foreground">Preview Deployments</h3>
					<p className="text-xs text-muted-foreground mt-1 max-w-md">
						Automatically deploy pull-request / merge-request branches as isolated preview environments. Requires Git provider setup.
					</p>
				</div>
				<label className="flex items-center gap-2 cursor-pointer">
					<input type="checkbox" checked={enabled} onChange={e => setEnabled(e.target.checked)} className="accent-primary w-4.5 h-4.5" />
				</label>
			</section>

			{!isGitProvider && (
				<section className="bg-amber-500/5 border border-amber-500/20 rounded-xl p-4 flex items-start gap-2.5">
					<Info className="w-4 h-4 text-amber-500 shrink-0 mt-0.5" />
					<p className="text-xs text-amber-500/80 leading-relaxed">
						Preview deployments require a GitHub, GitLab, Gitea, or custom Git repository source setup. Please configure a Git provider in the General tab first.
					</p>
				</section>
			)}

			{enabled && isGitProvider && (
				<>
					{/* Preview Config */}
					<section className="bg-card border border-border rounded-xl p-5 flex flex-col gap-4">
						<h3 className="text-sm font-bold text-foreground">Configuration</h3>
						<div className="grid grid-cols-2 gap-4">
							<div className="flex flex-col gap-1.5">
								<span className="text-xs font-bold text-muted-foreground">Wildcard Domain</span>
								<Input placeholder="*.preview.example.com" value={wildcard} onChange={e => setWildcard(e.target.value)} className="bg-card border-border text-xs" />
							</div>
							<div className="flex flex-col gap-1.5">
								<span className="text-xs font-bold text-muted-foreground">Container Port</span>
								<Input type="number" placeholder="3000" value={port} onChange={e => setPort(e.target.value)} className="bg-card border-border text-xs" />
							</div>
						</div>
						<div className="flex flex-col gap-1.5 w-32">
							<span className="text-xs font-bold text-muted-foreground">Max Active Previews</span>
							<Input type="number" min="1" max="10" value={limit} onChange={e => setLimit(e.target.value)} className="bg-card border-border text-xs" />
						</div>

						<div className="flex justify-end mt-2">
							<Button className="bg-primary hover:bg-primary/95 text-primary-foreground font-semibold flex items-center gap-1.5 h-9 rounded-lg text-xs">
								Save Configuration
							</Button>
						</div>
					</section>

					{/* Active previews */}
					<section className="bg-card border border-border rounded-xl p-5">
						<h3 className="text-sm font-bold text-foreground">Active Previews</h3>
						<p className="text-xs text-muted-foreground mt-1 mb-6">Currently active preview environments for open pull/merge requests.</p>
						<div className="flex flex-col items-center justify-center py-10 text-center text-muted-foreground">
							<Eye className="w-8 h-8 opacity-30 mb-2" />
							<p className="text-xs font-medium">No active preview environments running</p>
						</div>
					</section>
				</>
			)}
		</div>
	);
}
