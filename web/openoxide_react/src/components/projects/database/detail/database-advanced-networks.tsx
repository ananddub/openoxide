import {useEffect, useMemo, useState} from 'react';
import {Network} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Switch} from '#/components/ui/switch';
import {SettingsSheet} from '#/components/shared/settings-sheet';
import {SheetGroup, SheetNote} from '#/components/shared/field';
import {cn} from '#/api/utils';
import {toast} from 'sonner';
import {
	useDatabaseNetworkList,
	useDatabaseUpdate,
	parseDatabaseKind,
} from '#/api/live';
import {formatApiError} from '#/api/utils';

export function DatabaseAdvancedNetworks({database, onUpdated}: any) {
	const {data: networks} = useDatabaseNetworkList();
	const patchDatabase = useDatabaseUpdate();
	const [selected, setSelected] = useState<string[]>([]);
	const [detached, setDetached] = useState(false);
	const [saving, setSaving] = useState(false);

	useEffect(() => {
		setSelected((database?.network_ids || []).map(String));
		setDetached(Number(database?.detach_rustploy_network || 0) !== 0);
	}, [database]);

	const available = useMemo(
		() =>
			((networks as any[]) || []).filter(
				network =>
					!database?.server_id ||
					!network.server_id ||
					Number(network.server_id) === Number(database.server_id),
			),
		[networks, database?.server_id],
	);

	const toggle = (id: string) =>
		setSelected(current =>
			current.includes(id)
				? current.filter(item => item !== id)
				: [...current, id],
		);

	const save = async () => {
		if (detached && selected.length === 0)
			return toast.error(
				'Attach at least one network before detaching Openoxide network',
			);
		setSaving(true);
		try {
			await patchDatabase.mutateAsync({
				kind: parseDatabaseKind(database?.kind || 'postgres'),
				id: Number(database.id),
				network_ids: selected,
				detach_rustploy_network: detached ? 1 : 0,
			});
			toast.success('Database networks updated');
			onUpdated();
		} catch (error) {
			toast.error(formatApiError(error));
		} finally {
			setSaving(false);
		}
	};

	return (
		<SettingsSheet
			title="Networks"
			description="Overlay networks and isolation"
			icon={<Network className="size-4" />}
			footer={
				<Button
					onClick={save}
					disabled={saving}
					className="h-9 gap-1.5 px-4 text-sm">
					{saving ? 'Saving...' : 'Save changes'}
				</Button>
			}>
			<SheetGroup
				title="Isolation"
				description="Choose whether the default Openoxide network stays attached.">
				<div className="flex items-center justify-between gap-4 rounded-xl border border-border bg-muted/20 px-4 py-3.5">
					<div className="min-w-0">
						<p className="text-sm font-medium text-foreground">
							Detach openoxide-network
						</p>
						<p className="mt-0.5 text-[13px] leading-snug text-muted-foreground">
							Only the networks you attach below will remain after
							redeploy.
						</p>
					</div>
					<Switch
						checked={detached}
						onCheckedChange={setDetached}
						className="shrink-0"
					/>
				</div>
			</SheetGroup>

			<SheetGroup
				title="Attached networks"
				description="Overlay networks this database joins.">
				{available.length === 0 ? (
					<SheetNote>
						No database networks are available for this server.
					</SheetNote>
				) : (
					<div className="flex flex-col gap-2.5">
						{available.map(network => {
							const id = String(network.id);
							const checked = selected.includes(id);
							return (
								<label
									key={id}
									className={cn(
										'flex cursor-pointer items-center justify-between gap-4 rounded-xl border px-4 py-3 transition-colors',
										checked
											? 'border-primary/40 bg-primary/[0.04]'
											: 'border-border hover:bg-muted/30',
									)}>
									<span className="min-w-0">
										<span className="block truncate text-sm font-medium text-foreground">
											{network.name}
										</span>
										<span className="block truncate font-mono text-xs text-muted-foreground">
											{network.docker_network_name}
										</span>
									</span>
									<input
										type="checkbox"
										checked={checked}
										onChange={() => toggle(id)}
										className="size-4 shrink-0 accent-primary"
									/>
								</label>
							);
						})}
					</div>
				)}
			</SheetGroup>
		</SettingsSheet>
	);
}
