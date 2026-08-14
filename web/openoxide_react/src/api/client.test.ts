// @vitest-environment jsdom
import {beforeEach, describe, expect, it, vi} from 'vitest';

import {applyOrganizationHeader} from './client';
import {persistActiveOrganizationId} from '#/stores/organization-context';

describe('organization-scoped API client', () => {
	beforeEach(() => {
		localStorage.clear();
		vi.restoreAllMocks();
	});

	it('sends the active organization on resource requests', () => {
		persistActiveOrganizationId(3);
		const request = new Request('http://localhost/api/applications/3');

		applyOrganizationHeader(request);

		expect(request.headers.get('X-Organization-Id')).toBe('3');
	});

	it('does not scope the organization bootstrap list', () => {
		persistActiveOrganizationId(3);
		const request = new Request('http://localhost/api/organizations');

		applyOrganizationHeader(request);

		expect(request.headers.get('X-Organization-Id')).toBeNull();
	});
});
