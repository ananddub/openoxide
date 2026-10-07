import {
	AUDIT_TOOL_KEYS,
	type AuditToolKey,
	type ServerAuditPayload,
} from '#/api/live/server-audit';

/** What each audited binary does on this host, and whether a deployment needs it. */
const TOOL_META: Record<
	AuditToolKey,
	{hint: string; label: string; required: boolean}
> = {
	docker: {
		label: 'Docker',
		hint: 'Runs every service on this host.',
		required: true,
	},
	git: {
		label: 'Git',
		hint: 'Clones and updates your repositories.',
		required: true,
	},
	rclone: {
		label: 'Rclone',
		hint: 'Needed by backup and bucket jobs.',
		required: false,
	},
	rsync: {
		label: 'Rsync',
		hint: 'Needed by file transfer jobs.',
		required: false,
	},
	nixpacks: {
		label: 'Nixpacks',
		hint: 'Builder for zero-config apps.',
		required: false,
	},
	railpack: {
		label: 'Railpack',
		hint: 'Alternative application builder.',
		required: false,
	},
	buildpacks: {
		label: 'Buildpacks',
		hint: 'The `pack` CLI behind the Buildpack builder.',
		required: false,
	},
};

export type AuditToolRow = {
	key: AuditToolKey;
	label: string;
	hint: string;
	required: boolean;
	installed: boolean;
	version: string;
};

/** Tool rows in the order the backend probes them. */
export function auditToolRows(audit: ServerAuditPayload): AuditToolRow[] {
	return AUDIT_TOOL_KEYS.map(key => ({
		key,
		...TOOL_META[key],
		...audit.tools[key],
	}));
}

export type AuditCheckRow = {
	key: string;
	label: string;
	detail: string;
	ok: boolean;
};

/** The four engine and filesystem conditions the panel reports next to the tools. */
export function auditCheckRows(
	audit: ServerAuditPayload,
): AuditCheckRow[] {
	return [
		{
			key: 'swarm',
			label: 'Docker Swarm',
			ok: audit.swarmActive,
			detail: audit.swarmActive ? 'active' : 'not active',
		},
		{
			key: 'network',
			label: 'Overlay network',
			ok: audit.networkExists,
			detail: audit.networkExists ? 'present' : 'missing',
		},
		{
			key: 'base',
			label: 'Base directory',
			ok: audit.baseDirectoryExists,
			detail: audit.baseDirectoryExists ? 'present' : 'missing',
		},
		{
			key: 'group',
			label: 'docker group membership',
			ok: audit.dockerGroupMember,
			detail: audit.dockerGroupMember ? 'member' : 'not a member',
		},
	];
}

/** Entrypoint ports the proxy needs that something else already holds. */
export function blockedAuditPorts(audit: ServerAuditPayload): number[] {
	return audit.ports
		.filter(port => !port.available)
		.map(port => port.port);
}

/** Every condition that would stop a deployment, ready to show as one list. */
export function auditIssues(audit: ServerAuditPayload): string[] {
	const issues = auditToolRows(audit)
		.filter(row => row.required && !row.installed)
		.map(row => `${row.label} is not installed`);

	auditCheckRows(audit).forEach(row => {
		if (!row.ok) issues.push(`${row.label}: ${row.detail}`);
	});

	const blocked = blockedAuditPorts(audit);
	if (blocked.length > 0) {
		issues.push(`Port ${blocked.join(', ')} already in use`);
	}
	return issues;
}
