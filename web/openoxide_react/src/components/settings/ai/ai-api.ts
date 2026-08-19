import {getApiBaseUrl} from '#/api/client';

function apiUrl(path: string) {
	const base = getApiBaseUrl();
	return base === '/api' ? `/api${path}` : `${base}${path}`;
}

export async function aiRequest(path: string, init?: RequestInit) {
	const response = await fetch(apiUrl(path), {
		credentials: 'include',
		headers: {
			'Content-Type': 'application/json',
			...init?.headers,
		},
		...init,
	});

	if (!response.ok) {
		throw new Error(
			(await response.text()) || `Request failed (${response.status})`,
		);
	}

	return response.status === 204 ? null : response.json();
}
