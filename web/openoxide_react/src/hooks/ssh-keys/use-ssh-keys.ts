import {useState} from 'react';
import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';
import {
	useSshKeyCreate,
	useSshKeyGeneratePair,
	useSshKeyDelete,
} from '#/api/live';

export function useCreateSshKey(
	onClose: () => void,
	onSuccess?: () => void,
) {
	const [name, setName] = useState('');
	const [description, setDescription] = useState('');
	const [privateKey, setPrivateKey] = useState('');
	const [publicKey, setPublicKey] = useState('');
	const [submitting, setSubmitting] = useState(false);
	const [generatingType, setGeneratingType] = useState<
		'ed25519' | 'rsa' | null
	>(null);

	const createMutation = useSshKeyCreate();
	const generatePairMutation = useSshKeyGeneratePair();

	const handleGeneratePair = async (type: 'ed25519' | 'rsa') => {
		setGeneratingType(type);
		try {
			const res: any = await generatePairMutation.mutateAsync({
				key_type: type,
			});
			const pair = res?.payload ?? res;
			setPublicKey(pair?.public_key || '');
			setPrivateKey(pair?.private_key || '');
			toast.success(
				`Generated ${type.toUpperCase()} key pair! Fill in details and click Save.`,
			);
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
				name: name.trim(),
				description: description.trim() || undefined,
				private_key: privateKey.trim(),
				public_key: publicKey.trim() || undefined,
			});
			toast.success('SSH Key created successfully');
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
	const [deleting, setDeleting] = useState(false);
	const deleteMutation = useSshKeyDelete();

	const handleDelete = async () => {
		setDeleting(true);
		try {
			await deleteMutation.mutateAsync(keyId);
			toast.success('SSH key deleted');
			onClose();
		} catch (err: unknown) {
			toast.error(formatApiError(err));
		} finally {
			setDeleting(false);
		}
	};

	return {deleting, handleDelete};
}
