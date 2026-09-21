import {Box, Layers2, Database as DbIcon} from 'lucide-react';
import {
	PostgresqlIcon,
	MysqlIcon,
	MariadbIcon,
	MongodbIcon,
	RedisIcon,
	LibsqlIcon,
} from '#/components/icons/db-icons';

export function ServiceTypeIcon({
	type,
	dbKind,
}: {
	type: 'APP' | 'COMPOSE' | 'DATABASE';
	dbKind?: string;
}) {
	if (type === 'APP') return <Box className="size-4 text-foreground" />;
	if (type === 'COMPOSE')
		return <Layers2 className="size-4 text-foreground" />;
	const kind = (dbKind || '').toLowerCase();
	if (kind.includes('postgres'))
		return <PostgresqlIcon className="size-5 shrink-0" />;
	if (kind.includes('mysql'))
		return <MysqlIcon className="size-5 shrink-0" />;
	if (kind.includes('mariadb'))
		return <MariadbIcon className="size-5 shrink-0" />;
	if (kind.includes('mongo'))
		return <MongodbIcon className="size-5 shrink-0" />;
	if (kind.includes('redis'))
		return <RedisIcon className="size-5 shrink-0" />;
	if (kind.includes('libsql'))
		return <LibsqlIcon className="size-5 shrink-0" />;
	return <DbIcon className="size-4 text-foreground" />;
}

export function getStatusDotColor(flags: {
	isStopping: boolean;
	isRunning: boolean;
	isStartingOrDeploying: boolean;
	statusKey: string;
}): string {
	if (flags.isStopping) return 'bg-orange-500 animate-pulse';
	if (flags.isRunning) return 'bg-emerald-500';
	if (
		flags.statusKey.includes('error') ||
		flags.statusKey.includes('fail') ||
		flags.statusKey.includes('unhealthy') ||
		flags.statusKey.includes('crash')
	)
		return 'bg-rose-500';
	if (flags.isStartingOrDeploying) return 'bg-amber-500 animate-pulse';
	return 'bg-muted-foreground/40';
}
