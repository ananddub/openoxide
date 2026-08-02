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
		<div className="w-full max-w-5xl mx-auto">
			<Card className="bg-card border border-border/80 rounded-2xl shadow-xs overflow-hidden">
				<CardHeader className="flex flex-row items-center justify-between flex-wrap gap-4 pb-4">
					<div>
						<CardTitle className="text-lg font-bold flex items-center gap-2.5">
							<div className="size-8 rounded-xl bg-primary/10 border border-primary/20 flex items-center justify-center text-primary shrink-0">
								<Mail className="size-4" />
							</div>
							<span>Invitations</span>
						</CardTitle>
						<CardDescription className="text-xs text-muted-foreground mt-0.5">
							Create and manage pending invitations to your organization.
						</CardDescription>
					</div>
					{invitations.length > 0 && <AddInvitationModal />}
				</CardHeader>
				<CardContent className="space-y-4 pt-4 border-t border-border/60">
					{isLoading ? (
						<div className="flex flex-row gap-2 items-center justify-center text-sm text-muted-foreground min-h-[20vh]">
							<Loader2 className="animate-spin size-4 text-primary" />
							<span>Loading invitations...</span>
						</div>
					) : !invitations || invitations.length === 0 ? (
						<div className="flex flex-col items-center gap-3 min-h-[20vh] justify-center text-center p-6 border border-dashed border-border rounded-xl bg-muted/10">
							<Users className="size-8 text-muted-foreground/40" />
							<span className="text-sm font-semibold text-foreground">
								No pending invitations
							</span>
							<span className="text-xs text-muted-foreground max-w-sm">
								Invite new team members to collaborate on projects.
							</span>
							<AddInvitationModal />
						</div>
					) : (
						<div className="flex flex-col gap-4 min-h-[20vh]">
							{/* Invitations Table */}
						</div>
					)}
				</CardContent>
			</Card>
		</div>
	);
};
