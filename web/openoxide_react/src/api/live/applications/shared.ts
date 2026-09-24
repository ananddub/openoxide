import {createServiceClient, useConnectMutation} from '../query';
import {OverviewService} from '#/gen/connect/openoxide/api/v1/overview_pb';
import {ApplicationService} from '#/gen/connect/openoxide/api/v1/application_pb';

export const applicationClient = createServiceClient(ApplicationService);
export const overviewClient = createServiceClient(OverviewService);

/** snake_case-optional id coercion that keeps `undefined` out of the proto. */
export const big = (value: number | bigint | undefined) =>
	value !== undefined ? BigInt(value) : undefined;

/** Builds an id-only mutation hook (deploy/stop/delete style RPCs) with the given cache invalidation set; defaults to the `applications` prefix. */
export const idMutation = (
	call: (id: bigint) => Promise<unknown>,
	invalidate: string[][] = [['applications']],
) =>
	function useAppIdMutation() {
		return useConnectMutation((id: number | bigint) => call(BigInt(id)), {
			invalidate,
		});
	};

/** Builds an input-driven mutation hook scoped to the applications cache. */
export const inputMutation = <Input>(
	call: (input: Input) => Promise<unknown>,
	invalidate: string[][] = [['applications']],
) =>
	function useAppInputMutation() {
		return useConnectMutation(call, {invalidate});
	};
