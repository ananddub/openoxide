import {useState} from 'react';
import {Eye, EyeOff} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {Field} from '#/components/shared/field';

/** Read-only display field — Geist muted surface, mono value.
 *  Theme-aware so it never renders as a black slab in the light theme. */
const READONLY_CLS =
	'h-9 border-input bg-muted/40 font-mono text-xs text-foreground read-only:cursor-default';

/** Read-only labelled input — house `Field` grammar (label + control). */
export function ReadonlyField({
	label,
	value,
	className = '',
}: {
	label: string;
	value: string;
	className?: string;
}) {
	return (
		<Field label={label} className={className}>
			<Input readOnly value={value} className={READONLY_CLS} />
		</Field>
	);
}

/** Read-only input with a reveal/hide toggle (password or URL). */
export function SecretField({
	label,
	value,
	className = '',
}: {
	label: string;
	value: string;
	className?: string;
}) {
	const [show, setShow] = useState(false);
	return (
		<Field label={label} className={className}>
			<div className="flex items-center gap-2">
				<Input
					readOnly
					type={show ? 'text' : 'password'}
					value={value}
					className={READONLY_CLS}
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
			</div>
		</Field>
	);
}

export {Input, Button};
