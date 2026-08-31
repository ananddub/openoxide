import {createServiceClient, useConnectMutation, useResourceListQuery} from './query';
import {TagService} from '#/gen/connect/openoxide/api/v1/tag_pb';

export type TagPayload = {
	id: number;
	name: string;
	color: string;
	organization_id: number;
	created_at: number;
};

const tagClient = createServiceClient(TagService);

/** All tags (legacy live: useTagListAll). */
export function useTagListAll() {
	return useResourceListQuery<TagPayload>(
		['tags', 'list'],
		() => tagClient.listTags({}),
	);
}

/** Create a tag (name + color in the payload). */
export function useTagCreate() {
	return useConnectMutation((input: {name: string; color: string}) =>
		tagClient.createTag({payload: JSON.stringify(input)}),
	);
}

/** Update a tag (name + color in the payload). */
export function useTagUpdate() {
	return useConnectMutation((input: {id: number; name: string; color: string}) => {
		const {id, ...body} = input;
		return tagClient.updateTag({id: BigInt(id), payload: JSON.stringify(body)});
	});
}

/** Delete a tag by id. */
export function useTagDelete() {
	return useConnectMutation((id: number) =>
		tagClient.deleteTag({id: BigInt(id)}),
	);
}
