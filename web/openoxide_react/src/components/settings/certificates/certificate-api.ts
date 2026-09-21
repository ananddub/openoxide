import {CertificateOperationsService} from '#/gen/connect/openoxide/api/v1/certificate_operations_pb';
import {RemoteServerService} from '#/gen/connect/openoxide/api/v1/remote_server_pb';
import {createServiceClient} from '#/api/live/query';
import {readResource, readResourceList} from '#/api/live/resource';

export type CertificatePayload = {
	// Mirrors the backend `CertificateResponseDto` wire shape.
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

export type CertificateRenewalPayload = {
	// Mirrors the backend `CertificateRenewalDto` wire shape.
	id: number;
	certificate_id: number;
	organization_id: number;
	status: string;
	previous_expires_at?: number;
	new_expires_at?: number;
	error?: string;
	started_at: number;
	finished_at?: number;
};

export type RemoteServerPayload = {
	id: number;
	name: string;
	ip_address: string;
	[key: string]: unknown;
};

/** Create/update request body (backend Create/PatchCertificateDto fields). */
export type CertificateInput = {
	name?: string;
	certificate_data?: string;
	private_key?: string;
	certificate_path?: string;
	auto_renew?: number;
	server_id?: string;
};

const certificateClient = createServiceClient(
	CertificateOperationsService,
);

const remoteServerClient = createServiceClient(RemoteServerService);

export async function listCertificates() {
	return readResourceList<CertificatePayload>(
		await certificateClient.listCertificates({}),
	);
}

export async function listRemoteServers() {
	return readResourceList<RemoteServerPayload>(
		await remoteServerClient.listRemoteServers({}),
	);
}

export async function createCertificate(body: CertificateInput) {
	return readResource<CertificatePayload>(
		await certificateClient.createCertificate({
			payload: JSON.stringify(body),
		}),
	);
}

export async function updateCertificate(
	id: number,
	body: CertificateInput,
) {
	return readResource<CertificatePayload>(
		await certificateClient.updateCertificate({
			id: BigInt(id),
			payload: JSON.stringify(body),
		}),
	);
}

export async function deleteCertificate(id: number) {
	await certificateClient.deleteCertificate({id: BigInt(id)});
}

export async function listCertificateRenewals(id: number) {
	return readResourceList<CertificateRenewalPayload>(
		await certificateClient.listRenewals({id: BigInt(id)}),
	);
}

export async function renewCertificate(
	id: number,
	body: Record<string, unknown>,
) {
	return certificateClient.renewCertificate({
		id: BigInt(id),
		payload: JSON.stringify(body),
	});
}
