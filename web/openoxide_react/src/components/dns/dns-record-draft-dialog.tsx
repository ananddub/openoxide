import {Button} from '#/components/ui/button';
import {
	Dialog,
	DialogContent,
	DialogDescription,
	DialogHeader,
	DialogTitle,
} from '#/components/ui/dialog';
import {Input} from '#/components/ui/input';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';

export type RecordDraft = {
	zoneId: string;
	recordId?: string;
	recordType: 'A' | 'CNAME';
	name: string;
	content: string;
	ttl: string;
};

/** A/CNAME record create/edit sub-dialog inside the zones viewer. */
export function DnsRecordDraftDialog({
	draft,
	saving,
	onChange,
	onClose,
	onSave,
}: {
	draft: RecordDraft | null;
	saving: boolean;
	onChange: (draft: RecordDraft) => void;
	onClose: () => void;
	onSave: () => void;
}) {
	return (
		<Dialog open={!!draft} onOpenChange={value => !value && onClose()}>
			<DialogContent className="sm:max-w-md">
				<DialogHeader>
					<DialogTitle>
						{draft?.recordId ? 'Edit Record' : 'Add Record'}
					</DialogTitle>
					<DialogDescription>
						Only A and CNAME records are supported.
					</DialogDescription>
				</DialogHeader>
				{draft && (
					<div className="grid gap-3">
						<Select
							value={draft.recordType}
							onValueChange={value =>
								onChange({...draft, recordType: value as 'A' | 'CNAME'})
							}>
							<SelectTrigger>
								<SelectValue />
							</SelectTrigger>
							<SelectContent>
								<SelectItem value="A">A</SelectItem>
								<SelectItem value="CNAME">CNAME</SelectItem>
							</SelectContent>
						</Select>
						<Input
							placeholder="app.example.com"
							value={draft.name}
							onChange={event =>
								onChange({...draft, name: event.target.value})
							}
						/>
						<Input
							placeholder={
								draft.recordType === 'A'
									? '203.0.113.10'
									: 'target.example.com'
							}
							value={draft.content}
							onChange={event =>
								onChange({...draft, content: event.target.value})
							}
						/>
						<Input
							type="number"
							placeholder="TTL (optional)"
							value={draft.ttl}
							onChange={event =>
								onChange({...draft, ttl: event.target.value})
							}
						/>
						<Button onClick={onSave} disabled={saving}>
							{saving
								? 'Saving...'
								: draft.recordId
									? 'Update Record'
									: 'Create Record'}
						</Button>
					</div>
				)}
			</DialogContent>
		</Dialog>
	);
}
