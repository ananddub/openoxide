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
	kind?: string;
	project_id?: number;
	status?: string;
	created_at?: number;
	updated_at?: number;
}

export interface ComposeItem {
	id: number;
	name?: string;
	project_id?: number;
	status?: string;
	created_at?: number;
	updated_at?: number;
}

export interface RemoteServerItem {
	id: number;
	name: string;
	ip: string;
	port?: number;
	user?: string;
	status?: string;
	cpu_usage?: number;
	memory_usage?: number;
	disk_usage?: number;
}

export interface OverviewServiceItem {
	id: number;
	name: string;
	type: 'APP' | 'COMPOSE' | 'DATABASE';
	project_id: number;
	environment_id?: number;
	status?: string;
	createdAt?: number;
	dbKind?: string;
}

export interface DeploymentItem {
	id: number;
	status: string;
	commit_message?: string;
	created_at?: number;
	project_id?: number;
	application_id?: number;
}

export interface DomainItem {
	id: number;
	domain: string;
	service_type?: string;
	service_id?: number;
	ssl_enabled?: boolean;
}

export interface BackupItem {
	id: number;
	name: string;
	status?: string;
	file_size?: number;
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
	cron_expression: string;
	enabled?: boolean;
}

export interface SshKeyItem {
	id: number;
	title: string;
	public_key: string;
}

export interface DestinationItem {
	id: number;
	name: string;
	server_id?: number;
}

export interface UserProfileItem {
	id?: number;
	email?: string;
	name?: string;
	role?: string;
	avatar?: string;
}

export interface MemberItem {
	id: string;
	user_id?: number;
	email: string;
	name?: string;
	role: string;
	avatar?: string;
	banned?: boolean;
	created_at?: number;
}

export interface InviteItem {
	id: string;
	email: string;
	role: string;
	created_at?: number;
}

export interface AppStoreState {
	vaultProviders: VaultProviderItem[];
	dnsProviders: DnsProviderItem[];
	projects: ProjectItem[];
	applications: ApplicationItem[];
	databases: DatabaseItem[];
	composes: ComposeItem[];
	servers: RemoteServerItem[];
	overviewServices: OverviewServiceItem[];
	deployments: DeploymentItem[];
	swarmNodes: any[];
	domains: DomainItem[];
	backups: BackupItem[];
	tags: TagItem[];
	schedules: ScheduleItem[];
	sshKeys: SshKeyItem[];
	destinations: DestinationItem[];
	profile: UserProfileItem | null;
	members: MemberItem[];
	invites: InviteItem[];

	isHydrated: boolean;
	isWsConnected: boolean;

	setVaultProviders: (providers: VaultProviderItem[]) => void;
	addVaultProvider: (provider: VaultProviderItem) => void;
	updateVaultProvider: (id: number | string, updated: Partial<VaultProviderItem>) => void;
	deleteVaultProvider: (id: number | string) => void;

	setDnsProviders: (providers: DnsProviderItem[]) => void;
	addDnsProvider: (provider: DnsProviderItem) => void;
	updateDnsProvider: (id: number | string, updated: Partial<DnsProviderItem>) => void;
	deleteDnsProvider: (id: number | string) => void;

	setProjects: (projects: ProjectItem[]) => void;
	addProject: (project: ProjectItem) => void;
	updateProject: (id: number | string, updated: Partial<ProjectItem>) => void;
	deleteProject: (id: number | string) => void;

	setApplications: (applications: ApplicationItem[]) => void;
	addApplication: (app: ApplicationItem) => void;
	updateApplication: (id: number | string, updated: Partial<ApplicationItem>) => void;
	deleteApplication: (id: number | string) => void;

	setDatabases: (databases: DatabaseItem[]) => void;
	addDatabase: (db: DatabaseItem) => void;
	updateDatabase: (id: number | string, updated: Partial<DatabaseItem>) => void;
	deleteDatabase: (id: number | string) => void;

	setComposes: (composes: ComposeItem[]) => void;
	addCompose: (compose: ComposeItem) => void;
	updateCompose: (id: number | string, updated: Partial<ComposeItem>) => void;
	deleteCompose: (id: number | string) => void;

