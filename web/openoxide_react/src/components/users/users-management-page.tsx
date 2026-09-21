import {UserPlus} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {useUsersManagement} from './management/use-users-management';
import {MembersCard} from './management/members-card';
import {InvitationsCard} from './management/invitations-card';
import {TeamDialogs} from './management/dialogs';

/**
 * Settings → Team, OpenShip's team-page shape: one quiet list card per subject
 * (Members, Pending Invitations) with slim headers, divide-y avatar rows and an
 * inline role control — no table chrome. All state, mutations and dialogs live
 * in ./management so this stays a thin composition.
 */
export function UsersManagementPage() {
	const ctl = useUsersManagement();

	return (
		<div className="flex w-full animate-in flex-col gap-6 duration-150 fade-in">
			<div className="flex flex-wrap items-start justify-between gap-4 sm:items-center">
				<div className="flex flex-col gap-0.5">
					<h2 className="text-sm font-medium text-foreground">Team</h2>
					<p className="text-xs text-muted-foreground">
						Members, roles, and pending invitations in this workspace.
					</p>
				</div>
				{ctl.canAny('member:create', 'invitation:create') && (
					<Button
						onClick={() => ctl.setIsCreateOpen(true)}
						className="gap-2 px-4 text-sm">
						<UserPlus className="size-4" />
						Add or invite
					</Button>
				)}
			</div>

			<MembersCard ctl={ctl} />
			<InvitationsCard ctl={ctl} />
			<TeamDialogs ctl={ctl} />
		</div>
	);
}
