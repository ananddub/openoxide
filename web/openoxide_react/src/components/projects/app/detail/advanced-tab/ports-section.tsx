import {useState} from 'react';
import {Globe} from 'lucide-react';
import {toast} from 'sonner';
import {Badge} from '#/components/ui/badge';
import {
	useApplicationPortDelete,
	useApplicationPortList,
	type ApplicationPortPayload,
} from '#/api/live';
import {formatApiError} from '#/api/utils';
import {
	AddResourceButton,
	AdvancedSection,
	EmptyRow,
	ResourceRow,
} from '#/components/projects/detail/advanced/shared';
import {PortAddDialog} from './port-add-dialog';

export function PortsSection({app}: {app: any}) {
	const {data: ports = []} = useApplicationPortList(app.id);
	const remove = useApplicationPortDelete();
	const [open, setOpen] = useState(false);
	const [deletingId, setDeletingId] = useState<number | null>(null);

	const del = async (port: ApplicationPortPayload) => {
		setDeletingId(port.id);
		try {
			await remove.mutateAsync({application_id: app.id, id: port.id});
			toast.success('Port removed');
		} catch (err) {
			toast.error(formatApiError(err));
		} finally {
			setDeletingId(null);
		}
	};

	return (
		<AdvancedSection
			icon={<Globe className="size-4 text-primary" />}
			title="Ports"
			description="Publish container ports on the host or through the swarm ingress mesh."
			action={
				<AddResourceButton
					onClick={() => setOpen(true)}
					label="Add Port"
				/>
			}>
			<div className="flex flex-col gap-2">
				{ports.length === 0 ? (
					<EmptyRow label="No published ports." />
				) : (
					ports.map(port => (
						<ResourceRow
							key={port.id}
							title={`${port.published_port} → ${port.target_port}/${port.protocol.toLowerCase()}`}
							subtitle={
								port.publish_mode === 'INGRESS'
									? 'Swarm ingress mesh'
									: 'Host binding'
							}
							badges={
								<Badge
									variant="secondary"
									className="text-[10px] font-semibold">
									{port.publish_mode}
								</Badge>
							}
							onDelete={() => del(port)}
							deleting={deletingId === port.id}
						/>
					))
				)}
			</div>
			<PortAddDialog open={open} onOpenChange={setOpen} appId={app.id} />
		</AdvancedSection>
	);
}

// ── Volumes / Mounts ───────────────────────────────────────────────────
