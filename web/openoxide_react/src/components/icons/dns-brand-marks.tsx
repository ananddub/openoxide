import type {SimpleIcon} from 'simple-icons';
import {siHostinger, siSpaceship} from 'simple-icons';

/** The supported providers with no hand-drawn mark in `provider-icons`. */
const brandGlyphs: Record<string, SimpleIcon> = {
	HOSTINGER: siHostinger,
	SPACESHIP: siSpaceship,
};

export function DnsProviderMark({
	value,
	className = 'size-4 shrink-0',
}: {
	value: string;
	className?: string;
}) {
	const glyph = brandGlyphs[value.toUpperCase()];
	if (!glyph) return null;
	return (
		<svg viewBox="0 0 24 24" className={className} aria-hidden="true">
			<path fill={`#${glyph.hex}`} d={glyph.path} />
		</svg>
	);
}
