import {DatabaseKind} from '#/gen/connect/openoxide/api/v1/database_pb';
import {databaseClient, kindIdMutation} from './shared';

const DEPLOY_INVALIDATE = [['databases'], ['deployments']];

/** Deploy a database of the given kind by id (legacy REST: POST /databases/{kind}/{id}/deploy). */
export const useDatabaseDeploy = kindIdMutation(
	(kind, id) => databaseClient.deploy({kind, id}),
	DEPLOY_INVALIDATE,
);

/** Redeploy a database of the given kind by id. */
export const useDatabaseRedeploy = kindIdMutation(
	(kind, id) => databaseClient.redeploy({kind, id}),
	DEPLOY_INVALIDATE,
);

/** Reload a database's configuration by id. */
export const useDatabaseReload = kindIdMutation((kind, id) =>
	databaseClient.reload({kind, id}),
);

/** Start a stopped database of the given kind by id. */
export const useDatabaseStart = kindIdMutation((kind, id) =>
	databaseClient.start({kind, id}),
);

/** Stop a running database of the given kind by id. */
export const useDatabaseStop = kindIdMutation((kind, id) =>
	databaseClient.stop({kind, id}),
);

/** Cancel the running deployment of a database of the given kind by id. */
export const useDatabaseCancel = kindIdMutation(
	(kind, id) => databaseClient.cancel({kind, id}),
	DEPLOY_INVALIDATE,
);

/** Map a legacy kind string ('postgres', 'mysql', ...) to the proto enum. */
export function parseDatabaseKind(kind: string): DatabaseKind {
	const value = String(kind || '').toLowerCase();
	if (value.includes('mariadb')) return DatabaseKind.MARIADB;
	if (value.includes('mysql')) return DatabaseKind.MYSQL;
	if (value.includes('mongo')) return DatabaseKind.MONGO;
	if (value.includes('redis')) return DatabaseKind.REDIS;
	if (value.includes('libsql')) return DatabaseKind.LIBSQL;
	return DatabaseKind.POSTGRES;
}
