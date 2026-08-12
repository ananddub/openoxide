import React, {useMemo} from 'react';

const TAG_COLORS = [
	'hsl(200, 80%, 50%)', // Vibrant Sky Blue
	'hsl(150, 80%, 45%)', // Vibrant Mint
	'hsl(300, 75%, 55%)', // Vibrant Purple
	'hsl(25, 85%, 55%)',  // Vibrant Peach
	'hsl(270, 80%, 55%)', // Vibrant Lavender
	'hsl(340, 80%, 55%)', // Vibrant Rose
	'hsl(170, 80%, 45%)', // Vibrant Aqua
	'hsl(50, 85%, 45%)',  // Vibrant Yellow
	'hsl(235, 80%, 60%)', // Vibrant Periwinkle
	'hsl(10, 80%, 55%)',  // Vibrant Coral
	'hsl(180, 80%, 45%)', // Vibrant Turquoise
	'hsl(320, 80%, 55%)', // Vibrant Orchid
	'hsl(90, 80%, 45%)',  // Vibrant Lime
	'hsl(260, 80%, 60%)', // Vibrant Amethyst
	'hsl(30, 85%, 55%)',  // Vibrant Orange
] as const;

function hashCode(str: string): number {
	let hash = 0;
	for (let i = 0; i < str.length; i++) {
		hash = (hash << 5) - hash + str.charCodeAt(i);
		hash |= 0;
	}
	return hash;
}

export const ContainerTag = React.memo(({name}: {name: string}) => {
	const color = useMemo(() => {
		const index = Math.abs(hashCode(name)) % TAG_COLORS.length;
		return TAG_COLORS[index];
	}, [name]);

	return (
		<span
			className="inline-flex items-center justify-center w-28 sm:w-36 px-2 py-[0.15rem] rounded-xs text-[11px] font-mono font-semibold text-white truncate shrink-0 select-none shadow-xs text-center"
			style={{backgroundColor: color}}
			title={`Container: ${name}`}
		>
			{name}
		</span>
	);
});
