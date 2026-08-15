import { useEffect } from 'react';
import { useAppStore } from '#/stores/app-store';
import { $api } from '#/api/query';
import {
	usePermissionGroupMembers,
	usePermissionGroupInvites,
	useAuthWhoAmI,
} from 'virtual:openoxide-live';

export function useRealtimeSync() {
	// Setters from Zustand Store
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
	const setProfile = useAppStore((state) => state.setProfile);
	const setMembers = useAppStore((state) => state.setMembers);
	const setInvites = useAppStore((state) => state.setInvites);

	const setHydrated = useAppStore((state) => state.setHydrated);
	const setWsConnected = useAppStore((state) => state.setWsConnected);

	// 1. Live Socket.IO Stream Hooks from virtual:openoxide-live
	const { data: liveMembers } = usePermissionGroupMembers();
	const { data: liveInvites } = usePermissionGroupInvites();
	const { data: liveProfile } = useAuthWhoAmI();

	// 2. HTTP Initial Hydration for static resources
	const { data: rawVaults } = $api.useQuery('get', '/vault-providers' as any, {} as any);
	const { data: rawDns } = $api.useQuery('get', '/dns-providers' as any, {} as any);
	const { data: rawProjects } = $api.useQuery('get', '/projects' as any, {} as any);
	const { data: rawServers } = $api.useQuery('get', '/remote-servers' as any, {} as any);
	const { data: rawDeployments } = $api.useQuery('get', '/deployments' as any, {} as any);
	const { data: rawSshKeys } = $api.useQuery('get', '/ssh-keys' as any, {} as any);
	const { data: rawDestinations } = $api.useQuery('get', '/destinations' as any, {} as any);
	const { data: rawTags } = $api.useQuery('get', '/tags' as any, {} as any);

	// Sync Live Realtime Socket Stream into Zustand Store
	useEffect(() => {
		if (liveMembers && Array.isArray(liveMembers)) {
			setMembers(liveMembers as any);
		}
	}, [liveMembers, setMembers]);

	useEffect(() => {
		if (liveInvites && Array.isArray(liveInvites)) {
			setInvites(liveInvites as any);
		}
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

	// Hydration Sync Effects
	useEffect(() => { if (rawVaults && Array.isArray(rawVaults)) setVaultProviders(rawVaults as any); }, [rawVaults, setVaultProviders]);
	useEffect(() => { if (rawDns && Array.isArray(rawDns)) setDnsProviders(rawDns as any); }, [rawDns, setDnsProviders]);
	useEffect(() => { if (rawProjects && Array.isArray(rawProjects)) setProjects(rawProjects as any); }, [rawProjects, setProjects]);
	useEffect(() => { if (rawServers && Array.isArray(rawServers)) setServers(rawServers as any); }, [rawServers, setServers]);
	useEffect(() => { if (rawDeployments && Array.isArray(rawDeployments)) setDeployments(rawDeployments as any); }, [rawDeployments, setDeployments]);
	useEffect(() => { if (rawSshKeys && Array.isArray(rawSshKeys)) setSshKeys(rawSshKeys as any); }, [rawSshKeys, setSshKeys]);
	useEffect(() => { if (rawDestinations && Array.isArray(rawDestinations)) setDestinations(rawDestinations as any); }, [rawDestinations, setDestinations]);
	useEffect(() => { if (rawTags && Array.isArray(rawTags)) setTags(rawTags as any); }, [rawTags, setTags]);

	useEffect(() => {
		setHydrated(true);
		setWsConnected(true);
	}, [setHydrated, setWsConnected]);
}
