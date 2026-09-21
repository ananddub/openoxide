import {Mail} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {
	ListCard,
	ListRow,
	ListEmpty,
	RowTile,
} from '#/components/settings/list-card';
import type {UsersManagementController as Ctl} from './use-users-management';

/** Pending invitations card: email rows with created date, role, revoke. */
export function InvitationsCard({ctl}: {ctl: Ctl}) {
	const {invites, handleRevokeInvitation} = ctl;
	return (
		<ListCard
			title={`Pending invitations (${invites.length})`}
			description="Invited emails that have not accepted yet — valid for 7 days.">
			{invites.length === 0 ? (
				<ListEmpty
					icon={Mail}
					title="No pending invitations"
					description="All invited team members have accepted, or no invites exist yet."
				/>
			) : (
				<div className="divide-y divide-border/40">
					{invites.map(invite => (
						<ListRow
							key={String(invite.id)}
							icon={
								<RowTile className="bg-muted/50">
									<Mail className="size-4 text-muted-foreground" />
								</RowTile>
							}
							title={invite.email}
							subtitle={`Invited ${
								invite.created_at
									? new Date(
											Number(invite.created_at) * 1000,
										).toLocaleDateString()
									: 'recently'
							} · role ${invite.role.toLowerCase()}`}
							badges={
								<span className="rounded-md bg-amber-500/10 px-1.5 py-0.5 text-[10px] font-medium text-amber-400">
									Pending
								</span>
							}>
							<Button
								variant="ghost"
								size="sm"
								onClick={() => void handleRevokeInvitation(invite.id)}
								className="h-7 text-xs text-muted-foreground hover:text-destructive">
								Revoke
							</Button>
						</ListRow>
					))}
				</div>
			)}
		</ListCard>
	);
}
