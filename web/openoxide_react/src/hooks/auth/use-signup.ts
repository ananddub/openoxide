import {useMutation} from '@tanstack/react-query';
import {AuthService} from '#/gen/connect/openoxide/api/v1/auth_pb';
import {createServiceClient} from '#/api/live/query';
import {useApplyAuthReply} from './use-apply-auth-reply';
import type {SignUpSchema} from '#/schema/signup.schema';

const authClient = createServiceClient(AuthService);

export function useSignup(onErrorCallback?: (msg: string) => void) {
	const applyReply = useApplyAuthReply();

	return useMutation({
		mutationFn: (data: SignUpSchema) =>
			authClient.signup({
				email: data.email,
				password: data.password,
				firstName: data.firstName,
				lastName: data.lastName,
			}),
		onSuccess: res => {
			if (
				!applyReply(res, 'Owner registered and logged in successfully!')
			) {
				onErrorCallback?.(
					'An unexpected error occurred. Please try again.',
				);
			}
		},
		onError: (error: unknown) => {
			const detail = error as {message?: string};
			onErrorCallback?.(
				detail?.message || 'Signup failed. Please try again.',
			);
		},
	});
}
