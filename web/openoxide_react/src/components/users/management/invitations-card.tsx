import {Mail} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {timeAgo} from '#/components/activity/activity-model';
import {
	ListCard,
	ListRow,
	RowTile,
	SectionLabel,
} from '#/components/settings/list-card';
import type {UsersManagementController as Ctl} from './use-users-management';

/** Pending invitations — a second, quieter card under its own label. */
export function InvitationsCard({ctl}: {ctl: Ctl}) {
	const {invites, handleRevokeInvitation} = ctl;

	if (invites.length === 0) return null;

	return (
		<div className="flex flex-col gap-2">
			<SectionLabel>Pending invitations ({invites.length})</SectionLabel>
			<ListCard>
				{invites.map(invite => (
					<ListRow
						key={String(invite.id)}
						icon={
							<RowTile>
								<Mail className="size-4 text-muted-foreground" />
							</RowTile>
						}
						title={invite.email}
						subtitle={[
							invite.created_at
								? `Invited ${timeAgo(Number(invite.created_at))}`
								: 'Invited recently',
							invite.role ? `as ${invite.role.toLowerCase()}` : null,
						]
							.filter(Boolean)
							.join(' · ')}
						badges={
							<span className="rounded-md bg-amber-500/10 px-1.5 py-0.5 text-[10px] font-medium text-amber-500">
								Pending
							</span>
						}>
						<Button
							variant="ghost"
							size="sm"
							onClick={() => void handleRevokeInvitation(invite.id)}
							className="h-7 rounded-lg text-xs text-muted-foreground hover:text-destructive">
							Revoke
						</Button>
					</ListRow>
				))}
			</ListCard>
		</div>
	);
}
