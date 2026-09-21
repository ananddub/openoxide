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
			const response = await gitProviderClient.authorize({id: BigInt(id)});
			const data = response.payload ? JSON.parse(response.payload) : null;
			if (!data?.url)
				throw new Error('Provider returned no authorization URL');
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
			toast.error(formatApiError(e));
		}
	};

	const handleDelete = async (id: number) => {
		if (!confirm('Delete this Git provider?')) return;
		try {
			await remove.mutateAsync({params: {path: {id}}} as any);
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
			let result: any;
			if (editing) {
				result = await update.mutateAsync({
					params: {path: {id: editing.id, kind}},
					body: buildGitProviderPayload(kind, form),
				} as any);
			} else {
				result = await create.mutateAsync({
					params: {path: {kind}},
					body: buildGitProviderPayload(kind, form),
				} as any);
			}
			toast.success(
				editing ? 'Git provider updated' : 'Git provider created',
			);
			setOpen(false);
			await list.refetch();
			const providerId =
				editing?.id || result?.data?.provider?.id || result?.provider?.id;
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
