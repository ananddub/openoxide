import {useEffect, useId} from 'react';
import type {ReactNode} from 'react';
import {setPageCrumbs, type Crumb} from '#/components/layouts/topbar/topbar-slots';
import {cn} from '#/api/utils';

/**
 * Vercel-style page header — MINIMAL. The slim top bar carries the
 * breadcrumb trail CENTERED (this component registers its crumbs there
 * via topbar-slots.ts). The page-local header paints NO title block:
 * no back arrow, no icon, no title, no description, no suffix — the
 * page starts straight with its content.
 *
 * Only two things render in-page:
 * - `actions` (right-aligned, only when the page passes them) so pages
 *   keep their buttons (refresh / create / status…).
 * - `children` (tab strips etc.) sitting on the closing hairline.
 */

export type TitlebarCrumb = Crumb;

export function Titlebar({
	breadcrumbs,
	actions,
	children,
	separator = true,
	className,
}: {
	/** Registered into the top bar's center slot (not painted here). */
	breadcrumbs?: TitlebarCrumb[];
	/** Right-aligned page actions (refresh / create / status…). */
	actions?: ReactNode;
	children?: ReactNode;
	/** Render the closing full-bleed hairline. Off when the page paints its
	 *  own border-b below the header (e.g. a tab strip just under it). */
	separator?: boolean;
	className?: string;
}) {
	const key = useId();

	// Publish the crumb trail to the top bar's center slot. Serialized
	// deps keep the effect to real content changes.
	useEffect(() => {
		setPageCrumbs(key, breadcrumbs ?? []);
		return () => setPageCrumbs(key, null);
		// eslint-disable-next-line react-hooks/exhaustive-deps
	}, [key, JSON.stringify(breadcrumbs ?? null)]);

	return (
		<header className={cn('flex w-full flex-col gap-3', className)}>
			{actions && (
				<div className="flex shrink-0 flex-wrap items-center justify-end gap-2">
					{actions}
				</div>
			)}

			{children && <div className="self-stretch">{children}</div>}

			{/* The closing hairline only divides real header content from the
			    page body. When the header paints nothing (crumbs live in the
			    top bar, no actions/children), the rule would just float as a
			    stray line — so skip it. */}
			{separator && (actions || children) && (
				/* Full-bleed hairline: swallows the page gutter so the rule
				   runs edge-to-edge (sidebar → right edge of the main area). */
				<div
					aria-hidden
					className="-mx-4 self-stretch border-t border-border"
				/>
			)}
		</header>
	);
}
