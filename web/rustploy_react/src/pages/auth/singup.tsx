import {Header, Wrapper} from '#/components/auth/shared';
import {SignUpForm} from '#/components/auth/signup.form';
import {createFileRoute, Link} from '@tanstack/react-router';

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
			<div className="mt-6 text-center text-sm text-muted-foreground">
				Already have an account?{' '}
				<Link
					to="/singin"
					className="font-medium text-primary hover:underline">
					Sign In
				</Link>
			</div>
		</Wrapper>
	);
}
