import {useState, type FormEvent, type ReactNode} from 'react';
import {
	Check,
	Copy,
	Loader2,
	ShieldCheck,
	ShieldOff,
	KeyRound,
} from 'lucide-react';
import {toast} from 'sonner';
import {AuthService} from '#/gen/connect/openoxide/api/v1/auth_pb';
import type {TwoFactorSetup} from '#/gen/connect/openoxide/api/v1/auth_pb';
import {createServiceClient, useConnectQuery} from '#/api/live/query';
import {formatApiError} from '#/api/utils';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {
	Sheet,
	SheetContent,
	SheetDescription,
	SheetFooter,
	SheetHeader,
	SheetTitle,
} from '#/components/ui/sheet';
import {ListCard, RowTile} from '#/components/settings/list-card';

const authClient = createServiceClient(AuthService);

async function copyToClipboard(text: string, label: string) {
	try {
		await navigator.clipboard.writeText(text);
		toast.success(`${label} copied`);
	} catch {
		toast.error('Could not copy to clipboard');
	}
}

/** Settings → Account → Two-factor authentication: status row plus the setup/manage sheet. */
export function TwoFactorCard() {
	const [open, setOpen] = useState(false);
	const status = useConnectQuery(['auth', 'two-factor'], () =>
		authClient.getTwoFactorStatus({}),
	);
	const enabled = status.data?.enabled ?? false;

	return (
		<>
			<ListCard>
				<div className="flex items-center gap-3.5 px-4 py-3">
					<RowTile>
						<ShieldCheck className="size-4" />
					</RowTile>
					<span className="min-w-0 flex-1 text-sm text-muted-foreground">
						Two-factor authentication
					</span>
					<span
						className={
							enabled
								? 'shrink-0 text-xs font-medium text-emerald-500'
								: 'shrink-0 text-xs font-medium text-muted-foreground'
						}>
						{enabled ? 'On' : 'Off'}
					</span>
					<Button
						variant="outline"
						size="sm"
						type="button"
						onClick={() => setOpen(true)}
						className="h-8 shrink-0 rounded-lg text-xs font-medium">
						{enabled ? 'Manage' : 'Turn on'}
					</Button>
				</div>
			</ListCard>

			<Sheet open={open} onOpenChange={setOpen}>
				{open && (
					<TwoFactorSheet
						enabled={enabled}
						onDone={() => {
							setOpen(false);
							void status.refetch();
						}}
					/>
				)}
			</Sheet>
		</>
	);
}

type Mode = 'setup' | 'regenerate' | 'disable';

function TwoFactorSheet({
	enabled,
	onDone,
}: {
	enabled: boolean;
	onDone: () => void;
}) {
	const [mode, setMode] = useState<Mode>(enabled ? 'disable' : 'setup');
	const [password, setPassword] = useState('');
	const [code, setCode] = useState('');
	const [pending, setPending] = useState(false);
	const [error, setError] = useState<string | null>(null);
	const [setup, setSetup] = useState<TwoFactorSetup | null>(null);
	const [recoveryCodes, setRecoveryCodes] = useState<string[] | null>(
		null,
	);

	const handleSetup = async (event: FormEvent) => {
		event.preventDefault();
		setError(null);
		setPending(true);
		try {
			const result = await authClient.setupTwoFactor({password});
			setSetup(result);
			setPassword('');
		} catch (err) {
			setError(formatApiError(err) || 'Could not start 2FA setup');
		} finally {
			setPending(false);
		}
	};

	const handleEnable = async (event: FormEvent) => {
		event.preventDefault();
		setError(null);
		setPending(true);
		try {
			await authClient.enableTwoFactor({code: code.trim()});
			toast.success('Two-factor authentication enabled');
			onDone();
		} catch (err) {
			setError(formatApiError(err) || 'Invalid code; try the next one');
		} finally {
			setPending(false);
		}
	};

	const handleRegenerate = async (event: FormEvent) => {
		event.preventDefault();
		setError(null);
		setPending(true);
		try {
			const result = await authClient.regenerateRecoveryCodes({
				password,
				code: code.trim(),
			});
			setRecoveryCodes(result.recoveryCodes);
			setPassword('');
			setCode('');
		} catch (err) {
			setError(formatApiError(err) || 'Could not regenerate codes');
		} finally {
			setPending(false);
		}
	};

	const handleDisable = async (event: FormEvent) => {
		event.preventDefault();
		setError(null);
		setPending(true);
		try {
			await authClient.disableTwoFactor({password, code: code.trim()});
			toast.success('Two-factor authentication disabled');
			onDone();
		} catch (err) {
			setError(formatApiError(err) || 'Could not disable 2FA');
		} finally {
			setPending(false);
		}
	};

	const title = !enabled
		? 'Set up two-factor authentication'
		: mode === 'regenerate'
			? 'Regenerate recovery codes'
			: 'Disable two-factor authentication';

	return (
		<SheetContent
			side="right"
			className="gap-0 p-0 sm:data-[side=right]:w-[460px]">
			<SheetHeader className="px-5 pt-5 pb-4">
				<SheetTitle>{title}</SheetTitle>
				<SheetDescription>
					{!enabled
						? 'Add a second step to sign-in with an authenticator app.'
						: mode === 'regenerate'
							? 'Your previous recovery codes stop working immediately.'
							: 'Sign-in will only require your password again.'}
				</SheetDescription>
			</SheetHeader>

			{!enabled && setup ? (
				<form
					onSubmit={handleEnable}
					className="flex min-h-0 flex-1 flex-col gap-4 overflow-y-auto px-5 pt-2 pb-5">
					<SecretBlock setup={setup} />
					<Field label="Authenticator code">
						<Input
							placeholder="123456"
							inputMode="numeric"
							autoComplete="one-time-code"
							maxLength={10}
							value={code}
							onChange={e => setCode(e.target.value)}
						/>
					</Field>
					<SheetError message={error} />
					<SheetFooter className="px-0 py-0">
						<Button
							type="submit"
							disabled={pending}
							className="h-8 gap-1.5 px-4 text-xs">
							{pending ? (
								<Loader2 className="size-3.5 animate-spin" />
							) : (
								<Check className="size-3.5" />
							)}
							Turn on
						</Button>
					</SheetFooter>
				</form>
			) : (
				<form
					onSubmit={
						!enabled
							? handleSetup
							: mode === 'regenerate'
								? handleRegenerate
								: handleDisable
					}
					className="flex min-h-0 flex-1 flex-col gap-4 overflow-y-auto px-5 pt-2 pb-5">
					{recoveryCodes && <RecoveryCodesBlock codes={recoveryCodes} />}
					<Field label="Password">
						<Input
							type="password"
							autoComplete="current-password"
							placeholder="Your password"
							value={password}
							onChange={e => setPassword(e.target.value)}
						/>
					</Field>
					{enabled && (
						<Field label="Authenticator code">
							<Input
								placeholder="123456"
								inputMode="numeric"
								autoComplete="one-time-code"
								maxLength={10}
								value={code}
								onChange={e => setCode(e.target.value)}
							/>
						</Field>
					)}
					<SheetError message={error} />
					<SheetFooter className="px-0 py-0">
						{enabled && mode !== 'regenerate' ? (
							<Button
								type="button"
								variant="ghost"
								onClick={() => {
									setMode('regenerate');
									setError(null);
								}}
								className="h-8 text-xs">
								<KeyRound className="size-3.5" />
								Recovery codes
							</Button>
						) : (
							enabled && (
								<Button
									type="button"
									variant="ghost"
									onClick={() => {
										setMode('disable');
										setError(null);
									}}
									className="h-8 text-xs">
									<ShieldOff className="size-3.5" />
									Disable 2FA
								</Button>
							)
						)}
						<Button
							type="submit"
							disabled={pending}
							className="h-8 gap-1.5 px-4 text-xs">
							{pending && <Loader2 className="size-3.5 animate-spin" />}
							{!enabled
								? 'Continue'
								: mode === 'regenerate'
									? 'Regenerate'
									: 'Disable'}
						</Button>
					</SheetFooter>
				</form>
			)}
		</SheetContent>
	);
}

