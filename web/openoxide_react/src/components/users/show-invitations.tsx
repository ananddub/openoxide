import {useState} from 'react';
import {Loader2, Mail, Users} from 'lucide-react';
import {
	Card,
	CardContent,
	CardDescription,
	CardHeader,
	CardTitle,
} from '#/components/ui/card';
import {AddInvitationModal} from './add-invitation-modal';

export const ShowInvitations = () => {
	const [invitations] = useState<any[]>([]);
	const [isLoading] = useState(false);

	return (
		<div className="mx-auto w-full max-w-5xl">
			<Card className="overflow-hidden rounded-2xl border border-border/80 bg-card shadow-xs">
				<CardHeader className="flex flex-row flex-wrap items-center justify-between gap-4 pb-4">
					<div>
						<CardTitle className="flex items-center gap-2.5 text-lg font-bold">
							<div className="flex size-8 shrink-0 items-center justify-center rounded-xl border border-primary/20 bg-primary/10 text-primary">
								<Mail className="size-4" />
							</div>
							<span>Invitations</span>
						</CardTitle>
						<CardDescription className="mt-0.5 text-xs text-muted-foreground">
							Create and manage pending invitations to your organization.
						</CardDescription>
					</div>
					{invitations.length > 0 && <AddInvitationModal />}
				</CardHeader>
				<CardContent className="space-y-4 border-t border-border/60 pt-4">
					{isLoading ? (
						<div className="flex min-h-[20vh] flex-row items-center justify-center gap-2 text-sm text-muted-foreground">
							<Loader2 className="size-4 animate-spin text-primary" />
							<span>Loading invitations...</span>
						</div>
					) : !invitations || invitations.length === 0 ? (
						<div className="flex min-h-[20vh] flex-col items-center justify-center gap-3 rounded-xl border border-dashed border-border bg-muted/10 p-6 text-center">
							<Users className="size-8 text-muted-foreground/40" />
							<span className="text-sm font-semibold text-foreground">
								No pending invitations
							</span>
							<span className="max-w-sm text-xs text-muted-foreground">
								Invite new team members to collaborate on projects.
							</span>
							<AddInvitationModal />
						</div>
					) : (
						<div className="flex min-h-[20vh] flex-col gap-4">
							{/* Invitations Table */}
						</div>
					)}
				</CardContent>
			</Card>
		</div>
	);
};
