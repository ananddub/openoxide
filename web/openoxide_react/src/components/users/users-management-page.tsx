import {UserPlus} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import {
	SectionHeader,
	Toolbar,
	ToolbarSearch,
	ViewToggle,
	useListView,
} from '#/components/settings/list-card';
import {
	ROLES,
	useUsersManagement,
} from './management/use-users-management';
import {MembersCard} from './management/members-card';
import {InvitationsCard} from './management/invitations-card';
import {TeamDialogs} from './management/dialogs';

/**
 * Settings → Team — the resource-page grammar: a section title, then a
 * toolbar (search + role filter + grid/list), then the member grid or one
 * bordered list card of avatar rows, then pending invitations under a quiet
 * label. All state, mutations and dialogs live in ./management so this stays
 * a thin composition.
 */
export function UsersManagementPage() {
	const ctl = useUsersManagement();
	const [view, setView] = useListView('settings-team-view');

	return (
		<div className="flex w-full animate-in flex-col gap-5 duration-200 fade-in">
			<SectionHeader
				title="Team"
				description="People, roles and pending invitations for this organization."
				action={
					ctl.canAny('member:create', 'invitation:create') && (
						<Button
							onClick={() => ctl.openCreate('invite')}
							className="h-9 gap-1.5">
							<UserPlus className="size-3.5" />
							Add member
						</Button>
					)
				}
			/>

			<Toolbar
				action={
					<>
						<Select
							value={ctl.roleFilter}
							onValueChange={v => ctl.setRoleFilter(v ?? 'ALL')}>
							<SelectTrigger className="h-9 w-auto rounded-lg bg-card">
								<SelectValue />
							</SelectTrigger>
							<SelectContent className="bg-card">
								<SelectItem value="ALL">All roles</SelectItem>
								{ROLES.map(r => (
									<SelectItem key={r} value={r} className="capitalize">
										{r}
									</SelectItem>
								))}
							</SelectContent>
						</Select>
						<ViewToggle view={view} onChange={setView} />
					</>
				}>
				<ToolbarSearch
					value={ctl.searchQuery}
					onChange={ctl.setSearchQuery}
					placeholder="Search members by name or email"
					className="min-w-[200px] flex-1 sm:max-w-none"
				/>
			</Toolbar>

			<MembersCard ctl={ctl} view={view} />
			<InvitationsCard ctl={ctl} />
			<TeamDialogs ctl={ctl} />
		</div>
	);
}
