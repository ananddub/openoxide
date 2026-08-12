import {toast} from 'sonner';
import {client} from '#/api/client';
import {useMutation} from '@tanstack/react-query';
import {useNavigate} from '@tanstack/react-router';
import {useAuthStore} from '#/stores/auth-store';
import type {SignUpSchema} from '#/schema/signup.schema';

export function useSignup(onErrorCallback?: (msg: string) => void) {
	const navigate = useNavigate();
	const setAuth = useAuthStore(state => state.setAuth);

	return useMutation({
		mutationFn: async (data: SignUpSchema) => {
			return client.POST('/auth/signup', {
				body: data,
			});
		},
		onSuccess: ({data: res, error: err}) => {
			const errorBody = err as unknown as Record<string, unknown> | null;
			if (errorBody != null) {
				onErrorCallback?.(
					(errorBody.message as string) ||
						'An unexpected error occurred. Please try again.',
				);
			} else if (res?.user) {
				localStorage.setItem('openoxide-auth-session', JSON.stringify(res));
				setAuth({
					id: res.user.user_id,
					email: res.user.email || '',
					firstName: res.user.first_name,
					lastName: res.user.last_name,
				});
				toast.success('Owner registered and logged in successfully!');
				navigate({to: '/'});
			}
		},
	});
}
