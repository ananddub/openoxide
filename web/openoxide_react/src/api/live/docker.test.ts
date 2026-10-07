import {describe, expect, it} from 'vitest';

import type {
	GlobalContainer,
	GlobalImage,
	GlobalVolume,
} from '#/gen/connect/openoxide/api/v1/global_pb';

import {
	mapContainer,
	mapDiskUsage,
	mapImage,
	mapVolume,
	shortId,
} from '#/api/live/docker';

const container = {
	id: 'f8e2c1a09b7d4e5f6a1b2c3d4e5f60718293a4b5c6d7e8f9',
	names: '/openoxide-agent',
	image: 'openoxide/agent:latest',
	command: '"/app/agent"',
	created_at: '2 hours ago',
	running_for: 'Up 2 hours',
	ports: '8080/tcp',
	status: 'Up 2 hours',
	state: 'running',
	size: '12B',
	labels: '',
	mounts: '/var/run/docker.sock->/var/run/docker.sock:rw',
	networks: 'openoxide_net',
} as unknown as GlobalContainer;

describe('server docker mappers', () => {
	it('trims a 64-hex container id to the CLI prefix and drops the name slash', () => {
		const mapped = mapContainer(container);
		expect(mapped.id).toBe('f8e2c1a09b7d');
		expect(mapped.name).toBe('openoxide-agent');
		expect(mapped.running).toBe(true);
	});

	it('keeps a short name-like id untouched and treats a stopped state as not running', () => {
		const stopped = mapContainer({
			...container,
			id: 'my-container',
			state: 'exited',
		});
		expect(stopped.id).toBe('my-container');
		expect(stopped.running).toBe(false);
		expect(shortId('abc')).toBe('abc');
	});

	it('builds the image reference from repository and tag', () => {
		const mapped = mapImage({
			id: 'sha256:0123456789abcdef0123456789abcdef',
			repository: 'postgres',
			tag: '16-alpine',
			digest: '',
			created_at: '',
			created_since: '3 weeks ago',
			size: '286MB',
			shared_size: '-',
			unique_size: '286MB',
			containers: '0',
		} as unknown as GlobalImage);
		expect(mapped.reference).toBe('postgres:16-alpine');
	});

	it('falls back to the short id when an image has no repository', () => {
		const dangling = mapImage({
			id: 'deadbeefcafebabe0000111122223333',
			repository: '',
			tag: '',
		} as unknown as GlobalImage);
		expect(dangling.reference).toBe('deadbeefcafe');
	});

	it('keeps a volume keyed by name with its mountpoint', () => {
		const mapped = mapVolume({
			name: 'agent_data',
			driver: 'local',
			scope: 'local',
			mountpoint: '/var/lib/docker/volumes/agent_data/_data',
			labels: '',
			availability: 'available',
			group: '',
			links: '',
			size: '4.2GB',
		} as unknown as GlobalVolume);
		expect(mapped.name).toBe('agent_data');
		expect(mapped.size).toBe('4.2GB');
	});
});

describe('server docker disk usage parsing', () => {
	it('reads the `{{json .}}` rows the CLI emits', () => {
		const rows = mapDiskUsage(
			JSON.stringify({
				items: [
					{
						Type: 'Images',
						'Total count': 3,
						Active: '3',
						Size: '1.2GB',
						Reclaimable: '0B',
					},
					{
						Type: 'Build Storage',
						'Total count': 12,
						Active: '3',
						Size: '1.1GB',
						Reclaimable: '1.1GB',
					},
				],
			}),
		);
		expect(rows).toHaveLength(2);
		expect(rows[0].total).toBe('3');
		expect(rows[1].reclaimable).toBe('1.1GB');
	});

	it('reads snake_case rows and reports nothing reclaimable as a dash', () => {
		const rows = mapDiskUsage(
			JSON.stringify({items: [{type: 'Containers', total_count: 1}]}),
		);
		expect(rows[0].type).toBe('Containers');
		expect(rows[0].total).toBe('1');
		expect(rows[0].size).toBe('—');
	});

	it('returns no rows for malformed or unrelated payloads', () => {
		expect(mapDiskUsage('')).toEqual([]);
		expect(mapDiskUsage('{oops')).toEqual([]);
		expect(mapDiskUsage('{"items": {"Type": "Images"}}')).toEqual([]);
	});
});