	setServers: (servers: RemoteServerItem[]) => void;
	addServer: (server: RemoteServerItem) => void;
	updateServer: (id: number | string, updated: Partial<RemoteServerItem>) => void;
	deleteServer: (id: number | string) => void;

	setOverviewServices: (services: OverviewServiceItem[]) => void;
	setDeployments: (deployments: DeploymentItem[]) => void;
	setSwarmNodes: (nodes: any[]) => void;
	setDomains: (domains: DomainItem[]) => void;
	setBackups: (backups: BackupItem[]) => void;

	setTags: (tags: TagItem[]) => void;
	addTag: (tag: TagItem) => void;
	deleteTag: (id: number | string) => void;

	setSchedules: (schedules: ScheduleItem[]) => void;
	addSchedule: (schedule: ScheduleItem) => void;
	updateSchedule: (id: number | string, updated: Partial<ScheduleItem>) => void;
	deleteSchedule: (id: number | string) => void;

	setSshKeys: (sshKeys: SshKeyItem[]) => void;
	setDestinations: (destinations: DestinationItem[]) => void;
	setProfile: (profile: UserProfileItem) => void;

	setMembers: (members: MemberItem[]) => void;
	addMember: (member: MemberItem) => void;
	updateMember: (id: string, updated: Partial<MemberItem>) => void;
	deleteMember: (id: string) => void;

	setInvites: (invites: InviteItem[]) => void;
	addInvite: (invite: InviteItem) => void;
	deleteInvite: (id: string) => void;

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
	overviewServices: [],
	deployments: [],
	swarmNodes: [],
	domains: [],
	backups: [],
	tags: [],
	schedules: [],
	sshKeys: [],
	destinations: [],
	profile: null,
	members: [],
	invites: [],

	isHydrated: false,
	isWsConnected: false,

	setVaultProviders: (providers) => set({ vaultProviders: providers }),
	addVaultProvider: (provider) =>
		set((state) => ({ vaultProviders: [provider, ...state.vaultProviders.filter((p) => String(p.id) !== String(provider.id))] })),
	updateVaultProvider: (id, updated) =>
		set((state) => ({ vaultProviders: state.vaultProviders.map((p) => (String(p.id) === String(id) ? { ...p, ...updated } : p)) })),
	deleteVaultProvider: (id) =>
		set((state) => ({ vaultProviders: state.vaultProviders.filter((p) => String(p.id) !== String(id)) })),

	setDnsProviders: (providers) => set({ dnsProviders: providers }),
	addDnsProvider: (provider) =>
		set((state) => ({ dnsProviders: [provider, ...state.dnsProviders.filter((p) => String(p.id) !== String(provider.id))] })),
	updateDnsProvider: (id, updated) =>
		set((state) => ({ dnsProviders: state.dnsProviders.map((p) => (String(p.id) === String(id) ? { ...p, ...updated } : p)) })),
	deleteDnsProvider: (id) =>
		set((state) => ({ dnsProviders: state.dnsProviders.filter((p) => String(p.id) !== String(id)) })),

	setProjects: (projects) => set({ projects }),
	addProject: (project) =>
		set((state) => ({ projects: [project, ...state.projects.filter((p) => String(p.id) !== String(project.id))] })),
	updateProject: (id, updated) =>
		set((state) => ({ projects: state.projects.map((p) => (String(p.id) === String(id) ? { ...p, ...updated } : p)) })),
	deleteProject: (id) =>
		set((state) => ({ projects: state.projects.filter((p) => String(p.id) !== String(id)) })),

	setApplications: (applications) => set({ applications }),
	addApplication: (app) =>
		set((state) => ({ applications: [app, ...state.applications.filter((a) => String(a.id) !== String(app.id))] })),
	updateApplication: (id, updated) =>
		set((state) => ({ applications: state.applications.map((a) => (String(a.id) === String(id) ? { ...a, ...updated } : a)) })),
	deleteApplication: (id) =>
		set((state) => ({ applications: state.applications.filter((a) => String(a.id) !== String(id)) })),

