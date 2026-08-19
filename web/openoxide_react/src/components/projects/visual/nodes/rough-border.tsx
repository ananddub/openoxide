import {useEffect, useRef} from 'react';
import rough from 'roughjs';

interface RoughBorderProps {
	width: number;
	height: number;
	color: string;
	dashed?: boolean;
	roughness?: number;
	fill?: string;
}

export function RoughBorder({
	width,
	height,
	color,
	dashed = false,
	roughness = 1.4,
	fill,
}: RoughBorderProps) {
	const svgRef = useRef<SVGSVGElement>(null);

	useEffect(() => {
		if (!svgRef.current) return;
		svgRef.current.innerHTML = '';
		const rc = rough.svg(svgRef.current);
		const rect = rc.rectangle(3, 3, width - 6, height - 6, {
			stroke: color,
			strokeWidth: 2,
			roughness,
			fill: fill || 'none',
			fillStyle: fill ? 'solid' : undefined,
			strokeLineDash: dashed ? [8, 6] : undefined,
		});
		svgRef.current.appendChild(rect);
	}, [width, height, color, dashed, roughness, fill]);

	return (
		<svg
			ref={svgRef}
			width={width}
			height={height}
			className="pointer-events-none absolute inset-0"
			style={{overflow: 'visible'}}
		/>
	);
}
