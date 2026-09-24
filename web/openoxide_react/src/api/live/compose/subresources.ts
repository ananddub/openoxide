// Compose sub-resources: volumes/mounts and patches. The list RPCs return typed entities; requests carry the entity fields directly (composeId + entity fields). Payload shapes stay in ./types.

import {useConnectMutation, useConnectQuery} from '../query';
import {
	ComposeMountType,
	ComposePatchType,
} from '#/gen/connect/openoxide/api/v1/compose_pb';
import {composeClient} from './shared';
import {
	toComposeMountPayload,
	toComposePatchPayload,
	type ComposeMountPayload,
	type ComposePatchPayload,
} from './types';

// The mount/patch cards keep passing uppercase enum names ('VOLUME', 'DELETE', …); map them onto the generated enum members.
const mountTypeOf = (name: string): ComposeMountType => {
	if (name === ComposeMountType[ComposeMountType.BIND]) {
		return ComposeMountType.BIND;
	}
	if (name === ComposeMountType[ComposeMountType.FILE]) {
		return ComposeMountType.FILE;
	}
	return ComposeMountType.VOLUME;
};

const patchTypeOf = (name: string): ComposePatchType =>
	name === ComposePatchType[ComposePatchType.DELETE]
		? ComposePatchType.DELETE
		: ComposePatchType.UPDATE;

/** Volumes/mounts of a compose project (Advanced → Volumes). */
export function useComposeMountList(compose_id: number | bigint) {
	return useConnectQuery<ComposeMountPayload[]>(
		['composes', 'mounts', compose_id.toString()],
		async () => {
			const response = await composeClient.listMounts({
				composeId: BigInt(compose_id),
			});
			return response.mounts.map(toComposeMountPayload);
		},
	);
}

export function useComposeMountUpsert() {
	return useConnectMutation(
		async (input: {
			compose_id: number | bigint;
			id?: number | bigint;
			mount_type: string;
			host_path?: string | null;
			volume_name?: string | null;
			file_path?: string | null;
			content?: string | null;
			mount_path: string;
		}) => {
			const fields = {
				mountType: mountTypeOf(input.mount_type),
				hostPath: input.host_path || undefined,
				volumeName: input.volume_name || undefined,
				filePath: input.file_path || undefined,
				content: input.content || undefined,
				mountPath: input.mount_path,
			};
			return input.id === undefined
				? composeClient.createMount({
						composeId: BigInt(input.compose_id),
						...fields,
					})
				: composeClient.updateMount({
						composeId: BigInt(input.compose_id),
						id: BigInt(input.id),
						...fields,
					});
		},
		{invalidate: [['composes', 'mounts']]},
	);
}

export function useComposeMountDelete() {
	return useConnectMutation(
		async (input: {compose_id: number | bigint; id: number | bigint}) =>
			composeClient.deleteMount({
				composeId: BigInt(input.compose_id),
				id: BigInt(input.id),
			}),
		{invalidate: [['composes', 'mounts']]},
	);
}

/** Compose patches (file overlays) — Advanced → Patches. */
export function useComposePatchList(compose_id: number | bigint) {
	return useConnectQuery<ComposePatchPayload[]>(
		['composes', 'patches', compose_id.toString()],
		async () => {
			const response = await composeClient.listPatches({
				composeId: BigInt(compose_id),
			});
			return response.patches.map(toComposePatchPayload);
		},
	);
}

export function useComposePatchUpsert() {
	return useConnectMutation(
		async (input: {
			compose_id: number | bigint;
			id?: number | bigint;
			patch_type: string;
			file_path: string;
			enabled?: boolean;
			content?: string;
		}) => {
			const fields = {
				patchType: patchTypeOf(input.patch_type),
				filePath: input.file_path,
				enabled: input.enabled ?? true,
				content: input.content ?? '',
			};
			return input.id === undefined
				? composeClient.createPatch({
						composeId: BigInt(input.compose_id),
						...fields,
					})
				: composeClient.updatePatch({
						composeId: BigInt(input.compose_id),
						id: BigInt(input.id),
						...fields,
					});
		},
		{invalidate: [['composes', 'patches']]},
	);
}

export function useComposePatchDelete() {
	return useConnectMutation(
		async (input: {compose_id: number | bigint; id: number | bigint}) =>
			composeClient.deletePatch({
				composeId: BigInt(input.compose_id),
				id: BigInt(input.id),
			}),
		{invalidate: [['composes', 'patches']]},
	);
}
