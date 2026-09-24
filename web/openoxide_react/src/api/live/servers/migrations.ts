import {useConnectMutation, useConnectQuery} from '../query';
import {
	RemoteServerApplicationMigrationStatus,
	RemoteServerDependencyMigrationStatus,
	RemoteServerMigrationObjectKind,
	RemoteServerMigrationPhase,
	type RemoteServerApplicationMigration,
	type RemoteServerDependencyMigration,
	type RemoteServerMigrationObject,
} from '#/gen/connect/openoxide/api/v1/remote_server_pb';
import {remoteServerClient} from './shared';
import type {
	MigrationObjectPayload,
	ServerMigrationPayload,
	StatefulMigrationPayload,
} from './types';

/** Maps a dependency-migration message; enums become their name strings. */
export function toServerMigrationPayload(
	migration: RemoteServerDependencyMigration,
): ServerMigrationPayload {
	return {
		migration_id: migration.migrationId,
		status: RemoteServerDependencyMigrationStatus[migration.status],
		source_server_id: Number(migration.sourceServerId),
		target_server_id: Number(migration.targetServerId),
		applications: Number(migration.applications),
		build_assignments: Number(migration.buildAssignments),
		compose_projects: Number(migration.composeProjects),
		databases: Number(migration.databases),
		certificates: Number(migration.certificates),
		schedules: Number(migration.schedules),
		queued_applications: Number(migration.queuedApplications),
		queued_compose_projects: Number(migration.queuedComposeProjects),
		error: migration.error ?? null,
	};
}

/** Maps a stateful (`application_migrations`) message. */
export function toStatefulMigrationPayload(
	migration: RemoteServerApplicationMigration,
): StatefulMigrationPayload {
	return {
		id: migration.id,
		organization_id: Number(migration.organizationId),
		service_id: Number(migration.serviceId),
		source_server_id: Number(migration.sourceServerId),
		target_server_id: Number(migration.targetServerId),
		status: RemoteServerApplicationMigrationStatus[migration.status],
		phase: RemoteServerMigrationPhase[migration.phase],
		progress: Number(migration.progress),
		cancel_requested: migration.cancelRequested ? 1 : 0,
		error: migration.error ?? null,
		log_path: migration.logPath,
		created_at: Number(migration.createdAt),
		updated_at: Number(migration.updatedAt),
	};
}

/** Maps a per-object checkpoint message. */
export function toMigrationObjectPayload(
	object: RemoteServerMigrationObject,
): MigrationObjectPayload {
	return {
		migration_id: object.migrationId,
		obj_key: object.objKey,
		kind: RemoteServerMigrationObjectKind[object.kind],
		source_ref: object.sourceRef,
		dest_ref: object.destRef,
		status: object.status,
		bytes: Number(object.bytes),
		error: object.error ?? null,
		created_at: Number(object.createdAt),
		updated_at: Number(object.updatedAt),
	};
}

export function useRemoteServerMigrateDependencies() {
	return useConnectMutation(
		async (input: {
			id: number | bigint;
			target_server_id: number | bigint;
		}) => {
			const response = await remoteServerClient.migrateDependencies({
				id: BigInt(input.id),
				targetServerId: BigInt(input.target_server_id),
			});
			return response.migration
				? toServerMigrationPayload(response.migration)
				: undefined;
		},
		{invalidate: [['servers']]},
	);
}

/** Watch a dependency migration by its UUID. Pass `pollMs` to keep the wizard live between realtime table-watch invalidations (the migration runner updates rows in bursts, so a short interval smooths the progress bar). */
export function useRemoteServerMigration(
	migrationId: string,
	options?: {pollMs?: number},
) {
	return useConnectQuery<ServerMigrationPayload>(
		['servers', 'migration', migrationId],
		async () => {
			const response = await remoteServerClient.getMigration({
				migrationId,
			});
			if (!response.migration) throw new Error('migration not found');
			return toServerMigrationPayload(response.migration);
		},
		{
			enabled: migrationId !== '',
			refetchInterval: options?.pollMs,
		},
	);
}

/** Roll back a dependency migration (by migration UUID). */
export function useRemoteServerRollbackMigration() {
	return useConnectMutation(
		async (migrationId: string) => {
			const response = await remoteServerClient.rollbackMigration({
				migrationId,
			});
			return response.migration
				? toServerMigrationPayload(response.migration)
				: undefined;
		},
		{invalidate: [['servers']]},
	);
}

/** Request cooperative cancellation of a RUNNING stateful (`application_migrations`) migration. The runner honours the flag at the next phase/object boundary. */
export function useRemoteServerCancelMigration() {
	return useConnectMutation(
		async (migrationId: string) => {
			const response = await remoteServerClient.cancelMigration({
				migrationId,
			});
			return response.migration
				? toStatefulMigrationPayload(response.migration)
				: undefined;
		},
		{invalidate: [['servers']]},
	);
}

/** Trigger (or resume) a stateful migration of one service. `serviceId` is the service id; returns the new (or resumed RUNNING) migration row — watch it via `useServiceMigrations` (newest first) or fetch its checkpoints via `useMigrationObjects`. */
export function useRemoteServerMigrateService() {
	return useConnectMutation(
		async (input: {
			serviceId: number | bigint;
			targetServerId: number | bigint;
		}) => {
			const response = await remoteServerClient.migrateService({
				id: BigInt(input.serviceId),
				targetServerId: BigInt(input.targetServerId),
			});
			if (!response.migration) {
				throw new Error('migration did not start');
			}
			return toStatefulMigrationPayload(response.migration);
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
				serviceId: BigInt(serviceId),
			});
			return list.migrations.map(toStatefulMigrationPayload);
		},
		{
			enabled: serviceId !== 0n && serviceId !== 0,
			refetchInterval: options?.pollMs,
		},
	);
}

/** Per-object checkpoints of one stateful migration (by migration UUID). */
export function useMigrationObjects(
	migrationId: string,
	options?: {pollMs?: number},
) {
	return useConnectQuery<MigrationObjectPayload[]>(
		['servers', 'migration-objects', migrationId],
		async () => {
			const list = await remoteServerClient.listMigrationObjects({
				migrationId,
			});
			return list.objects.map(toMigrationObjectPayload);
		},
		{enabled: migrationId !== '', refetchInterval: options?.pollMs},
	);
}
