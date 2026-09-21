import type {
	GitProviderKind,
	GitProviderFormState,
} from './git-provider-types';

/** Shape the dialog form into the create/update API payload per kind. */
export function buildGitProviderPayload(
	kind: GitProviderKind,
	form: GitProviderFormState,
) {
	const base = {name: form.name.trim(), shared: form.shared === 'true'};
	if (kind === 'github') {
		return {
			provider: base,
			app_name: form.app_name || undefined,
			app_id: form.app_id ? Number(form.app_id) : undefined,
			client_id: form.client_id || undefined,
			client_secret: form.client_secret || undefined,
			installation_id: form.installation_id || undefined,
			private_key: form.private_key || undefined,
		};
	}
	if (kind === 'gitlab') {
		return {
			provider: base,
			url: form.url,
			internal_url: form.internal_url || undefined,
			application_id: form.application_id || undefined,
			redirect_uri: form.redirect_uri || undefined,
			secret: form.secret || undefined,
			access_token: form.access_token || undefined,
			refresh_token: form.refresh_token || undefined,
			group_name: form.group_name || undefined,
		};
	}
	if (kind === 'gitea') {
		return {
			provider: base,
			url: form.url,
			internal_url: form.internal_url || undefined,
			redirect_uri: form.redirect_uri || undefined,
			client_id: form.client_id || undefined,
			client_secret: form.client_secret || undefined,
			access_token: form.access_token || undefined,
			refresh_token: form.refresh_token || undefined,
			scopes: form.scopes || undefined,
		};
	}
	return {
		provider: base,
		username: form.username || undefined,
		email: form.email || undefined,
		app_password: form.app_password || undefined,
		api_token: form.api_token || undefined,
		workspace: form.workspace || undefined,
	};
}
