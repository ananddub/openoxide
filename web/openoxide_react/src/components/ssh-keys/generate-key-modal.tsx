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
import {
	Sparkles,
	RefreshCw,
	Terminal,
	Copy,
	Check,
	CheckCircle2,
} from 'lucide-react';

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
	const [generatedKey, setGeneratedKey] = useState<{
		private_key: string;
		public_key: string;
	} | null>(null);
	const [copiedCmd, setCopiedCmd] = useState(false);

	const generatePairMutation = $api.useMutation(
		'post',
		'/ssh-keys/generate-pair' as any,
	);
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
		} catch (err: unknown) {
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
		} catch (err: unknown) {
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
		<Dialog
			open={isOpen}
			onOpenChange={open => !open && handleCloseModal()}>
			<DialogContent className="w-full min-w-0 rounded-xl border-border bg-card p-6 shadow-xl sm:max-w-lg">
				<DialogHeader className="border-b border-border/40 pb-3">
					<DialogTitle className="flex items-center gap-2 text-sm font-bold text-foreground">
						<Sparkles className="h-4 w-4 shrink-0 text-primary" />
						<span>Generate SSH Key Pair</span>
					</DialogTitle>
					<DialogDescription className="text-xs text-muted-foreground">
						Generate secure ED25519 or RSA SSH key pair in memory
					</DialogDescription>
				</DialogHeader>

				{generatedKey ? (
					<div className="flex flex-col gap-4 py-2">
						<div className="flex items-center gap-2.5 rounded-xl border border-emerald-500/30 bg-emerald-500/10 p-3 text-xs font-bold text-emerald-500">
							<CheckCircle2 className="h-4 w-4 shrink-0" />
							<span>Key Pair "{name}" Generated in Memory</span>
						</div>

						{setupCommand && (
							<div className="flex flex-col gap-1.5 pt-1">
								<div className="flex items-center justify-between">
									<label className="flex items-center gap-1.5 text-xs font-semibold text-foreground">
										<Terminal className="h-3.5 w-3.5 shrink-0 text-primary" />
										<span>Authorize Key on Remote Server</span>
									</label>
									<Button
										variant="outline"
										size="sm"
										onClick={handleCopyCommand}
										className="h-7 gap-1.5 px-2.5 text-xs font-medium">
										{copiedCmd ? (
											<Check className="h-3.5 w-3.5 text-emerald-500" />
										) : (
											<Copy className="h-3.5 w-3.5" />
										)}
										{copiedCmd ? 'Copied' : 'Copy Command'}
									</Button>
								</div>
								<div className="max-h-24 overflow-y-auto rounded-md border border-border/50 bg-muted/40 p-2.5 font-mono text-[11px] leading-relaxed break-all text-muted-foreground select-all">
									{setupCommand}
								</div>
							</div>
						)}

						<div className="flex items-center justify-end border-t border-border/40 pt-3">
							<Button
								onClick={handleSaveKey}
								disabled={saving}
								className="h-9 w-full bg-primary px-6 text-xs font-bold text-primary-foreground shadow-md hover:bg-primary/90 sm:w-auto">
								{saving ? 'Saving...' : 'Save Key Pair'}
							</Button>
						</div>
					</div>
				) : (
					<form
						onSubmit={handleGenerate}
						className="mt-2 flex flex-col gap-3.5">
						<div className="flex flex-col gap-1.5">
							<label className="text-xs font-semibold text-foreground">
								Key Name *
							</label>
							<Input
								value={name}
								onChange={e => setName(e.target.value)}
								placeholder="e.g. Production Key"
								className="h-9 rounded-md border-border bg-background px-3 text-xs"
							/>
						</div>

						<div className="flex flex-col gap-1.5">
							<label className="text-xs font-semibold text-foreground">
								Description (Optional)
							</label>
							<Input
								value={description}
								onChange={e => setDescription(e.target.value)}
								placeholder="Deploy key for web cluster"
								className="h-9 rounded-md border-border bg-background px-3 text-xs"
							/>
						</div>

						<div className="flex flex-col gap-1.5">
							<label className="text-xs font-semibold text-foreground">
								Algorithm Type
							</label>
							<Select
								value={keyType}
								onValueChange={val =>
									val && setKeyType(val as 'ed25519' | 'rsa')
								}>
								<SelectTrigger className="h-9 rounded-md border-border bg-background px-3 text-xs">
									<SelectValue placeholder="Select Algorithm" />
								</SelectTrigger>
								<SelectContent>
									<SelectItem value="ed25519">
										ED25519 (Recommended - High Security & Fast)
									</SelectItem>
									<SelectItem value="rsa">
										RSA 4096-bit (Legacy Compatibility)
									</SelectItem>
								</SelectContent>
							</Select>
						</div>

						<div className="flex items-center justify-end border-t border-border/50 pt-3">
							<Button
								type="submit"
								disabled={generating}
								className="h-9 w-full gap-1.5 bg-primary px-6 text-xs font-bold text-primary-foreground shadow-md hover:bg-primary/90 sm:w-auto">
								{generating ? (
									<RefreshCw className="h-3.5 w-3.5 animate-spin" />
								) : (
									<Sparkles className="h-3.5 w-3.5" />
								)}
								{generating ? 'Generating...' : 'Generate Key Pair'}
							</Button>
						</div>
					</form>
				)}
			</DialogContent>
		</Dialog>
	);
}
