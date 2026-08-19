export type GitProviderKind = 'github' | 'gitlab' | 'gitea' | 'bitbucket';

export const GIT_PROVIDER_LABELS: Record<GitProviderKind, string> = {
	github: 'GitHub App',
	gitlab: 'GitLab',
	gitea: 'Gitea',
	bitbucket: 'Bitbucket',
};

export interface GitProviderFormState {
	name: string;
	url: string;
	internal_url: string;
	app_name: string;
	app_id: string;
	client_id: string;
	client_secret: string;
	installation_id: string;
	private_key: string;
	application_id: string;
	redirect_uri: string;
	secret: string;
	access_token: string;
	refresh_token: string;
	group_name: string;
	scopes: string;
	username: string;
	email: string;
	app_password: string;
	api_token: string;
	workspace: string;
	shared: string;
}

export const INITIAL_GIT_PROVIDER_FORM: GitProviderFormState = {
	name: '',
	url: '',
	internal_url: '',
	app_name: '',
	app_id: '',
	client_id: '',
	client_secret: '',
	installation_id: '',
	private_key: '',
	application_id: '',
	redirect_uri: '',
	secret: '',
	access_token: '',
	refresh_token: '',
	group_name: '',
	scopes: '',
	username: '',
	email: '',
	app_password: '',
	api_token: '',
	workspace: '',
	shared: 'true',
};

export function getGithubCallbackBaseUrl(): string {
	const configured = import.meta.env.VITE_API_URL;
	if (configured && /^https?:\/\//i.test(configured)) {
		return configured.replace(/\/$/, '');
	}
	if (typeof window === 'undefined') return 'http://127.0.0.1:4000';
	const host = window.location.hostname === 'localhost' ? '127.0.0.1' : window.location.hostname;
	return `${window.location.protocol}//${host}:4000`;
}
