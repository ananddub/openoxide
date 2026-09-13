import {createServiceClient, useResourceListQuery, useConnectQuery} from './query';
import {CertificateOperationsService} from '#/gen/connect/openoxide/api/v1/certificate_operations_pb';

export type CertificatePayload = {
	id: string;
	name: string;
	certificate_data: string;
	has_private_key: boolean;
	certificate_path: string;
	auto_renew: number;
	server_id?: string;
	organization_id: number;
	created_at: number;
	updated_at: number;
};

const certificateClient = createServiceClient(CertificateOperationsService);

/** Certificates (legacy live: useCertificateList). */
export function useCertificateList() {
	return useResourceListQuery<CertificatePayload>(
		['certificates', 'list'],
		() => certificateClient.listCertificates({}),
	);
}

/**
 * Live certificate list. The table-watch stream invalidates the
 * `certificates` prefix on table changes, which refetches this query
 * while the page is mounted (and only refreshes its staleness when it
 * is not) — the query cache is the store, no zustand slice involved.
 */
export function useCertificateItems() {
	return useConnectQuery<CertificatePayload[]>(
		['certificates'],
		async () =>
			certificateClient.listCertificates({}).then((response) =>
				response.items.map(
					(item) => JSON.parse(item.payload) as CertificatePayload,
				),
			),
	);
}
