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

interface AppStoreState {
	// Vault Providers State
	vaultProviders: VaultProviderItem[];
	isVaultLoading: boolean;
	setVaultProviders: (providers: VaultProviderItem[]) => void;
	addVaultProvider: (provider: VaultProviderItem) => void;
	updateVaultProvider: (id: number, provider: Partial<VaultProviderItem>) => void;
	deleteVaultProvider: (id: number) => void;

	// DNS Providers State
	dnsProviders: DnsProviderItem[];
	isDnsLoading: boolean;
	setDnsProviders: (providers: DnsProviderItem[]) => void;
	addDnsProvider: (provider: DnsProviderItem) => void;
	updateDnsProvider: (id: number, provider: Partial<DnsProviderItem>) => void;
	deleteDnsProvider: (id: number) => void;

	// Projects State
	projects: ProjectItem[];
	isProjectsLoading: boolean;
	setProjects: (projects: ProjectItem[]) => void;
	addProject: (project: ProjectItem) => void;
	updateProject: (id: number, project: Partial<ProjectItem>) => void;
	deleteProject: (id: number) => void;

	// Applications State
	applications: ApplicationItem[];
	isApplicationsLoading: boolean;
	setApplications: (applications: ApplicationItem[]) => void;
	addApplication: (app: ApplicationItem) => void;
	updateApplication: (id: number, app: Partial<ApplicationItem>) => void;
	deleteApplication: (id: number) => void;

	// Databases State
	databases: DatabaseItem[];
	isDatabasesLoading: boolean;
	setDatabases: (databases: DatabaseItem[]) => void;
	addDatabase: (db: DatabaseItem) => void;
	updateDatabase: (id: number, db: Partial<DatabaseItem>) => void;
	deleteDatabase: (id: number) => void;

	// Composes State
	composes: ComposeItem[];
	isComposesLoading: boolean;
	setComposes: (composes: ComposeItem[]) => void;
	addCompose: (compose: ComposeItem) => void;
	updateCompose: (id: number, compose: Partial<ComposeItem>) => void;
	deleteCompose: (id: number) => void;

	// Servers State
	servers: ServerItem[];
	isServersLoading: boolean;
	setServers: (servers: ServerItem[]) => void;
	addServer: (server: ServerItem) => void;
	updateServer: (id: number, server: Partial<ServerItem>) => void;
	deleteServer: (id: number) => void;

	// Global Sync Hydration Status
	isHydrated: boolean;
	isWsConnected: boolean;
	setHydrated: (hydrated: boolean) => void;
	setWsConnected: (connected: boolean) => void;
}

export const useAppStore = create<AppStoreState>((set) => ({
	// Vault Providers Initial State & Actions
	vaultProviders: [],
	isVaultLoading: true,
	setVaultProviders: (providers) => set({ vaultProviders: providers, isVaultLoading: false }),
	addVaultProvider: (provider) =>
		set((state) => ({
			vaultProviders: [provider, ...state.vaultProviders.filter((p) => p.id !== provider.id)],
		})),
	updateVaultProvider: (id, updated) =>
		set((state) => ({
			vaultProviders: state.vaultProviders.map((p) => (p.id === id ? { ...p, ...updated } : p)),
		})),
	deleteVaultProvider: (id) =>
		set((state) => ({
			vaultProviders: state.vaultProviders.filter((p) => p.id !== id),
		})),

	// DNS Providers Initial State & Actions
	dnsProviders: [],
	isDnsLoading: true,
	setDnsProviders: (providers) => set({ dnsProviders: providers, isDnsLoading: false }),
	addDnsProvider: (provider) =>
		set((state) => ({
			dnsProviders: [provider, ...state.dnsProviders.filter((p) => p.id !== provider.id)],
		})),
	updateDnsProvider: (id, updated) =>
		set((state) => ({
			dnsProviders: state.dnsProviders.map((p) => (p.id === id ? { ...p, ...updated } : p)),
		})),
	deleteDnsProvider: (id) =>
		set((state) => ({
			dnsProviders: state.dnsProviders.filter((p) => p.id !== id),
		})),

	// Projects Initial State & Actions
	projects: [],
	isProjectsLoading: true,
	setProjects: (projects) => set({ projects, isProjectsLoading: false }),
	addProject: (project) =>
		set((state) => ({
			projects: [project, ...state.projects.filter((p) => p.id !== project.id)],
		})),
	updateProject: (id, updated) =>
		set((state) => ({
			projects: state.projects.map((p) => (p.id === id ? { ...p, ...updated } : p)),
		})),
	deleteProject: (id) =>
		set((state) => ({
			projects: state.projects.filter((p) => p.id !== id),
		})),

	// Applications Initial State & Actions
	applications: [],
	isApplicationsLoading: true,
	setApplications: (applications) => set({ applications, isApplicationsLoading: false }),
	addApplication: (app) =>
		set((state) => ({
			applications: [app, ...state.applications.filter((a) => a.id !== app.id)],
		})),
	updateApplication: (id, updated) =>
		set((state) => ({
			applications: state.applications.map((a) => (a.id === id ? { ...a, ...updated } : a)),
		})),
	deleteApplication: (id) =>
		set((state) => ({
			applications: state.applications.filter((a) => a.id !== id),
		})),

	// Databases Initial State & Actions
	databases: [],
	isDatabasesLoading: true,
	setDatabases: (databases) => set({ databases, isDatabasesLoading: false }),
	addDatabase: (db) =>
		set((state) => ({
			databases: [db, ...state.databases.filter((d) => d.id !== db.id)],
		})),
	updateDatabase: (id, updated) =>
		set((state) => ({
			databases: state.databases.map((d) => (d.id === id ? { ...d, ...updated } : d)),
		})),
	deleteDatabase: (id) =>
		set((state) => ({
			databases: state.databases.filter((d) => d.id !== id),
		})),

	// Composes Initial State & Actions
	composes: [],
	isComposesLoading: true,
	setComposes: (composes) => set({ composes, isComposesLoading: false }),
	addCompose: (compose) =>
		set((state) => ({
			composes: [compose, ...state.composes.filter((c) => c.id !== compose.id)],
		})),
	updateCompose: (id, updated) =>
		set((state) => ({
			composes: state.composes.map((c) => (c.id === id ? { ...c, ...updated } : c)),
		})),
	deleteCompose: (id) =>
		set((state) => ({
			composes: state.composes.filter((c) => c.id !== id),
		})),

	// Servers Initial State & Actions
	servers: [],
	isServersLoading: true,
	setServers: (servers) => set({ servers, isServersLoading: false }),
	addServer: (server) =>
		set((state) => ({
			servers: [server, ...state.servers.filter((s) => s.id !== server.id)],
		})),
	updateServer: (id, updated) =>
		set((state) => ({
			servers: state.servers.map((s) => (s.id === id ? { ...s, ...updated } : s)),
		})),
	deleteServer: (id) =>
		set((state) => ({
			servers: state.servers.filter((s) => s.id !== id),
		})),

	// Hydration & Connection Status
	isHydrated: false,
	isWsConnected: false,
	setHydrated: (hydrated) => set({ isHydrated: hydrated }),
	setWsConnected: (connected) => set({ isWsConnected: connected }),
}));
