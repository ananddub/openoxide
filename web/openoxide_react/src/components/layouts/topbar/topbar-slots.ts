import {useSyncExternalStore} from 'react';

/**
 * Page → top-bar breadcrumb registration.
 *
 * The slim top bar shows ONLY the breadcrumb trail, centered. Page
 * shells (Titlebar) publish their crumb trail here on mount; the TopBar
 * subscribes and renders it centered. Pages that register nothing fall
 * back to the plain route title.
 */

export type Crumb = {
	label: string;
	/** Route link. Omit for the current page (rendered as the active pill). */
	to?: string;
	params?: Record<string, string>;
	search?: Record<string, unknown>;
	icon?: React.ComponentType<{className?: string; strokeWidth?: number}>;
};

type Registry = Record<string, Crumb[]>;

let registry: Registry = {};
const listeners = new Set<() => void>();
let snapshot: Registry = registry;

function emit() {
	snapshot = {...registry};
	listeners.forEach(l => l());
}

/** Register/replace this page's crumb trail (call during render). */
export function setPageCrumbs(key: string, crumbs: Crumb[] | null) {
	if (crumbs === null) {
		if (registry[key] === undefined) return;
		const {[key]: _drop, ...rest} = registry;
		registry = rest;
	} else {
		registry = {...registry, [key]: crumbs};
	}
	emit();
}

function subscribe(listener: () => void) {
	listeners.add(listener);
	return () => listeners.delete(listener);
}

function getSnapshot() {
	return snapshot;
}

/** The first registered crumb trail, or null when no page registered one. */
export function usePageCrumbs(): Crumb[] | null {
	const reg = useSyncExternalStore(subscribe, getSnapshot, getSnapshot);
	const first = Object.values(reg)[0];
	return first ?? null;
}
