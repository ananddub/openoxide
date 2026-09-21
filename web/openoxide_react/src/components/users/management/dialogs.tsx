import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {Label} from '#/components/ui/label';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import {
	Dialog,
	DialogContent,
	DialogDescription,
	DialogHeader,
	DialogTitle,
} from '#/components/ui/dialog';
import {
	AlertDialog,
	AlertDialogAction,
	AlertDialogContent,
	AlertDialogDescription,
	AlertDialogHeader,
	AlertDialogTitle,
} from '#/components/ui/alert-dialog';
import {
	ROLES,
	type UsersManagementController as Ctl,
} from './use-users-management';

/** All team dialogs: add/invite, change-role, and remove-member confirm. */
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
			<Dialog open={isCreateOpen} onOpenChange={setIsCreateOpen}>
				<DialogContent className="sm:max-w-md">
					<DialogHeader>
						<DialogTitle>Add or invite a user</DialogTitle>
						<DialogDescription>
							Add a member directly to your workspace or send an email
							invitation.
						</DialogDescription>
					</DialogHeader>

					<form
						onSubmit={handleCreateOrInvite}
						className="mt-2 flex flex-col gap-4">
						<div className="flex flex-col gap-1.5">
							<Label className="text-xs font-semibold text-foreground">
								Action
							</Label>
							<Select
								value={actionType}
								onValueChange={val =>
									setActionType(val as 'direct' | 'invite')
								}>
								<SelectTrigger className="h-9 text-xs">
									<SelectValue />
								</SelectTrigger>
								<SelectContent>
									<SelectItem value="direct" className="text-xs">
										Direct add member
									</SelectItem>
									<SelectItem value="invite" className="text-xs">
										Send email invitation
									</SelectItem>
								</SelectContent>
							</Select>
						</div>

						<div className="flex flex-col gap-1.5">
							<Label className="text-xs font-semibold text-foreground">
								Email address
							</Label>
							<Input
								type="email"
								placeholder="user@company.com"
								value={inputEmail}
								onChange={e => setInputEmail(e.target.value)}
								required
								className="h-9 text-sm"
							/>
						</div>

						<div className="flex flex-col gap-1.5">
							<Label className="text-xs font-semibold text-foreground">
								Role
							</Label>
							<Select
								value={selectedRole}
								onValueChange={v => setSelectedRole(v ?? 'developer')}>
								<SelectTrigger className="h-9 text-xs">
									<SelectValue />
								</SelectTrigger>
								<SelectContent>
									<SelectItem value="admin" className="text-xs">
										Admin
									</SelectItem>
									<SelectItem value="developer" className="text-xs">
										Developer
									</SelectItem>
									<SelectItem value="viewer" className="text-xs">
										Viewer
									</SelectItem>
								</SelectContent>
							</Select>
						</div>

						<Button
							type="submit"
							disabled={isSubmitting}
							className="mt-2 h-9 w-full text-sm">
							{isSubmitting
								? 'Saving...'
								: actionType === 'direct'
									? 'Add user'
									: 'Send invite'}
						</Button>
					</form>
				</DialogContent>
			</Dialog>

			<Dialog
				open={Boolean(editingRoleUser)}
				onOpenChange={() => setEditingRoleUser(null)}>
				<DialogContent className="sm:max-w-xs">
					<DialogHeader>
						<DialogTitle>Change role</DialogTitle>
						<DialogDescription className="font-mono text-xs">
							{editingRoleUser?.email}
						</DialogDescription>
					</DialogHeader>
					<div className="mt-2 flex flex-col gap-2">
						{ROLES.filter(r => r !== 'owner').map(r => (
							<Button
								key={r}
								variant={
									editingRoleUser?.role.toLowerCase() === r
										? 'default'
										: 'outline'
								}
								onClick={() => void handleUpdateRole(r)}
								className="h-9 justify-start capitalize">
								{r}
							</Button>
						))}
					</div>
				</DialogContent>
			</Dialog>

			<AlertDialog
				open={Boolean(deleteTarget)}
				onOpenChange={() => setDeleteTarget(null)}>
				<AlertDialogContent className="sm:max-w-md">
					<AlertDialogHeader>
						<AlertDialogTitle>Remove workspace member</AlertDialogTitle>
						<AlertDialogDescription>
							Are you sure you want to remove{' '}
							<span className="font-mono font-semibold text-foreground">
								{deleteTarget?.email}
							</span>
							?
						</AlertDialogDescription>
					</AlertDialogHeader>
					<AlertDialogAction
						onClick={confirmDeleteUser}
						disabled={isDeleting}
						className="h-9 w-full bg-destructive text-white hover:bg-destructive/90">
						{isDeleting ? 'Removing...' : 'Confirm remove'}
					</AlertDialogAction>
				</AlertDialogContent>
			</AlertDialog>
		</>
	);
}
