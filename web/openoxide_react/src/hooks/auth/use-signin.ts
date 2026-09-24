import {useMutation} from '@tanstack/react-query';
import {AuthService} from '#/gen/connect/openoxide/api/v1/auth_pb';
import {createServiceClient} from '#/api/live/query';
import {useApplyAuthReply} from './use-apply-auth-reply';
import type {SignInSchema} from '#/schema/signin.schema';

const authClient = createServiceClient(AuthService);

export function useSignin(onErrorCallback?: (msg: string) => void) {
	const applyReply = useApplyAuthReply();

	return useMutation({
		mutationFn: (data: SignInSchema) =>
			authClient.login({
				email: data.email,
				password: data.password,
			}),
		onSuccess: res => {
			if (!applyReply(res, 'Successfully logged in!')) {
				onErrorCallback?.('Invalid email or password. Please try again.');
			}
		},
		onError: (error: unknown) => {
			const detail = error as {message?: string};
			onErrorCallback?.(
				detail?.message || 'Invalid email or password. Please try again.',
			);
		},
	});
}
