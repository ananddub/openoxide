import {toast} from 'sonner';
import {client} from '#/api/client';
import {useMutation} from '@tanstack/react-query';
import {useNavigate} from '@tanstack/react-router';
import {useAuthStore} from '#/stores/auth-store';
import type {SignInSchema} from '#/schema/signin.schema';

export function useSignin(onErrorCallback?: (msg: string) => void) {
	const navigate = useNavigate();
	const setAuth = useAuthStore(state => state.setAuth);

	return useMutation({
		mutationFn: async (data: SignInSchema) => {
			return client.POST('/auth/login', {
				body: data,
			});
		},
		onSuccess: ({data: res, error: err}) => {
			const errorBody = err as unknown as Record<string, unknown> | null;
			if (errorBody != null) {
				let message =
					(errorBody.message as string) ||
					'Invalid email or password. Please try again.';
				if (errorBody.code === 'INVALID_CREDENTIALS') {
					message = 'Invalid email or password. Please try again.';
				}
				onErrorCallback?.(message);
			} else if (res?.user) {
				localStorage.setItem('openoxide-auth-session', JSON.stringify(res));
				setAuth({
					id: res.user.user_id,
					email: res.user.email || '',
					firstName: res.user.first_name,
					lastName: res.user.last_name,
				});
				toast.success('Successfully logged in!');
				navigate({to: '/'});
			}
		},
	});
}
