// ── Advanced sub-resources: ports / mounts / redirects / security ── Payload types and the proto→payload mappers live in ./types; requests carry the entity fields directly (applicationId + entity fields).

import {useConnectMutation, useConnectQuery} from '../query';
import {
	ApplicationMountType,
	ApplicationPortProtocol,
	ApplicationPortPublishMode,
} from '#/gen/connect/openoxide/api/v1/application_pb';
import {applicationClient} from './shared';
import {
	toApplicationMountPayload,
	toApplicationPortPayload,
	toApplicationRedirectPayload,
	toApplicationSecurityPayload,
	type ApplicationMountPayload,
	type ApplicationPortPayload,
	type ApplicationRedirectPayload,
	type ApplicationSecurityPayload,
} from './types';

// The cards keep passing uppercase enum names ('TCP', 'HOST', 'VOLUME');
// map them onto the generated enum members for the typed requests.
const protocolOf = (name: string): ApplicationPortProtocol =>
	name === ApplicationPortProtocol[ApplicationPortProtocol.UDP]
		? ApplicationPortProtocol.UDP
		: ApplicationPortProtocol.TCP;

const publishModeOf = (name: string): ApplicationPortPublishMode =>
	name === ApplicationPortPublishMode[ApplicationPortPublishMode.INGRESS]
		? ApplicationPortPublishMode.INGRESS
		: ApplicationPortPublishMode.HOST;

const mountTypeOf = (name: string): ApplicationMountType => {
	if (name === ApplicationMountType[ApplicationMountType.BIND]) {
		return ApplicationMountType.BIND;
	}
	if (name === ApplicationMountType[ApplicationMountType.FILE]) {
		return ApplicationMountType.FILE;
	}
	return ApplicationMountType.VOLUME;
};

/** Delete hook taking `{application_id, id}` with scoped invalidation. */
const subDelete = (
	scope: string,
	call: (applicationId: bigint, id: bigint) => Promise<unknown>,
) =>
	function useSubDelete() {
		return useConnectMutation(
			(input: {application_id: number | bigint; id: number | bigint}) =>
				call(BigInt(input.application_id), BigInt(input.id)),
			{invalidate: [['applications', scope]]},
		);
	};

/* ── Ports ─────────────────────────────────────────────────────────── */

export function useApplicationPortList(id: number | bigint) {
	return useConnectQuery<ApplicationPortPayload[]>(
		['applications', 'ports', id.toString()],
		async () => {
			const response = await applicationClient.listPorts({
				applicationId: BigInt(id),
			});
			return response.ports.map(toApplicationPortPayload);
		},
	);
}

export function useApplicationPortCreate() {
	return useConnectMutation(
		async (input: {
			application_id: number | bigint;
			published_port: number;
			target_port: number;
			protocol: string;
			publish_mode: string;
		}) =>
			applicationClient.createPort({
				applicationId: BigInt(input.application_id),
				publishedPort: BigInt(input.published_port),
				targetPort: BigInt(input.target_port),
				protocol: protocolOf(input.protocol),
				publishMode: publishModeOf(input.publish_mode),
			}),
		{invalidate: [['applications', 'ports']]},
	);
}

export const useApplicationPortDelete = subDelete(
	'ports',
	(applicationId, id) => applicationClient.deletePort({applicationId, id}),
);

/* ── Mounts ────────────────────────────────────────────────────────── */

export function useApplicationMountList(id: number | bigint) {
	return useConnectQuery<ApplicationMountPayload[]>(
		['applications', 'mounts', id.toString()],
		async () => {
			const response = await applicationClient.listMounts({
				applicationId: BigInt(id),
			});
			return response.mounts.map(toApplicationMountPayload);
		},
	);
}

export function useApplicationMountUpsert() {
	return useConnectMutation(
		async (input: {
			application_id: number | bigint;
			id?: number | bigint;
			mount_type: string;
			host_path?: string | null;
			volume_name?: string | null;
			file_path?: string | null;
			content?: string | null;
			mount_path: string;
		}) => {
			const fields = {
				mountType: mountTypeOf(input.mount_type),
				hostPath: input.host_path || undefined,
				volumeName: input.volume_name || undefined,
				filePath: input.file_path || undefined,
				content: input.content || undefined,
				mountPath: input.mount_path,
			};
			return input.id === undefined
				? applicationClient.createMount({
						applicationId: BigInt(input.application_id),
						...fields,
					})
				: applicationClient.updateMount({
						applicationId: BigInt(input.application_id),
						id: BigInt(input.id),
						...fields,
					});
		},
		{invalidate: [['applications', 'mounts']]},
	);
}

export const useApplicationMountDelete = subDelete(
	'mounts',
	(applicationId, id) =>
		applicationClient.deleteMount({applicationId, id}),
);

/* ── Redirects ─────────────────────────────────────────────────────── */

export function useApplicationRedirectList(id: number | bigint) {
	return useConnectQuery<ApplicationRedirectPayload[]>(
		['applications', 'redirects', id.toString()],
		async () => {
			const response = await applicationClient.listRedirects({
				applicationId: BigInt(id),
			});
			return response.redirects.map(toApplicationRedirectPayload);
		},
	);
}

export function useApplicationRedirectUpsert() {
	return useConnectMutation(
		async (input: {
			application_id: number | bigint;
			id?: number | bigint;
			regex: string;
			replacement: string;
			permanent: boolean;
		}) => {
			const fields = {
				regex: input.regex,
				replacement: input.replacement,
				permanent: input.permanent,
			};
			return input.id === undefined
				? applicationClient.createRedirect({
						applicationId: BigInt(input.application_id),
						...fields,
					})
				: applicationClient.updateRedirect({
						applicationId: BigInt(input.application_id),
						id: BigInt(input.id),
						...fields,
					});
		},
		{invalidate: [['applications', 'redirects']]},
	);
}

export const useApplicationRedirectDelete = subDelete(
	'redirects',
	(applicationId, id) =>
		applicationClient.deleteRedirect({applicationId, id}),
);

/* ── Security (basic auth) ─────────────────────────────────────────── */

/** `getSecurity` returns the application's whole rule set. */
export function useApplicationSecurityList(id: number | bigint) {
	return useConnectQuery<ApplicationSecurityPayload[]>(
		['applications', 'security', id.toString()],
		async () => {
			const response = await applicationClient.getSecurity({
				applicationId: BigInt(id),
			});
			return response.security.map(toApplicationSecurityPayload);
		},
	);
}

export function useApplicationSecurityUpsert() {
	return useConnectMutation(
		async (input: {
			application_id: number | bigint;
			id?: number | bigint;
			username: string;
			password: string;
		}) => {
			const fields = {username: input.username, password: input.password};
			return input.id === undefined
				? applicationClient.createSecurity({
						applicationId: BigInt(input.application_id),
						...fields,
					})
				: applicationClient.updateSecurity({
						applicationId: BigInt(input.application_id),
						id: BigInt(input.id),
						...fields,
					});
		},
		{invalidate: [['applications', 'security']]},
	);
}

export const useApplicationSecurityDelete = subDelete(
	'security',
	(applicationId, id) =>
		applicationClient.deleteSecurity({applicationId, id}),
);
