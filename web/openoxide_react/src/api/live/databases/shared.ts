import {createServiceClient, useConnectMutation} from '../query';
import {OverviewOperationsService} from '#/gen/connect/openoxide/api/v1/overview_operations_pb';
import {
	DatabaseService,
	DatabaseKind,
} from '#/gen/connect/openoxide/api/v1/database_pb';
import {DatabaseNetworkService} from '#/gen/connect/openoxide/api/v1/database_network_pb';

export const databaseClient = createServiceClient(DatabaseService);
export const overviewClient = createServiceClient(
	OverviewOperationsService,
);
export const databaseNetworkClient = createServiceClient(
	DatabaseNetworkService,
);

/** snake_case-optional id coercion that keeps `undefined` out of the proto. */
export const big = (value: number | bigint | undefined) =>
	value !== undefined ? BigInt(value) : undefined;

/**
 * Builds a `{kind, id}`-driven mutation hook (deploy/stop/delete style RPCs)
 * with the given cache invalidation set; defaults to the `databases` prefix.
 */
export const kindIdMutation = (
	call: (kind: DatabaseKind, id: bigint) => Promise<unknown>,
	invalidate: string[][] = [['databases']],
) =>
	function useDatabaseKindIdMutation() {
		return useConnectMutation(
			(input: {kind: DatabaseKind; id: number | bigint}) =>
				call(input.kind, BigInt(input.id)),
			{invalidate},
		);
	};

/** Builds an input-driven mutation hook scoped to the databases cache. */
export const dbInputMutation = <Input>(
	call: (input: Input) => Promise<unknown>,
	invalidate: string[][] = [['databases']],
) =>
	function useDatabaseInputMutation() {
		return useConnectMutation(call, {invalidate});
	};
