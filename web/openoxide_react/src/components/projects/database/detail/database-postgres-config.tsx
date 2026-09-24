import {useEffect, useState} from 'react';
import {SlidersHorizontal} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {Textarea} from '#/components/ui/textarea';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import {SettingsSheet} from '#/components/shared/settings-sheet';
import {Field, FieldGrid, SheetGroup} from '#/components/shared/field';
import {toast} from 'sonner';
import {
	useDatabasePostgresAdvancedConfig,
	useDatabaseUpdatePostgresAdvancedConfig,
} from '#/api/live';
import {formatApiError} from '#/api/utils';

const REPLICATION_ROLES = [
	{value: 'STANDALONE', label: 'Standalone'},
	{value: 'PRIMARY', label: 'Primary'},
	{value: 'REPLICA', label: 'Replica'},
];

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

	// The proto carries one JSON blob, so the backend folds the replica connection fields into the settings map on GET. Lift them back out for the form and show only the real postgresql.conf tunables in the box.
	const REPLICA_KEYS = [
		'primary_host',
		'primary_port',
		'replication_user',
	];
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
			if (separator < 1)
				return toast.error(`Invalid PostgreSQL setting: ${line}`);
			parsed[line.slice(0, separator).trim()] = line
				.slice(separator + 1)
				.trim();
		}
		if (role === 'REPLICA' && (!host.trim() || !user.trim()))
			return toast.error(
				'Primary host and replication user are required for replicas',
			);

		setSaving(true);
		try {
			// Replica fields ride inside the settings map; the backend lifts them into the DTO and keeps them for the next GET round-trip.
			if (role === 'REPLICA') {
				parsed.primary_host = host.trim();
				parsed.primary_port = String(port || 5432);
				parsed.replication_user = user.trim();
				if (password.trim()) parsed.replication_password = password.trim();
			}
			await update.mutateAsync({
				id,
				replication_role: role,
				settings: parsed,
			});
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

	return (
		<SettingsSheet
			title="PostgreSQL Configuration"
			description="postgresql.conf overrides and replication"
			icon={<SlidersHorizontal className="size-4" />}
			footer={
				<Button
					onClick={save}
					disabled={saving}
					className="h-9 gap-1.5 px-4 text-sm">
					{saving ? 'Saving...' : 'Save changes'}
				</Button>
			}>
			<SheetGroup
				title="Replication"
				description="Stream replication role for this instance.">
				<Field label="Role">
					<Select
						value={role}
						onValueChange={next => next && setRole(next)}>
						<SelectTrigger className="w-full">
							<SelectValue placeholder="Select role" />
						</SelectTrigger>
						<SelectContent>
							{REPLICATION_ROLES.map(option => (
								<SelectItem key={option.value} value={option.value}>
									{option.label}
								</SelectItem>
							))}
						</SelectContent>
					</Select>
				</Field>
				{role === 'REPLICA' && (
					<FieldGrid>
						<Field label="Primary host">
							<Input
								value={host}
								onChange={event => setHost(event.target.value)}
							/>
						</Field>
						<Field label="Primary port">
							<Input
								type="number"
								value={port}
								onChange={event => setPort(event.target.value)}
							/>
						</Field>
						<Field label="Replication user">
							<Input
								value={user}
								onChange={event => setUser(event.target.value)}
							/>
						</Field>
						<Field
							label="Replication password"
							hint={
								(data as any)?.replication_password_configured
									? 'Leave blank to keep the current password.'
									: 'Stored on the server once set.'
							}>
							<Input
								type="password"
								value={password}
								onChange={event => setPassword(event.target.value)}
								placeholder="••••••••"
							/>
						</Field>
					</FieldGrid>
				)}
			</SheetGroup>

			<SheetGroup
				title="postgresql.conf overrides"
				description="One key=value per line. Lines starting with # are ignored.">
				<Textarea
					value={settings}
					onChange={event => setSettings(event.target.value)}
					placeholder={'max_connections=200\nshared_buffers=256MB'}
					className="min-h-40 font-mono"
				/>
			</SheetGroup>
		</SettingsSheet>
	);
}
