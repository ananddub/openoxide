import {useConnectMutation, useConnectQuery} from '../query';
import {readResource} from '#/api/live/resource';
import {remoteServerClient} from './shared';
import type {
	MigrationObjectPayload,
	ServerMigrationPayload,
	StatefulMigrationPayload,
} from './types';

export function useRemoteServerMigrateDependencies() {
	return useConnectMutation(
		(input: {id: number | bigint; target_server_id: number | bigint}) =>
			remoteServerClient.migrateDependencies({
				id: BigInt(input.id),
				payload: JSON.stringify({
					target_server_id: Number(input.target_server_id),
				}),
			}),
		{invalidate: [['servers']]},
	);
}

/**
 * Watch a dependency migration. The backend takes the migration id as a
 * plain (non-JSON) payload string. Pass `pollMs` to keep the wizard live
 * between realtime table-watch invalidations (the migration runner updates
 * rows in bursts, so a short interval smooths the progress bar).
 */
export function useRemoteServerMigration(
	migrationId: string,
	options?: {pollMs?: number},
) {
	return useConnectQuery<ServerMigrationPayload>(
		['servers', 'migration', migrationId],
		async () =>
			readResource<ServerMigrationPayload>(
				await remoteServerClient.getMigration({payload: migrationId}),
			),
		{
			enabled: migrationId !== '',
			refetchInterval: options?.pollMs,
		},
	);
}

/** Roll back a dependency migration (migration id as a plain payload string). */
export function useRemoteServerRollbackMigration() {
	return useConnectMutation(
		(migrationId: string) =>
			remoteServerClient.rollbackMigration({payload: migrationId}),
		{invalidate: [['servers']]},
	);
}

/**
 * Request cooperative cancellation of a RUNNING stateful
 * (`application_migrations`) migration. The runner honours the flag at the
 * next phase/object boundary; the id is passed as a plain payload string.
 */
export function useRemoteServerCancelMigration() {
	return useConnectMutation(
		(migrationId: string) =>
			remoteServerClient.cancelMigration({payload: migrationId}),
		{invalidate: [['servers']]},
	);
}

/**
 * Trigger (or resume) a stateful migration of one service. `id` is the
 * service id; the payload carries the target server. Returns the new (or
 * resumed RUNNING) migration row — watch it via `useServiceMigrations`
 * (newest first) or fetch its checkpoints via `useMigrationObjects`.
 */
export function useRemoteServerMigrateService() {
	return useConnectMutation(
		async (input: {
			serviceId: number | bigint;
			targetServerId: number | bigint;
		}) => {
			const result = await remoteServerClient.migrateService({
				id: BigInt(input.serviceId),
				payload: JSON.stringify({
					target_server_id: Number(input.targetServerId),
				}),
			});
			return JSON.parse(
				result.payload ?? '{}',
			) as StatefulMigrationPayload;
		},
		{invalidate: [['servers']]},
	);
}

/** All stateful migrations for one service, newest first. */
export function useServiceMigrations(
	serviceId: bigint | number,
	options?: {pollMs?: number},
) {
	return useConnectQuery<StatefulMigrationPayload[]>(
		['servers', 'migrations', String(serviceId)],
		async () => {
			const list = await remoteServerClient.listMigrations({
				id: BigInt(serviceId),
			});
			return list.items.map(
				item => JSON.parse(item.payload) as StatefulMigrationPayload,
			);
		},
		{
			enabled: serviceId !== 0n && serviceId !== 0,
			refetchInterval: options?.pollMs,
		},
	);
}

/** Per-object checkpoints of one stateful migration (id as plain payload). */
export function useMigrationObjects(
	migrationId: string,
	options?: {pollMs?: number},
) {
	return useConnectQuery<MigrationObjectPayload[]>(
		['servers', 'migration-objects', migrationId],
		async () => {
			const list = await remoteServerClient.listMigrationObjects({
				payload: migrationId,
			});
			return list.items.map(
				item => JSON.parse(item.payload) as MigrationObjectPayload,
			);
		},
		{enabled: migrationId !== '', refetchInterval: options?.pollMs},
	);
}
