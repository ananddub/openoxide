import {useMutation} from '@tanstack/react-query';
import {AuthService} from '#/gen/connect/openoxide/api/v1/auth_pb';
import {createServiceClient} from '#/api/live/query';
import {useApplyAuthReply} from './use-apply-auth-reply';
import type {SignInSchema} from '#/schema/signin.schema';

const authClient = createServiceClient(AuthService);

export type SignInData = SignInSchema & {
	twoFactorCode?: string;
	recoveryCode?: string;
};

export function useSignin(
	onErrorCallback?: (msg: string) => void,
	redirect?: string,
) {
	const applyReply = useApplyAuthReply();

	return useMutation({
		mutationFn: (data: SignInData) =>
			authClient.login({
				email: data.email,
				password: data.password,
				twoFactorCode: data.twoFactorCode || undefined,
				recoveryCode: data.recoveryCode || undefined,
			}),
		onSuccess: res => {
			if (!applyReply(res, 'Successfully logged in!', redirect)) {
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
