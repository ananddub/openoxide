import { create } from 'zustand';

export interface VaultProviderItem {
	id: number;
	name: string;
	provider_type: string;
	api_url: string;
	auth_token: string;
	namespace?: string;
	config_json?: string;
	created_at: number;
	updated_at: number;
}

export interface DnsProviderItem {
	id: number;
	name: string;
	provider_type: string;
	credentials_json: string;
	created_at: number;
	updated_at: number;
}

export interface ProjectItem {
	id: number;
	name: string;
	description?: string;
	created_at?: number;
	updated_at?: number;
	applications?: any[];
	composes?: any[];
	postgresDbs?: any[];
	mysqlDbs?: any[];
	mariadbDbs?: any[];
	mongoDbs?: any[];
	redisDbs?: any[];
}

export interface ApplicationItem {
	id: number;
	name?: string;
	app_name?: string;
	project_id?: number;
	status?: string;
	build_type?: string;
	created_at?: number;
	updated_at?: number;
}

export interface DatabaseItem {
	id: number;
	name?: string;
	database_name?: string;
	db_type?: string;
	project_id?: number;
	status?: string;
	created_at?: number;
	updated_at?: number;
}

export interface ComposeItem {
	id: number;
	name?: string;
	app_name?: string;
	project_id?: number;
	status?: string;
	created_at?: number;
	updated_at?: number;
}

export interface ServerItem {
	id: number;
	name: string;
	ip_address?: string;
	status?: string;
	is_primary?: boolean;
	created_at?: number;
}

export interface DeploymentItem {
	id: number;
	app_id?: number;
	compose_id?: number;
	status?: string;
	title?: string;
	description?: string;
	created_at?: number;
}

export interface TagItem {
	id: number;
	name: string;
	color?: string;
}

export interface ScheduleItem {
	id: number;
	name: string;
	cron_expression?: string;
	status?: string;
}

export interface SwarmNodeItem {
	id: string;
	hostname?: string;
	role?: string;
	status?: string;
	availability?: string;
}

export interface SshKeyItem {
	id: number;
	name: string;
	public_key?: string;
}

export interface DestinationItem {
	id: number;
	name: string;
	provider_type?: string;
}

export interface UserProfileItem {
	id?: number;
	email?: string;
	name?: string;
	role?: string;
}

interface AppStoreState {
	// Vault & DNS
	vaultProviders: VaultProviderItem[];
	dnsProviders: DnsProviderItem[];

	// Core Projects & Apps
	projects: ProjectItem[];
	applications: ApplicationItem[];
	databases: DatabaseItem[];
	composes: ComposeItem[];
	servers: ServerItem[];

	// Deployments & Swarm
	deployments: DeploymentItem[];
	swarmNodes: SwarmNodeItem[];

	// Tags, Schedules, Keys, Destinations, Profile
	tags: TagItem[];
	schedules: ScheduleItem[];
	sshKeys: SshKeyItem[];
	destinations: DestinationItem[];
	profile: UserProfileItem | null;

	// Global Sync Hydration Status
	isHydrated: boolean;
	isWsConnected: boolean;

	// Setters & Actions
	setVaultProviders: (providers: VaultProviderItem[]) => void;
	addVaultProvider: (provider: VaultProviderItem) => void;
	updateVaultProvider: (id: number, provider: Partial<VaultProviderItem>) => void;
	deleteVaultProvider: (id: number) => void;

	setDnsProviders: (providers: DnsProviderItem[]) => void;
	addDnsProvider: (provider: DnsProviderItem) => void;
	updateDnsProvider: (id: number, provider: Partial<DnsProviderItem>) => void;
	deleteDnsProvider: (id: number) => void;

	setProjects: (projects: ProjectItem[]) => void;
	addProject: (project: ProjectItem) => void;
	updateProject: (id: number, project: Partial<ProjectItem>) => void;
	deleteProject: (id: number) => void;

	setApplications: (applications: ApplicationItem[]) => void;
	addApplication: (app: ApplicationItem) => void;
	updateApplication: (id: number, app: Partial<ApplicationItem>) => void;
	deleteApplication: (id: number) => void;

	setDatabases: (databases: DatabaseItem[]) => void;
	addDatabase: (db: DatabaseItem) => void;
	updateDatabase: (id: number, db: Partial<DatabaseItem>) => void;
	deleteDatabase: (id: number) => void;

	setComposes: (composes: ComposeItem[]) => void;
	addCompose: (compose: ComposeItem) => void;
	updateCompose: (id: number, compose: Partial<ComposeItem>) => void;
	deleteCompose: (id: number) => void;

	setServers: (servers: ServerItem[]) => void;
	addServer: (server: ServerItem) => void;
	updateServer: (id: number, server: Partial<ServerItem>) => void;
	deleteServer: (id: number) => void;

	setDeployments: (deployments: DeploymentItem[]) => void;
	addDeployment: (deployment: DeploymentItem) => void;
	updateDeployment: (id: number, deployment: Partial<DeploymentItem>) => void;