function Field({label, children}: {label: string; children: ReactNode}) {
	return (
		<label className="flex flex-col gap-1.5">
			<span className="text-xs font-semibold text-foreground">
				{label}
			</span>
			{children}
		</label>
	);
}

function SheetError({message}: {message: string | null}) {
	if (!message) return null;
	return (
		<p className="rounded-lg border border-destructive/40 bg-destructive/10 px-3 py-2 text-xs text-destructive">
			{message}
		</p>
	);
}

function SecretBlock({setup}: {setup: TwoFactorSetup}) {
	return (
		<div className="flex flex-col gap-3 rounded-lg border bg-muted/40 p-3.5">
			<p className="text-xs leading-relaxed text-muted-foreground">
				Add this secret to your authenticator app, then enter the 6-digit
				code it shows.
			</p>
			<div className="flex items-center gap-2">
				<code className="min-w-0 flex-1 truncate rounded-md bg-background px-2.5 py-1.5 font-mono text-xs">
					{setup.secret}
				</code>
				<Button
					type="button"
					variant="outline"
					size="sm"
					className="h-7 shrink-0 px-2 text-xs"
					onClick={() => void copyToClipboard(setup.secret, 'Secret')}>
					<Copy className="size-3" />
					Copy
				</Button>
			</div>
			{setup.otpauthUrl && (
				<div className="flex items-center gap-2">
					<span className="min-w-0 flex-1 truncate text-[11px] text-muted-foreground">
						{setup.otpauthUrl}
					</span>
					<Button
						type="button"
						variant="ghost"
						size="sm"
						className="h-7 shrink-0 px-2 text-xs"
						onClick={() =>
							void copyToClipboard(setup.otpauthUrl, 'Setup link')
						}>
						<Copy className="size-3" />
					</Button>
				</div>
			)}
			<RecoveryCodesBlock codes={setup.recoveryCodes} />
		</div>
	);
}

function RecoveryCodesBlock({codes}: {codes: string[]}) {
	if (codes.length === 0) return null;
	return (
		<div className="flex flex-col gap-2 rounded-lg border bg-muted/40 p-3.5">
			<div className="flex items-center justify-between gap-2">
				<span className="text-xs font-semibold text-foreground">
					Recovery codes
				</span>
				<Button
					type="button"
					variant="ghost"
					size="sm"
					className="h-7 shrink-0 px-2 text-xs"
					onClick={() =>
						void copyToClipboard(codes.join('\n'), 'Recovery codes')
					}>
					<Copy className="size-3" />
					Copy
				</Button>
			</div>
			<p className="text-[11px] leading-relaxed text-muted-foreground">
				Store these somewhere safe. Each code signs you in once if you lose
				your authenticator.
			</p>
			<div className="grid grid-cols-2 gap-1.5 font-mono text-xs">
				{codes.map(item => (
					<span key={item} className="rounded bg-background px-2 py-1">
						{item}
					</span>
				))}
			</div>
		</div>
	);
}
