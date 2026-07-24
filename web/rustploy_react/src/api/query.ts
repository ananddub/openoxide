import createQueryClient from 'openapi-react-query';
import {client} from '#/api/client';

// Type-safe react-query wrapper over the openapi-fetch client.
// Usage:
//   const { data, isLoading } = $api.useQuery('get', '/projects/{id}', { params: { path: { id: 1 } } })
//   const mutation = $api.useMutation('post', '/applications')
export const $api = createQueryClient(client);
