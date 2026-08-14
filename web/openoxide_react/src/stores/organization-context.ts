export const ACTIVE_ORGANIZATION_STORAGE_KEY =
	'openoxide-active-organization-id';
export const ORGANIZATION_CHANGED_EVENT = 'openoxide:organization-changed';

export function getActiveOrganizationId(): number | null {
	const value = localStorage.getItem(ACTIVE_ORGANIZATION_STORAGE_KEY);
	if (!value) return null;
	const id = Number(value);
	return Number.isSafeInteger(id) && id > 0 ? id : null;
}

export function persistActiveOrganizationId(id: number | null) {
	if (id == null) {
		localStorage.removeItem(ACTIVE_ORGANIZATION_STORAGE_KEY);
	} else {
		localStorage.setItem(ACTIVE_ORGANIZATION_STORAGE_KEY, String(id));
	}
	window.dispatchEvent(
		new CustomEvent(ORGANIZATION_CHANGED_EVENT, {detail: id}),
	);
}
