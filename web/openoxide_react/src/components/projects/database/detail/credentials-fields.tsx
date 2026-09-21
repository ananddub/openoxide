import {useState} from 'react';
import {Eye, EyeOff} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';

const READONLY_CLS = 'h-9 bg-muted/30 font-mono text-xs';

function FieldLabel({children}: {children: React.ReactNode}) {
	return (
		<label className="text-xs font-semibold text-muted-foreground">
			{children}
		</label>
	);
}

/** Read-only labelled input. */
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
		<div className={`flex flex-col gap-1.5 ${className}`}>
			<FieldLabel>{label}</FieldLabel>
			<Input readOnly value={value} className={READONLY_CLS} />
		</div>
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
		<div className={`flex flex-col gap-1.5 ${className}`}>
			<FieldLabel>{label}</FieldLabel>
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
		</div>
	);
}

export {FieldLabel, Input, Button};
