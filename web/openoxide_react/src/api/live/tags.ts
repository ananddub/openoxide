import {createServiceClient, useResourceListQuery} from './query';
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
