import React, {useState} from 'react';
import {Link} from '@tanstack/react-router';

import {Folder, MoreHorizontal, SquarePen, Trash2} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {TagBadge} from '#/components/shared/tag-badge';
import {
	DropdownMenu,
	DropdownMenuTrigger,
	DropdownMenuContent,
	DropdownMenuItem,
	DropdownMenuSeparator,
} from '#/components/ui/dropdown';
import {useProjectCardData} from './use-project-card-data';
import {HandleProjectDialog} from './handle-project-dialog';

type Project = Record<string, any>;

type ProjectCardProps = {
	project: Project;
	onDelete: (id: number) => void;
};

/** "Aug 26" style short date from a proto int64 (bigint at runtime). */
function shortDate(timestamp?: number | bigint) {
	const seconds = Number(timestamp ?? 0);
	if (!seconds) return 'recently';
	return new Date(seconds * 1000).toLocaleDateString(undefined, {
		month: 'short',
		day: 'numeric',
	});
}

/** Aggregate project health → a tiny Vercel-style status ring. */
const STATUS_RING: Record<
	string,
	{color: string; frac: number; spin: boolean; label: string}
> = {
	RUNNING: {
		color: 'text-emerald-500',
		frac: 1,
		spin: false,
		label: 'Running',
	},
	ERROR: {color: 'text-rose-500', frac: 1, spin: false, label: 'Error'},
	DEPLOYING: {
		color: 'text-amber-400',
		frac: 0.7,
		spin: true,
		label: 'Deploying',
	},
	STOPPED: {
		color: 'text-muted-foreground/40',
		frac: 0,
		spin: false,
		label: 'Stopped',
	},
};

function StatusRing({status}: {status: string}) {
	const cfg = STATUS_RING[status];
	if (!cfg) return null;
	const r = 6;
	const c = 2 * Math.PI * r;
	const arc = c * cfg.frac;
	return (
		<svg
			viewBox="0 0 16 16"
			role="img"
			aria-label={cfg.label}
			className={`size-2 shrink-0 ${cfg.color} ${cfg.spin ? 'animate-spin' : ''}`}>
			<title>{cfg.label}</title>
			<circle
				cx="8"
				cy="8"
				r={r}
				fill="none"
				stroke="currentColor"
				strokeOpacity="0.2"
				strokeWidth="1.5" />
			{arc > 0 && (
				<circle
					cx="8"
					cy="8"
					r={r}
					fill="none"
					stroke="currentColor"
					strokeWidth="1.5"
					strokeLinecap="round"
					strokeDasharray={`${arc} ${c - arc}`}
					transform="rotate(-90 8 8)" />
			)}
		</svg>
	);
}

/**
 * Vercel-style project tile — airy two-tier layout: circular identity
 * avatar + name with a muted subtitle line on top, action menu on the
 * right, and a quiet meta row (counts + short created date) pinned to the
 * bottom. Flat background-100 surface, hairline edge, no elevation.
 */
export const ProjectCard: React.FC<ProjectCardProps> = ({
	project,
	onDelete,
}) => {
	const [isUpdateOpen, setIsUpdateOpen] = useState(false);

	const {
		status,
		totalServices,
		totalEnvironments,
		projectServerNames,
		projectTagNames,
		getTagColor,
	} = useProjectCardData(project);

	const subtitle =
		projectServerNames[0] ||
		(totalServices === 0 ? 'No services yet' : 'Localhost');

	return (
		<>
			<div className="group relative flex min-h-[132px] flex-col gap-3 rounded-xl border bg-card p-4 transition-colors hover:bg-accent/40">
				{/* Clickable Card Overlay */}
				<Link
					to="/projects/$id"
					params={{id: String(project.id)}}
					preload="intent"
					className="absolute inset-0 z-0 rounded-xl"
					aria-label={`Open project ${project.name}`}
				/>

				{/* Identity row */}
				<div className="pointer-events-none relative z-10 flex items-start justify-between gap-2.5">
					<div className="flex min-w-0 items-center gap-3">
						<div className="flex size-9 shrink-0 items-center justify-center rounded-full border bg-background text-foreground">
							<Folder className="size-4 text-muted-foreground" />
						</div>
						<div className="flex min-w-0 flex-col">
							<h3 className="truncate text-sm font-semibold text-foreground">
								{project.name}
							</h3>
							<span className="mt-0.5 truncate text-xs text-muted-foreground">
								{subtitle}
							</span>
						</div>
					</div>

					{/* Status + actions */}
					<div className="pointer-events-auto relative z-10 flex shrink-0 items-center gap-2">
						{status && <StatusRing status={status} />}
						<DropdownMenu>
							<DropdownMenuTrigger
								render={
									<Button
										variant="ghost"
										size="icon"
										className="size-7 rounded-md text-muted-foreground hover:bg-muted/80 hover:text-foreground focus-visible:ring-0"
											title={`${project.name} actions`}
										aria-label={`Project ${project.name} actions`}>
										<MoreHorizontal className="size-4" />
									</Button>
								}
							/>
							<DropdownMenuContent align="end" className="w-48">
								<DropdownMenuItem
									onClick={() => setIsUpdateOpen(true)}
									className="flex cursor-pointer items-center gap-2 text-sm text-foreground">
									<SquarePen className="size-4 text-muted-foreground" />
									<span>Update</span>
								</DropdownMenuItem>
								<DropdownMenuSeparator />
								<DropdownMenuItem
									onClick={() => onDelete(project.id)}
									className="flex cursor-pointer items-center gap-2 text-sm text-danger focus:text-danger">
									<Trash2 className="size-4" />
									<span>Delete</span>
								</DropdownMenuItem>
							</DropdownMenuContent>
						</DropdownMenu>
					</div>
				</div>

				{/* Tags */}
				{projectTagNames.length > 0 && (
					<div className="pointer-events-none relative z-10 flex flex-wrap gap-1.5">
						{projectTagNames.map(tag => (
							<TagBadge
								key={tag}
								name={tag}
								color={getTagColor(tag)}
								className="px-1.5 py-0 text-[10px]"
							/>
						))}
					</div>
				)}

				{/* Quiet meta row pinned to the bottom */}
				<div className="pointer-events-none relative z-10 mt-auto flex items-center gap-1.5 text-xs text-muted-foreground">
					<span className="font-medium text-foreground">
						{totalServices}
					</span>
					<span>{totalServices === 1 ? 'service' : 'services'}</span>
					<span aria-hidden className="text-muted-foreground/40">
						·
					</span>
					<span className="font-medium text-foreground">
						{totalEnvironments}
					</span>
					<span>
						{totalEnvironments === 1 ? 'env' : 'envs'}
					</span>
					<span className="ml-auto shrink-0">
						{shortDate(project.created_at)}
					</span>
				</div>
			</div>

			{/* Edit/Update Dialog */}
			<HandleProjectDialog
				project={project}
				isOpen={isUpdateOpen}
				onOpenChange={setIsUpdateOpen}
			/>
		</>
	);
};
