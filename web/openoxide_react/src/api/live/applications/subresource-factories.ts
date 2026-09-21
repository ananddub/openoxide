// Shared factories behind the application sub-resource hooks: every
// sub-resource lists via resource rows and mutates via
// (application id + optional row id).

import {
	useActiveOrgId,
	useConnectMutation,
	useResourceListQuery,
} from '../query';

type ResourceRows = Promise<{items: {payload: string}[]}>;

/** List hook over the resource-row envelope, keyed per application. */
export const subList = <T>(
	scope: string,
	call: (id: bigint, organizationId: bigint) => ResourceRows,
) =>
	function useSubList(id: number | bigint) {
		const orgId = useActiveOrgId();
		return useResourceListQuery<T>(
			['applications', scope, id.toString()],
			() => call(BigInt(id), orgId),
		);
	};

/** Delete hook taking `{application_id, id}` with scoped invalidation. */
export const subDelete = (
	scope: string,
	call: (id: bigint, organizationId: bigint) => Promise<unknown>,
) =>
	function useSubDelete() {
		return useConnectMutation(
			(input: {application_id: number | bigint; id: number | bigint}) =>
				call(BigInt(input.id), BigInt(input.application_id)),
			{invalidate: [['applications', scope]]},
		);
	};

/** Upsert hook: presence of `input.id` picks update vs create. */
export const subUpsert = <
	T extends {application_id: number | bigint; id?: number | bigint},
>(
	scope: string,
	buildPayload: (input: T) => Record<string, unknown>,
	create: (applicationId: bigint, payload: string) => Promise<unknown>,
	update: (
		id: bigint,
		applicationId: bigint,
		payload: string,
	) => Promise<unknown>,
) =>
	function useSubUpsert() {
		return useConnectMutation(
			(input: T) => {
				const payload = JSON.stringify(buildPayload(input));
				return input.id
					? update(BigInt(input.id), BigInt(input.application_id), payload)
					: create(BigInt(input.application_id), payload);
			},
			{invalidate: [['applications', scope]]},
		);
	};
