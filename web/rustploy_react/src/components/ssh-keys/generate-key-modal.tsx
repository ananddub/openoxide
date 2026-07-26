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
	const [saving, setSaving] = useState(false);
	const [generatedKey, setGeneratedKey] = useState<{private_key: string; public_key: string} | null>(null);
	const [copiedCmd, setCopiedCmd] = useState(false);

	const generatePairMutation = $api.useMutation('post', '/ssh-keys/generate-pair');
	const createMutation = $api.useMutation('post', '/ssh-keys');

	const handleGenerate = async (e: React.FormEvent) => {
		e.preventDefault();
		if (!name) {
			toast.error('Key Name is required');
			return;
		}

		setGenerating(true);
		try {
			const res = await generatePairMutation.mutateAsync({
				body: {
					key_type: keyType,
				} as any,
			});
			setGeneratedKey(res as any);
			toast.success('Key Pair generated in memory!');
		} catch (err: any) {
			toast.error(formatApiError(err));
		} finally {
			setGenerating(false);
		}
	};

	const handleSaveKey = async () => {
		if (!generatedKey || !name) return;
		setSaving(true);
		try {
			await createMutation.mutateAsync({
				body: {
					name,
					description: description || undefined,
					public_key: generatedKey.public_key,
					private_key: generatedKey.private_key,
				},
			});
			toast.success(`SSH Key "${name}" saved to database!`);
			onSuccess();
			handleCloseModal();
		} catch (err: any) {
			toast.error(formatApiError(err));
		} finally {
			setSaving(false);
		}
	};

	const setupCommand = generatedKey?.public_key
		? `mkdir -p ~/.ssh && echo "${generatedKey.public_key.trim()}" >> ~/.ssh/authorized_keys && chmod 700 ~/.ssh && chmod 600 ~/.ssh/authorized_keys`
		: '';

	const handleCopyCommand = () => {
		if (setupCommand) {
			navigator.clipboard.writeText(setupCommand);
			setCopiedCmd(true);
			toast.success('Server authorization command copied!');
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
					<DialogTitle className="text-sm font-bold text-foreground flex items-center gap-2">
						<Sparkles className="w-4 h-4 text-primary shrink-0" />
						<span>Generate SSH Key Pair</span>
					</DialogTitle>
					<DialogDescription className="text-xs text-muted-foreground">
						Generate secure ED25519 or RSA SSH key pair in memory
					</DialogDescription>
				</DialogHeader>

				{generatedKey ? (
					<div className="flex flex-col gap-4 py-2">
						<div className="p-3 bg-emerald-500/10 border border-emerald-500/30 rounded-xl flex items-center gap-2.5 text-xs text-emerald-500 font-bold">
							<CheckCircle2 className="w-4 h-4 shrink-0" />
							<span>Key Pair "{name}" Generated in Memory</span>
						</div>

						{setupCommand && (
							<div className="flex flex-col gap-1.5 pt-1">
								<div className="flex items-center justify-between">
									<label className="text-xs font-semibold text-foreground flex items-center gap-1.5">
										<Terminal className="w-3.5 h-3.5 text-primary shrink-0" />
										<span>Authorize Key on Remote Server</span>
									</label>
									<Button variant="outline" size="sm" onClick={handleCopyCommand} className="h-7 text-xs font-medium gap-1.5 px-2.5">
										{copiedCmd ? <Check className="w-3.5 h-3.5 text-emerald-500" /> : <Copy className="w-3.5 h-3.5" />}
										{copiedCmd ? 'Copied' : 'Copy Command'}
									</Button>
								</div>
								<div className="p-2.5 bg-muted/40 border border-border/50 rounded-md text-[11px] font-mono text-muted-foreground break-all select-all leading-relaxed max-h-24 overflow-y-auto">
									{setupCommand}
								</div>
							</div>
						)}

						<div className="flex items-center justify-between pt-3 border-t border-border/40">
							<Button variant="ghost" onClick={handleCloseModal} className="h-8 text-xs px-3">
								Cancel
							</Button>
							<Button onClick={handleSaveKey} disabled={saving} className="h-8 text-xs font-semibold px-5">
								{saving ? 'Saving...' : 'Save Key Pair'}
							</Button>
						</div>
					</div>
				) : (
					<form onSubmit={handleGenerate} className="flex flex-col gap-3.5 mt-2">
						<div className="flex flex-col gap-1.5">
							<label className="text-xs font-semibold text-foreground">Key Name *</label>
							<Input
								value={name}
								onChange={e => setName(e.target.value)}
								placeholder="e.g. Production Key"
								className="h-9 text-xs bg-background border-border rounded-md px-3"
							/>
						</div>

						<div className="flex flex-col gap-1.5">
							<label className="text-xs font-semibold text-foreground">Description (Optional)</label>
							<Input
								value={description}
								onChange={e => setDescription(e.target.value)}
								placeholder="Deploy key for web cluster"
								className="h-9 text-xs bg-background border-border rounded-md px-3"
							/>
						</div>

						<div className="flex flex-col gap-1.5">
							<label className="text-xs font-semibold text-foreground">Algorithm Type</label>
							<Select value={keyType} onValueChange={(val: any) => setKeyType(val)}>
								<SelectTrigger className="h-9 text-xs bg-background border-border rounded-md px-3">
									<SelectValue placeholder="Select Algorithm" />
								</SelectTrigger>
								<SelectContent>
									<SelectItem value="ed25519">ED25519 (Recommended - High Security & Fast)</SelectItem>
									<SelectItem value="rsa">RSA 4096-bit (Legacy Compatibility)</SelectItem>
								</SelectContent>
							</Select>
						</div>

						<div className="flex items-center justify-end gap-2 pt-3 border-t border-border/50">
							<Button type="button" variant="ghost" onClick={handleCloseModal} className="h-8 text-xs px-3">
								Cancel
							</Button>
							<Button type="submit" disabled={generating} className="h-8 text-xs font-medium gap-1.5 px-4">
								{generating ? <RefreshCw className="w-3.5 h-3.5 animate-spin" /> : <Sparkles className="w-3.5 h-3.5" />}
								{generating ? 'Generating...' : 'Generate Key Pair'}
							</Button>
						</div>
					</form>
				)}
			</DialogContent>
		</Dialog>
	);
}
