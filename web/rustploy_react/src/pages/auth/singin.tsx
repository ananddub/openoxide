import {Header, Wrapper} from '#/components/auth/shared';
import {SignInForm} from '#/components/auth/signin.form';
import {createFileRoute} from '@tanstack/react-router';

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
		</Wrapper>
	);
}
