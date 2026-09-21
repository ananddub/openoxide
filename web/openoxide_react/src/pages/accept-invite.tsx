import {createFileRoute, useNavigate} from '@tanstack/react-router';
import {Loader2, PartyPopper} from 'lucide-react';
import {useEffect, useState} from 'react';
import {toast} from 'sonner';
import {useConnectMutation, createServiceClient} from '#/api/live/query';
import {PermissionService} from '#/gen/connect/openoxide/api/v1/permission_pb';
import {formatApiError} from '#/api/utils';
import {Button} from '#/components/ui/button';

const permissionClient = createServiceClient(PermissionService);

export const Route = createFileRoute('/_app/accept-invite')({
	validateSearch: (search: Record<string, unknown>): {token?: string} => ({
		token: typeof search.token === 'string' ? search.token : undefined,
	}),
	component: AcceptInvitePage,
});

/**
 * Landing target of the invite email link (/accept-invite?token=...).
 * AcceptInvite matches the token against the signed-in user's email
 * server-side; on success the user joins the organization and the sidebar
 * workspace switcher (which refetches ['organizations'] on invalidation)
 * immediately offers the new workspace.
 */
function AcceptInvitePage() {
	const {token} = Route.useSearch();
	const navigate = useNavigate();
	const [status, setStatus] = useState<'idle' | 'accepting' | 'accepted'>(
		'idle',
	);

	const accept = useConnectMutation(
		(tokenValue: string) =>
			permissionClient.acceptInvite({
				payload: JSON.stringify({token: tokenValue}),
			}),
		{invalidate: [['organizations'], ['permission-invites']]},
	);

	useEffect(() => {
		if (!token || status !== 'idle') return;
		setStatus('accepting');
		accept
			.mutateAsync(token)
			.then(() => {
				setStatus('accepted');
				toast.success('Invitation accepted — welcome aboard!');
			})
			.catch((error: unknown) => {
				setStatus('idle');
				toast.error(formatApiError(error));
			});
		// eslint-disable-next-line react-hooks/exhaustive-deps
	}, [token]);

	return (
		<div className="mx-auto flex min-h-[50vh] w-full max-w-md flex-col items-center justify-center gap-4 p-6 text-center">
			{status === 'accepted' ? (
				<>
					<PartyPopper className="size-8 text-primary" />
					<h1 className="text-xl font-bold">You're in!</h1>
					<p className="text-xs text-muted-foreground">
						The organization is now available in the workspace
						switcher.
					</p>
					<Button
						size="sm"
						className="h-9 text-xs"
						onClick={() => navigate({to: '/'})}>
						Go to Home
					</Button>
				</>
			) : (
				<>
					<Loader2 className="size-6 animate-spin text-muted-foreground" />
					<h1 className="text-lg font-bold">
						{token ? 'Accepting invitation…' : 'No invitation token'}
					</h1>
					{!token && (
						<p className="text-xs text-muted-foreground">
							Open the link from your invitation email.
						</p>
					)}
				</>
			)}
		</div>
	);
}
