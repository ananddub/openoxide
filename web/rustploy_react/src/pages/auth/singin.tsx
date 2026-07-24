import {Header, Wrapper} from '#/components/auth/shared';
import {SignInForm} from '#/components/auth/signin.form';
import {createFileRoute, Link} from '@tanstack/react-router';

export const Route = createFileRoute('/_auth/singin')({
	component: RouteComponent,
});

function RouteComponent() {
	return (
		<Wrapper>
			<Header
				title="Welcome Back"
				subtitle="Sign in to your account to manage your deployments"
			/>
			<SignInForm />
			<div className="mt-6 text-center text-sm text-muted-foreground">
				Don't have an account?{' '}
				<Link
					to="/singup"
					className="font-medium text-primary hover:underline">
					Sign Up
				</Link>
			</div>
		</Wrapper>
	);
}
