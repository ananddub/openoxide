import {describe, expect, it} from 'vitest';
import {
	canonicalizeLiveValue,
	liveArgsKey,
	matchesLiveInvalidation,
} from '@openoxide/vite/live-key';

const deploymentQuery = {
	status: null,
	state: null,
	application_id: null,
	compose_id: 2n,
	database_id: null,
	server_id: null,
	limit: 20n,
	offset: null,
};

describe('live endpoint argument matching', () => {
	it('canonicalizes safe bigint values to JSON numbers', () => {
		expect(canonicalizeLiveValue([2n, Number.MAX_SAFE_INTEGER])).toEqual([
			2,
			Number.MAX_SAFE_INTEGER,
		]);
	});

	it('keeps unsafe bigint values lossless', () => {
		const value = BigInt(Number.MAX_SAFE_INTEGER) + 1n;
		expect(canonicalizeLiveValue(value)).toBe(value.toString());
	});

	it('matches equivalent query objects regardless of key order', () => {
		const serverArgs = [
			{
				application_id: null,
				compose_id: 2,
				database_id: null,
				limit: 20,
				offset: null,
				server_id: null,
				state: null,
				status: null,
			},
		];
		expect(liveArgsKey([deploymentQuery])).toBe(liveArgsKey(serverArgs));
	});

	it('matches endpoint-wide invalidation for every query variant', () => {
		const message = {endpoint: 'DeploymentController::list', args: null};
		expect(
			matchesLiveInvalidation(
				'DeploymentController::list',
				[deploymentQuery],
				message,
			),
		).toBe(true);
		expect(
			matchesLiveInvalidation(
				'DeploymentController::list',
				[{...deploymentQuery, limit: null}],
				message,
			),
		).toBe(true);
	});

	it('keeps exact invalidations scoped to the matching variant', () => {
		const message = {
			endpoint: 'DeploymentController::list',
			args: [deploymentQuery],
		};
		expect(
			matchesLiveInvalidation(
				'DeploymentController::list',
				[deploymentQuery],
				message,
			),
		).toBe(true);
		expect(
			matchesLiveInvalidation(
				'DeploymentController::list',
				[{...deploymentQuery, limit: null}],
				message,
			),
		).toBe(false);
		expect(
			matchesLiveInvalidation(
				'ComposeController::get',
				[deploymentQuery],
				message,
			),
		).toBe(false);
	});
});
