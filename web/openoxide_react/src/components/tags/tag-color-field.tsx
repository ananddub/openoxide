import {useRef} from 'react';
import {Check, Palette} from 'lucide-react';
import {Input} from '#/components/ui/input';

const PRESET_COLORS = [
	'#3b82f6', // Blue
	'#ef4444', // Red
	'#10b981', // Green
	'#f59e0b', // Amber
	'#8b5cf6', // Purple
	'#ec4899', // Pink
	'#6366f1', // Indigo
	'#06b6d4', // Cyan
	'#64748b', // Slate
];

/** Full color editor: native picker swatch, hex input and a preset row. Shared by the tag create / update dialog. */
export function TagColorField({
	color,
	onChange,
}: {
	color: string;
	onChange: (color: string) => void;
}) {
	const colorInputRef = useRef<HTMLInputElement>(null);

	return (
		<div className="flex flex-col gap-2">
			<label className="text-xs font-semibold text-foreground">
				Color (Optional)
			</label>
			<div className="flex items-center gap-3">
				<label
					className="relative flex size-9 shrink-0 cursor-pointer items-center justify-center rounded-lg shadow-2xs transition-opacity hover:opacity-80"
					style={{backgroundColor: color || '#3b82f6'}}
					onClick={() => colorInputRef.current?.click()}>
					<div className="flex items-center justify-center">
						{!color && <Palette className="h-4 w-4 text-white" />}
					</div>
					<input
						ref={colorInputRef}
						type="color"
						className="pointer-events-none absolute top-0 left-0 size-9 opacity-0"
						value={color || '#3b82f6'}
						onChange={e => onChange(e.target.value)}
					/>
				</label>
				<div className="flex-1">
					<Input
						placeholder="#3b82f6"
						value={color || ''}
						onChange={e => {
							const value = e.target.value;
							if (value.startsWith('#') || value === '') {
								onChange(value);
							}
						}}
						className="h-9 font-mono text-xs"
					/>
				</div>
			</div>

			{/* Quick Preset Colors */}
			<div className="flex flex-wrap items-center gap-1.5 pt-1">
				{PRESET_COLORS.map(c => (
					<button
						key={c}
						type="button"
						onClick={() => onChange(c)}
						className={`flex size-6 items-center justify-center rounded-full transition-transform ${
							color === c
								? 'scale-110 ring-2 ring-primary'
								: 'opacity-80 hover:scale-105 hover:opacity-100'
						}`}
						style={{backgroundColor: c}}>
						{color === c && (
							<Check className="h-3 w-3 text-white drop-shadow-xs" />
						)}
					</button>
				))}
			</div>
		</div>
	);
}
