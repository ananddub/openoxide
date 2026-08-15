import { useEffect } from 'react';
import { useAppStore } from '#/stores/app-store';
import { $api } from '#/api/query';

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
	const setSwarmNodes = useAppStore((state) => state.setSwarmNodes);
	const setTags = useAppStore((state) => state.setTags);
	const setSchedules = useAppStore((state) => state.setSchedules);
	const setSshKeys = useAppStore((state) => state.setSshKeys);
	const setDestinations = useAppStore((state) => state.setDestinations);
	const setProfile = useAppStore((state) => state.setProfile);
	const setMembers = useAppStore((state) => state.setMembers);
	const setInvites = useAppStore((state) => state.setInvites);

	const setHydrated = useAppStore((state) => state.setHydrated);
	const setWsConnected = useAppStore((state) => state.setWsConnected);

	// Action Mutations from Zustand Store
	const addVaultProvider = useAppStore((state) => state.addVaultProvider);
	const updateVaultProvider = useAppStore((state) => state.updateVaultProvider);
	const deleteVaultProvider = useAppStore((state) => state.deleteVaultProvider);

	const addDnsProvider = useAppStore((state) => state.addDnsProvider);
	const updateDnsProvider = useAppStore((state) => state.updateDnsProvider);
	const deleteDnsProvider = useAppStore((state) => state.deleteDnsProvider);

	const addProject = useAppStore((state) => state.addProject);
	const updateProject = useAppStore((state) => state.updateProject);
	const deleteProject = useAppStore((state) => state.deleteProject);

	const addApplication = useAppStore((state) => state.addApplication);
	const updateApplication = useAppStore((state) => state.updateApplication);
	const deleteApplication = useAppStore((state) => state.deleteApplication);

	const addDatabase = useAppStore((state) => state.addDatabase);
	const updateDatabase = useAppStore((state) => state.updateDatabase);
	const deleteDatabase = useAppStore((state) => state.deleteDatabase);

	const addCompose = useAppStore((state) => state.addCompose);
	const updateCompose = useAppStore((state) => state.updateCompose);
	const deleteCompose = useAppStore((state) => state.deleteCompose);

	const addServer = useAppStore((state) => state.addServer);
	const updateServer = useAppStore((state) => state.updateServer);
	const deleteServer = useAppStore((state) => state.deleteServer);

	const addDeployment = useAppStore((state) => state.addDeployment);
	const updateDeployment = useAppStore((state) => state.updateDeployment);

	const addMember = useAppStore((state) => state.addMember);
	const updateMember = useAppStore((state) => state.updateMember);
	const deleteMember = useAppStore((state) => state.deleteMember);

	const addInvite = useAppStore((state) => state.addInvite);
	const deleteInvite = useAppStore((state) => state.deleteInvite);

	// Fetch Initial Hydration State for All Resources
	const { data: rawVaults } = $api.useQuery('get', '/vault-providers' as any, {} as any);
	const { data: rawDns } = $api.useQuery('get', '/dns-providers' as any, {} as any);
	const { data: rawProjects } = $api.useQuery('get', '/projects' as any, {} as any);
	const { data: rawServers } = $api.useQuery('get', '/remote-servers' as any, {} as any);
	const { data: rawDeployments } = $api.useQuery('get', '/deployments' as any, {} as any);
	const { data: rawSshKeys } = $api.useQuery('get', '/ssh-keys' as any, {} as any);
	const { data: rawDestinations } = $api.useQuery('get', '/destinations' as any, {} as any);
	const { data: rawTags } = $api.useQuery('get', '/tags' as any, {} as any);
	const { data: rawProfile } = $api.useQuery('get', '/auth/me' as any, {} as any);
	const { data: rawMembers } = $api.useQuery('get', '/permission-groups/members' as any, {} as any);
	const { data: rawInvites } = $api.useQuery('get', '/permission-groups/invites' as any, {} as any);

	// Hydration Sync Effects
	useEffect(() => { if (rawVaults && Array.isArray(rawVaults)) setVaultProviders(rawVaults as any); }, [rawVaults, setVaultProviders]);
	useEffect(() => { if (rawDns && Array.isArray(rawDns)) setDnsProviders(rawDns as any); }, [rawDns, setDnsProviders]);
	useEffect(() => { if (rawProjects && Array.isArray(rawProjects)) setProjects(rawProjects as any); }, [rawProjects, setProjects]);
	useEffect(() => { if (rawServers && Array.isArray(rawServers)) setServers(rawServers as any); }, [rawServers, setServers]);
	useEffect(() => { if (rawDeployments && Array.isArray(rawDeployments)) setDeployments(rawDeployments as any); }, [rawDeployments, setDeployments]);
	useEffect(() => { if (rawSshKeys && Array.isArray(rawSshKeys)) setSshKeys(rawSshKeys as any); }, [rawSshKeys, setSshKeys]);
	useEffect(() => { if (rawDestinations && Array.isArray(rawDestinations)) setDestinations(rawDestinations as any); }, [rawDestinations, setDestinations]);
	useEffect(() => { if (rawTags && Array.isArray(rawTags)) setTags(rawTags as any); }, [rawTags, setTags]);
	useEffect(() => { if (rawProfile) setProfile(rawProfile as any); }, [rawProfile, setProfile]);
	useEffect(() => { if (rawMembers && Array.isArray(rawMembers)) setMembers(rawMembers as any); }, [rawMembers, setMembers]);
	useEffect(() => { if (rawInvites && Array.isArray(rawInvites)) setInvites(rawInvites as any); }, [rawInvites, setInvites]);

	useEffect(() => { setHydrated(true); }, [setHydrated]);

	// Global WebSocket Event Stream Engine
	useEffect(() => {
		const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
		const wsUrl = `${protocol}//${window.location.host}/ws/events`;
		let socket: WebSocket | null = null;
		let reconnectTimer: any = null;

		const connect = () => {
			try {
				socket = new WebSocket(wsUrl);

				socket.onopen = () => { setWsConnected(true); };

				socket.onmessage = (event) => {
					try {
						const msg = JSON.parse(event.data);
						if (!msg || !msg.type) return;

						switch (msg.type) {
							// Vault Live Events
							case 'VAULT_CREATED': if (msg.data) addVaultProvider(msg.data); break;
							case 'VAULT_UPDATED': if (msg.data && msg.data.id) updateVaultProvider(msg.data.id, msg.data); break;
							case 'VAULT_DELETED': if (msg.id) deleteVaultProvider(msg.id); break;

							// DNS Live Events
							case 'DNS_CREATED': if (msg.data) addDnsProvider(msg.data); break;
							case 'DNS_UPDATED': if (msg.data && msg.data.id) updateDnsProvider(msg.data.id, msg.data); break;
							case 'DNS_DELETED': if (msg.id) deleteDnsProvider(msg.id); break;

							// Projects Live Events
							case 'PROJECT_CREATED': if (msg.data) addProject(msg.data); break;
							case 'PROJECT_UPDATED': if (msg.data && msg.data.id) updateProject(msg.data.id, msg.data); break;
							case 'PROJECT_DELETED': if (msg.id) deleteProject(msg.id); break;

							// Applications Live Events
							case 'APP_CREATED': if (msg.data) addApplication(msg.data); break;
							case 'APP_UPDATED': if (msg.data && msg.data.id) updateApplication(msg.data.id, msg.data); break;
							case 'APP_DELETED': if (msg.id) deleteApplication(msg.id); break;

							// Databases Live Events
							case 'DATABASE_CREATED': if (msg.data) addDatabase(msg.data); break;
							case 'DATABASE_UPDATED': if (msg.data && msg.data.id) updateDatabase(msg.data.id, msg.data); break;
							case 'DATABASE_DELETED': if (msg.id) deleteDatabase(msg.id); break;

							// Compose Live Events
							case 'COMPOSE_CREATED': if (msg.data) addCompose(msg.data); break;
							case 'COMPOSE_UPDATED': if (msg.data && msg.data.id) updateCompose(msg.data.id, msg.data); break;
							case 'COMPOSE_DELETED': if (msg.id) deleteCompose(msg.id); break;

							// Servers Live Events
							case 'SERVER_CREATED': if (msg.data) addServer(msg.data); break;
							case 'SERVER_UPDATED': if (msg.data && msg.data.id) updateServer(msg.data.id, msg.data); break;
							case 'SERVER_DELETED': if (msg.id) deleteServer(msg.id); break;

							// Deployments Live Events
							case 'DEPLOYMENT_CREATED': if (msg.data) addDeployment(msg.data); break;
							case 'DEPLOYMENT_UPDATED': if (msg.data && msg.data.id) updateDeployment(msg.data.id, msg.data); break;

							// Members & Invites Live Events
							case 'MEMBER_CREATED': if (msg.data) addMember(msg.data); break;
							case 'MEMBER_UPDATED': if (msg.data && msg.data.id) updateMember(String(msg.data.id), msg.data); break;
							case 'MEMBER_DELETED': if (msg.id) deleteMember(String(msg.id)); break;
							case 'INVITE_CREATED': if (msg.data) addInvite(msg.data); break;
							case 'INVITE_DELETED': if (msg.id) deleteInvite(String(msg.id)); break;

							default: break;
						}
					} catch (e) { /* ping frames ignored */ }
				};

				socket.onclose = () => {
					setWsConnected(false);
					reconnectTimer = setTimeout(connect, 3000);
				};

				socket.onerror = () => {
					setWsConnected(false);
					if (socket) socket.close();
				};
			} catch (e) {
				reconnectTimer = setTimeout(connect, 3000);
			}
		};

		connect();

		return () => {
			if (socket) socket.close();
			if (reconnectTimer) clearTimeout(reconnectTimer);
		};
	}, [
		addVaultProvider, updateVaultProvider, deleteVaultProvider,
		addDnsProvider, updateDnsProvider, deleteDnsProvider,
		addProject, updateProject, deleteProject,
		addApplication, updateApplication, deleteApplication,
		addDatabase, updateDatabase, deleteDatabase,
		addCompose, updateCompose, deleteCompose,
		addServer, updateServer, deleteServer,
		addDeployment, updateDeployment,
		addMember, updateMember, deleteMember,
		addInvite, deleteInvite,
		setWsConnected,
	]);
}
