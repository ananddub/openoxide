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

	it('accepts an empty endpoint because the panel fills in the remote IP and listen port', () => {
		expect(
			validatePrivateNetworkForm({...managed, endpoint: ''}),
		).toBeNull();
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
		).toContain('endpoint port must be between 1 and 65535');
		expect(
			validatePrivateNetworkForm({...managed, listenPort: '0'}),
		).toContain('listen port must be between 1 and 65535');
	});

	it('accepts the documentation placeholder endpoint', () => {
		expect(
			validatePrivateNetworkForm({
				...managed,
				endpoint: 'panel.example.com:51820',
			}),
		).toBeNull();
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
