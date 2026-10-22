import {Header, Wrapper} from '#/components/auth/shared';
import {SignInForm} from '#/components/auth/signin.form';
import {createFileRoute, Link} from '@tanstack/react-router';

export const Route = createFileRoute('/_auth/singin')({
	validateSearch: (
		search: Record<string, unknown>,
	): {redirect?: string} => {
		const redirect =
			typeof search.redirect === 'string' ? search.redirect : undefined;
		// Only same-app absolute paths; never protocol-relative or external.
		const safe =
			redirect?.startsWith('/') && !redirect.startsWith('//')
				? redirect
				: undefined;
		return {redirect: safe};
	},
	component: RouteComponent,
});

function RouteComponent() {
	const {redirect} = Route.useSearch();

	return (
		<Wrapper>
			<Header
				title="Welcome Back"
				subtitle="Sign in to your account to manage your deployments"
			/>
			<SignInForm redirect={redirect} />
			<div className="mt-6 text-center text-sm text-muted-foreground">
				Don't have an account?{' '}
				<Link
					to="/singup"
					search={{redirect}}
					className="font-medium text-primary hover:underline">
					Sign Up
				</Link>
			</div>
		</Wrapper>
	);
}
