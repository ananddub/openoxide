import {ShieldAlert} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {Label} from '#/components/ui/label';
import {Switch} from '#/components/ui/switch';
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
	DialogFooter,
	DialogHeader,
	DialogTitle,
} from '#/components/ui/dialog';
import type {NotificationPayload} from '#/api/live';
import {
	PROVIDERS,
	providerMeta,
	type ProviderId,
} from './notification-providers';
import {TRIGGERS, type DraftState} from './notification-triggers';
import {useNotificationSave} from './use-notification-save';

export function NotificationChannelDialog({
	draft,
	setDraft,
	editing,
	orgId,
	onClose,
}: {
	draft: DraftState | null;
	setDraft: (d: DraftState | null) => void;
	editing: NotificationPayload | null;
	orgId: bigint;
	onClose: () => void;
}) {
	const provider = draft ? providerMeta(draft.provider) : null;
	const save = useNotificationSave({
		draft,
		editing,
		provider,
		orgId,
		onSaved: onClose,
	});
	const patch = (d: Partial<DraftState>) =>
		draft && setDraft({...draft, ...d});

	return (
		<Dialog
			open={draft !== null}
			onOpenChange={open => !open && onClose()}>
			<DialogContent className="max-h-[85vh] overflow-y-auto sm:max-w-lg">
				<DialogHeader>
					<DialogTitle>
						{editing ? 'Edit channel' : 'Add notification channel'}
					</DialogTitle>
					<DialogDescription>
						{editing
							? 'Rename the channel or change which events it fires on.'
							: 'Pick a provider, fill its connection details, then choose events.'}
					</DialogDescription>
				</DialogHeader>

				{draft && (
					<div className="flex flex-col gap-4">
						{/* Name */}
						<div className="flex flex-col gap-1.5">
							<Label className="text-xs font-semibold">Name</Label>
							<Input
								placeholder="e.g. Ops alerts"
								value={draft.name}
								onChange={e => patch({name: e.target.value})}
								className="h-9 text-sm"
							/>
						</div>

						{/* Provider picker + description */}
						{!editing && (
							<div className="flex flex-col gap-1.5">
								<Label className="text-xs font-semibold">Provider</Label>
								<Select
									value={draft.provider}
									onValueChange={v =>
										patch({
											provider: (v as ProviderId) ?? 'DISCORD',
											config: {},
										})
									}>
									<SelectTrigger className="h-9 text-sm">
										<SelectValue />
									</SelectTrigger>
									<SelectContent>
										{PROVIDERS.map(p => (
											<SelectItem
												key={p.id}
												value={p.id}
												className="text-xs">
												{p.label}
											</SelectItem>
										))}
									</SelectContent>
								</Select>
								{provider && (
									<p className="text-xs text-muted-foreground">
										{provider.description}
									</p>
								)}
							</div>
						)}

						{/* Provider config fields */}
						{!editing && provider && provider.fields.length > 0 && (
							<div className="flex flex-col gap-3 rounded-xl bg-muted/20 p-4">
								{provider.fields.map(f => (
									<label key={f.key} className="flex flex-col gap-1.5">
										<span className="text-xs font-medium text-foreground/80">
											{f.label}
										</span>
										<Input
											type={f.type ?? 'text'}
											placeholder={f.placeholder}
											value={draft.config[f.key] ?? ''}
											onChange={e =>
												patch({
													config: {
														...draft.config,
														[f.key]: e.target.value,
													},
												})
											}
											className="h-9 bg-muted/40 text-sm"
										/>
									</label>
								))}
							</div>
						)}

						{/* Event triggers */}
						<div>
							<p className="mb-2 flex items-center gap-1.5 text-xs font-semibold text-foreground">
								<ShieldAlert className="size-3.5 text-muted-foreground" />
								Fire on
							</p>
							<div className="divide-y divide-border/40 rounded-xl bg-muted/20">
								{TRIGGERS.map(t => (
									<div
										key={t.key}
										className="flex items-center justify-between px-4 py-2.5">
										<span className="text-sm text-foreground">
											{t.label}
										</span>
										<Switch
											checked={draft.triggers[t.key] === true}
											onCheckedChange={v =>
												patch({
													triggers: {...draft.triggers, [t.key]: v},
												})
											}
										/>
									</div>
								))}
							</div>
						</div>

						<DialogFooter>
							<Button type="button" variant="ghost" onClick={onClose}>
								Cancel
							</Button>
							<Button type="button" onClick={() => void save()}>
								{editing ? 'Save changes' : 'Create channel'}
							</Button>
						</DialogFooter>
					</div>
				)}
			</DialogContent>
		</Dialog>
	);
}
