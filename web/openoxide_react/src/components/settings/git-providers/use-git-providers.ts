import {useState} from 'react';
import {toast} from 'sonner';
import {$api} from '#/api/query';
import {getApiBaseUrl} from '#/api/client';
import {formatApiError} from '#/api/utils';
import {
	type GitProviderKind,
	type GitProviderFormState,
	INITIAL_GIT_PROVIDER_FORM,
} from './git-provider-types';

export function useGitProviders() {
	const list = $api.useQuery('get', '/git-providers' as any, {} as any);
	const create = $api.useMutation('post', '/git-providers/{kind}' as any);
	const update = $api.useMutation('put', '/git-providers/{id}/{kind}' as any);
	const remove = $api.useMutation('delete', '/git-providers/{id}' as any);
	const test = $api.useMutation('post', '/git-providers/{id}/test' as any);

	const [open, setOpen] = useState(false);
	const [manifestOpen, setManifestOpen] = useState(false);
	const [editing, setEditing] = useState<any>(null);
	const [kind, setKind] = useState<GitProviderKind>('github');
	const [busy, setBusy] = useState(false);
	const [form, setForm] = useState<GitProviderFormState>(INITIAL_GIT_PROVIDER_FORM);

	const providers = (list.data?.data || list.data || []) as any[];

	const setField = (key: string, value: string) => {
		setForm((prev) => ({...prev, [key]: value}));
	};

	const openCreate = (providerKind: GitProviderKind = 'github') => {
		setEditing(null);
		setKind(providerKind);
		if (providerKind === 'github') {
			setManifestOpen(true);
			return;
		}
		setForm({
			...INITIAL_GIT_PROVIDER_FORM,
			url: providerKind === 'gitlab' ? 'https://gitlab.com' : 'https://gitea.com',
			redirect_uri: `${window.location.origin}/api/git-providers/${providerKind}/oauth/callback`,
		});
		setOpen(true);
	};

	const openEdit = (provider: any) => {
		setEditing(provider);
		setKind(provider.provider_type as GitProviderKind);
		setForm({
			...INITIAL_GIT_PROVIDER_FORM,
			name: provider.name,
			shared: String(provider.shared),
			...provider.config,
		});
		setOpen(true);
	};

	const buildPayload = () => {
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
	};

	const authHeaders = () => {
		const session = JSON.parse(localStorage.getItem('openoxide-auth-session') || '{}');
		const headers: Record<string, string> = {};
		if (session?.tokens?.access_token) headers.Authorization = `Bearer ${session.tokens.access_token}`;
		const org = localStorage.getItem('openoxide-active-organization-id');
		if (org) headers['X-Organization-Id'] = org;
		return headers;
	};

	const handleAuthorize = async (id: number) => {
		try {
			const response = await fetch(`${getApiBaseUrl()}/git-providers/${id}/authorize`, {
				headers: authHeaders(),
			});
			if (!response.ok) throw new Error('Authorization could not be started');
			const data = await response.json();
			if (!data.url) throw new Error('Provider returned no authorization URL');
			window.open(data.url, '_blank', 'noopener,noreferrer');
		} catch {
			toast.error('Could not start authorization');
		}
	};

	const handleTest = async (id: number) => {
		try {
			await test.mutateAsync({params: {path: {id}}} as any);
			toast.success('Connection successful');
		} catch (e) {
			toast.error(formatApiError(e, 'Connection failed'));
		}
	};

	const handleDelete = async (id: number) => {
		if (!confirm('Delete this Git provider?')) return;
		try {
			await remove.mutateAsync({params: {path: {id}}} as any);
			toast.success('Git provider deleted');
			await list.refetch();
		} catch (e) {
			toast.error(formatApiError(e, 'Failed to delete Git provider'));
		}
	};

	const save = async () => {
		if (!form.name.trim()) return toast.error('Provider name is required');
		setBusy(true);
		try {
			let result: any;
			if (editing) {
				result = await update.mutateAsync({params: {path: {id: editing.id, kind}}, body: buildPayload()} as any);
			} else {
				result = await create.mutateAsync({params: {path: {kind}}, body: buildPayload()} as any);
			}
			toast.success(editing ? 'Git provider updated' : 'Git provider created');
			setOpen(false);
			await list.refetch();
			const providerId = editing?.id || result?.data?.provider?.id || result?.provider?.id;
			if (!editing && providerId && (kind === 'gitlab' || kind === 'gitea')) {
				await handleAuthorize(providerId);
			}
		} catch (e) {
			toast.error(formatApiError(e, 'Failed to save Git provider'));
		} finally {
			setBusy(false);
		}
	};

	return {
		providers,
		open,
		manifestOpen,
		editing,
		kind,
		busy,
		form,
		setOpen,
		setManifestOpen,
		setField,
		openCreate,
		openEdit,
		handleAuthorize,
		handleTest,
		handleDelete,
		save,
	};
}
