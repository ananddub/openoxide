import {type ClassValue, clsx} from 'clsx';
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
		return error;
	}
	if (typeof error === 'object' && error !== null) {
		const errObj = error as Record<string, unknown>;
		if (typeof errObj.error === 'string') {
			if (typeof errObj.details === 'object' && errObj.details !== null) {
				const details = Object.entries(errObj.details as Record<string, string[]>)
					.map(([field, errs]) => `${field}: ${Array.isArray(errs) ? errs.join(', ') : errs}`)
					.join('; ');
				return `${errObj.error} (${details})`;
			}
			return errObj.error;
		}
		if (typeof errObj.message === 'string') return errObj.message;
	}
	return 'An unexpected error occurred';
}
