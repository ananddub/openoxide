import {useEffect} from 'react';
import {useAppStore} from '#/stores/app-store';
import {useOrganizationStore} from '#/stores/organization-store';
import {
	usePermissionGroupMembers,
	usePermissionGroupInvites,
	useAuthWhoAmI,
	useVaultList,
	useDnsList,
	useCertificateList,
	useAiListSettings,
	useProjectListByOrganization,
	useRemoteServerList,
	useDeploymentList,
	useScheduleListByOrganization,
	useSshKeyList,
	useDestinationList,
	useTagListAll,
	useOverviewServices,
	useOverviewDomains,
	useOverviewBackups,
} from '#/api/live';

/**
 * Root sync between live ConnectRPC queries and the Zustand app store.
 *
 * Data flow: queries fetch on mount (60s cache), the table-watch stream
 * (useTableWatch) invalidates them on DB changes, and this hook mirrors
 * the refreshed query data into the legacy Zustand stores that pages
 * still consume. Zustand setters are stable references, so each sync
 * effect only runs when its query data actually changes.
 */

/** Passes a live query's list payload straight into a store setter. */
function useSyncList<T>(data: T[] | undefined, set: (rows: T[]) => void) {
	useEffect(() => {
		if (Array.isArray(data)) set(data);
	}, [data, set]);
}

/**
 * Same as useSyncList, but the payload is mapped through `transform`
 * first. Transforms live at module level so the effect deps stay stable
 * and the effect only re-runs on actual data changes.
 */
function useSyncListAs<T, R>(
	data: T[] | undefined,
	set: (rows: R[]) => void,
	transform: (rows: T[]) => R[],
) {
	useEffect(() => {
		if (Array.isArray(data)) set(transform(data));
	}, [data, set, transform]);
}

type LiveRow = Record<string, any>;

const toProfile = (profile: LiveRow) => ({
	id: profile.user_id || profile.id,
	email: profile.email,
	name: `${profile.first_name || ''} ${profile.last_name || ''}`.trim(),
	avatar: profile.avatar,
});

const toAiSettings = (rows: LiveRow[]): any =>
	rows.map((item) => ({
		...item,
		id: Number(item.id),
		organization_id: Number(item.organization_id),
		created_at: Number(item.created_at),
		updated_at: Number(item.updated_at),
	}));

const toCertificates = (rows: LiveRow[]): any =>
	rows.map((item) => ({
		...item,
		id: String(item.id),
		auto_renew: Number(item.auto_renew),
		has_private_key: item.has_private_key ?? Boolean(item.private_key),
		organization_id: Number(item.organization_id),
		created_at: Number(item.created_at),
		updated_at: Number(item.updated_at),
	}));

const toDomains = (rows: LiveRow[]) =>
	rows.map((d) => ({
		id: Number(d.id),
		domain: d.host,
		host: d.host,
		path: d.path || '/',
		internal_path: d.internal_path || '/',
		port: d.port || 80,
		https: !!d.https,
		strip_path: !!d.strip_path,
		certificate_type: String(d.certificate_type || 'NONE').toUpperCase(),
		custom_cert_resolver: d.custom_cert_resolver || undefined,
		custom_entrypoint: d.custom_entrypoint || undefined,
		middlewares: d.middlewares || '[]',
		domain_type: d.domain_type,
		application_id: d.application_id ? Number(d.application_id) : undefined,
		compose_id: d.compose_id ? Number(d.compose_id) : undefined,
		service_name: d.service_name,
		project_name: d.project_name,
		created_at: Number(d.created_at || 0),
		updated_at: Number(d.updated_at || 0),
	}));

const toBackups = (rows: LiveRow[]) =>
	rows.map((b) => ({
		id: Number(b.id),
		name: b.name,
		backup_type: b.backup_type,
		status: b.status,
		destination: b.destination,
		created_at: Number(b.created_at),
	}));

