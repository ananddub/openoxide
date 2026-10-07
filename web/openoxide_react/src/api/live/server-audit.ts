import {createServiceClient, useConnectMutation} from './query';
import {
	type ServerAudit,
	ServerControllerService,
	type ServerToolState,
} from '#/gen/connect/openoxide/api/v1/server_controller_pb';

const serverControllerClient = createServiceClient(
	ServerControllerService,
);

/** The tools the panel probes for on every server; `Audit` answers one state per key. */
export const AUDIT_TOOL_KEYS = [
	'docker',
	'git',
	'rclone',
	'rsync',
	'nixpacks',
	'railpack',
	'buildpacks',
] as const;

export type AuditToolKey = (typeof AUDIT_TOOL_KEYS)[number];

/** One tool probe: whether the binary answered and what version it reported. */
export type AuditToolPayload = {
	installed: boolean;
	version: string;
};

/** Result of one `ServerControllerService.Audit` run — an SSH round-trip, so it is only ever produced on demand. */
export type ServerAuditPayload = {
	osId: string;
	architecture: string;
	tools: Record<AuditToolKey, AuditToolPayload>;
	swarmActive: boolean;
	networkExists: boolean;
	baseDirectoryExists: boolean;
	dockerGroupMember: boolean;
	ports: {port: number; available: boolean}[];
};

function toAuditTool(tool: ServerToolState | undefined): AuditToolPayload {
	return {
		installed: tool?.installed ?? false,
		version: tool?.version ?? '',
	};
}

function toServerAuditPayload(audit: ServerAudit): ServerAuditPayload {
	return {
		osId: audit.osId,
		architecture: audit.architecture,
		tools: {
			docker: toAuditTool(audit.docker),
			git: toAuditTool(audit.git),
			rclone: toAuditTool(audit.rclone),
			rsync: toAuditTool(audit.rsync),
			nixpacks: toAuditTool(audit.nixpacks),
			railpack: toAuditTool(audit.railpack),
			buildpacks: toAuditTool(audit.buildpacks),
		},
		swarmActive: audit.swarmActive,
		networkExists: audit.networkExists,
		baseDirectoryExists: audit.baseDirectoryExists,
		dockerGroupMember: audit.dockerGroupMember,
		ports: audit.ports.map(port => ({
			port: port.port,
			available: port.available,
		})),
	};
}

/**
 * Audit a server's installed toolchain, swarm state and port availability. A
 * mutation, never a query: each call opens SSH and runs a batch of remote
 * checks, which is exactly what a $5 host should not be doing on a timer.
 */
export function useServerAudit() {
	return useConnectMutation<bigint, ServerAuditPayload>(async id => {
		const response = await serverControllerClient.audit({id});
		if (!response.audit) throw new Error('The audit returned nothing.');
		return toServerAuditPayload(response.audit);
	});
}
