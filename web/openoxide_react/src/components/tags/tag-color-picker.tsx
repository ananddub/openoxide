import {Input} from '#/components/ui/input';

/** Preset swatch palette shared by every tag color picker. */
export const PRESET_COLORS = [
	'#3b82f6', // Blue
	'#10b981', // Green
	'#f59e0b', // Amber
	'#ef4444', // Red
	'#8b5cf6', // Purple
	'#ec4899', // Pink
	'#06b6d4', // Cyan
	'#64748b', // Slate
];

/** Swatch row + hex input for choosing a tag color. */
export function TagColorPicker({
	color,
	onChange,
}: {
	color: string;
	onChange: (color: string) => void;
}) {
	return (
		<>
			<div className="flex flex-wrap items-center gap-2">
				{PRESET_COLORS.map(c => (
					<button
						key={c}
						type="button"
						onClick={() => onChange(c)}
						aria-label={`Use ${c}`}
						className={`size-6 cursor-pointer rounded-full transition-transform ${
							color === c
								? 'scale-125 ring-2 ring-primary/60'
								: 'opacity-75 hover:opacity-100'
						}`}
						style={{backgroundColor: c}}
					/>
				))}
			</div>
			<div className="mt-1 flex items-center gap-2">
				<span className="font-mono text-[11px] text-muted-foreground">
					Hex:
				</span>
				<Input
					type="text"
					value={color}
					onChange={e => onChange(e.target.value)}
					className="h-8 w-28 bg-muted/20 font-mono text-[11px]"
				/>
			</div>
		</>
	);
}
