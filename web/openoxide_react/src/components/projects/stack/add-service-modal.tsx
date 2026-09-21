import {useState} from 'react';
import {Check} from 'lucide-react';
import {
	Sheet,
	SheetContent,
	SheetDescription,
	SheetFooter,
	SheetHeader,
	SheetTitle,
} from '#/components/ui/sheet';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {Label} from '#/components/ui/label';
import {StackTemplateGrid} from './stack-template-grid';
import type {Template} from './stack-templates';

// ─── Add service side panel (Vercel "Add New" pattern) ───────────────────────

export function AddServiceModal({
	onAdd,
	onClose,
}: {
	onAdd: (t: Template, name: string) => void;
	onClose: () => void;
}) {
	const [selected, setSelected] = useState<Template | null>(null);
	const [name, setName] = useState('');

	const confirm = () => {
		if (!selected) return;
		const n =
			name.trim() || selected.label.toLowerCase().replace(/\s/g, '-');
		onAdd(selected, n);
		onClose();
	};

	return (
		<Sheet open onOpenChange={open => !open && onClose()}>
			<SheetContent>
				<SheetHeader>
					<SheetTitle>Add Service</SheetTitle>
					<SheetDescription>
						Pick a stack template to add to this environment.
					</SheetDescription>
				</SheetHeader>

				<div className="flex-1 overflow-y-auto px-6 py-5">
					<StackTemplateGrid
						selected={selected}
						onSelect={t => {
							setSelected(t);
							setName(t.label.toLowerCase());
						}}
					/>

					{selected && (
						<div className="mt-5 flex flex-col gap-1.5">
							<Label htmlFor="service-name">Service name</Label>
							<Input
								id="service-name"
								autoFocus
								value={name}
								onChange={e => setName(e.target.value)}
								onKeyDown={e => e.key === 'Enter' && confirm()}
								placeholder={selected.label.toLowerCase()}
							/>
						</div>
					)}
				</div>

				<SheetFooter>
					<Button variant="outline" onClick={onClose}>
						Cancel
					</Button>
					<Button onClick={confirm} disabled={!selected} className="gap-1.5">
						<Check className="size-3.5" /> Add Service
					</Button>
				</SheetFooter>
			</SheetContent>
		</Sheet>
	);
}
