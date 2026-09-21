import {Box} from 'lucide-react';
import type {ContainerItem} from './container-inspect-modal';
import type {InspectType} from './container-inspect-body';
import {ContainerRow} from './container-row';

interface ComposeContainersTableProps {
	containers: ContainerItem[];
	onOpenModal: (container: ContainerItem, type: InspectType) => void;
	onAction: (
		container: ContainerItem,
		action: 'start' | 'stop' | 'restart' | 'kill',
	) => void;
}

export function ComposeContainersTable({
	containers,
	onOpenModal,
	onAction,
}: ComposeContainersTableProps) {
	if (containers.length === 0) {
		return (
			<section className="flex flex-col items-center justify-center rounded-xl bg-card p-12 text-center shadow-sm">
				<div className="mb-3 flex size-12 items-center justify-center rounded-xl bg-muted text-muted-foreground">
					<Box className="h-6 w-6" />
				</div>
				<h4 className="text-sm font-bold text-foreground">
					No Containers Deployed
				</h4>
				<p className="mt-1 max-w-sm text-xs text-muted-foreground">
					There are no active live Docker containers for this stack. Deploy
					or start the stack to launch containers.
				</p>
			</section>
		);
	}

	return (
		<section className="overflow-hidden rounded-xl border bg-card">
			<div className="overflow-x-auto">
				<table className="w-full text-left text-xs">
					<thead className="border- border-b bg-muted/50 text-[10px] font-bold tracking-wider text-muted-foreground uppercase">
						<tr>
							<th className="px-4 py-3">Container & Service</th>
							<th className="px-4 py-3">Container ID</th>
							<th className="px-4 py-3">Image</th>
							<th className="px-4 py-3">Status</th>
							<th className="px-4 py-3 text-right">Actions</th>
						</tr>
					</thead>
					<tbody className="divide-y divide-border/60">
						{containers.map(c => (
							<ContainerRow
								key={c.id}
								container={c}
								onOpenModal={onOpenModal}
								onAction={onAction}
							/>
						))}
					</tbody>
				</table>
			</div>
		</section>
	);
}
