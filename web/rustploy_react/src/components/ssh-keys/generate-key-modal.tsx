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
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import {$api} from '#/api/query';
import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';
import {Sparkles, RefreshCw} from 'lucide-react';

interface GenerateKeyModalProps {
	isOpen: boolean;
	onClose: () => void;
	onSuccess: () => void;
}

export function GenerateKeyModal({
	isOpen,
	onClose,
	onSuccess,
}: GenerateKeyModalProps) {
	const [name, setName] = useState('');
	const [description, setDescription] = useState('');
	const [keyType, setKeyType] = useState<'ed25519' | 'rsa'>('ed25519');
	const [generating, setGenerating] = useState(false);

	const generateMutation = $api.useMutation('post', '/ssh-keys/generate');

	const handleGenerate = async (e: React.FormEvent) => {
		e.preventDefault();
		if (!name) {
			toast.error('Key Name is required');
			return;
		}

		setGenerating(true);
		try {
			await generateMutation.mutateAsync({
				body: {
					name,
					description: description || undefined,
					key_type: keyType,
				},
			});
			toast.success('New SSH Key pair generated successfully!');
			setName('');
			setDescription('');
			setKeyType('ed25519');
			onSuccess();
			onClose();
		} catch (err: any) {
			toast.error(formatApiError(err));
		} finally {
			setGenerating(false);
		}
	};

	return (
		<Dialog open={isOpen} onOpenChange={open => !open && onClose()}>
			<DialogContent className="sm:max-w-xl md:max-w-2xl w-full bg-card border-border p-6 shadow-xl rounded-xl">
				<DialogHeader className="pb-3 border-b border-border/40">
					<DialogTitle className="text-base font-bold text-foreground flex items-center gap-2">
						<Sparkles className="w-5 h-5 text-primary" />
						Generate New SSH Key Pair
					</DialogTitle>
					<DialogDescription className="text-xs text-muted-foreground">
						Automatically generate a secure ED25519 or RSA SSH key pair directly on the server
					</DialogDescription>
				</DialogHeader>

				<form onSubmit={handleGenerate} className="flex flex-col gap-4 mt-2">
					<div className="flex flex-col gap-1.5">
						<label className="text-xs font-semibold text-foreground">Key Name *</label>
						<Input
							value={name}
							onChange={e => setName(e.target.value)}
							placeholder="e.g. My Dokploy Deploy Key"
							className="h-10 text-xs bg-background border-border rounded-md px-3"
						/>
					</div>

					<div className="flex flex-col gap-1.5">
						<label className="text-xs font-semibold text-foreground">Description (Optional)</label>
						<Input
							value={description}
							onChange={e => setDescription(e.target.value)}
							placeholder="For automated Git deployments"
							className="h-10 text-xs bg-background border-border rounded-md px-3"
						/>
					</div>

					<div className="flex flex-col gap-1.5">
						<label className="text-xs font-semibold text-foreground">Algorithm / Key Type</label>
						<Select value={keyType} onValueChange={(val: any) => setKeyType(val)}>
							<SelectTrigger className="h-10 text-xs font-sans bg-background border-border rounded-md w-full px-3">
								<SelectValue placeholder="Select Algorithm" />
							</SelectTrigger>
							<SelectContent>
								<SelectItem value="ed25519" className="text-xs font-sans">
									ED25519 (Recommended - Faster & Highly Secure)
								</SelectItem>
								<SelectItem value="rsa" className="text-xs font-sans">
									RSA 4096-bit (Legacy Server Compatibility)
								</SelectItem>
							</SelectContent>
						</Select>
					</div>

					<div className="flex items-center justify-end gap-3 pt-4 border-t border-border/40 mt-2">
						<Button type="submit" disabled={generating} className="h-9 text-xs font-semibold px-6 gap-2">
							{generating ? <RefreshCw className="w-4 h-4 animate-spin" /> : <Sparkles className="w-4 h-4" />}
							{generating ? 'Generating...' : 'Generate Key Pair'}
						</Button>
					</div>
				</form>
			</DialogContent>
		</Dialog>
	);
}
