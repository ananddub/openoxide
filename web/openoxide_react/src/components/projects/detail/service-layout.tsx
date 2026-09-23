import type {ReactNode} from 'react';
import {FolderOpen} from 'lucide-react';
import {Titlebar} from '#/components/shared/titlebar';

/**
 * Service detail shell — minimal Vercel grammar. The crumb trail
 * (`Projects / Project Details / <name>`) registers into the slim top bar;
 * in-page only the closing hairline remains, then full-width content.
 *
 * No env switcher / status / type / refresh row: those actions were dropped
 * from the application, compose and database detail pages, so this shell no
 * longer paints them. The service's tabs live in the sidebar rail
 * (PageContextNav), not in this header.
 */

export function ServiceLayout({
	id,
	name,
	children,
}: {
	/** Project id — the breadcrumb's parent link. */
	id: string;
	name: string;
	children: ReactNode;
}) {
	return (
		<div className="mx-auto flex w-full max-w-[1920px] animate-in flex-col gap-6 px-4 pt-4 pb-6 duration-200 fade-in">
			<Titlebar
				breadcrumbs={[
					{label: 'Projects', to: '/projects', icon: FolderOpen},
					{
						label: 'Project Details',
						to: '/projects/$id',
						params: {id},
					},
					{label: name},
				]}
			/>

			{/* Full-width content */}
			<div className="min-w-0">{children}</div>
		</div>
	);
}
