import {useState, useEffect} from 'react';
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
import {
	Boxes,
	Cloud,
	Droplets,
	Eye,
	EyeOff,
	Leaf,
	Save,
	Server as ServerIcon,
	Wind,
} from 'lucide-react';
import {DestinationTestButton} from '#/components/destinations/destination-test-button';
import {
	OptionRow,
	StepBody,
	StepCheckpoints,
	WizardField,
	type WizardStep,
} from '#/components/shared/sheet-wizard';
import {
	PROVIDERS,
	useDestinationForm,
} from '#/components/destinations/use-destination-form';
import type {DestinationResponse} from '#/types/api-helpers';

interface CreateDestinationModalProps {
	isOpen: boolean;
	onClose: () => void;
	onSuccess: () => void;
	editingDestination?: DestinationResponse | null;
}

const FORM_ID = 'create-destination-sheet-form';

const STEPS: WizardStep[] = [
	{id: 'provider', label: 'Provider'},
	{id: 'bucket', label: 'Bucket'},
	{id: 'keys', label: 'Keys'},
];

const PROVIDER_META: Record<string, {icon: typeof Cloud; desc: string}> = {
	aws: {icon: Cloud, desc: 'Amazon S3 buckets.'},
	r2: {icon: Wind, desc: 'Cloudflare R2 — no egress fees.'},
	minio: {icon: Boxes, desc: 'Self-hosted S3-compatible storage.'},
	digitalocean: {icon: Droplets, desc: 'DigitalOcean Spaces.'},
	wasabi: {icon: Leaf, desc: 'Wasabi hot cloud storage.'},
	custom: {icon: ServerIcon, desc: 'Any other S3 API endpoint.'},
};

