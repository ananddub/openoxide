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
		subtitle: 'They join with this role immediately',
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
		isSubmitting,
		handleCreateOrInvite,
		editingRoleUser,
		setEditingRoleUser,
		handleUpdateRole,
		deleteTarget,
		setDeleteTarget,
		isDeleting,
		confirmDeleteUser,
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
									className="bg-background"
								/>
							</WizardField>

							<WizardField
								label="Role"
								required
								hint={ROLE_DESC[selectedRole]}>
								<Select
									value={selectedRole}
									onValueChange={v => setSelectedRole(v ?? 'developer')}>
									<SelectTrigger className="bg-background">
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
						<SheetTitle>Change role</SheetTitle>
						<SheetDescription className="font-mono">
							{editingRoleUser?.email}
						</SheetDescription>
					</SheetHeader>
					<div className="flex min-h-0 flex-1 flex-col gap-2 overflow-y-auto px-6 py-6">
						{ROLES.filter(r => r !== 'owner').map(r => {
							const current = editingRoleUser?.role.toLowerCase() === r;
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
		</>
	);
}
