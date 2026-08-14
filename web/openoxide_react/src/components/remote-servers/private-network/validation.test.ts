import {describe, expect, it} from 'vitest';

import {validatePrivateNetworkForm} from './validation';

const managed = {
	mode: 'MANAGED_WIREGUARD' as const,
	managedHost: '10.77.2.2',
	endpoint: '203.0.113.10:51820',
	listenPort: '51820',
	privateHost: '',
};

describe('private network form validation', () => {
	it('accepts a complete managed WireGuard configuration', () => {
		expect(validatePrivateNetworkForm(managed)).toBeNull();
	});

	it('requires a panel endpoint', () => {
		expect(
			validatePrivateNetworkForm({...managed, endpoint: ''}),
		).toContain('Remote WireGuard endpoint is required');
	});

	it('requires the endpoint port to match the remote listen port', () => {
		expect(
			validatePrivateNetworkForm({
				...managed,
				endpoint: '203.0.113.10:52180',
				listenPort: '51820',
			}),
		).toContain('must match');
	});

	it('rejects invalid public and local ports', () => {
		expect(
			validatePrivateNetworkForm({
				...managed,
				endpoint: '203.0.113.10:70000',
			}),
		).toContain('valid UDP port');
		expect(
			validatePrivateNetworkForm({...managed, listenPort: '0'}),
		).toContain('between 1 and 65535');
	});

	it('rejects placeholder endpoints', () => {
		expect(
			validatePrivateNetworkForm({
				...managed,
				endpoint: 'panel.example.com:51820',
			}),
		).toContain('placeholder');
	});

	it('requires a reachable host for external providers', () => {
		expect(
			validatePrivateNetworkForm({
				...managed,
				mode: 'EXTERNAL_PRIVATE_NETWORK',
				endpoint: '',
				privateHost: '',
			}),
		).toContain('Private IP or hostname');
	});

	it('allows direct SSH without private-network fields', () => {
		expect(
			validatePrivateNetworkForm({
				...managed,
				mode: 'DIRECT_SSH',
				endpoint: '',
				managedHost: null,
			}),
		).toBeNull();
	});
});
