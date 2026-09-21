/** Minimal pill toggle used by deploy/build settings cards. */
export function ToggleSwitch({
	checked,
	onCheckedChange,
	disabled,
	title,
}: {
	checked: boolean;
	onCheckedChange: (next: boolean) => void;
	disabled?: boolean;
	title?: string;
}) {
	return (
		<button
			type="button"
			title={title}
			disabled={disabled}
			onClick={() => onCheckedChange(!checked)}
			className={`relative inline-flex h-4.5 w-8 shrink-0 items-center rounded-full transition-colors ${
				checked && !disabled ? 'bg-primary' : 'bg-muted'
			} ${disabled ? 'cursor-not-allowed opacity-50' : 'cursor-pointer'}`}>
			<span
				className={`pointer-events-none block h-3 w-3 rounded-full bg-background shadow-lg transition-transform ${checked ? 'translate-x-4.5' : 'translate-x-0.5'}`}
			/>
		</button>
	);
}
