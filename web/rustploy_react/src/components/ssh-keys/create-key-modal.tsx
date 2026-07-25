import {useState} from 'react';
import {
	Dialog,
	DialogContent,
	DialogDescription,
	DialogHeader,
	DialogTitle,
} from '#/components/ui/dialog';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {Textarea} from '#/components/ui/textarea';
import {$api} from '#/api/query';
import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';
import {Key} from 'lucide-react';

interface CreateKeyModalProps {
	isOpen: boolean;
	onClose: () => void;
	onSuccess: () => void;
}

export function CreateKeyModal({
	isOpen,
	onClose,
	onSuccess,
}: CreateKeyModalProps) {
	const [name, setName] = useState('');
	const [description, setDescription] = useState('');
	const [privateKey, setPrivateKey] = useState('');
	const [publicKey, setPublicKey] = useState('');
	const [submitting, setSubmitting] = useState(false);

	const createMutation = $api.useMutation('post', '/ssh-keys');

	const handleSubmit = async (e: React.FormEvent) => {
		e.preventDefault();
		if (!name || !privateKey || !publicKey) {
			toast.error('Name, Private Key, and Public Key are required');
			return;
		}

		setSubmitting(true);
		try {
			await createMutation.mutateAsync({
				body: {
					name,
					description: description || undefined,
					private_key: privateKey,
					public_key: publicKey,
				},
			});
			toast.success('SSH Key added successfully');
			setName('');
			setDescription('');
			setPrivateKey('');
			setPublicKey('');
			onSuccess();
			onClose();
		} catch (err: any) {
			toast.error(formatApiError(err));
		} finally {
			setSubmitting(false);
		}
	};

	return (
		<Dialog open={isOpen} onOpenChange={open => !open && onClose()}>
			<DialogContent className="sm:max-w-2xl md:max-w-3xl w-full bg-card border-border p-6 shadow-xl rounded-xl">
				<DialogHeader className="pb-3 border-b border-border/40">
					<DialogTitle className="text-base font-bold text-foreground flex items-center gap-2">
						<Key className="w-5 h-5 text-primary" />
						Add Existing SSH Key
					</DialogTitle>
					<DialogDescription className="text-xs text-muted-foreground">
						Paste your existing SSH private and public key pair credentials
					</DialogDescription>
				</DialogHeader>

				<form onSubmit={handleSubmit} className="flex flex-col gap-4 mt-2">
					<div className="flex flex-col gap-1.5">
						<label className="text-xs font-semibold text-foreground">Key Name *</label>
						<Input
							value={name}
							onChange={e => setName(e.target.value)}
							placeholder="e.g. Production Server Key"
							className="h-10 text-xs bg-background border-border rounded-md px-3"
						/>
					</div>

					<div className="flex flex-col gap-1.5">
						<label className="text-xs font-semibold text-foreground">Description (Optional)</label>
						<Input
							value={description}
							onChange={e => setDescription(e.target.value)}
							placeholder="Deploy key for web cluster"
							className="h-10 text-xs bg-background border-border rounded-md px-3"
						/>
					</div>

					<div className="flex flex-col gap-1.5">
						<label className="text-xs font-semibold text-foreground">Public Key *</label>
						<Textarea
							value={publicKey}
							onChange={e => setPublicKey(e.target.value)}
							placeholder="ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAI... user@server"
							className="h-20 text-xs font-mono bg-background border-border rounded-md p-3 resize-none"
						/>
					</div>

					<div className="flex flex-col gap-1.5">
						<label className="text-xs font-semibold text-foreground">Private Key *</label>
						<Textarea
							value={privateKey}
							onChange={e => setPrivateKey(e.target.value)}
							placeholder="-----BEGIN OPENSSH PRIVATE KEY-----&#10;...&#10;-----END OPENSSH PRIVATE KEY-----"
							className="h-32 text-xs font-mono bg-background border-border rounded-md p-3 resize-none"
						/>
					</div>

					<div className="flex items-center justify-end gap-3 pt-4 border-t border-border/40 mt-1">
						<Button type="submit" disabled={submitting} className="h-9 text-xs font-semibold px-6">
							{submitting ? 'Saving...' : 'Add SSH Key'}
						</Button>
					</div>
				</form>
			</DialogContent>
		</Dialog>
	);
}
