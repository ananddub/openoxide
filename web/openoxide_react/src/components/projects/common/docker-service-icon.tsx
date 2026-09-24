import {useState, useMemo} from 'react';
import {
	resolveLocalIcon,
	isValidDockerImage,
} from '#/lib/service-icon-resolver';
import {siDocker, siGit} from 'simple-icons';
import {
	PostgresqlIcon,
	MysqlIcon,
	MariadbIcon,
} from '#/components/icons/db/postgres-mysql';

const BRAND_ICONS: Record<
	string,
	(p: {className?: string}) => React.ReactElement
> = {
	postgresql: PostgresqlIcon,
	mysql: MysqlIcon,
	mariadb: MariadbIcon,
};

/** 3-tier hybrid Docker service icon:  1. match against local simple-icons set,  2. remote dashboard-icons CDN for valid image tags,  3. fallback to the Docker logo. */
export function DockerServiceIcon({
	image,
	size = 36,
}: {
	image: string;
	size?: number;
}) {
	const [failed, setFailed] = useState(false);

	const cleanImageName = useMemo(() => {
		if (!isValidDockerImage(image)) return '';
		let n = image.split(':')[0].split('@')[0];
		if (n.includes('/')) {
			const parts = n.split('/');
			n = parts[parts.length - 1];
		}
		return n.toLowerCase().trim();
	}, [image]);

	const known = useMemo(() => resolveLocalIcon(image), [image]);

	const tile: React.CSSProperties = {
		width: size,
		height: size,
		borderRadius: 9,
		flexShrink: 0,
		display: 'flex',
		alignItems: 'center',
		justifyContent: 'center',
	};

	if (known?.kind === 'git') {
		return (
			<div style={{...tile, background: `${known.color}18`}}>
				<svg
					viewBox="0 0 24 24"
					style={{width: 20, height: 20, fill: known.color}}>
					<path d={siGit.path} />
				</svg>
			</div>
		);
	}

	if (known?.brand && BRAND_ICONS[known.brand]) {
		const BrandIcon = BRAND_ICONS[known.brand];
		return (
			<div
				style={{
					...tile,
					background: `${known.color}18`,
					padding: size > 30 ? 5 : 3,
				}}>
				<BrandIcon className="size-full" />
			</div>
		);
	}

	if (known) {
		return (
			<div style={{...tile, background: `${known.color}18`}}>
				<svg
					viewBox={known.viewBox || '0 0 24 24'}
					style={{width: 20, height: 20, fill: known.color}}>
					{Array.isArray(known.paths) ? (
						known.paths.map((p, idx) => <path key={idx} d={p} />)
					) : (
						<path d={known.path} />
					)}
				</svg>
			</div>
		);
	}

	if (cleanImageName && !failed) {
		const iconUrl = `https://cdn.jsdelivr.net/gh/walkxcode/dashboard-icons/svg/${cleanImageName}.svg`;
		return (
			<div
				style={{
					...tile,
					background: 'rgba(255,255,255,0.05)',
					border: '1px solid rgba(255,255,255,0.08)',
					padding: 4,
					overflow: 'hidden',
				}}>
				<img
					src={iconUrl}
					alt={cleanImageName}
					onError={() => setFailed(true)}
					style={{width: '100%', height: '100%', objectFit: 'contain'}}
				/>
			</div>
		);
	}

	return (
		<div style={{...tile, background: '#1e90ff18'}}>
			<svg
				viewBox="0 0 24 24"
				style={{width: 20, height: 20, fill: '#1e90ff'}}>
				<path d={siDocker.path} />
			</svg>
		</div>
	);
}
