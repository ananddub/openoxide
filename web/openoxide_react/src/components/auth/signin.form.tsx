import * as React from 'react';
import {useForm} from 'react-hook-form';
import {valibotResolver} from '@hookform/resolvers/valibot';
import {
	Mail,
	ArrowRight,
	ArrowLeft,
	KeyRound,
	Loader2,
	ShieldCheck,
} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Field, FieldGroup, FieldLabel} from '#/components/ui/field';
import {Input} from '#/components/ui/input';
import {AuthError} from '#/components/auth/shared';
import {AuthInput, AuthPasswordInput} from './fields';
import {signInSchema, type SignInSchema} from '#/schema/signin.schema';
import {useSignin} from '#/hooks/auth/use-signin';

/** Backend message for `AuthError::TwoFactorRequired` (Unauthenticated). */
const TWO_FACTOR_REQUIRED = /two-factor authentication code is required/i;

export function SignInForm({redirect}: {redirect?: string}) {
	const [apiError, setApiError] = React.useState<string | null>(null);
	const [credentials, setCredentials] =
		React.useState<SignInSchema | null>(null);
	const [step, setStep] = React.useState<'credentials' | 'two-factor'>(
		'credentials',
	);
	const [twoFactorCode, setTwoFactorCode] = React.useState('');
	const [recoveryCode, setRecoveryCode] = React.useState('');
	const [useRecoveryCode, setUseRecoveryCode] = React.useState(false);

	const {mutate, isPending: isLoading} = useSignin(message => {
		// Password was right, but the account needs a second factor: switch
		// to the code step instead of showing an "invalid credentials" error.
		if (TWO_FACTOR_REQUIRED.test(message)) {
			setStep('two-factor');
			setApiError(null);
			return;
		}
		setApiError(message);
	}, redirect);

	const {
		register,
		handleSubmit,
		formState: {errors},
	} = useForm<SignInSchema>({
		resolver: valibotResolver(signInSchema),
	});

	const onSubmit = (data: SignInSchema) => {
		setApiError(null);
		setCredentials(data);
		mutate(data);
	};

	const onSubmitSecondFactor = (event: React.FormEvent) => {
		event.preventDefault();
		if (!credentials) return;
		const code = twoFactorCode.trim();
		const recovery = recoveryCode.trim();
		if (useRecoveryCode ? !recovery : !code) {
			setApiError(
				useRecoveryCode
					? 'Enter one of your recovery codes.'
					: 'Enter the code from your authenticator app.',
			);
			return;
		}
		setApiError(null);
		mutate({
			...credentials,
			twoFactorCode: useRecoveryCode ? undefined : code,
			recoveryCode: useRecoveryCode ? recovery : undefined,
		});
	};

	const backToCredentials = () => {
		setStep('credentials');
		setApiError(null);
		setTwoFactorCode('');
		setRecoveryCode('');
	};

	if (step === 'two-factor') {
		return (
			<form
				onSubmit={onSubmitSecondFactor}
				className="flex flex-col gap-6">
				<AuthError message={apiError} />
				<div className="flex items-start gap-3 rounded-lg border bg-muted/40 px-3.5 py-3">
					<ShieldCheck className="mt-0.5 size-4 shrink-0 text-primary" />
					<p className="text-xs leading-relaxed text-muted-foreground">
						Two-factor authentication is on for{' '}
						<span className="font-medium text-foreground">
							{credentials?.email}
						</span>
						. Enter the code from your authenticator app to finish signing
						in.
					</p>
				</div>

				{useRecoveryCode ? (
					<Field>
						<FieldLabel
							htmlFor="recovery-code"
							className="font-mono text-[11px] font-normal tracking-[0.08em] text-muted-foreground uppercase">
							Recovery code
						</FieldLabel>
						<div className="relative">
							<span className="absolute inset-y-0 left-0 flex items-center pl-3.5 text-muted-foreground">
								<KeyRound className="size-4" />
							</span>
							<Input
								id="recovery-code"
								className="h-10 pl-10 font-mono"
								placeholder="xxxx-xxxx-xxxx"
								autoComplete="one-time-code"
								value={recoveryCode}
								onChange={e => setRecoveryCode(e.target.value)}
								disabled={isLoading}
							/>
						</div>
					</Field>
				) : (
					<Field>
						<FieldLabel
							htmlFor="two-factor-code"
							className="font-mono text-[11px] font-normal tracking-[0.08em] text-muted-foreground uppercase">
							Authentication code
						</FieldLabel>
						<div className="relative">
							<span className="absolute inset-y-0 left-0 flex items-center pl-3.5 text-muted-foreground">
								<ShieldCheck className="size-4" />
							</span>
							<Input
								id="two-factor-code"
								className="h-10 pl-10 font-mono tracking-[0.25em]"
								placeholder="123456"
								inputMode="numeric"
								autoComplete="one-time-code"
								maxLength={10}
								value={twoFactorCode}
								onChange={e => setTwoFactorCode(e.target.value)}
								disabled={isLoading}
							/>
						</div>
					</Field>
				)}

				<Button
					type="submit"
					className="flex h-10 w-full items-center justify-center rounded-lg bg-primary font-semibold text-primary-foreground transition-all hover:bg-primary/90 hover:shadow-lg hover:shadow-primary/10"
					disabled={isLoading}>
					{isLoading ? (
						<>
							<Loader2 className="animate-spin" data-icon="inline-start" />
							Verifying...
						</>
					) : (
						<>
							Verify code
							<ArrowRight data-icon="inline-end" />
						</>
					)}
				</Button>

				<div className="flex items-center justify-between text-xs">
					<button
						type="button"
						className="font-medium text-primary/80 transition-colors hover:text-primary hover:underline"
						onClick={() => {
							setUseRecoveryCode(current => !current);
							setApiError(null);
						}}>
						{useRecoveryCode
							? 'Use an authenticator code'
							: 'Use a recovery code'}
					</button>
					<button
						type="button"
						className="flex items-center gap-1 text-muted-foreground transition-colors hover:text-foreground"
						onClick={backToCredentials}>
						<ArrowLeft className="size-3" />
						Back
					</button>
				</div>
			</form>
		);
	}

	return (
		<form
			onSubmit={handleSubmit(onSubmit)}
			className="flex flex-col gap-6">
			<AuthError message={apiError} />
			<FieldGroup>
				<AuthInput
					id="email"
					label="Email Address"
					type="email"
					placeholder="name@example.com"
					icon={Mail}
					disabled={isLoading}
					error={errors.email}
					registration={register('email')}
				/>
				<AuthPasswordInput
					id="password"
					label="Password"
					placeholder="••••••••"
					disabled={isLoading}
					error={errors.password}
					registration={register('password')}
					forgotPasswordLink={
						<a
							href="#"
							onClick={e => e.preventDefault()}
							className="text-xs font-medium text-primary/80 transition-colors hover:text-primary hover:underline">
							Forgot password?
						</a>
					}
				/>
			</FieldGroup>
			<Button
				type="submit"
				className="flex h-10 w-full items-center justify-center rounded-lg bg-primary font-semibold text-primary-foreground transition-all hover:bg-primary/90 hover:shadow-lg hover:shadow-primary/10"
				disabled={isLoading}>
				{isLoading ? (
					<>
						<Loader2 className="animate-spin" data-icon="inline-start" />
						Signing in...
					</>
				) : (
					<>
						Sign In
						<ArrowRight
							className="transition-transform group-hover/button:translate-x-0.5"
							data-icon="inline-end"
						/>
					</>
				)}
			</Button>
		</form>
	);
}