	setDatabases: (databases) => set({ databases }),
	addDatabase: (db) =>
		set((state) => ({ databases: [db, ...state.databases.filter((d) => String(d.id) !== String(db.id))] })),
	updateDatabase: (id, updated) =>
		set((state) => ({ databases: state.databases.map((d) => (String(d.id) === String(id) ? { ...d, ...updated } : d)) })),
	deleteDatabase: (id) =>
		set((state) => ({ databases: state.databases.filter((d) => String(d.id) !== String(id)) })),

	setComposes: (composes) => set({ composes }),
	addCompose: (compose) =>
		set((state) => ({ composes: [compose, ...state.composes.filter((c) => String(c.id) !== String(compose.id))] })),
	updateCompose: (id, updated) =>
		set((state) => ({ composes: state.composes.map((c) => (String(c.id) === String(id) ? { ...c, ...updated } : c)) })),
	deleteCompose: (id) =>
		set((state) => ({ composes: state.composes.filter((c) => String(c.id) !== String(id)) })),

	setServers: (servers) => set({ servers }),
	addServer: (server) =>
		set((state) => ({ servers: [server, ...state.servers.filter((s) => String(s.id) !== String(server.id))] })),
	updateServer: (id, updated) =>
		set((state) => ({ servers: state.servers.map((s) => (String(s.id) === String(id) ? { ...s, ...updated } : s)) })),
	deleteServer: (id) =>
		set((state) => ({ servers: state.servers.filter((s) => String(s.id) !== String(id)) })),

	setOverviewServices: (services) => set({ overviewServices: services }),
	setDeployments: (deployments) => set({ deployments }),
	setSwarmNodes: (nodes) => set({ swarmNodes: nodes }),
	setDomains: (domains) => set({ domains }),
	setBackups: (backups) => set({ backups }),

	setTags: (tags) => set({ tags }),
	addTag: (tag) =>
		set((state) => ({ tags: [tag, ...state.tags.filter((t) => String(t.id) !== String(tag.id))] })),
	deleteTag: (id) =>
		set((state) => ({ tags: state.tags.filter((t) => String(t.id) !== String(id)) })),

	setSchedules: (schedules) => set({ schedules }),
	addSchedule: (schedule) =>
		set((state) => ({ schedules: [schedule, ...state.schedules.filter((s) => String(s.id) !== String(schedule.id))] })),
	updateSchedule: (id, updated) =>
		set((state) => ({ schedules: state.schedules.map((s) => (String(s.id) === String(id) ? { ...s, ...updated } : s)) })),
	deleteSchedule: (id) =>
		set((state) => ({ schedules: state.schedules.filter((s) => String(s.id) !== String(id)) })),

	setSshKeys: (sshKeys) => set({ sshKeys }),
	setDestinations: (destinations) => set({ destinations }),
	setProfile: (profile) => set((state) => ({ profile: { ...state.profile, ...profile } })),

	setMembers: (members) => set({ members }),
	addMember: (member) =>
		set((state) => {
			const mId = String(member.user_id || member.id);
			return { members: [member, ...state.members.filter((m: any) => String(m.user_id || m.id) !== mId)] };
		}),
	updateMember: (id, updated) =>
		set((state) => ({
			members: state.members.map((m: any) =>
				(String(m.user_id || m.id) === String(id) ? { ...m, ...updated } : m)
			),
		})),
	deleteMember: (id) =>
		set((state) => ({
			members: state.members.filter((m: any) => String(m.user_id || m.id) !== String(id)),
		})),

	setInvites: (invites) => set({ invites }),
	addInvite: (invite) =>
		set((state) => ({ invites: [invite, ...state.invites.filter((i: any) => String(i.id) !== String(invite.id))] })),
	deleteInvite: (id) =>
		set((state) => ({ invites: state.invites.filter((i: any) => String(i.id) !== String(id)) })),

	setHydrated: (hydrated) => set({ isHydrated: hydrated }),
	setWsConnected: (connected) => set({ isWsConnected: connected }),
}));
