import * as React from 'react';
import {useForm} from 'react-hook-form';
import {valibotResolver} from '@hookform/resolvers/valibot';
import {Mail, User, ArrowRight, Loader2} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {FieldGroup} from '#/components/ui/field';
import {AuthError} from '#/components/auth/shared';
import {AuthInput, AuthPasswordInput} from './fields';
import {signUpSchema, type SignUpSchema} from '#/schema/signup.schema';
import {useSignup} from '#/hooks/use-signup';

export function SignUpForm() {
	const [apiError, setApiError] = React.useState<string | null>(null);
	const {mutate, isPending: isLoading} = useSignup(setApiError);

	const {
		register,
		handleSubmit,
		formState: {errors},
	} = useForm<SignUpSchema>({
		resolver: valibotResolver(signUpSchema),
	});

	const onSubmit = (data: SignUpSchema) => {
		setApiError(null);
		mutate(data);
	};

	return (
		<form
			onSubmit={handleSubmit(onSubmit)}
			className="flex flex-col gap-5">
			<AuthError message={apiError} />
			<FieldGroup>
				<div className="grid grid-cols-2 gap-4">
					<AuthInput
						id="firstName"
						label="First Name"
						type="text"
						placeholder="John"
						icon={User}
						disabled={isLoading}
						error={errors.firstName}
						registration={register('firstName')}
					/>
					<AuthInput
						id="lastName"
						label="Last Name"
						type="text"
						placeholder="Doe"
						icon={User}
						disabled={isLoading}
						error={errors.lastName}
						registration={register('lastName')}
					/>
				</div>
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
				/>
				<AuthPasswordInput
					id="confirmPassword"
					label="Confirm Password"
					placeholder="••••••••"
					disabled={isLoading}
					error={errors.confirmPassword}
					registration={register('confirmPassword')}
				/>
			</FieldGroup>
			<Button
				type="submit"
				className="flex h-10 w-full items-center justify-center rounded-lg bg-primary font-semibold text-primary-foreground transition-all hover:bg-primary/90 hover:shadow-lg hover:shadow-primary/10"
				disabled={isLoading}>
				{isLoading ? (
					<>
						<Loader2 className="animate-spin" data-icon="inline-start" />
						Creating account...
					</>
				) : (
					<>
						Create Account
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
