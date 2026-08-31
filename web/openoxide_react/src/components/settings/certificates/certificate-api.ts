import {CertificateService} from '#/gen/connect/openoxide/api/v1/certificate_pb';
import {CertificateOperationsService} from '#/gen/connect/openoxide/api/v1/certificate_operations_pb';
import {RemoteServerService} from '#/gen/connect/openoxide/api/v1/remote_server_pb';
import {createServiceClient} from '#/api/live/query';
import {readResource, readResourceList} from '#/api/live/resource';

export type CertificatePayload = {
	id: number;
	organization_id: number;
	name: string;
	domain: string;
	destination_id: string;
	auto_renew: number;
	created_at: number;
	updated_at: number;
};

export type CertificateRenewalPayload = {
	id: number;
	certificate_id: number;
	status: string;
	created_at: number;
};

export type RemoteServerPayload = {
	id: number;
	name: string;
	ip_address: string;
	[key: string]: unknown;
};

const certificateClient = createServiceClient(CertificateService);
const certificateOpsClient = createServiceClient(CertificateOperationsService);
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

export async function createCertificate(
	body: Partial<CertificatePayload>,
) {
	return readResource<CertificatePayload>(
		await certificateClient.createCertificate({
			payload: JSON.stringify(body),
		}),
	);
}

export async function updateCertificate(
	id: number,
	body: Partial<CertificatePayload>,
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
		await certificateOpsClient.listRenewals({id: BigInt(id)}),
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
