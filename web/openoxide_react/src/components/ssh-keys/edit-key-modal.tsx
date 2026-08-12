import type {SshKeyResponse} from '#/types/api-helpers';

interface EditKeyModalProps {
	isOpen: boolean;
	sshKey: SshKeyResponse | null;
	onClose: () => void;
	onSuccess?: () => void;
}

export function EditKeyModal(_props: EditKeyModalProps) {
	return null;
}
