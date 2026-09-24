import {
	CertificateRenewalStatus,
	CertificateService,
	type Certificate,
	type CertificateRenewal,
} from '#/gen/connect/openoxide/api/v1/certificate_pb';
import {
	RemoteServerService,
	type RemoteServer,
} from '#/gen/connect/openoxide/api/v1/remote_server_pb';
import {createServiceClient} from '#/api/live/query';

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
	/** Legacy 0/1 number; truthy enables auto-renew. */
	auto_renew?: number;
	server_id?: string;
};

const certificateClient = createServiceClient(CertificateService);

const remoteServerClient = createServiceClient(RemoteServerService);

function toCertificatePayload(
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

function renewalStatusName(status: CertificateRenewalStatus): string {
	switch (status) {
		case CertificateRenewalStatus.RUNNING:
			return 'RUNNING';
		case CertificateRenewalStatus.SUCCEEDED:
			return 'SUCCEEDED';
		case CertificateRenewalStatus.FAILED:
			return 'FAILED';
		default:
			return 'PENDING';
	}
}

function toRenewalPayload(
	renewal: CertificateRenewal,
): CertificateRenewalPayload {
	return {
		id: Number(renewal.id),
		certificate_id: Number(renewal.certificateId),
		organization_id: Number(renewal.organizationId),
		status: renewalStatusName(renewal.status),
		previous_expires_at:
			renewal.previousExpiresAt === undefined
				? undefined
				: Number(renewal.previousExpiresAt),
		new_expires_at:
			renewal.newExpiresAt === undefined
				? undefined
				: Number(renewal.newExpiresAt),
		error: renewal.error,
		started_at: Number(renewal.startedAt),
		finished_at:
			renewal.finishedAt === undefined
				? undefined
				: Number(renewal.finishedAt),
	};
}

function toRemoteServerPayload(server: RemoteServer): RemoteServerPayload {
	return {
		id: Number(server.id),
		name: server.name,
		ip_address: server.ipAddress,
	};
}

export async function listCertificates(): Promise<CertificatePayload[]> {
	const response = await certificateClient.listCertificates({});
	return response.certificates.map(toCertificatePayload);
}

export async function listRemoteServers(): Promise<RemoteServerPayload[]> {
	const response = await remoteServerClient.listRemoteServers({});
	return response.servers.map(toRemoteServerPayload);
}

export async function createCertificate(
	body: CertificateInput,
): Promise<CertificatePayload | undefined> {
	const response = await certificateClient.createCertificate({
		name: body.name ?? '',
		certificateData: body.certificate_data ?? '',
		privateKey: body.private_key ?? '',
		certificatePath: body.certificate_path ?? '',
		autoRenew: Boolean(body.auto_renew),
		serverId: body.server_id,
	});
	return response.certificate
		? toCertificatePayload(response.certificate)
		: undefined;
}

export async function updateCertificate(
	id: number,
	body: CertificateInput,
): Promise<CertificatePayload | undefined> {
	const response = await certificateClient.updateCertificate({
		id: BigInt(id),
		name: body.name,
		certificateData: body.certificate_data,
		privateKey: body.private_key,
		certificatePath: body.certificate_path,
		autoRenew:
			body.auto_renew === undefined ? undefined : Boolean(body.auto_renew),
		// Absent keeps the server; present-but-empty detaches it.
		serverId: body.server_id,
	});
	return response.certificate
		? toCertificatePayload(response.certificate)
		: undefined;
}

export async function deleteCertificate(id: number) {
	await certificateClient.deleteCertificate({id: BigInt(id)});
}

export async function listCertificateRenewals(
	id: number,
): Promise<CertificateRenewalPayload[]> {
	const response = await certificateClient.listRenewals({
		certificateId: BigInt(id),
	});
	return response.renewals.map(toRenewalPayload);
}

export async function renewCertificate(
	id: number,
	body: Record<string, unknown>,
): Promise<CertificatePayload | undefined> {
	const response = await certificateClient.renewCertificate({
		id: BigInt(id),
		certificateData: String(body.certificate_data ?? ''),
		privateKey: String(body.private_key ?? ''),
	});
	return response.certificate
		? toCertificatePayload(response.certificate)
		: undefined;
}
