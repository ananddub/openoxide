import {useState} from 'react';
import {Eye, EyeOff, Pencil, Check, X, RefreshCw} from 'lucide-react';
import {Field} from '#/components/shared/field';
import {Input, Button} from './credentials-fields';

/** Read-only password display that flips into an editable row with a reveal toggle, save and cancel control. */
export function PasswordEditorField({
	currentPassword,
	onSave,
	className = '',
}: {
	currentPassword: string;
	onSave: (next: string) => Promise<void>;
	className?: string;
}) {
	const [show, setShow] = useState(false);
	const [isEditing, setIsEditing] = useState(false);
	const [value, setValue] = useState(currentPassword);
	const [saving, setSaving] = useState(false);

	const startEdit = () => {
		setValue(currentPassword);
		setIsEditing(true);
	};

	const commit = async () => {
		setSaving(true);
		try {
			await onSave(value);
			setIsEditing(false);
		} finally {
			setSaving(false);
		}
	};

	return (
		<Field label="Password" className={className}>
			<div className="flex items-center gap-2">
				{isEditing ? (
					<>
						<Input
							type={show ? 'text' : 'password'}
							value={value}
							onChange={e => setValue(e.target.value)}
							placeholder="Enter new password"
							className="h-9  font-mono text-xs"
						/>
						<Button
							variant="outline"
							size="sm"
							type="button"
							onClick={() => setShow(!show)}
							className="h-9 shrink-0 px-2.5">
							{show ? (
								<EyeOff className="size-3.5" />
							) : (
								<Eye className="size-3.5" />
							)}
						</Button>
						<Button
							variant="default"
							size="sm"
							type="button"
							disabled={saving}
							onClick={commit}
							className="h-9 shrink-0 gap-1.5 bg-primary px-3 text-xs font-semibold text-primary-foreground">
							{saving ? (
								<RefreshCw className="size-3.5 animate-spin" />
							) : (
								<Check className="size-3.5" />
							)}{' '}
							Save
						</Button>
						<Button
							variant="ghost"
							size="sm"
							type="button"
							onClick={() => {
								setIsEditing(false);
								setValue(currentPassword);
							}}
							className="h-9 shrink-0 px-2 text-xs">
							<X className="size-3.5" />
						</Button>
					</>
				) : (
					<>
						<Input
							readOnly
							type={show ? 'text' : 'password'}
							value={currentPassword || ''}
							className="h-9 bg-muted/30 font-mono text-xs"
						/>
						<Button
							variant="outline"
							size="sm"
							type="button"
							onClick={() => setShow(!show)}
							className="h-9 shrink-0 px-2.5">
							{show ? (
								<EyeOff className="size-3.5" />
							) : (
								<Eye className="size-3.5" />
							)}
						</Button>
						<Button
							variant="outline"
							size="sm"
							type="button"
							onClick={startEdit}
							className="h-9 shrink-0 px-2.5">
							<Pencil className="size-3.5 text-muted-foreground" />
						</Button>
					</>
				)}
			</div>
		</Field>
	);
}
