import {Mail, Users} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import {
	Sheet,
	SheetContent,
	SheetDescription,
	SheetFooter,
	SheetHeader,
	SheetTitle,
} from '#/components/ui/sheet';
import {ConfirmDeleteDialog} from '#/components/settings/confirm-delete-dialog';
import {OptionRow, WizardField} from '#/components/shared/sheet-wizard';
import {PermissionOverridesSheet} from './permission-overrides-sheet';
import {
	ROLES,
	ROLE_DESC,
	type UsersManagementController as Ctl,
} from './use-users-management';

const ACTIONS = [
	{
		id: 'direct',
		icon: Users,
		title: 'Add a member',
		subtitle: 'They must already have an account on this panel',
	},
	{
		id: 'invite',
		icon: Mail,
		title: 'Send an invitation',
		subtitle: 'They accept over email before joining',
	},
] as const;

/** Team overlays: add/invite sheet, change-role sheet, remove confirm. */
export function TeamDialogs({ctl}: {ctl: Ctl}) {
	const {
		isCreateOpen,
		setIsCreateOpen,
		actionType,
		setActionType,
		inputEmail,
		setInputEmail,
		selectedRole,
		setSelectedRole,
		selectedGroupId,
		setSelectedGroupId,
		groups,
		isSubmitting,
		handleCreateOrInvite,
		editingRoleUser,
		setEditingRoleUser,
		handleUpdateRole,
		handleAssignGroup,
		deleteTarget,
		setDeleteTarget,
		isDeleting,
		confirmDeleteUser,
		overridesTarget,
		setOverridesTarget,
	} = ctl;

	return (
		<>
			<Sheet open={isCreateOpen} onOpenChange={setIsCreateOpen}>
				<SheetContent>
					<SheetHeader>
						<SheetTitle>New team member</SheetTitle>
						<SheetDescription>
							Pick how they join, then choose the role they get in this
							organization.
						</SheetDescription>
					</SheetHeader>

					<form
						onSubmit={handleCreateOrInvite}
						className="flex min-h-0 flex-1 flex-col">
						<div className="flex min-h-0 flex-1 flex-col gap-5 overflow-y-auto px-6 py-6">
							<div className="flex flex-col gap-2">
								{ACTIONS.map(a => {
									const Icon = a.icon;
									return (
										<OptionRow
											key={a.id}
											selected={actionType === a.id}
											onClick={() => setActionType(a.id)}
											icon={<Icon className="size-5" />}
											title={a.title}
											subtitle={a.subtitle}
										/>
									);
								})}
							</div>

							<WizardField label="Email address" required>
								<Input
									type="email"
									placeholder="user@company.com"
									value={inputEmail}
									onChange={e => setInputEmail(e.target.value)}
									required
									autoFocus
								/>
							</WizardField>

							<WizardField
								label="Role"
								required
								hint={ROLE_DESC[selectedRole]}>
								<Select
									value={selectedRole}
									onValueChange={v => setSelectedRole(v ?? 'member')}>
									<SelectTrigger>
										<SelectValue />
									</SelectTrigger>
									<SelectContent className="bg-card">
										{ROLES.filter(r => r !== 'owner').map(r => (
											<SelectItem key={r} value={r} className="capitalize">
												{r}
											</SelectItem>
										))}
									</SelectContent>
								</Select>
							</WizardField>

							<WizardField
								label="Permission group"
								hint="Grants this member fine-grained actions (e.g. deploy, manage domains).">
								<Select
									value={selectedGroupId}
									onValueChange={v => setSelectedGroupId(v ?? '')}>
									<SelectTrigger>
										<SelectValue placeholder="None (role only)" />
									</SelectTrigger>
									<SelectContent className="bg-card">
										{groups.map(g => (
											<SelectItem key={String(g.id)} value={String(g.id)}>
												{g.name}
											</SelectItem>
										))}
									</SelectContent>
								</Select>
							</WizardField>
						</div>

						<SheetFooter>
							<Button
								type="submit"
								disabled={isSubmitting || !inputEmail.trim()}
								className="h-10 gap-1.5 px-5">
								{actionType === 'direct'
									? isSubmitting
										? 'Adding…'
										: 'Add member'
									: isSubmitting
										? 'Sending…'
										: 'Send invite'}
							</Button>
						</SheetFooter>
					</form>
				</SheetContent>
			</Sheet>

			<Sheet
				open={Boolean(editingRoleUser)}
				onOpenChange={() => setEditingRoleUser(null)}>
				<SheetContent>
					<SheetHeader>
						<SheetTitle>Role &amp; permission group</SheetTitle>
						<SheetDescription className="font-mono">
							{editingRoleUser?.email}
						</SheetDescription>
					</SheetHeader>
					<div className="flex min-h-0 flex-1 flex-col gap-2 overflow-y-auto px-6 py-6">
						<p className="text-xs font-medium text-muted-foreground">
							Role
						</p>
						{ROLES.filter(r => r !== 'owner').map(r => {
							const current =
								(editingRoleUser?.role ?? '').toLowerCase() === r;
							return (
								<OptionRow
									key={r}
									selected={current}
									onClick={() => void handleUpdateRole(r)}
									title={r[0].toUpperCase() + r.slice(1)}
									subtitle={current ? 'Current role' : ROLE_DESC[r]}
								/>
							);
						})}

						<p className="mt-3 text-xs font-medium text-muted-foreground">
							Permission group
						</p>
						<OptionRow
							selected={!editingRoleUser?.group_id}
							onClick={() => void handleAssignGroup('')}
							title="None"
							subtitle="Access comes from the role only"
						/>
						{groups.map(g => {
							const current =
								BigInt(editingRoleUser?.group_id ?? 0) === g.id;
							return (
								<OptionRow
									key={String(g.id)}
									selected={current}
									onClick={() => void handleAssignGroup(String(g.id))}
									title={g.name}
									subtitle={
										current
											? 'Current group'
											: `${g.actions.length} actions granted`
									}
								/>
							);
						})}
					</div>
				</SheetContent>
			</Sheet>

			<ConfirmDeleteDialog
				open={Boolean(deleteTarget)}
				onOpenChange={() => setDeleteTarget(null)}
				title="Remove workspace member"
				description={
					<>
						They lose access to this organization and everything it runs.{' '}
						<span className="font-mono">{deleteTarget?.email}</span> can be
						invited back later.
					</>
				}
				confirmLabel={isDeleting ? 'Removing…' : 'Remove member'}
				busy={isDeleting}
				onConfirm={confirmDeleteUser}
			/>

			<PermissionOverridesSheet
				member={overridesTarget}
				onClose={() => setOverridesTarget(null)}
			/>
		</>
	);
}
