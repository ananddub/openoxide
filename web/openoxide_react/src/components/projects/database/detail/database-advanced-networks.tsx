import {useEffect, useMemo, useState} from 'react';
import {Network, Save} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Switch} from '#/components/ui/switch';
import {toast} from 'sonner';
import {$api} from '#/api/query';
import {formatApiError} from '#/api/utils';
import {databaseEndpoint} from './database-endpoint';

export function DatabaseAdvancedNetworks({database, onUpdated}: any) {
	const {data: networks} = $api.useQuery('get', '/database-networks');
	const patchDatabase = $api.useMutation(
		'patch',
		databaseEndpoint(database) as any,
	);
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
			return toast.error('Attach at least one network before detaching Openoxide network');
		setSaving(true);
		try {
			await patchDatabase.mutateAsync({
				params: {path: {id: Number(database.id)}},
				body: {
					network_ids: selected,
					detach_rustploy_network: detached ? 1 : 0,
				},
			} as any);
			toast.success('Database networks updated');
			onUpdated();
		} catch (error) {
			toast.error(formatApiError(error));
		} finally {
			setSaving(false);
		}
	};

	return (
		<section className="flex flex-col gap-5 rounded-2xl border border-border/80 bg-card p-6 shadow-sm">
			<div>
				<h3 className="flex items-center gap-2 text-base font-bold">
					<Network className="size-4 text-primary" /> Networks
				</h3>
				<p className="mt-0.5 text-xs text-muted-foreground">
					Attach overlay networks or isolate this database from the shared Openoxide network.
				</p>
			</div>
			<div className="flex items-center justify-between gap-4 rounded-xl border p-4">
				<div>
					<p className="text-xs font-bold">Detach openoxide-network</p>
					<p className="mt-1 text-[11px] text-muted-foreground">Only attached networks will be available after redeploy.</p>
				</div>
				<Switch checked={detached} onCheckedChange={setDetached} />
			</div>
			<div className="flex flex-col gap-2 border-t border-border/40 pt-4">
				{available.length === 0 ? (
					<p className="rounded-lg border border-dashed p-4 text-center text-xs text-muted-foreground">No database networks available for this server.</p>
				) : available.map(network => {
					const id = String(network.id);
					return (
						<label key={id} className="flex cursor-pointer items-center justify-between rounded-lg border p-3 hover:bg-muted/20">
							<span><span className="block text-xs font-bold">{network.name}</span><span className="text-[11px] text-muted-foreground">{network.docker_network_name}</span></span>
							<input type="checkbox" checked={selected.includes(id)} onChange={() => toggle(id)} className="size-4 accent-primary" />
						</label>
					);
				})}
			</div>
			<div className="flex justify-end border-t border-border/40 pt-4">
				<Button onClick={save} disabled={saving} className="h-9 gap-1.5 text-xs"><Save className="size-3.5" /> {saving ? 'Saving...' : 'Save Networks'}</Button>
			</div>
		</section>
	);
}
