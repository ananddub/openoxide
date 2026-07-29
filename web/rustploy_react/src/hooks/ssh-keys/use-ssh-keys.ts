import {useState} from 'react';
import {useQueryClient} from '@tanstack/react-query';
import {$api} from '#/api/query';
import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';

export function useCreateSshKey(onClose: () => void, onSuccess?: () => void) {
	const queryClient = useQueryClient();
	const [name, setName] = useState('');
	const [description, setDescription] = useState('');
	const [privateKey, setPrivateKey] = useState('');
	const [publicKey, setPublicKey] = useState('');
	const [submitting, setSubmitting] = useState(false);
	const [generatingType, setGeneratingType] = useState<'ed25519' | 'rsa' | null>(null);

	const createMutation = $api.useMutation('post', '/ssh-keys');
	const generatePairMutation = $api.useMutation('post', '/ssh-keys/generate-pair');

	const handleGeneratePair = async (type: 'ed25519' | 'rsa') => {
		setGeneratingType(type);
		try {
			const res = await generatePairMutation.mutateAsync({
				body: {key_type: type},
			});
			setPublicKey(res.public_key || '');
			setPrivateKey(res.private_key || '');
			toast.success(`Generated ${type.toUpperCase()} key pair! Fill in details and click Save.`);
		} catch (err: unknown) {
			toast.error(formatApiError(err));
		} finally {
			setGeneratingType(null);
		}
	};

	const handleSubmit = async (e: React.FormEvent) => {
		e.preventDefault();
		if (!name.trim()) {
			toast.error('Please enter a key name');
			return;
		}
		if (!privateKey.trim()) {
			toast.error('Please provide or generate a private key');
			return;
		}

		setSubmitting(true);
		try {
			await createMutation.mutateAsync({
				body: {
					name: name.trim(),
					description: description.trim() || undefined,
					private_key: privateKey.trim(),
					public_key: publicKey.trim() || undefined,
				} as any,
			});
			toast.success('SSH Key created successfully');
			queryClient.invalidateQueries({queryKey: ['get', '/ssh-keys']});
			onSuccess?.();
			onClose();
		} catch (err: unknown) {
			toast.error(formatApiError(err));
		} finally {
			setSubmitting(false);
		}
	};

	return {
		name,
		setName,
		description,
		setDescription,
		privateKey,
		setPrivateKey,
		publicKey,
		setPublicKey,
		submitting,
		generatingType,
		handleGeneratePair,
		handleSubmit,
	};
}

export function useDeleteSshKey(keyId: number, onClose: () => void) {
	const queryClient = useQueryClient();
	const [deleting, setDeleting] = useState(false);
	const deleteMutation = $api.useMutation('delete', '/ssh-keys/{id}');

	const handleDelete = async () => {
		setDeleting(true);
		try {
			await deleteMutation.mutateAsync({
				params: {path: {id: keyId}},
			});
			toast.success('SSH key deleted');
			queryClient.invalidateQueries({queryKey: ['get', '/ssh-keys']});
			onClose();
		} catch (err: unknown) {
			toast.error(formatApiError(err));
		} finally {
			setDeleting(false);
		}
	};

	return {deleting, handleDelete};
}
