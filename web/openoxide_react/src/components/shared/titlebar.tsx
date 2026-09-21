import {Fragment} from 'react';
import type {ComponentType, ReactNode} from 'react';
import {Link} from '@tanstack/react-router';
import {cn} from '#/api/utils';

/**
 * Vercel-style page header (Geist grammar) — a horizontal ROW, not a
 * centered column:
 *
 * ┌──────────────────────────────────────────────────────────────┐
 * │ Projects / my-project / Deployments                          │ ← crumbs top-left
 * │ ┌────┐ Deployments                     [Refresh] [•••]       │ ← icon+title LEFT,
 * │ └────┘ 12 deployments across your infra                        actions RIGHT
 * │ ── Tabs ── ── Tabs ── ── Tabs ──                             │ ← children (tabs)
 * ├──────────────────────────────────────────────────────────────┤
 * ↑ full-bleed hairline — sidebar edge → right viewport edge
 *
 * - Crumbs: 13px, `/` slash separators, hover pill on links, the active
 *   (last) crumb gets Geist's gray selection pill.
 * - Title row: bordered icon tile + 20px semibold title + description
 *   underneath, actions pinned to the right edge of the same row.
 * - The closing hairline bleeds through the page's horizontal padding
 *   (`px-4 sm:px-6 lg:px-8`) so it runs end-to-end like the Vercel
 *   dashboard's header rule. Tabs sit ON the line when passed as children.
 */

export type TitlebarCrumb = {
	label: ReactNode;
	/** Route link. Omit for the current page (rendered as the active pill). */
	to?: string;
	params?: Record<string, string>;
	search?: Record<string, unknown>;
	icon?: ComponentType<{className?: string; strokeWidth?: number}>;
};

export function Titlebar({
	breadcrumbs,
	breadcrumbExtras,
	icon,
	title,
	titleSuffix,
	description,
	actions,
	children,
	separator = true,
	className,
}: {
	breadcrumbs?: TitlebarCrumb[];
	/** Inline controls right after the crumbs (env switcher, kebab…). */
	breadcrumbExtras?: ReactNode;
	/** Identity tile before the title (provider icon, kind glyph…). */
	icon?: ReactNode;
	title?: ReactNode;
	/** Inline node right after the title (status pill, count chip…). */
	titleSuffix?: ReactNode;
	description?: ReactNode;
	actions?: ReactNode;
	children?: ReactNode;
	/** Render the closing full-bleed hairline. Off when the page paints its
	 *  own border-b below the header (e.g. a tab strip just under it). */
	separator?: boolean;
	className?: string;
}) {
	return (
		<header className={cn('flex w-full flex-col gap-3', className)}>
			{breadcrumbs && breadcrumbs.length > 0 && (
				<nav
					aria-label="Breadcrumb"
					className="flex max-w-full flex-wrap items-center gap-1 text-[13px] leading-none font-medium text-muted-foreground">
					{breadcrumbs.map((crumb, index) => {
						const isLast = index === breadcrumbs.length - 1;
						const content = (
							<>
								{crumb.icon && (
									<crumb.icon
										className="size-3.5 shrink-0"
										strokeWidth={1.7}
									/>
								)}
								<span className="max-w-[240px] truncate">
									{crumb.label}
								</span>
							</>
						);
						return (
							<Fragment key={`${index}-${String(crumb.label)}`}>
								{index > 0 && (
									<span
										aria-hidden
										className="px-0.5 text-muted-foreground/50 select-none">
										/
									</span>
								)}
								{crumb.to && !isLast ? (
									<Link
										to={crumb.to as never}
										params={crumb.params as never}
										search={crumb.search as never}
										className="-mx-1 flex items-center gap-1.5 rounded-md px-1.5 py-1 transition-colors outline-none hover:bg-muted hover:text-foreground focus-visible:ring-2 focus-visible:ring-ring/40">
										{content}
									</Link>
								) : (
									<span
										aria-current={isLast ? 'page' : undefined}
										className={cn(
											'-mx-1 flex items-center gap-1.5 rounded-md px-1.5 py-1',
											isLast && 'bg-muted text-foreground',
										)}>
										{content}
									</span>
								)}
							</Fragment>
						);
					})}
					{breadcrumbExtras && (
						<span className="ml-1.5 flex items-center gap-1.5">
							{breadcrumbExtras}
						</span>
					)}
				</nav>
			)}

			{/* Title row: identity + title left · actions right. Omitted entirely
			    when the page passes nothing (e.g. a centered title rendered
			    below the hairline, like Settings). */}
			{(icon || title || titleSuffix || description || actions) && (
				<div className="flex flex-wrap items-center justify-between gap-x-4 gap-y-2">
					<div className="flex min-w-0 items-center gap-3">
						{icon && (
							<div className="flex size-10 shrink-0 items-center justify-center rounded-lg border bg-card [&_svg]:size-5">
								{icon}
							</div>
						)}
						<div className="min-w-0">
							<div className="flex min-w-0 items-center gap-2.5">
								{title && (
									<h1 className="truncate text-xl font-semibold tracking-[-0.02em] text-foreground">
										{title}
									</h1>
								)}
								{titleSuffix}
							</div>
							{description && (
								<p className="truncate text-[13px] text-muted-foreground">
									{description}
								</p>
							)}
						</div>
					</div>

					{actions && (
						<div className="flex shrink-0 flex-wrap items-center justify-end gap-2">
							{actions}
						</div>
					)}
				</div>
			)}

			{children && <div className="self-stretch">{children}</div>}

			{separator && (
				/* Full-bleed hairline: swallows the page gutter so the rule
				   runs edge-to-edge (sidebar → right edge of the main area). */
				<div
					aria-hidden
					className="-mx-4 self-stretch border-t border-border sm:-mx-6 lg:-mx-8"
				/>
			)}
		</header>
	);
}
