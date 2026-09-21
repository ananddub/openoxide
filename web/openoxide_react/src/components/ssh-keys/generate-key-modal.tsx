import {useSshKeyCreate, useSshKeyGeneratePair} from '#/api/live/ssh_keys';
import {useState} from 'react';
import {
	Sheet,
	SheetContent,
	SheetDescription,
	SheetHeader,
	SheetTitle,
} from '#/components/ui/sheet';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';
import {Sparkles, RefreshCw} from 'lucide-react';
import {GenerateKeyResult} from '#/components/ssh-keys/generate-key-result';

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

	const generatePairMutation = useSshKeyGeneratePair();
	const createMutation = useSshKeyCreate();

	const handleGenerate = async (e: React.FormEvent) => {
		e.preventDefault();
		if (!name) {
			toast.error('Key Name is required');
			return;
		}

		setGenerating(true);
		try {
			const res: any = await generatePairMutation.mutateAsync({
				key_type: keyType,
			});
			// ResourceResponse wraps the pair as a JSON string payload.
			const generated =
				typeof res?.payload === 'string'
					? JSON.parse(res.payload)
					: (res?.payload ?? res);
			setGeneratedKey(generated);
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
				name,
				description: description || undefined,
				public_key: generatedKey.public_key,
				private_key: generatedKey.private_key,
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

	const handleCloseModal = () => {
		setName('');
		setDescription('');
		setKeyType('ed25519');
		setGeneratedKey(null);
		onClose();
	};

	return (
		<Sheet
			open={isOpen}
			onOpenChange={open => !open && handleCloseModal()}>
			<SheetContent>
				<SheetHeader>
					<SheetTitle className="flex items-center gap-2">
						<Sparkles className="size-4 shrink-0 text-primary" />
						<span>Generate SSH Key Pair</span>
					</SheetTitle>
					<SheetDescription>
						Generate secure ED25519 or RSA SSH key pair in memory
					</SheetDescription>
				</SheetHeader>

				{generatedKey ? (
					<GenerateKeyResult
						name={name}
						publicKey={generatedKey.public_key}
						saving={saving}
						onSave={handleSaveKey}
					/>
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
								className="border- h-9 rounded-md bg-background px-3 text-xs"
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
								className="border- h-9 rounded-md bg-background px-3 text-xs"
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
								<SelectTrigger className="border- h-9 rounded-md bg-background px-3 text-xs">
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

						<div className="flex items-center justify-end border-t  pt-3">
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
			</SheetContent>
		</Sheet>
	);
}
