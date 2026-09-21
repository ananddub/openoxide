import type {TraefikLogEntry} from './requests-table';

/** Whether a log entry matches the selected HTTP status class filter. */
export function matchesStatusFilter(
	item: TraefikLogEntry,
	statusFilter: string,
): boolean {
	if (statusFilter === 'all') return true;
	const code = Number(item.status);
	if (statusFilter === '1xx') return code >= 100 && code < 200;
	if (statusFilter === '2xx') return code >= 200 && code < 300;
	if (statusFilter === '3xx') return code >= 300 && code < 400;
	if (statusFilter === '4xx') return code >= 400 && code < 500;
	if (statusFilter === '5xx') return code >= 500 && code < 600;
	return true;
}
