import {Header, Wrapper} from '#/components/auth/shared';
import {SignUpForm} from '#/components/auth/signup.form';
import {createFileRoute, Link} from '@tanstack/react-router';

export const Route = createFileRoute('/_auth/singup')({
	validateSearch: (
		search: Record<string, unknown>,
	): {redirect?: string} => {
		const redirect =
			typeof search.redirect === 'string' ? search.redirect : undefined;
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
				title="Create an account"
				subtitle="Start managing your servers and deploying with OpenOxide"
			/>
			<SignUpForm redirect={redirect} />
			<div className="mt-6 text-center text-sm text-muted-foreground">
				Already have an account?{' '}
				<Link
					to="/singin"
					search={{redirect}}
					className="font-medium text-primary hover:underline">
					Sign In
				</Link>
			</div>
		</Wrapper>
	);
}
