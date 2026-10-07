import {useState} from 'react';
import {ClipboardPaste, Loader2, RefreshCw, Sparkles} from 'lucide-react';
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
import {
	OptionRow,
	StepTimeline,
	type WizardStep,
} from '#/components/shared/sheet-wizard';

interface SshKeyCreateSheetProps {
	isOpen: boolean;
	onOpenChange: (open: boolean) => void;
}

type KeySource = 'generate' | 'paste';

const STEPS: WizardStep[] = [
	{id: 'source', label: 'Source'},
	{id: 'material', label: 'Key material'},
	{id: 'details', label: 'Details'},
];

const FORM_ID = 'ssh-key-form';

const STEP_DESCRIPTIONS: Record<string, string> = {
	source: 'Generate a fresh pair here, or bring your own key.',
	material: 'The key pair that will be stored in the panel.',
	details: 'Name it so you can pick it later when connecting servers.',
};

/** New SSH Key — checkpoint wizard: Source → Key material → Details, the same StepTimeline grammar as the new-server sheet. Nothing leaves this sheet until Save is pressed. */
export function SshKeyCreateSheet({
	isOpen,
	onOpenChange,
}: SshKeyCreateSheetProps) {
	const [source, setSource] = useState<KeySource | null>(null);
	const [name, setName] = useState('');
	const [description, setDescription] = useState('');
	const [privateKey, setPrivateKey] = useState('');
	const [publicKey, setPublicKey] = useState('');
	const [generating, setGenerating] = useState(false);
	const [submitting, setSubmitting] = useState(false);
	const [stepIdx, setStepIdx] = useState(0);

	const generatePairMutation = useSshKeyGeneratePair();
	const createMutation = useSshKeyCreate();

	const reset = () => {
		setSource(null);
		setName('');
		setDescription('');
		setPrivateKey('');
		setPublicKey('');
		setStepIdx(0);
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
					`Generated ${type.toUpperCase()} key pair — press Save when ready.`,
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
		} catch (err: unknown) {
			toast.error(
				err instanceof Error ? err.message : 'Failed to create SSH Key',
			);
		} finally {
			setSubmitting(false);
		}
	};

	const last = stepIdx === STEPS.length - 1;
	const step = STEPS[stepIdx].id;
	const canContinue =
		step === 'source'
			? source !== null
			: step === 'material'
				? !!privateKey.trim()
				: true;

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
					<SheetDescription>{STEP_DESCRIPTIONS[step]}</SheetDescription>
				</SheetHeader>

				<form
					id={FORM_ID}
					onSubmit={handleSubmit}
					className="flex min-h-0 flex-1 flex-col">
					<div className="flex min-h-0 flex-1 flex-col overflow-y-auto px-6 py-6">
						<StepTimeline steps={STEPS} idx={stepIdx}>
							<div
								key={step}
								className="animate-in duration-200 ease-out fade-in-0 slide-in-from-right-4">
								{step === 'source' ? (
									<div className="flex flex-col gap-2">
										<OptionRow
											selected={source === 'generate'}
											onClick={() => setSource('generate')}
											icon={<Sparkles className="size-4.5" />}
											title="Generate a new pair"
											subtitle="The panel creates an ED25519 or RSA key pair for you."
										/>
										<OptionRow
											selected={source === 'paste'}
											onClick={() => setSource('paste')}
											icon={<ClipboardPaste className="size-4.5" />}
											title="Paste an existing key"
											subtitle="Bring a key pair you already generated elsewhere."
										/>
									</div>
								) : step === 'material' ? (
									<div className="flex flex-col gap-4">
										{source === 'generate' && (
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
										)}

										<div className="flex flex-col gap-1.5">
											<Label
												htmlFor="ssh-key-private"
												className="text-xs font-semibold text-foreground">
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

										<div className="flex flex-col gap-1.5">
											<Label
												htmlFor="ssh-key-public"
												className="text-xs font-semibold text-foreground">
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

										{source === 'generate' && (
											<SshKeyDownloadButtons
												fileName={formattedFileName}
												publicKey={publicKey}
												privateKey={privateKey}
											/>
										)}
									</div>
								) : (
									<div className="flex flex-col gap-4">
										<div className="flex flex-col gap-1.5">
											<Label
												htmlFor="ssh-key-name"
												className="text-xs font-semibold text-foreground">
												Name
											</Label>
											<Input
												id="ssh-key-name"
												placeholder="e.g. Production Server Key"
												value={name}
												onChange={e => setName(e.target.value)}
												disabled={submitting}
												required
											/>
										</div>

										<div className="flex flex-col gap-1.5">
											<Label
												htmlFor="ssh-key-description"
												className="text-xs font-semibold text-foreground">
												Description
											</Label>
											<Input
												id="ssh-key-description"
												placeholder="Optional note about where this key is used"
												value={description}
												onChange={e => setDescription(e.target.value)}
												disabled={submitting}
											/>
										</div>
									</div>
								)}
							</div>
						</StepTimeline>
					</div>

					<SheetFooter className="justify-between">
						{stepIdx > 0 ? (
							<Button
								type="button"
								variant="outline"
								onClick={() => setStepIdx(i => i - 1)}>
								Back
							</Button>
						) : (
							<span />
						)}
						{last ? (
							<Button
								type="submit"
								form={FORM_ID}
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
						) : (
							<Button
								type="button"
								disabled={!canContinue}
								onClick={() => setStepIdx(i => i + 1)}>
								Continue
							</Button>
						)}
					</SheetFooter>
				</form>
			</SheetContent>
		</Sheet>
	);
}
