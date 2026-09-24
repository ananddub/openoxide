import {type ClassValue} from 'clsx';
import clsx from 'clsx';
import {twMerge} from 'tailwind-merge';

export function cn(...inputs: ClassValue[]) {
	return twMerge(clsx(inputs));
}

export function formatApiError(error: unknown): string {
	if (!error) return 'An unexpected error occurred';
	if (typeof error === 'string') {
		if (error.includes('UNIQUE constraint failed: projects.name')) {
			return 'A project with this name already exists.';
		}
		if (error.includes('UNIQUE constraint failed: organization.name')) {
			return 'An organization with this name already exists.';
		}
		if (
			error.toLowerCase().includes('invalid json') ||
			error.toLowerCase().includes('unexpected end')
		) {
			return 'Server returned an invalid or empty response.';
		}
		return error;
	}
	if (typeof error === 'object' && error !== null) {
		const errObj = error as Record<string, unknown>;
		if (typeof errObj.error === 'string') {
			if (typeof errObj.details === 'object' && errObj.details !== null) {
				const details = Object.entries(
					errObj.details as Record<string, string[]>,
				)
					.map(
						([field, errs]) =>
							`${field}: ${Array.isArray(errs) ? errs.join(', ') : errs}`,
					)
					.join('; ');
				return `${errObj.error} (${details})`;
			}
			return errObj.error;
		}
		if (typeof errObj.message === 'string') {
			if (
				errObj.message.toLowerCase().includes('invalid json') ||
				errObj.message.toLowerCase().includes('unexpected end')
			) {
				return 'Server returned an invalid or empty response.';
			}
			return errObj.message;
		}
	}
	return 'An unexpected error occurred';
}

/** The panel host's server row stores the agent's container DNS name as its `ip_address` (that is how the backend resolves the local gRPC endpoint — see MonitoringService::client). Users should see it as `localhost`, not a Docker service name. Remote rows already hold a real address. */
export function serverHostLabel(ipAddress?: string | null): string {
	const ip = String(ipAddress || '').trim();
	if (ip === 'openoxide-agent' || ip === 'openoxide-monitor')
		return 'localhost';
	return ip || '-';
}

/** True for the panel host's own server row (addressed through the local
 *  Agent, not SSH). */
export function isLocalServerRow(ipAddress?: string | null): boolean {
	const ip = String(ipAddress || '').trim();
	return (
		ip === 'openoxide-agent' ||
		ip === 'openoxide-monitor' ||
		ip === 'localhost' ||
		ip === '127.0.0.1'
	);
}
