import {createContext, useContext, useRef, type ReactNode} from 'react';

/**
 * Resource-scoped cache for live container metrics.
 *
 * A detail page wraps its tabs in `MonitoringCacheProvider`; the cache lives
 * as long as that page is mounted, so switching tabs (General ↔ Architecture ↔
 * Deployments) keeps the last metrics visible instead of blanking. When the
 * user leaves the resource (project list, registry, another service), the
 * provider unmounts and the cache is dropped — nothing lingers globally.
 */
type Cache = Map<string, unknown>;

const MonitoringCacheContext = createContext<Cache | null>(null);

export function MonitoringCacheProvider({children}: {children: ReactNode}) {
	const cacheRef = useRef<Cache>(new Map());
	return (
		<MonitoringCacheContext.Provider value={cacheRef.current}>
			{children}
		</MonitoringCacheContext.Provider>
	);
}

/** The current resource's metrics cache, or `null` outside a provider
 *  (callers then run without caching). */
export function useMonitoringCache(): Cache | null {
	return useContext(MonitoringCacheContext);
}
