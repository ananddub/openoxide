// @vitest-environment jsdom
import {act, renderHook, waitFor} from '@testing-library/react';
import {beforeEach, describe, expect, it, vi} from 'vitest';

const socketHarness = vi.hoisted(() => {
	const listeners = new Map<string, Set<(message?: any) => void>>();
	const socket = {
		connected: false,
		on: vi.fn((event: string, listener: (message?: any) => void) => {
			let eventListeners = listeners.get(event);
			if (!eventListeners) {
				eventListeners = new Set();
				listeners.set(event, eventListeners);
			}
			eventListeners.add(listener);
			return socket;
		}),
		off: vi.fn((event: string, listener: (message?: any) => void) => {
			listeners.get(event)?.delete(listener);
			return socket;
		}),
		emit: vi.fn(),
		connect: vi.fn(),
	};

	return {
		socket,
		serverEmit(event: string, message?: any) {
			for (const listener of listeners.get(event) ?? []) listener(message);
		},
		reset() {
			listeners.clear();
			socket.connected = false;
			socket.on.mockClear();
			socket.off.mockClear();
			socket.emit.mockClear();
			socket.connect.mockClear();
		},
	};
});

vi.mock('socket.io-client', () => ({io: () => socketHarness.socket}));

import {useAuthWhoAmI, useComposeGet} from 'virtual:openoxide-live';

describe('openoxide live HTTP-first hydration', () => {
	beforeEach(() => {
		localStorage.clear();
		localStorage.setItem(
			'openoxide-auth-session',
			JSON.stringify({tokens: {access_token: 'test-token'}}),
		);
		socketHarness.reset();
	});

	it('renders from one HTTP request without waiting for the socket', async () => {
		const composeId = BigInt(910_001);
		const fetchMock = vi.fn(
			async () =>
				new Response(
					JSON.stringify({id: Number(composeId), name: 'HTTP first'}),
					{
						status: 200,
						headers: {'content-type': 'application/json'},
					},
				),
		);
		vi.stubGlobal('fetch', fetchMock);

		const {result, unmount} = renderHook(() => [
			useComposeGet(composeId),
			useComposeGet(composeId),
		]);

		await waitFor(() => {
			expect(result.current[0].data?.name).toBe('HTTP first');
			expect(result.current[1].data?.name).toBe('HTTP first');
		});
		expect(fetchMock).toHaveBeenCalledTimes(1);
		expect(result.current[0].connected).toBe(false);

		act(() => {
			socketHarness.serverEmit('socket:ready');
			socketHarness.serverEmit('live:subscribed', {
				endpoint: 'ComposeController::get',
				args: [composeId],
			});
		});
		expect(fetchMock).toHaveBeenCalledTimes(1);

		act(() => {
			socketHarness.serverEmit('live:invalidate', {
				endpoint: 'ComposeController::get',
				args: [composeId],
			});
		});
		await waitFor(() => expect(fetchMock).toHaveBeenCalledTimes(2));

		act(() => {
			socketHarness.serverEmit('disconnect', 'transport close');
			socketHarness.serverEmit('socket:ready');
			socketHarness.serverEmit('live:subscribed', {
				endpoint: 'ComposeController::get',
				args: [composeId],
			});
		});
		await waitFor(() => expect(fetchMock).toHaveBeenCalledTimes(3));

		unmount();
		vi.unstubAllGlobals();
	});

	it('refetches the current profile after a whoami invalidation', async () => {
		const fetchMock = vi
			.fn()
			.mockResolvedValueOnce(
				new Response(
					JSON.stringify({
						user_id: 3,
						email: 'user@example.com',
						first_name: 'Old',
						last_name: 'Name',
						avatar: '#112233',
						role: 'OWNER',
						group_id: 1,
					}),
					{status: 200, headers: {'content-type': 'application/json'}},
				),
			)
			.mockResolvedValueOnce(
				new Response(
					JSON.stringify({
						user_id: 3,
						email: 'user@example.com',
						first_name: 'New',
						last_name: 'Name',
						avatar: '#445566',
						role: 'OWNER',
						group_id: 1,
					}),
					{status: 200, headers: {'content-type': 'application/json'}},
				),
			);
		vi.stubGlobal('fetch', fetchMock);

		const {result, unmount} = renderHook(() => useAuthWhoAmI());

		await waitFor(() =>
			expect(result.current.data?.first_name).toBe('Old'),
		);

		act(() => {
			socketHarness.serverEmit('live:invalidate', {
				endpoint: 'AuthController::who_am_i',
				args: [],
			});
		});

		await waitFor(() => {
			expect(result.current.data?.first_name).toBe('New');
			expect(result.current.data?.avatar).toBe('#445566');
		});
		expect(fetchMock).toHaveBeenCalledTimes(2);

		unmount();
		vi.unstubAllGlobals();
	});

	it('publishes a profile mutation result to every whoami consumer', async () => {
		const initialProfile = {
			user_id: 3,
			email: 'user@example.com',
			first_name: 'Old',
			last_name: 'Name',
			avatar: '#112233',
			role: 'OWNER',
			group_id: 1,
		};
		const fetchMock = vi.fn(
			async () =>
				new Response(JSON.stringify(initialProfile), {
					status: 200,
					headers: {'content-type': 'application/json'},
				}),
		);
		vi.stubGlobal('fetch', fetchMock);

		const {result, unmount} = renderHook(() => ({
			profile: useAuthWhoAmI(),
			sidebar: useAuthWhoAmI(),
		}));

		await waitFor(() => {
			expect(result.current.profile.data?.first_name).toBe('Old');
			expect(result.current.sidebar.data?.first_name).toBe('Old');
		});

		act(() => {
			result.current.profile.setData({
				...initialProfile,
				first_name: 'Instant',
				avatar: '#445566',
			});
		});

		await waitFor(() => {
			expect(result.current.profile.data?.first_name).toBe('Instant');
			expect(result.current.sidebar.data?.first_name).toBe('Instant');
			expect(result.current.sidebar.data?.avatar).toBe('#445566');
		});
		expect(fetchMock).toHaveBeenCalledTimes(1);

		unmount();
		vi.unstubAllGlobals();
	});
});
