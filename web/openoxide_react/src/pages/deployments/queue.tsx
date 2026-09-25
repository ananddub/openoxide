import {createFileRoute} from '@tanstack/react-router';
import {Activity, RefreshCw} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {
	ListEmpty,
	ListRegion,
	ListSkeleton,
	SectionHeader,
} from '#/components/settings/list-card';
import {DeploymentActivityList} from '#/components/deployments/deployment-activity-list';
import {cn} from '#/api/utils';
import {useDeployments} from '#/hooks/deployments/use-deployments';

export const Route = createFileRoute('/_app/Deployments/queue')({
	component: ActiveQueuePage,
});

/** Active Queue — its own page (no tab switch): the same SectionHeader grammar as the rest of the rail, the live builds table below. */
function ActiveQueuePage() {
	const dp = useDeployments();

	return (
		<div className="mx-auto flex h-full w-full max-w-[1920px] animate-in flex-col gap-6 px-4 pt-4 pb-6 duration-200 fade-in">
			<SectionHeader
				title="Active Queue"
				description="Builds and deploys streaming right now across every project."
				action={
					<>
						<span className="text-xs text-muted-foreground select-none">
							{dp.isLoading
								? 'Loading...'
								: `${dp.activeQueue.length} active`}
						</span>
						<Button
							variant="ghost"
							onClick={dp.handleRefresh}
							disabled={dp.refreshing}
							className="h-8 gap-1.5 text-xs font-medium text-muted-foreground hover:text-foreground">
							<RefreshCw
								className={cn('size-3.5', dp.refreshing && 'animate-spin')}
							/>
							Refresh
						</Button>
					</>
				}
			/>

			<ListRegion>
				{dp.isLoading ? (
					<ListSkeleton rows={6} />
				) : dp.activeQueue.length > 0 ? (
					<DeploymentActivityList deployments={dp.activeQueue} />
				) : (
					<ListEmpty
						icon={Activity}
						title="Queue is clear"
						description="Active builds and deploys stream here in real time as they start."
					/>
				)}
			</ListRegion>
		</div>
	);
}
