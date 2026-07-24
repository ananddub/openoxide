import {Header, Wrapper} from '#/components/auth/shared';
import {SignUpForm} from '#/components/auth/signup.form';
import {createFileRoute} from '@tanstack/react-router';

export const Route = createFileRoute('/_auth/singup')({
	component: RouteComponent,
});

function RouteComponent() {
	return (
		<Wrapper>
			<Header
				title="Create an account"
				subtitle="Start managing your servers and deploying with GoPloy"
			/>
			<SignUpForm />
		</Wrapper>
	);
}
