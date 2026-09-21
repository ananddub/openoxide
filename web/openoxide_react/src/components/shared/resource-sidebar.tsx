import {Link} from '@tanstack/react-router';
import {ArrowLeft, type LucideIcon} from 'lucide-react';

/**
 * Right-hand column shared by the resource list pages (SSH Keys, Tags,
 * Registry, Destinations, Vault). Keeps the 1600px grid balanced on wide
 * screens: the list owns the main column, this owns a sticky 340px rail.
 */

export interface ResourceRow {
	icon: LucideIcon;
	label: string;
	value: React.ReactNode;
}

export function ResourceInfoCard({
	title,
	icon: Icon,
	rows,
}: {
	title: string;
	icon: LucideIcon;
	rows: ResourceRow[];
}) {
	return (
		<div className="rounded-xl bg-card p-5 ring-1 ring-border/50">
			<div className="mb-4 flex items-center gap-2">
				<Icon className="size-4 text-muted-foreground" strokeWidth={2} />
				<h3 className="text-sm font-semibold text-foreground">{title}</h3>
			</div>
			<div className="space-y-3">
				{rows.map(row => (
					<div
						key={row.label}
						className="flex items-center justify-between gap-3">
						<div className="flex min-w-0 items-center gap-2">
							<div className="flex size-8 shrink-0 items-center justify-center rounded-lg bg-muted/60">
								<row.icon
									className="size-4 text-muted-foreground"
									strokeWidth={2}
								/>
							</div>
							<span className="text-sm text-muted-foreground">
								{row.label}
							</span>
						</div>
						<div className="min-w-0 truncate text-sm font-medium text-foreground">
							{row.value}
						</div>
					</div>
				))}
			</div>
		</div>
	);
}

export interface GuideStep {
	title: string;
	text: string;
}

export function ResourceGuideCard({
	title,
	icon: Icon,
	steps,
}: {
	title: string;
	icon: LucideIcon;
	steps: GuideStep[];
}) {
	return (
		<div className="rounded-xl bg-card p-5 ring-1 ring-border/50">
			<div className="mb-4 flex items-center gap-2">
				<Icon className="size-4 text-muted-foreground" strokeWidth={2} />
				<h3 className="text-sm font-semibold text-foreground">{title}</h3>
			</div>
			<ol className="space-y-3">
				{steps.map((step, i) => (
					<li key={step.title} className="flex gap-3">
						<span className="flex size-5 shrink-0 items-center justify-center rounded-full bg-muted font-mono text-[10px] font-medium text-muted-foreground">
							{i + 1}
						</span>
						<div className="min-w-0">
							<p className="text-xs font-medium text-foreground">
								{step.title}
							</p>
							<p className="mt-0.5 text-xs leading-relaxed text-muted-foreground">
								{step.text}
							</p>
						</div>
					</li>
				))}
			</ol>
		</div>
	);
}

export function ResourceLinksCard({
	title,
	icon: Icon,
	links,
}: {
	title: string;
	icon: LucideIcon;
	links: {label: string; to: string; hint?: string}[];
}) {
	return (
		<div className="rounded-xl bg-card p-5 ring-1 ring-border/50">
			<div className="mb-2 flex items-center gap-2">
				<Icon className="size-4 text-muted-foreground" strokeWidth={2} />
				<h3 className="text-sm font-semibold text-foreground">{title}</h3>
			</div>
			<div className="flex flex-col">
				{links.map(link => (
					<Link
						key={link.to + link.label}
						to={link.to}
						title={link.hint}
						className="flex items-center justify-between gap-2 rounded-lg px-2 py-2 text-sm text-muted-foreground transition-colors hover:bg-foreground/[0.04] hover:text-foreground">
						{link.label}
						<ArrowLeft className="size-3.5 rotate-180" />
					</Link>
				))}
			</div>
		</div>
	);
}