	setSwarmNodes: (nodes: SwarmNodeItem[]) => void;
	setTags: (tags: TagItem[]) => void;
	setSchedules: (schedules: ScheduleItem[]) => void;
	setSshKeys: (keys: SshKeyItem[]) => void;
	setDestinations: (destinations: DestinationItem[]) => void;
	setProfile: (profile: UserProfileItem) => void;

	setHydrated: (hydrated: boolean) => void;
	setWsConnected: (connected: boolean) => void;
}

export const useAppStore = create<AppStoreState>((set) => ({
	vaultProviders: [],
	dnsProviders: [],
	projects: [],
	applications: [],
	databases: [],
	composes: [],
	servers: [],
	deployments: [],
	swarmNodes: [],
	tags: [],
	schedules: [],
	sshKeys: [],
	destinations: [],
	profile: null,

	isHydrated: false,
	isWsConnected: false,

	setVaultProviders: (providers) => set({ vaultProviders: providers }),
	addVaultProvider: (provider) =>
		set((state) => ({ vaultProviders: [provider, ...state.vaultProviders.filter((p) => p.id !== provider.id)] })),
	updateVaultProvider: (id, updated) =>
		set((state) => ({ vaultProviders: state.vaultProviders.map((p) => (p.id === id ? { ...p, ...updated } : p)) })),
	deleteVaultProvider: (id) =>
		set((state) => ({ vaultProviders: state.vaultProviders.filter((p) => p.id !== id) })),

	setDnsProviders: (providers) => set({ dnsProviders: providers }),
	addDnsProvider: (provider) =>
		set((state) => ({ dnsProviders: [provider, ...state.dnsProviders.filter((p) => p.id !== provider.id)] })),
	updateDnsProvider: (id, updated) =>
		set((state) => ({ dnsProviders: state.dnsProviders.map((p) => (p.id === id ? { ...p, ...updated } : p)) })),
	deleteDnsProvider: (id) =>
		set((state) => ({ dnsProviders: state.dnsProviders.filter((p) => p.id !== id) })),

	setProjects: (projects) => set({ projects }),
	addProject: (project) =>
		set((state) => ({ projects: [project, ...state.projects.filter((p) => p.id !== project.id)] })),
	updateProject: (id, updated) =>
		set((state) => ({ projects: state.projects.map((p) => (p.id === id ? { ...p, ...updated } : p)) })),
	deleteProject: (id) =>
		set((state) => ({ projects: state.projects.filter((p) => p.id !== id) })),

	setApplications: (applications) => set({ applications }),
	addApplication: (app) =>
		set((state) => ({ applications: [app, ...state.applications.filter((a) => a.id !== app.id)] })),
	updateApplication: (id, updated) =>
		set((state) => ({ applications: state.applications.map((a) => (a.id === id ? { ...a, ...updated } : a)) })),
	deleteApplication: (id) =>
		set((state) => ({ applications: state.applications.filter((a) => a.id !== id) })),

	setDatabases: (databases) => set({ databases }),
	addDatabase: (db) =>
		set((state) => ({ databases: [db, ...state.databases.filter((d) => d.id !== db.id)] })),
	updateDatabase: (id, updated) =>
		set((state) => ({ databases: state.databases.map((d) => (d.id === id ? { ...d, ...updated } : d)) })),
	deleteDatabase: (id) =>
		set((state) => ({ databases: state.databases.filter((d) => d.id !== id) })),

	setComposes: (composes) => set({ composes }),
	addCompose: (compose) =>
		set((state) => ({ composes: [compose, ...state.composes.filter((c) => c.id !== compose.id)] })),
	updateCompose: (id, updated) =>
		set((state) => ({ composes: state.composes.map((c) => (c.id === id ? { ...c, ...updated } : c)) })),
	deleteCompose: (id) =>
		set((state) => ({ composes: state.composes.filter((c) => c.id !== id) })),

	setServers: (servers) => set({ servers }),
	addServer: (server) =>
		set((state) => ({ servers: [server, ...state.servers.filter((s) => s.id !== server.id)] })),
	updateServer: (id, updated) =>
		set((state) => ({ servers: state.servers.map((s) => (s.id === id ? { ...s, ...updated } : s)) })),
	deleteServer: (id) =>
		set((state) => ({ servers: state.servers.filter((s) => s.id !== id) })),

	setDeployments: (deployments) => set({ deployments }),
	addDeployment: (deployment) =>
		set((state) => ({ deployments: [deployment, ...state.deployments.filter((d) => d.id !== deployment.id)] })),
	updateDeployment: (id, updated) =>
		set((state) => ({ deployments: state.deployments.map((d) => (d.id === id ? { ...d, ...updated } : d)) })),

	setSwarmNodes: (swarmNodes) => set({ swarmNodes }),
	setTags: (tags) => set({ tags }),
	setSchedules: (schedules) => set({ schedules }),
	setSshKeys: (sshKeys) => set({ sshKeys }),
	setDestinations: (destinations) => set({ destinations }),
	setProfile: (profile) => set({ profile }),

	setHydrated: (hydrated) => set({ isHydrated: hydrated }),
	setWsConnected: (connected) => set({ isWsConnected: connected }),
}));
