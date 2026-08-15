import { useEffect } from 'react';
import { useAppStore } from '#/stores/app-store';
import { useOrganizationStore } from '#/stores/organization-store';
import {
	usePermissionGroupMembers,
	usePermissionGroupInvites,
	useAuthWhoAmI,
	useVaultList,
	useDnsList,
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
} from 'virtual:openoxide-live';

export function useRealtimeSync() {
	const activeOrg = useOrganizationStore((state) => state.activeOrg);
	const orgId = activeOrg?.id || 1;

	// Zustand Setters
	const setVaultProviders = useAppStore((state) => state.setVaultProviders);
	const setDnsProviders = useAppStore((state) => state.setDnsProviders);
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
	const setWsConnected = useAppStore((state) => state.setWsConnected);

	// 1. Live Socket.IO Reactive Stream Hooks from virtual:openoxide-live
	const { data: liveMembers } = usePermissionGroupMembers();
	const { data: liveInvites } = usePermissionGroupInvites();
	const { data: liveProfile } = useAuthWhoAmI();
	const { data: liveVaults } = useVaultList();
	const { data: liveDns } = useDnsList();
	const { data: liveProjects } = useProjectListByOrganization(BigInt(orgId));
	const { data: liveServers } = useRemoteServerList();
	const { data: liveDeployments } = useDeploymentList();
	const { data: liveSchedules } = useScheduleListByOrganization(BigInt(orgId));
	const { data: liveSshKeys } = useSshKeyList();
	const { data: liveDestinations } = useDestinationList();
	const { data: liveTags } = useTagListAll();

	// 2. Organization-Wide Services, Domains & Backups LIVE Socket.IO Sync (ZERO POLLING)
	const { data: liveServices } = useOverviewServices(BigInt(orgId));
	const { data: liveDomains } = useOverviewDomains(BigInt(orgId));
	const { data: liveBackups } = useOverviewBackups(BigInt(orgId));

	// Sync Live Realtime Streams directly into Zustand Store
	useEffect(() => {
		if (liveServices && Array.isArray(liveServices)) {
			setOverviewServices(liveServices as any);
		}
	}, [liveServices, setOverviewServices]);

	useEffect(() => {
		if (liveMembers && Array.isArray(liveMembers)) setMembers(liveMembers as any);
	}, [liveMembers, setMembers]);

	useEffect(() => {
		if (liveInvites && Array.isArray(liveInvites)) setInvites(liveInvites as any);
	}, [liveInvites, setInvites]);

	useEffect(() => {
		if (liveProfile) {
			setProfile({
				id: (liveProfile as any).user_id || (liveProfile as any).id,
				email: (liveProfile as any).email,
				name: `${(liveProfile as any).first_name || ''} ${(liveProfile as any).last_name || ''}`.trim(),
				avatar: (liveProfile as any).avatar,
			} as any);
		}
	}, [liveProfile, setProfile]);

	useEffect(() => {
		if (liveVaults && Array.isArray(liveVaults)) setVaultProviders(liveVaults as any);
	}, [liveVaults, setVaultProviders]);

	useEffect(() => {
		if (liveDns && Array.isArray(liveDns)) setDnsProviders(liveDns as any);
	}, [liveDns, setDnsProviders]);

	useEffect(() => {
		if (liveProjects && Array.isArray(liveProjects)) setProjects(liveProjects as any);
	}, [liveProjects, setProjects]);

	useEffect(() => {
		if (liveServers && Array.isArray(liveServers)) setServers(liveServers as any);
	}, [liveServers, setServers]);

	useEffect(() => {
		if (liveDeployments && Array.isArray(liveDeployments)) setDeployments(liveDeployments as any);
	}, [liveDeployments, setDeployments]);

	useEffect(() => {
		if (liveSchedules && Array.isArray(liveSchedules)) setSchedules(liveSchedules as any);
	}, [liveSchedules, setSchedules]);

	useEffect(() => {
		if (liveSshKeys && Array.isArray(liveSshKeys)) setSshKeys(liveSshKeys as any);
	}, [liveSshKeys, setSshKeys]);

	useEffect(() => {
		if (liveDestinations && Array.isArray(liveDestinations)) setDestinations(liveDestinations as any);
	}, [liveDestinations, setDestinations]);

	useEffect(() => {
		if (liveTags && Array.isArray(liveTags)) setTags(liveTags as any);
	}, [liveTags, setTags]);

	useEffect(() => {
		if (liveDomains && Array.isArray(liveDomains)) {
			setDomains(
				liveDomains.map((d: any) => ({
					id: Number(d.id),
					domain: d.host,
					host: d.host,
					path: d.path || '/',
					port: d.port || 80,
					https: !!d.https,
					application_id: d.application_id ? Number(d.application_id) : undefined,
					compose_id: d.compose_id ? Number(d.compose_id) : undefined,
					service_name: d.service_name,
					project_name: d.project_name,
				})) as any
			);
		}
	}, [liveDomains, setDomains]);

	useEffect(() => {
		if (liveBackups && Array.isArray(liveBackups)) {
			setBackups(
				liveBackups.map((b: any) => ({
					id: Number(b.id),
					name: b.name,
					backup_type: b.backup_type,
					status: b.status,
					destination: b.destination,
					created_at: Number(b.created_at),
				})) as any
			);
		}
	}, [liveBackups, setBackups]);

	useEffect(() => {
		setHydrated(true);
		setWsConnected(true);
	}, [setHydrated, setWsConnected]);
}
