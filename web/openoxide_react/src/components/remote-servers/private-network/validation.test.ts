import {describe, expect, it} from 'vitest';

import {validatePrivateNetworkForm} from './validation';

const managed = {
	mode: 'MANAGED_WIREGUARD' as const,
	managedHost: '10.77.2.2',
	endpoint: 'panel.example.com:51820',
	listenPort: '51820',
	privateHost: '',
};

describe('private network form validation', () => {
	it('accepts a complete managed WireGuard configuration', () => {
		expect(validatePrivateNetworkForm(managed)).toBeNull();
	});

	it('requires a panel endpoint', () => {
		expect(validatePrivateNetworkForm({...managed, endpoint: ''})).toContain('Panel public endpoint is required');
	});

	it('requires endpoint and listen ports to match', () => {
		expect(validatePrivateNetworkForm({...managed, endpoint: 'panel.example.com:51821'})).toContain(
			'must match',
		);
	});

	it('requires a reachable host for external providers', () => {
		expect(
			validatePrivateNetworkForm({...managed, mode: 'EXTERNAL_PRIVATE_NETWORK', endpoint: '', privateHost: ''}),
		).toContain('Private IP or hostname');
	});

	it('allows direct SSH without private-network fields', () => {
		expect(validatePrivateNetworkForm({...managed, mode: 'DIRECT_SSH', endpoint: '', managedHost: null})).toBeNull();
	});
});
