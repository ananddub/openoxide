import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';

/** One shape for "call a mutation, tell the user, refresh": success toasts the given message then runs `onDone`; failure toasts the RPC's own message (a generic label hid real build errors) after `onFail` reverts optimistic state. Never throws, so callers can `await` it inside a handler. */
export async function runResourceAction(
	run: () => Promise<unknown>,
	{
		success,
		onDone,
		onFail,
	}: {
		success: string;
		onDone?: () => void;
		onFail?: (err: unknown) => void;
	},
): Promise<boolean> {
	try {
		await run();
		toast.success(success);
		onDone?.();
		return true;
	} catch (err: unknown) {
		onFail?.(err);
		toast.error(formatApiError(err));
		return false;
	}
}
