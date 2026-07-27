import {Box, Layers2, Database as DbIcon} from 'lucide-react';
import {useNavigate} from '@tanstack/react-router';
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
	const navigate = useNavigate();

	const getStatusDotColor = (status: string) => {
		const s = status?.toLowerCase() || '';
		if (s.includes('running') || s.includes('active') || s.includes('healthy') || s.includes('up')) {
			return 'bg-emerald-500';
		}
		if (s.includes('error') || s.includes('fail') || s.includes('unhealthy') || s.includes('crash')) {
			return 'bg-rose-500';
		}
		if (s.includes('loading') || s.includes('deploying') || s.includes('starting') || s.includes('building')) {
			return 'bg-amber-500 animate-pulse';
		}
		return 'bg-zinc-500';
	};

	const getIcon = () => {
		if (type === 'APP') {
			return <Box className="size-4.5 text-primary" />;
		}
		if (type === 'COMPOSE') {
			return <Layers2 className="size-4.5 text-secondary-foreground" />;
		}
		const kind = (dbKind || '').toLowerCase();
		if (kind.includes('postgres')) return <PostgresqlIcon className="size-6 shrink-0" />;
		if (kind.includes('mysql')) return <MysqlIcon className="size-6 shrink-0" />;
		if (kind.includes('mariadb')) return <MariadbIcon className="size-6 shrink-0" />;
		if (kind.includes('mongo')) return <MongodbIcon className="size-6 shrink-0" />;
		if (kind.includes('redis')) return <RedisIcon className="size-6 shrink-0" />;
		if (kind.includes('libsql')) return <LibsqlIcon className="size-6 shrink-0" />;

		return <DbIcon className="size-5 text-muted-foreground" />;
	};

	const getBg = () => {
		if (type === 'APP') return 'bg-primary/5';
		return 'bg-secondary/60';
	};

	const handleNavigate = () => {
		if (type === 'APP') {
			navigate({ to: `/projects/${projectId}/app/${id}` as any });
		} else if (type === 'COMPOSE') {
			navigate({ to: `/projects/${projectId}/compose/${id}` as any });
		} else if (type === 'DATABASE') {
			navigate({
				to: `/projects/${projectId}/database/${id}` as any,
				search: {kind: dbKind || 'postgres'} as any,
			});
		}
	};

	return (
		<div
			onClick={handleNavigate}
			className="w-full bg-card/45 border border-border/80 hover:border-primary/20 hover:bg-card/75 transition-all duration-300 rounded-xl p-5 flex flex-col gap-4 cursor-pointer group shadow-sm backdrop-blur-[2px]">
			<div className="flex items-center gap-3">
				<div className={cn('size-10 rounded-lg flex items-center justify-center shrink-0 relative border border-border/40', getBg())}>
					{getIcon()}
					<span className={cn('absolute -top-0.5 -right-0.5 size-2.5 rounded-full border-2 border-card', getStatusDotColor(status))} />
				</div>
				<div className="min-w-0">
					<p className="font-semibold text-sm text-foreground truncate group-hover:text-primary transition-colors leading-snug">{name}</p>
					<p className="text-[11px] text-muted-foreground mt-0.5 font-medium uppercase tracking-wider">{subtitle}</p>
				</div>
			</div>

			<div className="flex items-center justify-between border-t border-border/40 pt-3">
				<span className="flex items-center gap-1.5 text-[11px] font-semibold text-muted-foreground uppercase tracking-wider">
					<span className={cn('size-1.5 rounded-full', getStatusDotColor(status))} />
					{status?.toLowerCase() || 'idle'}
				</span>
				<span className="text-[11px] text-muted-foreground/60 font-medium">
					{new Date(createdAt * 1000).toLocaleDateString(undefined, {
						day: '2-digit',
						month: 'short',
					})}
				</span>
			</div>
		</div>
	);
}
