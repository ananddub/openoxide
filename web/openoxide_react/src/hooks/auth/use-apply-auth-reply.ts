import {useNavigate} from '@tanstack/react-router';
import {toast} from 'sonner';
import {useAuthSession} from '#/stores/session-context';
import type {User} from '#/types/user';

/** The `{user, tokens}` pair both Login and Signup answer with. */
type AuthReply = {
	user?: {
		id: bigint;
		email?: string;
		firstName?: string;
		lastName?: string;
		avatar?: string;
	} | null;
	tokens?: unknown;
};

/** Shared landing for a successful login/signup: persist the session the router reads on reload (proto-native camelCase shape), push the user into session context, toast and continue to `redirect` (an invite/deep link) or the dashboard. Returns false when the reply carried no user, so the caller can show its own error copy. */
export function useApplyAuthReply() {
	const navigate = useNavigate();
	const {setAuth} = useAuthSession();

	return (res: AuthReply, successMessage: string, redirect?: string) => {
		const user = res.user;
		if (!user) return false;

		localStorage.setItem(
			'openoxide-auth-session',
			JSON.stringify({
				user: {
					id: Number(user.id),
					email: user.email,
					firstName: user.firstName,
					lastName: user.lastName,
					avatar: user.avatar,
				},
				tokens: res.tokens,
			}),
		);
		setAuth({
			id: Number(user.id),
			email: user.email || '',
			firstName: user.firstName,
			lastName: user.lastName,
		} as User);
		toast.success(successMessage);
		if (redirect?.startsWith('/') && !redirect.startsWith('//')) {
			// Full reload so the target route boots with the new session.
			window.location.assign(redirect);
		} else {
			navigate({to: '/'});
		}
		return true;
	};
}
