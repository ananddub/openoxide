import {createServiceClient, useConnectMutation} from '../query';
import {OverviewOperationsService} from '#/gen/connect/openoxide/api/v1/overview_operations_pb';
import {ComposeService} from '#/gen/connect/openoxide/api/v1/compose_pb';

export const composeClient = createServiceClient(ComposeService);
export const overviewClient = createServiceClient(
	OverviewOperationsService,
);

/** snake_case-optional id coercion that keeps `undefined` out of the proto. */
export const big = (value: number | bigint | undefined) =>
	value !== undefined ? BigInt(value) : undefined;

/**
 * Builds an id-only mutation hook (deploy/stop/delete style RPCs) with the
 * given cache invalidation set; defaults to the `composes` prefix.
 */
export const idMutation = (
	call: (id: bigint) => Promise<unknown>,
	invalidate: string[][] = [['composes']],
) =>
	function useComposeIdMutation() {
		return useConnectMutation((id: number | bigint) => call(BigInt(id)), {
			invalidate,
		});
	};

/** Builds an input-driven mutation hook scoped to the composes cache. */
export const inputMutation = <Input>(
	call: (input: Input) => Promise<unknown>,
	invalidate: string[][] = [['composes']],
) =>
	function useComposeInputMutation() {
		return useConnectMutation(call, {invalidate});
	};
