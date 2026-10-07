import {describe, expect, it} from 'vitest';
import type {
	AuditToolKey,
	AuditToolPayload,
	ServerAuditPayload,
} from '#/api/live/server-audit';
import {
	auditCheckRows,
	auditIssues,
	auditToolRows,
	blockedAuditPorts,
} from './audit-metrics';

const MISSING: AuditToolPayload = {installed: false, version: ''};

function auditWith(
	tools: Partial<Record<AuditToolKey, AuditToolPayload>> = {},
	overrides: Partial<ServerAuditPayload> = {},
): ServerAuditPayload {
	/** A server that could deploy right now, minus the optional builders. */
	const healthy: ServerAuditPayload = {
		osId: 'ubuntu',
		architecture: 'x86_64',
		tools: {
			docker: {installed: true, version: '27.1.1'},
			git: {installed: true, version: '2.43.0'},
			rclone: MISSING,
			rsync: {installed: true, version: '3.2.7'},
			nixpacks: MISSING,
			railpack: MISSING,
			buildpacks: MISSING,
		},
		swarmActive: true,
		networkExists: true,
		baseDirectoryExists: true,
		dockerGroupMember: true,
		ports: [
			{port: 80, available: true},
			{port: 443, available: true},
			{port: 443, available: true},
		],
	};
	return {
		...healthy,
		...overrides,
		tools: {...healthy.tools, ...tools},
	};
}

describe('auditToolRows', () => {
	it('keeps every tool in probe order with its metadata', () => {
		const rows = auditToolRows(auditWith({}));
		expect(rows.map(row => row.key)).toEqual([
			'docker',
			'git',
			'rclone',
			'rsync',
			'nixpacks',
			'railpack',
			'buildpacks',
		]);
		expect(rows.map(row => row.required)).toEqual([
			true,
			true,
			false,
			false,
			false,
			false,
			false,
		]);
	});

	it('carries the reported version through', () => {
		const rows = auditToolRows(
			auditWith({
				nixpacks: {installed: true, version: '0.29.0'},
			}),
		);
		const nixpacks = rows.find(row => row.key === 'nixpacks');
		expect(nixpacks).toMatchObject({installed: true, version: '0.29.0'});
	});
});

describe('auditCheckRows', () => {
	it('describes each condition with the words an operator reads', () => {
		const rows = auditCheckRows(
			auditWith({}, {swarmActive: false, dockerGroupMember: false}),
		);
		expect(rows).toEqual([
			{
				key: 'swarm',
				label: 'Docker Swarm',
				detail: 'not active',
				ok: false,
			},
			{
				key: 'network',
				label: 'Overlay network',
				detail: 'present',
				ok: true,
			},
			{key: 'base', label: 'Base directory', detail: 'present', ok: true},
			{
				key: 'group',
				label: 'docker group membership',
				detail: 'not a member',
				ok: false,
			},
		]);
	});
});

describe('blockedAuditPorts', () => {
	it('reports only the ports something else is holding', () => {
		expect(blockedAuditPorts(auditWith({}))).toEqual([]);
		const held = auditWith({});
		held.ports = [
			{port: 80, available: false},
			{port: 443, available: true},
			{port: 443, available: false},
		];
		expect(blockedAuditPorts(held)).toEqual([80, 443]);
	});
});

describe('auditIssues', () => {
	it('is empty for a server that can deploy', () => {
		expect(auditIssues(auditWith({}))).toEqual([]);
	});

	it('names missing required tooling only', () => {
		const issues = auditIssues(
			auditWith({
				git: MISSING,
				rclone: MISSING,
			}),
		);
		expect(issues).toEqual(['Git is not installed']);
	});

	it('collects failed conditions and held ports in one list', () => {
		const audit = auditWith({docker: MISSING}, {networkExists: false});
		audit.ports = [
			{port: 80, available: false},
			{port: 443, available: true},
		];
		expect(auditIssues(audit)).toEqual([
			'Docker is not installed',
			'Overlay network: missing',
			'Port 80 already in use',
		]);
	});
});
