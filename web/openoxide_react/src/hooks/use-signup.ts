import {toast} from 'sonner';
import {useMutation} from '@tanstack/react-query';
import {useNavigate} from '@tanstack/react-router';
import {AuthService} from '#/gen/connect/openoxide/api/v1/auth_pb';
import {createServiceClient} from '#/api/live/query';
import {useAuthSession} from '#/stores/session-context';
import type {SignUpSchema} from '#/schema/signup.schema';

const authClient = createServiceClient(AuthService);

export function useSignup(onErrorCallback?: (msg: string) => void) {
	const navigate = useNavigate();
	const {setAuth} = useAuthSession();

	return useMutation({
		mutationFn: (data: SignUpSchema) =>
			authClient.signup({
				email: data.email,
				password: data.password,
				firstName: data.firstName,
				lastName: data.lastName,
			}),
		onSuccess: res => {
			if (!res.user) {
				onErrorCallback?.(
					'An unexpected error occurred. Please try again.',
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
			toast.success('Owner registered and logged in successfully!');
			navigate({to: '/'});
		},
		onError: (error: unknown) => {
			const detail = error as {message?: string};
			onErrorCallback?.(
				detail?.message || 'Signup failed. Please try again.',
			);
		},
	});
}
