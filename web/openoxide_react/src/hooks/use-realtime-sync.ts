import { useEffect } from 'react';
import { useAppStore } from '#/stores/app-store';
import { $api } from '#/api/query';
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
} from 'virtual:openoxide-live';

export function useRealtimeSync() {
	// Zustand Setters
	const setVaultProviders = useAppStore((state) => state.setVaultProviders);
	const setDnsProviders = useAppStore((state) => state.setDnsProviders);
	const setProjects = useAppStore((state) => state.setProjects);
	const setApplications = useAppStore((state) => state.setApplications);
	const setDatabases = useAppStore((state) => state.setDatabases);
	const setComposes = useAppStore((state) => state.setComposes);
	const setServers = useAppStore((state) => state.setServers);

	const setDeployments = useAppStore((state) => state.setDeployments);
	const setSshKeys = useAppStore((state) => state.setSshKeys);
	const setDestinations = useAppStore((state) => state.setDestinations);
	const setTags = useAppStore((state) => state.setTags);
	const setSchedules = useAppStore((state) => state.setSchedules);
	const setProfile = useAppStore((state) => state.setProfile);
	const setMembers = useAppStore((state) => state.setMembers);
	const setInvites = useAppStore((state) => state.setInvites);

	const setHydrated = useAppStore((state) => state.setHydrated);
	const setWsConnected = useAppStore((state) => state.setWsConnected);

	// 1. Live Socket.IO Reactive Stream Hooks from virtual:openoxide-live
	const { data: liveMembers } = usePermissionGroupMembers();
	const { data: liveInvites } = usePermissionGroupInvites();
	const { data: liveProfile } = useAuthWhoAmI();
	const { data: liveVaults } = useVaultList();
	const { data: liveDns } = useDnsList();
	const { data: liveProjects } = useProjectListByOrganization(1 as any);
	const { data: liveServers } = useRemoteServerList();
	const { data: liveDeployments } = useDeploymentList();
	const { data: liveSchedules } = useScheduleListByOrganization(1 as any);
	const { data: liveSshKeys } = useSshKeyList();
	const { data: liveDestinations } = useDestinationList();
	const { data: liveTags } = useTagListAll();

	// Sync Live Realtime Streams directly into Zustand Store
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
		setHydrated(true);
		setWsConnected(true);
	}, [setHydrated, setWsConnected]);
}
