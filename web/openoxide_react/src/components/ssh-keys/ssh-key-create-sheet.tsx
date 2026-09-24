import {useState} from 'react';
import {Loader2, RefreshCw, Sparkles} from 'lucide-react';
import {
	Sheet,
	SheetContent,
	SheetDescription,
	SheetFooter,
	SheetHeader,
	SheetTitle,
} from '#/components/ui/sheet';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {Label} from '#/components/ui/label';
import {Textarea} from '#/components/ui/textarea';
import {toast} from 'sonner';
import {useSshKeyCreate, useSshKeyGeneratePair} from '#/api/live/ssh_keys';
import {AuthorizeCommandBlock} from '#/components/ssh-keys/authorize-command-block';
import {SshKeyDownloadButtons} from '#/components/ssh-keys/ssh-key-download-buttons';

interface SshKeyCreateSheetProps {
	isOpen: boolean;
	onOpenChange: (open: boolean) => void;
}

/** New SSH Key — the same Geist sheet grammar as "New Project": right-side sheet, scrollable form, single Create action in the footer. Generating a pair fills the textareas in place; nothing leaves this dialog until Create is pressed. */
export function SshKeyCreateSheet({
	isOpen,
	onOpenChange,
}: SshKeyCreateSheetProps) {
	const [name, setName] = useState('');
	const [description, setDescription] = useState('');
	const [privateKey, setPrivateKey] = useState('');
	const [publicKey, setPublicKey] = useState('');
	const [generating, setGenerating] = useState(false);
	const [submitting, setSubmitting] = useState(false);

	const generatePairMutation = useSshKeyGeneratePair();
	const createMutation = useSshKeyCreate();

	const reset = () => {
		setName('');
		setDescription('');
		setPrivateKey('');
		setPublicKey('');
	};

	const handleGeneratePair = async (type: 'ed25519' | 'rsa') => {
		setGenerating(true);
		try {
			const res = await generatePairMutation.mutateAsync({
				key_type: type,
			});
			// The typed response carries the fresh key material directly.
			if (res.privateKey && res.publicKey) {
				setPrivateKey(res.privateKey);
				setPublicKey(res.publicKey);
				if (!name) {
					setName(
						`Auto-${type.toUpperCase()}-${Date.now().toString().slice(-4)}`,
					);
				}
				toast.success(
					`Generated ${type.toUpperCase()} key pair — press Create when ready.`,
				);
			}
		} catch (err: unknown) {
			toast.error(
				err instanceof Error
					? err.message
					: 'Failed to generate SSH key pair',
			);
		} finally {
			setGenerating(false);
		}
	};

	const handleSubmit = async (e: React.FormEvent) => {
		e.preventDefault();
		if (!name.trim() || !privateKey.trim()) {
			toast.error('Name and Private Key are required');
			return;
		}
		setSubmitting(true);
		try {
			await createMutation.mutateAsync({
				name: name.trim(),
				description: description.trim() || undefined,
				private_key: privateKey.trim(),
				public_key: publicKey.trim() || '',
			});
			toast.success('SSH Key created successfully');
			reset();
			onOpenChange(false);
		} catch (err: any) {
			toast.error(err?.message || 'Failed to create SSH Key');
		} finally {
			setSubmitting(false);
		}
	};

	const formattedFileName =
		name.trim().toLowerCase().replace(/\s+/g, '_') || 'id_rsa';

	return (
		<Sheet
			open={isOpen}
			onOpenChange={open => {
				onOpenChange(open);
				if (!open) reset();
			}}>
			<SheetContent side="right">
				<SheetHeader>
					<SheetTitle>New SSH Key</SheetTitle>
					<SheetDescription>
						Add an existing key pair or generate a new one.
					</SheetDescription>
				</SheetHeader>

				<form
					id="ssh-key-form"
					onSubmit={handleSubmit}
					className="flex flex-1 flex-col gap-5 overflow-y-auto px-6 py-6">
					{/* Name */}
					<div className="flex flex-col gap-1.5">
						<Label
							htmlFor="ssh-key-name"
							className="text-[13px] font-medium text-foreground">
							Name
						</Label>
						<Input
							id="ssh-key-name"
							placeholder="e.g. Production Server Key"
							value={name}
							onChange={e => setName(e.target.value)}
							disabled={submitting}
							required
							autoFocus
							className="bg-background"
						/>
					</div>

					{/* Description */}
					<div className="flex flex-col gap-1.5">
						<Label
							htmlFor="ssh-key-description"
							className="text-[13px] font-medium text-foreground">
							Description
						</Label>
						<Input
							id="ssh-key-description"
							placeholder="Optional note about where this key is used"
							value={description}
							onChange={e => setDescription(e.target.value)}
							disabled={submitting}
							className="bg-background"
						/>
					</div>

					{/* Generate pair */}
					<div className="grid grid-cols-2 gap-2.5">
						{(['ed25519', 'rsa'] as const).map(type => (
							<Button
								key={type}
								type="button"
								variant="outline"
								onClick={() => handleGeneratePair(type)}
								disabled={generating || submitting}
								className="font-medium">
								{generating ? (
									<RefreshCw className="size-3.5 animate-spin" />
								) : (
									<Sparkles className="size-3.5" />
								)}
								{type === 'ed25519'
									? 'Generate ED25519'
									: 'Generate RSA 4096'}
							</Button>
						))}
					</div>

					{/* Private key */}
					<div className="flex flex-col gap-1.5">
						<Label
							htmlFor="ssh-key-private"
							className="text-[13px] font-medium text-foreground">
							Private Key (PEM / OpenSSH)
						</Label>
						<Textarea
							id="ssh-key-private"
							placeholder="-----BEGIN OPENSSH PRIVATE KEY-----..."
							value={privateKey}
							onChange={e => setPrivateKey(e.target.value)}
							disabled={submitting}
							required
							rows={5}
							className="max-h-40 min-h-[96px] w-full overflow-y-auto rounded-lg bg-card font-mono text-xs break-all placeholder:text-muted-foreground focus-visible:ring-1 focus-visible:ring-ring"
						/>
					</div>

					{/* Public key */}
					<div className="flex flex-col gap-1.5">
						<Label
							htmlFor="ssh-key-public"
							className="text-[13px] font-medium text-foreground">
							Public Key (Optional)
						</Label>
						<Textarea
							id="ssh-key-public"
							placeholder="ssh-ed25519 AAAAC3NzaC1lZDI1NTE5..."
							value={publicKey}
							onChange={e => setPublicKey(e.target.value)}
							disabled={submitting}
							rows={3}
							className="max-h-32 min-h-[64px] w-full overflow-y-auto rounded-lg bg-card font-mono text-xs break-all placeholder:text-muted-foreground focus-visible:ring-1 focus-visible:ring-ring"
						/>
					</div>

					<AuthorizeCommandBlock publicKey={publicKey} />

					<SshKeyDownloadButtons
						fileName={formattedFileName}
						publicKey={publicKey}
						privateKey={privateKey}
					/>
				</form>

				<SheetFooter>
					<Button
						type="submit"
						form="ssh-key-form"
						disabled={submitting || !name.trim() || !privateKey.trim()}
						className="h-10 gap-1.5 px-5">
						{submitting ? (
							<>
								<Loader2 className="size-3.5 animate-spin" />
								Saving…
							</>
						) : (
							<span>Save</span>
						)}
					</Button>
				</SheetFooter>
			</SheetContent>
		</Sheet>
	);
}
