import {useEffect, useState} from 'react';
import {HexColorPicker} from 'react-colorful';
import {Check, Pipette} from 'lucide-react';
import {cn} from '#/api/utils';
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

const HEX = /^#(?:[0-9a-f]{3}|[0-9a-f]{6})$/i;

const normalize = (value: string) => {
	const v = value.trim().replace(/^#?/, '#');
	return HEX.test(v) ? v : null;
};

/**
 * Preset dots plus an inline free-choice panel. The panel hosts a
 * react-colorful saturation/hue field, skinned to the design tokens by the
 * `.tag-color-panel` rules in styles/index.css.
 */
export function TagColorPicker({
	color,
	onChange,
}: {
	color: string;
	onChange: (color: string) => void;
}) {
	const isPreset = PRESET_COLORS.some(
		c => c.toLowerCase() === color.toLowerCase(),
	);
	const [open, setOpen] = useState(!isPreset);
	const [draft, setDraft] = useState(color);

	useEffect(() => setDraft(color), [color]);

	const commitDraft = (value: string) => {
		const hex = normalize(value);
		if (hex) onChange(hex.toLowerCase());
	};

	return (
		<div>
			<div className="flex flex-wrap items-center gap-2">
				{PRESET_COLORS.map(c => {
					const selected = color.toLowerCase() === c.toLowerCase();
					return (
						<button
							key={c}
							type="button"
							onClick={() => {
								setOpen(false);
								onChange(c);
							}}
							aria-label={`Use ${c}`}
							aria-pressed={selected}
							className={cn(
								'flex size-8 shrink-0 cursor-pointer items-center justify-center rounded-full shadow-[inset_0_0_0_1px_rgba(0,0,0,0.08)] transition-transform dark:shadow-[inset_0_0_0_1px_rgba(255,255,255,0.12)]',
								selected
									? 'scale-105 ring-2 ring-foreground/70 ring-offset-2 ring-offset-popover'
									: 'hover:scale-105',
							)}
							style={{backgroundColor: c}}>
							{selected && (
								<Check className="size-3.5 text-white drop-shadow-[0_1px_1px_rgba(0,0,0,0.3)]" />
							)}
						</button>
					);
				})}

				<button
					type="button"
					onClick={() => setOpen(v => !v)}
					aria-expanded={open}
					title="Custom color"
					className={cn(
						'flex size-8 shrink-0 cursor-pointer items-center justify-center rounded-full shadow-[inset_0_0_0_1px_rgba(0,0,0,0.08)] transition-transform hover:scale-105 dark:shadow-[inset_0_0_0_1px_rgba(255,255,255,0.12)]',
						!isPreset &&
							'ring-2 ring-foreground/70 ring-offset-2 ring-offset-popover',
					)}
					style={{backgroundColor: isPreset ? 'var(--muted)' : color}}>
					<Pipette
						className={cn(
							'size-3.5',
							isPreset
								? 'text-muted-foreground'
								: 'text-white drop-shadow-[0_1px_1px_rgba(0,0,0,0.3)]',
						)}
					/>
				</button>
			</div>

			{open && (
				<div className="tag-color-panel mt-3 rounded-xl border border-border bg-secondary/40 p-3">
					<HexColorPicker
						color={normalize(color) ?? '#3b82f6'}
						onChange={onChange}
					/>
					<div className="mt-3 flex items-center gap-2">
						<span
							className="size-6 shrink-0 rounded-full shadow-[inset_0_0_0_1px_rgba(0,0,0,0.1)] dark:shadow-[inset_0_0_0_1px_rgba(255,255,255,0.14)]"
							style={{backgroundColor: color}}
						/>
						<Input
							value={draft}
							onChange={e => {
								setDraft(e.target.value);
								commitDraft(e.target.value);
							}}
							onBlur={() => setDraft(color)}
							spellCheck={false}
							maxLength={7}
							aria-label="Hex color"
							className="h-8 bg-background font-mono text-[13px] uppercase"
						/>
					</div>
				</div>
			)}
		</div>
	);
}
