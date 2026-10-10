import {matchOsIcon} from '#/lib/os-icon';
import {cn} from '#/api/utils';

/** Full-color OS brand glyph, sized by `className`. */
export function OsGlyph({
	distro,
	os,
	kernel,
	className,
}: {
	distro?: string | null;
	os?: string | null;
	kernel?: string | null;
	className?: string;
}) {
	const icon = matchOsIcon(distro, os, kernel);
	return (
		<svg
			viewBox="0 0 24 24"
			role="img"
			aria-label={icon.name}
			fill={icon.color}
			className={cn('shrink-0', className)}>
			{icon.paths.map((d, i) => (
				<path key={i} d={d} />
			))}
		</svg>
	);
}
