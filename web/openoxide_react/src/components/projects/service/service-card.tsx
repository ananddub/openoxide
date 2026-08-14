import {Box, Layers2, Database as DbIcon} from 'lucide-react';
import {Link} from '@tanstack/react-router';
import {cn} from '#/api/utils';
import {
	PostgresqlIcon,
	MysqlIcon,
	MariadbIcon,
	MongodbIcon,
	RedisIcon,
	LibsqlIcon,
} from '#/components/icons/db-icons';

interface ServiceCardProps {
	projectId: number;
	type: 'APP' | 'COMPOSE' | 'DATABASE';
	id: number;
	name: string;
	subtitle: string;
	status: string;
	createdAt: number;
	dbKind?: string;
}

export function ServiceCard({
	projectId,
	type,
	id,
	name,
	subtitle,
	status,
	createdAt,
	dbKind,
}: ServiceCardProps) {
	const getStatusDotColor = (status: string) => {
		const s = status?.toLowerCase() || '';
		if (s.includes('stopping') || s.includes('cancelling')) {
			return 'bg-orange-500 animate-pulse';
		}
		if (s.includes('running') || s.includes('active') || s.includes('healthy') || s.includes('up')) {
			return 'bg-emerald-500';
		}
		if (s.includes('error') || s.includes('fail') || s.includes('unhealthy') || s.includes('crash')) {
			return 'bg-rose-500';
		}
		if (s.includes('loading') || s.includes('deploying') || s.includes('starting') || s.includes('building')) {
			return 'bg-amber-500 animate-pulse';
		}
		return 'bg-muted-foreground/40';
	};

	const getIcon = () => {
		if (type === 'APP') {
			return <Box className="size-4 text-primary" />;
		}
		if (type === 'COMPOSE') {
			return <Layers2 className="size-4 text-amber-500" />;
		}
		const kind = (dbKind || '').toLowerCase();
		if (kind.includes('postgres')) return <PostgresqlIcon className="size-5 shrink-0" />;
		if (kind.includes('mysql')) return <MysqlIcon className="size-5 shrink-0" />;
		if (kind.includes('mariadb')) return <MariadbIcon className="size-5 shrink-0" />;
		if (kind.includes('mongo')) return <MongodbIcon className="size-5 shrink-0" />;
		if (kind.includes('redis')) return <RedisIcon className="size-5 shrink-0" />;
		if (kind.includes('libsql')) return <LibsqlIcon className="size-5 shrink-0" />;

		return <DbIcon className="size-4 text-emerald-500" />;
	};

	const getTargetRoute = () => {
		if (type === 'APP') {
			return {
				to: '/projects/$id/app/$appId',
				params: { id: String(projectId), appId: String(id) },
			};
		}
		if (type === 'COMPOSE') {
			return {
				to: '/projects/$id/compose/$composeId',
				params: { id: String(projectId), composeId: String(id) },
			};
		}
		return {
			to: '/projects/$id/database/$dbId',
			params: { id: String(projectId), dbId: String(id) },
			search: { kind: dbKind || 'postgres' },
		};
	};

	const target = getTargetRoute();

	return (
		<Link
			to={target.to as any}
			params={target.params as any}
			search={(target as any).search}
			preload="intent"
			className="w-full bg-card border border-border hover:border-border/80 transition-all duration-200 rounded-xl p-4 flex flex-col justify-between gap-3.5 cursor-pointer group shadow-2xs block text-left no-underline"
		>
			<div className="flex items-center gap-3">
				<div className="size-9 rounded-lg bg-muted/40 flex items-center justify-center shrink-0 relative border border-border/40">
					{getIcon()}
					<span className={cn('absolute -top-0.5 -right-0.5 size-2 rounded-full border-2 border-card', getStatusDotColor(status))} />
				</div>
				<div className="min-w-0">
					<p className="font-bold text-xs text-foreground truncate group-hover:text-primary transition-colors leading-snug">{name}</p>
					<p className="text-[10px] text-muted-foreground font-mono uppercase tracking-wider">{subtitle}</p>
				</div>
			</div>

			<div className="flex items-center justify-between border-t border-border/40 pt-2.5">
				<span className="flex items-center gap-1.5 text-[10px] font-bold text-muted-foreground uppercase tracking-wider">
					<span className={cn('size-1.5 rounded-full', getStatusDotColor(status))} />
					{status?.toLowerCase() || 'idle'}
				</span>
				<span className="text-[10px] text-muted-foreground/70 font-mono">
					{new Date(createdAt * 1000).toLocaleDateString(undefined, {
						day: '2-digit',
						month: 'short',
					})}
				</span>
			</div>
		</Link>
	);
}
