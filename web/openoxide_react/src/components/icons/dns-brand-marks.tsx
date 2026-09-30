import type {SimpleIcon} from 'simple-icons';
import {
	siAkamai,
	siAlibabacloud,
	siBaidu,
	siBunnydotnet,
	siCpanel,
	siExoscale,
	siGandi,
	siGcore,
	siGooglecloud,
	siHostinger,
	siHuawei,
	siIonos,
	siNamecheap,
	siNamesilo,
	siNetcup,
	siNetlify,
	siPlesk,
	siScaleway,
	siSpaceship,
	siVercel,
	siVultr,
	siYandexcloud,
} from 'simple-icons';

/** Glyph data for bridge providers that have no hand-drawn mark, keyed by wire value. */
const brandGlyphs: Record<string, SimpleIcon> = {
	ALIDNS: siAlibabacloud,
	BAIDUCLOUD: siBaidu,
	BUNNY: siBunnydotnet,
	CPANEL: siCpanel,
	EDGEDNS: siAkamai,
	EXOSCALE: siExoscale,
	GANDIV5: siGandi,
	GCORE: siGcore,
	GOOGLECLOUDDNS: siGooglecloud,
	HOSTINGER: siHostinger,
	HUAWEICLOUD: siHuawei,
	IONOS: siIonos,
	NAMECHEAP: siNamecheap,
	NAMESILO: siNamesilo,
	NETCUP: siNetcup,
	NETLIFY: siNetlify,
	PLESK: siPlesk,
	SCALEWAY: siScaleway,
	SPACESHIP: siSpaceship,
	VERCEL: siVercel,
	VULTR: siVultr,
	YANDEXCLOUD: siYandexcloud,
};

/** Stable per-provider hue so the tail of the list doesn't read as one grey block. */
const hueOf = (value: string) =>
	[...value].reduce(
		(hue, char) => (hue * 31 + char.charCodeAt(0)) % 360,
		7,
	);

const initialOf = (label: string, value: string) => {
	const head = label.replace(/\b(?:dns|cloud|net|com)\b/gi, '').trim();
	return (head[0] ?? value[0] ?? '?').toUpperCase();
};

/** Brand mark for a DNS provider: real artwork where simple-icons has it, else a tinted initial tile. */
export function DnsProviderMark({
	value,
	label,
	className = 'size-4 shrink-0',
}: {
	value: string;
	label: string;
	className?: string;
}) {
	const glyph = brandGlyphs[value.toUpperCase()];
	if (glyph)
		return (
			<svg viewBox="0 0 24 24" className={className} aria-hidden="true">
				<path fill={`#${glyph.hex}`} d={glyph.path} />
			</svg>
		);
	const hue = hueOf(value);
	// Drawn as an SVG <text> so the tile scales with whatever size class the caller passes.
	return (
		<svg viewBox="0 0 24 24" className={className} aria-hidden="true">
			<rect
				width="24"
				height="24"
				rx="5"
				fill={`hsl(${hue} 62% 50% / 0.18)`}
			/>
			<text
				x="12"
				y="12"
				textAnchor="middle"
				dominantBaseline="central"
				fontSize="13"
				fontWeight="700"
				fill={`hsl(${hue} 68% 62%)`}>
				{initialOf(label, value)}
			</text>
		</svg>
	);
}
