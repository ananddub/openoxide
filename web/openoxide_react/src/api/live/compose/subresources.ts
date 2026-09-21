import {useConnectMutation, useResourceListQuery} from '../query';
import {composeClient} from './shared';
import type {ComposeMountPayload, ComposePatchPayload} from './types';

/** Volumes/mounts of a compose project (Advanced → Volumes). */
export function useComposeMountList(compose_id: number | bigint) {
	return useResourceListQuery<ComposeMountPayload>(
		['composes', 'mounts', compose_id.toString()],
		() => composeClient.listMounts({composeId: BigInt(compose_id)}),
	);
}

export function useComposeMountUpsert() {
	return useConnectMutation(
		(input: {
			compose_id: number | bigint;
			id?: number | bigint;
			mount_type: string;
			host_path?: string | null;
			volume_name?: string | null;
			file_path?: string | null;
			content?: string | null;
			mount_path: string;
		}) =>
			composeClient[input.id ? 'updateMount' : 'createMount']({
				composeId: BigInt(input.compose_id),
				id: input.id !== undefined ? BigInt(input.id) : undefined,
				mountType: input.mount_type,
				hostPath: input.host_path ?? undefined,
				volumeName: input.volume_name ?? undefined,
				filePath: input.file_path ?? undefined,
				content: input.content ?? undefined,
				mountPath: input.mount_path,
			}),
		{invalidate: [['composes', 'mounts']]},
	);
}

export function useComposeMountDelete() {
	return useConnectMutation(
		(input: {compose_id: number | bigint; id: number | bigint}) =>
			composeClient.deleteMount({
				composeId: BigInt(input.compose_id),
				id: BigInt(input.id),
			}),
		{invalidate: [['composes', 'mounts']]},
	);
}

/** Compose patches (env/secret/config overlays) — Advanced → Patches. */
export function useComposePatchList(compose_id: number | bigint) {
	return useResourceListQuery<ComposePatchPayload>(
		['composes', 'patches', compose_id.toString()],
		() => composeClient.listPatches({composeId: BigInt(compose_id)}),
	);
}

export function useComposePatchUpsert() {
	return useConnectMutation(
		(input: {
			compose_id: number | bigint;
			id?: number | bigint;
			patch_type: string;
			payload: string;
		}) =>
			composeClient[input.id ? 'updatePatch' : 'createPatch']({
				composeId: BigInt(input.compose_id),
				id: input.id !== undefined ? BigInt(input.id) : undefined,
				patchType: input.patch_type,
				payload: input.payload,
			}),
		{invalidate: [['composes', 'patches']]},
	);
}

export function useComposePatchDelete() {
	return useConnectMutation(
		(input: {compose_id: number | bigint; id: number | bigint}) =>
			composeClient.deletePatch({
				composeId: BigInt(input.compose_id),
				id: BigInt(input.id),
			}),
		{invalidate: [['composes', 'patches']]},
	);
}
