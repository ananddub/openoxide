import * as React from 'react';
import {useForm} from 'react-hook-form';
import {valibotResolver} from '@hookform/resolvers/valibot';
import {Mail, ArrowRight, Loader2} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {FieldGroup} from '#/components/ui/field';
import {AuthError} from '#/components/auth/shared';
import {AuthInput, AuthPasswordInput} from './fields';
import {signInSchema, type SignInSchema} from '#/schema/signin.schema';
import {useSignin} from '#/hooks/use-signin';

export function SignInForm() {
	const [apiError, setApiError] = React.useState<string | null>(null);
	const {mutate, isPending: isLoading} = useSignin(setApiError);

	const {
		register,
		handleSubmit,
		formState: {errors},
	} = useForm<SignInSchema>({
		resolver: valibotResolver(signInSchema),
	});
	// const { data,error,isPending} = $api.useQuery("get","")
	const onSubmit = (data: SignInSchema) => {
		setApiError(null);
		mutate(data);
	};

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
