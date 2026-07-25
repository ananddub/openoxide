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
import {Sparkles, RefreshCw, Terminal, Copy, Check, CheckCircle2} from 'lucide-react';

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
	const [generatedKey, setGeneratedKey] = useState<any | null>(null);
	const [copiedCmd, setCopiedCmd] = useState(false);

	const generateMutation = $api.useMutation('post', '/ssh-keys/generate');

	const handleGenerate = async (e: React.FormEvent) => {
		e.preventDefault();
		if (!name) {
			toast.error('Key Name is required');
			return;
		}

		setGenerating(true);
		try {
			const res = await generateMutation.mutateAsync({
				body: {
					name,
					description: description || undefined,
					key_type: keyType,
				},
			});
			toast.success('New SSH Key pair generated successfully!');
			setGeneratedKey(res);
			onSuccess();
		} catch (err: any) {
			toast.error(formatApiError(err));
		} finally {
			setGenerating(false);
		}
	};

	const setupCommand = generatedKey?.public_key
		? `mkdir -p ~/.ssh && echo "${generatedKey.public_key.trim()}" >> ~/.ssh/authorized_keys && chmod 700 ~/.ssh && chmod 600 ~/.ssh/authorized_keys`
		: '';

	const handleCopyCommand = () => {
		if (setupCommand) {
			navigator.clipboard.writeText(setupCommand);
			setCopiedCmd(true);
			toast.success('Server setup command copied!');
			setTimeout(() => setCopiedCmd(false), 2000);
		}
	};

	const handleCloseModal = () => {
		setName('');
		setDescription('');
		setKeyType('ed25519');
		setGeneratedKey(null);
		onClose();
	};

	return (
		<Dialog open={isOpen} onOpenChange={open => !open && handleCloseModal()}>
			<DialogContent className="sm:max-w-xl md:max-w-2xl w-full bg-card border-border p-6 shadow-xl rounded-xl">
				<DialogHeader className="pb-3 border-b border-border/40">
					<DialogTitle className="text-base font-bold text-foreground flex items-center gap-2">
						<Sparkles className="w-5 h-5 text-primary" />
						Generate SSH Key Pair
					</DialogTitle>
					<DialogDescription className="text-xs text-muted-foreground">
						Instantly generate secure ED25519 or RSA SSH key pairs on Rustploy server
					</DialogDescription>
				</DialogHeader>

				{generatedKey ? (
					<div className="flex flex-col gap-4 py-2">
						<div className="p-3 bg-emerald-500/10 border border-emerald-500/30 rounded-xl flex items-center gap-2.5 text-xs text-emerald-500 font-bold">
							<CheckCircle2 className="w-5 h-5 shrink-0" />
							<span>Key Pair "{generatedKey.name}" Generated Successfully!</span>
						</div>

						{setupCommand && (
							<div className="bg-primary/5 p-3.5 rounded-xl border border-primary/20 flex flex-col gap-2 min-w-0 w-full">
								<div className="flex items-center justify-between gap-2 min-w-0">
									<span className="text-xs font-bold text-foreground flex items-center gap-1.5 truncate">
										<Terminal className="w-4 h-4 text-primary shrink-0" />
										Run on Remote Server to Authorize Key:
									</span>
									<Button variant="secondary" size="sm" onClick={handleCopyCommand} className="h-7 text-xs font-semibold gap-1 shrink-0">
										{copiedCmd ? <Check className="w-3.5 h-3.5 text-primary" /> : <Copy className="w-3.5 h-3.5" />}
										{copiedCmd ? 'Copied' : 'Copy Command'}
									</Button>
								</div>
								<div className="p-2.5 bg-background/80 border border-border/60 rounded-lg text-[11px] font-mono text-foreground break-all [word-break:break-all] select-all max-h-28 overflow-y-auto leading-relaxed">
									{setupCommand}
								</div>
							</div>
						)}

						<div className="flex items-center justify-end pt-3 border-t border-border/40">
							<Button onClick={handleCloseModal} className="h-9 text-xs font-semibold px-6">
								Done
							</Button>
						</div>
					</div>
				) : (
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
				)}
			</DialogContent>
		</Dialog>
	);
}
