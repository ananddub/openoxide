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
		expect(
			validatePrivateNetworkForm({...managed, endpoint: ''}),
		).toContain('Panel public endpoint is required');
	});

	it('allows a NAT-mapped public port to differ from the local listen port', () => {
		expect(
			validatePrivateNetworkForm({
				...managed,
				endpoint: 'panel.example.com:52180',
				listenPort: '51820',
			}),
		).toBeNull();
	});

	it('rejects invalid public and local ports', () => {
		expect(
			validatePrivateNetworkForm({
				...managed,
				endpoint: 'panel.example.com:70000',
			}),
		).toContain('valid UDP port');
		expect(
			validatePrivateNetworkForm({...managed, listenPort: '0'}),
		).toContain('between 1 and 65535');
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
