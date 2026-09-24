import {useState} from 'react';
import {toast} from 'sonner';
import {
	useGitProviderCreate,
	useGitProviderDelete,
	useGitProviderList,
	useGitProviderTest,
	useGitProviderUpdate,
} from '#/api/live';
import {createServiceClient} from '#/api/live/query';
import {GitProviderService} from '#/gen/connect/openoxide/api/v1/git_provider_pb';
import {formatApiError} from '#/api/utils';

const gitProviderClient = createServiceClient(GitProviderService);
import {
	type GitProviderKind,
	type GitProviderFormState,
	INITIAL_GIT_PROVIDER_FORM,
} from './git-provider-types';
import {buildGitProviderPayload} from './git-provider-payload';
import type {
	GitProviderInput,
	GitProviderPayload,
} from '#/api/live/git_providers';

export function useGitProviders() {
	const list = useGitProviderList();
	const create = useGitProviderCreate();
	const update = useGitProviderUpdate();
	const remove = useGitProviderDelete();
	const test = useGitProviderTest();

	const [open, setOpen] = useState(false);
	const [manifestOpen, setManifestOpen] = useState(false);
	const [editing, setEditing] = useState<any>(null);
	const [kind, setKind] = useState<GitProviderKind>('github');
	const [busy, setBusy] = useState(false);
	const [form, setForm] = useState<GitProviderFormState>(
		INITIAL_GIT_PROVIDER_FORM,
	);

	const providers = (list.data || []) as any[];

	const setField = (key: string, value: string) => {
		setForm(prev => ({...prev, [key]: value}));
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
			url:
				providerKind === 'gitlab'
					? 'https://gitlab.com'
					: 'https://gitea.com',
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

	const handleAuthorize = async (id: number) => {
		try {
			// Typed authorize request: providerId in, auth URL straight back.
			const response = await gitProviderClient.authorize({
				providerId: BigInt(id),
			});
			if (!response.url)
				throw new Error('Provider returned no authorization URL');
			window.open(response.url, '_blank', 'noopener,noreferrer');
		} catch {
			toast.error('Could not start authorization');
		}
	};

	const handleTest = async (id: number) => {
		try {
			await test.mutateAsync(id);
			toast.success('Connection successful');
		} catch (e) {
			toast.error(formatApiError(e));
		}
	};

	const handleDelete = async (id: number) => {
		try {
			await remove.mutateAsync(id);
			toast.success('Git provider deleted');
			await list.refetch();
		} catch (e) {
			toast.error(formatApiError(e));
		}
	};

	const save = async () => {
		if (!form.name.trim()) {
			toast.error('Provider name is required');
			return;
		}
		setBusy(true);
		try {
			// The builder returns `{provider: {name, shared}, ...fields}`;
			// flatten it into the typed create/update input.
			const {provider: base, ...fields} = buildGitProviderPayload(
				kind,
				form,
			);
			const input: GitProviderInput = {
				kind,
				name: base.name,
				shared: base.shared,
				...fields,
			};
			let result: GitProviderPayload | undefined;
			if (editing) {
				result = await update.mutateAsync({...input, id: editing.id});
			} else {
				result = await create.mutateAsync(input);
			}
			toast.success(
				editing ? 'Git provider updated' : 'Git provider created',
			);
			setOpen(false);
			await list.refetch();
			const providerId = editing?.id || result?.id;
			if (
				!editing &&
				providerId &&
				(kind === 'gitlab' || kind === 'gitea')
			) {
				await handleAuthorize(providerId);
			}
		} catch (e) {
			toast.error(formatApiError(e));
		} finally {
			setBusy(false);
		}
	};

	return {
		providers,
		isLoading: list.isLoading,
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