export function CreateDestinationModal({
	isOpen,
	onClose,
	onSuccess,
	editingDestination,
}: CreateDestinationModalProps) {
	const [showSecret, setShowSecret] = useState(false);
	const [stepIdx, setStepIdx] = useState(0);

	const f = useDestinationForm({
		isOpen,
		onClose,
		onSuccess,
		editingDestination,
	});

	useEffect(() => {
		if (isOpen) setStepIdx(0);
	}, [isOpen]);

	const step = STEPS[stepIdx].id;
	const last = stepIdx === STEPS.length - 1;

	const canContinue =
		step === 'bucket' ? !!f.name.trim() && !!f.bucket.trim() : true;

	const stepDesc: Record<string, string> = {
		provider:
			'Backups and volume snapshots go to an S3-compatible bucket.',
		bucket: 'Tell us where the data lives in that provider.',
		keys: 'Keys are stored encrypted and used only for backups.',
	};

	return (
		<Sheet open={isOpen} onOpenChange={open => !open && onClose()}>
			<SheetContent>
				<SheetHeader>
					<SheetTitle className="flex items-center gap-1.5">
						<Cloud className="size-4 text-muted-foreground" />
						{editingDestination
							? 'Edit Destination'
							: 'Add S3 Destination'}
					</SheetTitle>
					<SheetDescription>{stepDesc[step]}</SheetDescription>
					<StepCheckpoints steps={STEPS} idx={stepIdx} />
				</SheetHeader>

				<form
					id={FORM_ID}
					onSubmit={f.handleSubmit}
					className="flex-1 overflow-y-auto px-6 py-6">
					{step === 'provider' && (
						<StepBody key="provider">
							<div className="flex flex-col gap-2">
								{PROVIDERS.map(p => {
									const meta = PROVIDER_META[p.id];
									const Icon = meta?.icon ?? Cloud;
									return (
										<OptionRow
											key={p.id}
											selected={f.provider === p.id}
											onClick={() => f.handleProviderChange(p.id)}
											icon={<Icon className="size-5" />}
											title={p.name}
											subtitle={meta?.desc}
										/>
									);
								})}
							</div>
						</StepBody>
					)}

					{step === 'bucket' && (
						<StepBody key="bucket">
							<div className="flex flex-col gap-5">
								<WizardField label="Name" required>
									<Input
										value={f.name}
										onChange={e => f.setName(e.target.value)}
										placeholder="Nightly backups bucket"
										autoFocus
										className="bg-background"
									/>
								</WizardField>
								<WizardField label="Bucket" required>
									<Input
										value={f.bucket}
										onChange={e => f.setBucket(e.target.value)}
										placeholder="my-bucket-name"
										className="bg-background font-mono"
									/>
								</WizardField>
								<WizardField
									label="Region"
									optional
									hint={
										<>
											Used when signing requests —{' '}
											<code className="font-mono text-foreground">
												us-east-1
											</code>{' '}
											for AWS,{' '}
											<code className="font-mono text-foreground">
												auto
											</code>{' '}
											for R2.
										</>
									}>
									<Input
										value={f.region}
										onChange={e => f.setRegion(e.target.value)}
										placeholder="us-east-1"
										className="bg-background font-mono"
									/>
								</WizardField>
								<WizardField
									label="Endpoint"
									optional={f.provider !== 'custom'}
									required={f.provider === 'custom'}
									hint="Required for MinIO, Spaces, Wasabi or any custom S3 host.">
									<Input
										value={f.endpoint}
										onChange={e => f.setEndpoint(e.target.value)}
										placeholder="https://s3.amazonaws.com"
										className="bg-background font-mono"
									/>
								</WizardField>
							</div>
						</StepBody>
					)}

					{step === 'keys' && (
						<StepBody key="keys">
							<div className="flex flex-col gap-5">
								<WizardField label="Access Key ID" required>
									<Input
										value={f.accessKey}
										onChange={e => f.setAccessKey(e.target.value)}
										placeholder="AKIAIOSFODNN7EXAMPLE"
										autoFocus
										className="bg-background font-mono"
									/>
								</WizardField>
								<WizardField label="Secret Access Key" required>
									<div className="relative flex w-full items-center">
										<Input
											type={showSecret ? 'text' : 'password'}
											value={f.secretKey}
											onChange={e => f.setSecretKey(e.target.value)}
											placeholder="Secret Access Key"
											className="bg-background pr-10 font-mono"
										/>
										<button
											type="button"
											aria-label={
												showSecret ? 'Hide secret key' : 'Show secret key'
											}
											onClick={() => setShowSecret(v => !v)}
											className="absolute right-2 flex size-7 items-center justify-center rounded-md text-muted-foreground transition-colors hover:bg-muted/50 hover:text-foreground">
											{showSecret ? (
												<EyeOff className="size-4" />
											) : (
												<Eye className="size-4" />
											)}
										</button>
									</div>
								</WizardField>
								<DestinationTestButton
									status={f.testStatus}
									onClick={f.handleTestConnection}
								/>
							</div>
						</StepBody>
					)}
				</form>

				{last ? (
					<SheetFooter className="justify-between">
						<p className="self-center text-xs text-muted-foreground">
							Backups go to{' '}
							<span className="font-semibold text-foreground">
								{f.bucket.trim() || 'your bucket'}
							</span>
						</p>
						<div className="flex items-center gap-3">
							{stepIdx > 0 && (
								<Button
									variant="outline"
									onClick={() => setStepIdx(i => i - 1)}>
									Back
								</Button>
							)}
							<Button
								type="submit"
								form={FORM_ID}
								disabled={f.submitting || !canContinue}
								className="h-10 gap-1.5 px-5">
								<Save className="size-3.5" />
								{f.submitting
									? 'Saving…'
									: editingDestination
										? 'Save Changes'
										: 'Add Destination'}
							</Button>
						</div>
					</SheetFooter>
				) : (
					<SheetFooter>
						{stepIdx > 0 ? (
							<Button
								variant="outline"
								onClick={() => setStepIdx(i => i - 1)}>
								Back
							</Button>
						) : (
							<Button variant="outline" onClick={onClose}>
								Cancel
							</Button>
						)}
						<Button
							disabled={!canContinue}
							onClick={() => setStepIdx(i => i + 1)}>
							Continue
						</Button>
					</SheetFooter>
				)}
			</SheetContent>
		</Sheet>
	);
}