export function useRealtimeSync() {
	const activeOrg = useOrganizationStore((state) => state.activeOrg);
	const orgId = activeOrg?.id || 1;

	// Zustand setters
	const setVaultProviders = useAppStore((state) => state.setVaultProviders);
	const setDnsProviders = useAppStore((state) => state.setDnsProviders);
	const setAiSettings = useAppStore((state) => state.setAiSettings);
	const setCertificates = useAppStore((state) => state.setCertificates);
	const setProjects = useAppStore((state) => state.setProjects);
	const setServers = useAppStore((state) => state.setServers);
	const setDeployments = useAppStore((state) => state.setDeployments);
	const setSshKeys = useAppStore((state) => state.setSshKeys);
	const setDestinations = useAppStore((state) => state.setDestinations);
	const setTags = useAppStore((state) => state.setTags);
	const setSchedules = useAppStore((state) => state.setSchedules);
	const setProfile = useAppStore((state) => state.setProfile);
	const setMembers = useAppStore((state) => state.setMembers);
	const setInvites = useAppStore((state) => state.setInvites);
	const setDomains = useAppStore((state) => state.setDomains);
	const setBackups = useAppStore((state) => state.setBackups);
	const setOverviewServices = useAppStore((state) => state.setOverviewServices);
	const setHydrated = useAppStore((state) => state.setHydrated);

	// Live queries (stream-invalidated, 60s cache)
	const {data: liveMembers} = usePermissionGroupMembers();
	const {data: liveInvites} = usePermissionGroupInvites();
	const {data: liveProfile} = useAuthWhoAmI();
	const {data: liveVaults} = useVaultList();
	const {data: liveDns} = useDnsList();
	const {data: liveAiSettings} = useAiListSettings();
	const {data: liveCertificates} = useCertificateList();
	const {data: liveProjects} = useProjectListByOrganization(BigInt(orgId));
	const {data: liveServers} = useRemoteServerList();
	const {data: liveDeployments} = useDeploymentList();
	const {data: liveSchedules} = useScheduleListByOrganization(BigInt(orgId));
	const {data: liveSshKeys} = useSshKeyList();
	const {data: liveDestinations} = useDestinationList();
	const {data: liveTags} = useTagListAll();
	const {data: liveServices} = useOverviewServices(BigInt(orgId));
	const {data: liveDomains} = useOverviewDomains(BigInt(orgId));
	const {data: liveBackups} = useOverviewBackups(BigInt(orgId));

	// Query → store mirrors. Payload shapes are the legacy snake_case
	// payloads, so pass-through syncs bridge with the store types.
	useSyncList(liveMembers as any, setMembers);
	useSyncList(liveInvites as any, setInvites);
	useSyncList(liveVaults as any, setVaultProviders);
	useSyncList(liveDns as any, setDnsProviders);
	useSyncList(liveProjects as any, setProjects);
	useSyncList(liveServers as any, setServers);
	useSyncList(liveDeployments as any, setDeployments);
	useSyncList(liveSchedules as any, setSchedules);
	useSyncList(liveSshKeys as any, setSshKeys);
	useSyncList(liveDestinations as any, setDestinations);
	useSyncList(liveTags as any, setTags);
	useSyncList(liveServices as any, setOverviewServices);

	useSyncListAs(liveAiSettings as any, setAiSettings, toAiSettings);
	useSyncListAs(liveCertificates as any, setCertificates, toCertificates);
	useSyncListAs(liveDomains as any, setDomains, toDomains);
	useSyncListAs(liveBackups as any, setBackups, toBackups);

	useEffect(() => {
		if (liveProfile) setProfile(toProfile(liveProfile as any) as any);
	}, [liveProfile, setProfile]);

	// Connection liveness (isWsConnected) is owned by useTableWatch; this
	// hook only marks the store hydrated once mounted.
	useEffect(() => {
		setHydrated(true);
	}, [setHydrated]);
}
