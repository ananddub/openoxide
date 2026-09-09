import {toast} from 'sonner';
import {useMutation} from '@tanstack/react-query';
import {useNavigate} from '@tanstack/react-router';
import {AuthService} from '#/gen/connect/openoxide/api/v1/auth_pb';
import {createServiceClient} from '#/api/live/query';
import {useAuthSession} from '#/stores/session-context';
import type {SignInSchema} from '#/schema/signin.schema';

const authClient = createServiceClient(AuthService);

export function useSignin(onErrorCallback?: (msg: string) => void) {
	const navigate = useNavigate();
	const {setAuth} = useAuthSession();

	return useMutation({
		mutationFn: (data: SignInSchema) =>
			authClient.login({
				email: data.email,
				password: data.password,
			}),
		onSuccess: res => {
			if (!res.user) {
				onErrorCallback?.(
					'Invalid email or password. Please try again.',
				);
				return;
			}
			// Session cache stores the proto-native camelCase shape.
			const session = {
				user: {
					id: Number(res.user.id),
					email: res.user.email,
					firstName: res.user.firstName,
					lastName: res.user.lastName,
					avatar: res.user.avatar,
				},
				tokens: res.tokens,
			};
			localStorage.setItem(
				'openoxide-auth-session',
				JSON.stringify(session),
			);
			setAuth({
				id: Number(res.user.id),
				email: res.user.email || '',
				firstName: res.user.firstName,
				lastName: res.user.lastName,
			});
			toast.success('Successfully logged in!');
			navigate({to: '/'});
		},
		onError: (error: unknown) => {
			let message = 'Invalid email or password. Please try again.';
			const detail = error as {message?: string};
			if (detail?.message) {
				message = detail.message;
			}
			onErrorCallback?.(message);
		},
	});
}
