/**
 * Resource-pattern helpers: most openoxide services return
 * `ResourceResponse { id, kind, payload }` where `payload` is a JSON
 * string in the legacy snake_case REST shape. These helpers parse the
 * payload while preserving the wire shape consumers already depend on.
 */

export function readResource<Payload>(response: {
	payload: string;
}): Payload {
	return JSON.parse(response.payload) as Payload;
}

export function readResourceList<Payload>(response: {
	items: Array<{payload: string}>;
}): Payload[] {
	return response.items.map((item) => JSON.parse(item.payload) as Payload);
}
