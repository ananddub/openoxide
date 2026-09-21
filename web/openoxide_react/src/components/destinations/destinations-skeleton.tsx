import {Skeleton} from '#/components/ui/skeleton';

/** Loading placeholder rows for the destinations list. */
export function DestinationsSkeleton() {
	return (
		<div className="flex flex-col gap-2">
			{[1, 2, 3].map(i => (
				<div
					key={i}
					className="flex items-center gap-4 rounded-lg px-4 py-4">
					<Skeleton className="h-9 w-9 shrink-0 rounded-lg" />
					<div className="flex-1 space-y-2">
						<Skeleton className="h-3.5 w-40" />
						<Skeleton className="h-3 w-56" />
					</div>
					<Skeleton className="h-5 w-20 rounded-full" />
				</div>
			))}
		</div>
	);
}
