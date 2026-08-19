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

	const isGitProvider = [
		'GITHUB',
		'GITLAB',
		'GITEA',
		'BITBUCKET',
		'GIT',
	].includes(app.source_type || '');

	return (
		<div className="flex flex-col gap-6">
			{/* Enable toggle */}
			<section className="flex items-start justify-between rounded-xl border border-border bg-card p-5">
				<div>
					<h3 className="text-sm font-bold text-foreground">
						Preview Deployments
					</h3>
					<p className="mt-1 max-w-md text-xs text-muted-foreground">
						Automatically deploy pull-request / merge-request branches as
						isolated preview environments. Requires Git provider setup.
					</p>
				</div>
				<label className="flex cursor-pointer items-center gap-2">
					<input
						type="checkbox"
						checked={enabled}
						onChange={e => setEnabled(e.target.checked)}
						className="h-4.5 w-4.5 accent-primary"
					/>
				</label>
			</section>

			{!isGitProvider && (
				<section className="flex items-start gap-2.5 rounded-xl border border-amber-500/20 bg-amber-500/5 p-4">
					<Info className="mt-0.5 h-4 w-4 shrink-0 text-amber-500" />
					<p className="text-xs leading-relaxed text-amber-500/80">
						Preview deployments require a GitHub, GitLab, Gitea, or custom
						Git repository source setup. Please configure a Git provider in
						the General tab first.
					</p>
				</section>
			)}

			{enabled && isGitProvider && (
				<>
					{/* Preview Config */}
					<section className="flex flex-col gap-4 rounded-xl border border-border bg-card p-5">
						<h3 className="text-sm font-bold text-foreground">
							Configuration
						</h3>
						<div className="grid grid-cols-2 gap-4">
							<div className="flex flex-col gap-1.5">
								<span className="text-xs font-bold text-muted-foreground">
									Wildcard Domain
								</span>
								<Input
									placeholder="*.preview.example.com"
									value={wildcard}
									onChange={e => setWildcard(e.target.value)}
									className="border-border bg-card text-xs"
								/>
							</div>
							<div className="flex flex-col gap-1.5">
								<span className="text-xs font-bold text-muted-foreground">
									Container Port
								</span>
								<Input
									type="number"
									placeholder="3000"
									value={port}
									onChange={e => setPort(e.target.value)}
									className="border-border bg-card text-xs"
								/>
							</div>
						</div>
						<div className="flex w-32 flex-col gap-1.5">
							<span className="text-xs font-bold text-muted-foreground">
								Max Active Previews
							</span>
							<Input
								type="number"
								min="1"
								max="10"
								value={limit}
								onChange={e => setLimit(e.target.value)}
								className="border-border bg-card text-xs"
							/>
						</div>

						<div className="mt-2 flex justify-end">
							<Button className="flex h-9 items-center gap-1.5 rounded-lg bg-primary text-xs font-semibold text-primary-foreground hover:bg-primary/95">
								Save Configuration
							</Button>
						</div>
					</section>

					{/* Active previews */}
					<section className="rounded-xl border border-border bg-card p-5">
						<h3 className="text-sm font-bold text-foreground">
							Active Previews
						</h3>
						<p className="mt-1 mb-6 text-xs text-muted-foreground">
							Currently active preview environments for open pull/merge
							requests.
						</p>
						<div className="flex flex-col items-center justify-center py-10 text-center text-muted-foreground">
							<Eye className="mb-2 h-8 w-8 opacity-30" />
							<p className="text-xs font-medium">
								No active preview environments running
							</p>
						</div>
					</section>
				</>
			)}
		</div>
	);
}
