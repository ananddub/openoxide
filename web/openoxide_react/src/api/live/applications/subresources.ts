// ── Advanced sub-resources: ports / mounts / redirects / security ──
// Payload types live in ./types; the shared list/upsert/delete factories
// live in ./subresource-factories.

import {
	useActiveOrgId,
	useConnectMutation,
	useConnectQuery,
} from '../query';
import {applicationClient} from './shared';
import {subDelete, subList, subUpsert} from './subresource-factories';
import type {
	ApplicationMountPayload,
	ApplicationPortPayload,
	ApplicationRedirectPayload,
	ApplicationSecurityPayload,
} from './types';

/* ── Ports ─────────────────────────────────────────────────────────── */

export const useApplicationPortList = subList<ApplicationPortPayload>(
	'ports',
	(id, orgId) => applicationClient.listPorts({id, organizationId: orgId}),
);

export function useApplicationPortCreate() {
	return useConnectMutation(
		(input: {
			application_id: number | bigint;
			published_port: number;
			target_port: number;
			protocol: string;
			publish_mode: string;
		}) =>
			applicationClient.createPort({
				id: BigInt(input.application_id),
				payload: JSON.stringify({
					published_port: input.published_port,
					target_port: input.target_port,
					protocol: input.protocol,
					publish_mode: input.publish_mode,
				}),
			}),
		{invalidate: [['applications', 'ports']]},
	);
}

export const useApplicationPortDelete = subDelete('ports', (id, orgId) =>
	applicationClient.deletePort({id, organizationId: orgId}),
);

/* ── Mounts ────────────────────────────────────────────────────────── */

export const useApplicationMountList = subList<ApplicationMountPayload>(
	'mounts',
	(id, orgId) => applicationClient.listMounts({id, organizationId: orgId}),
);

export const useApplicationMountUpsert = subUpsert<{
	application_id: number | bigint;
	id?: number | bigint;
	mount_type: string;
	host_path?: string | null;
	volume_name?: string | null;
	file_path?: string | null;
	content?: string | null;
	mount_path: string;
}>(
	'mounts',
	input => ({
		mount_type: input.mount_type,
		host_path: input.host_path || null,
		volume_name: input.volume_name || null,
		file_path: input.file_path || null,
		content: input.content || null,
		mount_path: input.mount_path,
	}),
	(id, payload) => applicationClient.createMount({id, payload}),
	(id, applicationId, payload) =>
		applicationClient.updateMount({
			id,
			organizationId: applicationId,
			payload,
		}),
);

export const useApplicationMountDelete = subDelete('mounts', (id, orgId) =>
	applicationClient.deleteMount({id, organizationId: orgId}),
);

/* ── Redirects ─────────────────────────────────────────────────────── */

export const useApplicationRedirectList =
	subList<ApplicationRedirectPayload>('redirects', (id, orgId) =>
		applicationClient.listRedirects({id, organizationId: orgId}),
	);

export const useApplicationRedirectUpsert = subUpsert<{
	application_id: number | bigint;
	id?: number | bigint;
	regex: string;
	replacement: string;
	permanent: boolean;
}>(
	'redirects',
	input => ({
		regex: input.regex,
		replacement: input.replacement,
		permanent: input.permanent ? 1 : 0,
	}),
	(id, payload) => applicationClient.createRedirect({id, payload}),
	(id, applicationId, payload) =>
		applicationClient.updateRedirect({
			id,
			organizationId: applicationId,
			payload,
		}),
);

export const useApplicationRedirectDelete = subDelete(
	'redirects',
	(id, orgId) =>
		applicationClient.deleteRedirect({id, organizationId: orgId}),
);

/* ── Security (basic auth) ─────────────────────────────────────────── */

/** Security rules arrive as one JSON payload, not resource rows. */
export function useApplicationSecurityList(id: number | bigint) {
	const orgId = useActiveOrgId();
	return useConnectQuery<ApplicationSecurityPayload[]>(
		['applications', 'security', id.toString()],
		async () => {
			const response = await applicationClient.getSecurity({
				id: BigInt(id),
				organizationId: orgId,
			});
			try {
				return JSON.parse(response.payload || '[]');
			} catch {
				return [];
			}
		},
	);
}

export const useApplicationSecurityUpsert = subUpsert<{
	application_id: number | bigint;
	id?: number | bigint;
	username: string;
	password: string;
}>(
	'security',
	input => ({username: input.username, password: input.password}),
	(id, payload) => applicationClient.createSecurity({id, payload}),
	(id, applicationId, payload) =>
		applicationClient.updateSecurity({
			id,
			organizationId: applicationId,
			payload,
		}),
);

export const useApplicationSecurityDelete = subDelete(
	'security',
	(id, orgId) =>
		applicationClient.deleteSecurity({id, organizationId: orgId}),
);
