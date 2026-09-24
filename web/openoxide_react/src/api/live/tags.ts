import {
	createServiceClient,
	useConnectMutation,
	useConnectQuery,
} from './query';
import {TagService, type Tag} from '#/gen/connect/openoxide/api/v1/tag_pb';

export type TagPayload = {
	id: number;
	name: string;
	color: string;
	organization_id: number;
	created_at: number;
};

const tagClient = createServiceClient(TagService);

function toTagPayload(tag: Tag): TagPayload {
	return {
		id: Number(tag.id),
		name: tag.name,
		color: tag.color,
		organization_id: Number(tag.organizationId),
		created_at: Number(tag.createdAt),
	};
}

/** All tags (legacy live: useTagListAll). */
export function useTagListAll() {
	return useConnectQuery<TagPayload[]>(['tags', 'list'], async () => {
		const response = await tagClient.listTags({});
		return response.tags.map(toTagPayload);
	});
}

/** Create a tag. Organization scoping derives from the session on the
 * server; `organization_id` is optional and only forwarded when set. */
export function useTagCreate() {
	return useConnectMutation(
		(input: {
			name: string;
			color: string;
			organization_id?: number | bigint;
		}) =>
			tagClient.createTag({
				name: input.name,
				color: input.color,
				organizationId:
					input.organization_id !== undefined
						? BigInt(input.organization_id)
						: undefined,
			}),
		{invalidate: [['tags']]},
	);
}

/** Update a tag (absent fields keep their stored value). */
export function useTagUpdate() {
	return useConnectMutation(
		(input: {id: number | bigint; name?: string; color?: string}) =>
			tagClient.updateTag({
				id: BigInt(input.id),
				name: input.name,
				color: input.color,
			}),
		{invalidate: [['tags'], ['projects']]},
	);
}

/** Delete a tag by id. */
export function useTagDelete() {
	return useConnectMutation(
		(id: number | bigint) => tagClient.deleteTag({id: BigInt(id)}),
		// Deleting a tag drops it from project cards, so refresh those too.
		{invalidate: [['tags'], ['projects']]},
	);
}
