import {type ClassValue, clsx} from 'clsx';
import {twMerge} from 'tailwind-merge';

export function cn(...inputs: ClassValue[]) {
	return twMerge(clsx(inputs));
}

export function formatApiError(error: any): string {
	if (!error) return 'An unexpected error occurred';
	if (typeof error === 'string') {
		// Clean up common database errors for friendly user presentation
		if (error.includes('UNIQUE constraint failed: projects.name')) {
			return 'A project with this name already exists.';
		}
		if (error.includes('UNIQUE constraint failed: organization.name')) {
			return 'An organization with this name already exists.';
		}
		return error;
	}
	if (typeof error === 'object') {
		if (error.error) {
			if (error.details) {
				const details = Object.entries(error.details)
					.map(([field, errs]) => `${field}: ${(errs as string[]).join(', ')}`)
					.join('; ');
				return `${error.error} (${details})`;
			}
			return error.error;
		}
		if (error.message) return error.message;
	}
	return 'An unexpected error occurred';
}
