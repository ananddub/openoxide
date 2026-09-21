import {Loader2, Trash2} from 'lucide-react';
import {
	Dialog,
	DialogContent,
	DialogDescription,
	DialogFooter,
	DialogHeader,
	DialogTitle,
} from '#/components/ui/dialog';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import type {AccountSwitcherController} from './use-account-switcher';

/** Create / rename / delete-organization dialog for the account switcher. */
export function AccountOrgDialog({ctl}: {ctl: AccountSwitcherController}) {
	const {
		isFormOpen,
		setIsFormOpen,
		editingId,
		formName,
		setFormName,
		isSubmitting,
		confirmDelete,
		submitForm,
		remove,
	} = ctl;

	return (
		<Dialog open={isFormOpen} onOpenChange={setIsFormOpen}>
			<DialogContent className="sm:max-w-sm">
				<DialogHeader>
					<DialogTitle className="text-base font-semibold">
						{editingId ? 'Rename Organization' : 'New Organization'}
					</DialogTitle>
					<DialogDescription className="text-xs">
						{editingId
							? 'Update the workspace name.'
							: 'You will become the owner of the new workspace.'}
					</DialogDescription>
				</DialogHeader>
				<div className="space-y-2 py-2">
					<label className="text-[13px] font-medium text-foreground">
						Name
					</label>
					<Input
						value={formName}
						autoFocus
						onChange={event => setFormName(event.target.value)}
						onKeyDown={event => {
							if (event.key === 'Enter') void submitForm();
						}}
						placeholder="Acme Platform"
						className="h-10 text-xs"
					/>
				</div>
				<DialogFooter className="gap-2 sm:justify-between">
					{editingId && confirmDelete ? (
						<Button
							type="button"
							variant="ghost"
							size="sm"
							className="text-xs text-destructive"
							onClick={() => void remove(Number(editingId))}
							disabled={isSubmitting}>
							<Trash2 className="size-3.5" /> Click again to confirm delete
						</Button>
					) : (
						<span />
					)}
					<div className="flex gap-2">
						<Button
							type="button"
							variant="secondary"
							size="sm"
							className="h-9 text-xs"
							onClick={() => setIsFormOpen(false)}>
							Cancel
						</Button>
						<Button
							type="button"
							size="sm"
							className="h-9 text-xs"
							disabled={isSubmitting}
							onClick={() => void submitForm()}>
							{isSubmitting && (
								<Loader2 className="size-3.5 animate-spin" />
							)}
							{editingId ? 'Save' : 'Create'}
						</Button>
					</div>
				</DialogFooter>
			</DialogContent>
		</Dialog>
	);
}
