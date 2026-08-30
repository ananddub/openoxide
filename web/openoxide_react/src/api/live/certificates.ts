import {createServiceClient, useResourceListQuery} from './query';
import {CertificateService} from '#/gen/connect/openoxide/api/v1/certificate_pb';

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

const certificateClient = createServiceClient(CertificateService);

/** Certificates (legacy live: useCertificateList). */
export function useCertificateList() {
	return useResourceListQuery<CertificatePayload>(
		['certificates', 'list'],
		() => certificateClient.listCertificates({}),
	);
}
