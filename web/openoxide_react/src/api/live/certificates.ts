import {createServiceClient, useConnectQuery} from './query';
import {
	CertificateService,
	type Certificate,
} from '#/gen/connect/openoxide/api/v1/certificate_pb';

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

/** Typed `Certificate` → legacy snake_case payload (UI stability): the former JSON DTO carried `auto_renew` as 0/1 and numeric timestamps. */
export function toCertificatePayload(
	certificate: Certificate,
): CertificatePayload {
	return {
		id: certificate.id.toString(),
		name: certificate.name,
		certificate_data: certificate.certificateData,
		has_private_key: certificate.hasPrivateKey,
		certificate_path: certificate.certificatePath,
		auto_renew: certificate.autoRenew ? 1 : 0,
		server_id: certificate.serverId,
		organization_id: Number(certificate.organizationId),
		created_at: Number(certificate.createdAt),
		updated_at: Number(certificate.updatedAt),
	};
}

/** Certificates (legacy live: useCertificateList). */
export function useCertificateList() {
	return useConnectQuery<CertificatePayload[]>(
		['certificates', 'list'],
		async () => {
			const response = await certificateClient.listCertificates({});
			return response.certificates.map(toCertificatePayload);
		},
	);
}

/** Live certificate list. The table-watch stream invalidates the `certificates` prefix on table changes, which refetches this query while the page is mounted (and only refreshes its staleness when it is not) — the query cache is the store, no zustand slice involved. */
export function useCertificateItems() {
	return useConnectQuery<CertificatePayload[]>(
		['certificates'],
		async () => {
			const response = await certificateClient.listCertificates({});
			return response.certificates.map(toCertificatePayload);
		},
	);
}
