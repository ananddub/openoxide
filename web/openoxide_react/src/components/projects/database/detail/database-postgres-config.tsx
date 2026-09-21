import {useEffect, useState} from 'react';
import {Save, SlidersHorizontal} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {Textarea} from '#/components/ui/textarea';
import {toast} from 'sonner';
import {useDatabasePostgresAdvancedConfig, useDatabaseUpdatePostgresAdvancedConfig} from '#/api/live';
import {formatApiError} from '#/api/utils';

export function DatabasePostgresConfig({database, onUpdated}: any) {
	const id = Number(database?.id);
	const {data, refetch} = useDatabasePostgresAdvancedConfig(BigInt(id));
	const update = useDatabaseUpdatePostgresAdvancedConfig();
	const [settings, setSettings] = useState('');
	const [role, setRole] = useState('STANDALONE');
	const [host, setHost] = useState('');
	const [port, setPort] = useState('5432');
	const [user, setUser] = useState('');
	const [password, setPassword] = useState('');
	const [saving, setSaving] = useState(false);

	// The proto carries one JSON blob, so the backend folds the replica
	// connection fields into the settings map on GET. Lift them back out for
	// the form and show only the real postgresql.conf tunables in the box.
	const REPLICA_KEYS = ['primary_host', 'primary_port', 'replication_user'];
	useEffect(() => {
		const all = data?.settings || {};
		setSettings(
			Object.entries(all)
				.filter(([key]) => !REPLICA_KEYS.includes(key))
				.map(([key, value]) => `${key}=${value}`)
				.join('\n'),
		);
		setRole(data?.replication_role || 'STANDALONE');
		setHost(all.primary_host || '');
		setPort(all.primary_port || '5432');
		setUser(all.replication_user || '');
	}, [data]);

	const save = async () => {
		const parsed: Record<string, string> = {};
		for (const raw of settings.split('\n')) {
			const line = raw.trim();
			if (!line || line.startsWith('#')) continue;
			const separator = line.indexOf('=');
			if (separator < 1) return toast.error(`Invalid PostgreSQL setting: ${line}`);
			parsed[line.slice(0, separator).trim()] = line.slice(separator + 1).trim();
		}
		if (role === 'REPLICA' && (!host.trim() || !user.trim()))
			return toast.error('Primary host and replication user are required for replicas');

		setSaving(true);
		try {
			// Replica fields ride inside the settings map; the backend lifts
			// them into the DTO and keeps them for the next GET round-trip.
			if (role === 'REPLICA') {
				parsed.primary_host = host.trim();
				parsed.primary_port = String(port || 5432);
				parsed.replication_user = user.trim();
				if (password.trim())
					parsed.replication_password = password.trim();
			}
			await update.mutateAsync({id, replication_role: role, settings: parsed});
			toast.success('PostgreSQL configuration updated');
			setPassword('');
			await refetch();
			onUpdated();
		} catch (error) {
			toast.error(formatApiError(error));
		} finally {
			setSaving(false);
		}
	};

	return <section className="flex flex-col gap-5 rounded-2xl bg-card p-6 shadow-sm">
		<div><h3 className="flex items-center gap-2 text-base font-bold"><SlidersHorizontal className="size-4 text-primary" /> PostgreSQL Configuration</h3><p className="mt-0.5 text-xs text-muted-foreground">Configure postgresql.conf overrides and primary/replica behavior.</p></div>
		<div className="grid gap-4 border-t  pt-4 sm:grid-cols-2">
			<label className="flex flex-col gap-1.5 text-[11px] font-bold uppercase text-muted-foreground">Replication role<select value={role} onChange={event => setRole(event.target.value)} className="h-9 rounded-md  bg-background px-2 text-xs normal-case"><option value="STANDALONE">Standalone</option><option value="PRIMARY">Primary</option><option value="REPLICA">Replica</option></select></label>
			{role === 'REPLICA' && <><label className="flex flex-col gap-1.5 text-[11px] font-bold uppercase text-muted-foreground">Primary host<Input value={host} onChange={event => setHost(event.target.value)} className="h-9 text-xs normal-case" /></label><label className="flex flex-col gap-1.5 text-[11px] font-bold uppercase text-muted-foreground">Primary port<Input type="number" value={port} onChange={event => setPort(event.target.value)} className="h-9 text-xs normal-case" /></label><label className="flex flex-col gap-1.5 text-[11px] font-bold uppercase text-muted-foreground">Replication user<Input value={user} onChange={event => setUser(event.target.value)} className="h-9 text-xs normal-case" /></label><label className="flex flex-col gap-1.5 text-[11px] font-bold uppercase text-muted-foreground sm:col-span-2">Replication password<Input type="password" value={password} onChange={event => setPassword(event.target.value)} placeholder={(data as any)?.replication_password_configured ? 'Leave blank to keep current password' : 'Enter replication password'} className="h-9 text-xs normal-case" /></label></>}
			<label className="flex flex-col gap-1.5 text-[11px] font-bold uppercase text-muted-foreground sm:col-span-2">Settings (one key=value per line)<Textarea value={settings} onChange={event => setSettings(event.target.value)} placeholder={'max_connections=200\nshared_buffers=256MB'} className="min-h-32 font-mono text-xs normal-case" /></label>
		</div>
		<div className="flex justify-end border-t  pt-4"><Button onClick={save} disabled={saving} className="h-9 gap-1.5 text-xs"><Save className="size-3.5" /> {saving ? 'Saving...' : 'Save PostgreSQL Config'}</Button></div>
	</section>;
}
