import {Link} from '@tanstack/react-router';
import {Plus, SearchX, GitBranch} from 'lucide-react';

/**
 * Openship-style empty state: soft card-stack illustration (theme tokens,
 * so it tracks dark/light), medium-weight title, relaxed copy, and action
 * buttons. Filtered variant stays quiet — icon + message + clear action.
 */

export function EmptyState({
	hasFilters,
	onClearFilters,
	title,
	description,
}: {
	hasFilters: boolean;
	onClearFilters?: () => void;
	title?: string;
	description?: string;
}) {
	if (hasFilters) {
		return (
			<div className="rounded-xl bg-card p-16 text-center">
				<div className="mx-auto max-w-md">
					<div className="mx-auto mb-4 flex size-16 items-center justify-center rounded-full bg-muted/60">
						<SearchX className="size-7 text-muted-foreground/50" />
					</div>
					<h3 className="mb-2 text-lg font-medium text-foreground/80">
						No matches
					</h3>
					<p className="text-sm leading-relaxed text-muted-foreground">
						Nothing matches the current filters. Try a different search or
						clear the filters to see everything.
					</p>
					{onClearFilters && (
						<button
							type="button"
							onClick={onClearFilters}
							className="mt-4 text-xs font-medium text-primary hover:underline">
							Clear all filters
						</button>
					)}
				</div>
			</div>
		);
	}

	return (
		<div className="rounded-xl bg-card px-6 pb-10 text-center">
			{/* Card-stack illustration — same vector language as the home
			    welcome: layered panel, window dots, content lines, dashed
			    add slot. Uses theme surface/ink tokens so it adapts. */}
			<div className="relative mx-auto h-44 w-64">
				<svg
					className="absolute inset-0 h-full w-full"
					viewBox="0 0 260 180"
					fill="none">
					<rect
						x="75"
						y="45"
						width="130"
						height="95"
						rx="14"
						fill="var(--th-sf-04)"
					/>
					<rect
						x="65"
						y="35"
						width="130"
						height="95"
						rx="14"
						fill="var(--th-sf-03)"
						stroke="var(--border)"
						strokeWidth="1"
					/>
					<rect
						x="55"
						y="25"
						width="130"
						height="95"
						rx="14"
						fill="var(--th-card-bg)"
						stroke="var(--border)"
						strokeWidth="1"
					/>
					<rect
						x="55"
						y="25"
						width="130"
						height="28"
						rx="14"
						fill="var(--th-sf-05)"
					/>
					<circle cx="72" cy="39" r="4" fill="#ef4444" fillOpacity="0.6" />
					<circle cx="84" cy="39" r="4" fill="#eab308" fillOpacity="0.6" />
					<circle cx="96" cy="39" r="4" fill="#22c55e" fillOpacity="0.6" />
					<rect
						x="70"
						y="65"
						width="50"
						height="5"
						rx="2.5"
						fill="var(--th-on-12)"
					/>
					<rect
						x="70"
						y="76"
						width="85"
						height="4"
						rx="2"
						fill="var(--th-on-08)"
					/>
					<rect
						x="70"
						y="85"
						width="65"
						height="4"
						rx="2"
						fill="var(--th-on-08)"
					/>
					<circle
						cx="84"
						cy="108"
						r="10"
						fill="var(--th-on-05)"
						stroke="var(--th-on-10)"
						strokeWidth="1"
					/>
					<path d="M84 102l3 8h-6l3-8z" fill="var(--th-on-20)" />
					<circle cx="84" cy="106" r="2" fill="var(--th-on-30)" />
					<circle cx="210" cy="90" r="22" fill="var(--th-on-05)" />
					<circle
						cx="210"
						cy="90"
						r="16"
						fill="var(--th-card-bg)"
						stroke="var(--th-on-20)"
						strokeWidth="2"
						strokeDasharray="4 3"
					/>
					<path
						d="M210 82v16M202 90h16"
						stroke="var(--th-on-40)"
						strokeWidth="2"
						strokeLinecap="round"
					/>
					<circle cx="30" cy="60" r="4" fill="var(--th-on-10)" />
					<circle cx="40" cy="140" r="6" fill="var(--th-on-08)" />
					<circle cx="230" cy="40" r="3" fill="var(--th-on-12)" />
					<circle cx="245" cy="130" r="5" fill="var(--th-on-06)" />
					<path d="M25 100l2-4 2 4-4-2 4 0-4 2z" fill="var(--th-on-16)" />
					<path
						d="M220 150l1.5-3 1.5 3-3-1.5 3 0-3 1.5z"
						fill="var(--th-on-12)"
					/>
					<path
						d="M185 95 Q 192 92 195 90"
						stroke="var(--th-on-12)"
						strokeWidth="1.5"
						strokeDasharray="3 3"
						fill="none"
					/>
				</svg>
			</div>

			<h3 className="mb-2 text-lg font-medium text-foreground/80">
				{title ?? 'No deployments yet'}
			</h3>
			<p className="mx-auto mb-8 max-w-sm text-sm leading-relaxed text-muted-foreground">
				{description ??
					'Deploy your first app or database and its build history will appear here.'}
			</p>

			<div className="flex flex-col items-center justify-center gap-3 sm:flex-row">
				<Link
					to="/projects"
					search={{create: 1} as never}
					className="inline-flex items-center gap-2 rounded-xl bg-primary px-6 py-3 text-sm font-medium text-primary-foreground transition-all hover:-translate-y-0.5 hover:bg-primary/90 hover:shadow-lg hover:shadow-primary/25">
					<Plus className="size-4" />
					New Project
				</Link>
				<Link
					to="/projects"
					className="inline-flex items-center gap-2 rounded-xl bg-muted/50 px-6 py-3 text-sm font-medium text-foreground transition-colors hover:bg-muted">
					<GitBranch className="size-4" />
					Browse Projects
				</Link>
			</div>
		</div>
	);
}
