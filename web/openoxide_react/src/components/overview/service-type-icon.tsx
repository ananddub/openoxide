import type {ReactElement} from 'react';
import {Box, Layers2, Database as DbIcon} from 'lucide-react';
import {
	PostgresqlIcon,
	MysqlIcon,
	MariadbIcon,
	MongodbIcon,
	RedisIcon,
	LibsqlIcon,
} from '#/components/icons/db-icons';

export const TYPE_DISPLAY_NAMES: Record<string, string> = {
	APP: 'Application',
	COMPOSE: 'Compose Stacks',
	DATABASE: 'Database',
	postgres: 'PostgreSQL',
	mysql: 'MySQL',
	mariadb: 'MariaDB',
	mongo: 'MongoDB',
	redis: 'Redis',
	libsql: 'Libsql',
};

const DB_ICONS: [string, () => ReactElement][] = [
	['postgres', () => <PostgresqlIcon className="size-5.5 shrink-0" />],
	['mysql', () => <MysqlIcon className="size-5.5 shrink-0" />],
	['mariadb', () => <MariadbIcon className="size-5.5 shrink-0" />],
	['mongo', () => <MongodbIcon className="size-5.5 shrink-0" />],
	['redis', () => <RedisIcon className="size-5.5 shrink-0" />],
	['libsql', () => <LibsqlIcon className="size-5.5 shrink-0" />],
];

/** Service row icon: app/compose glyphs or a per-database-kind logo. */
export function ServiceTypeIcon({
	type,
	dbKind,
}: {
	type: string;
	dbKind?: string;
}) {
	if (type === 'APP')
		return <Box className="size-5 shrink-0 text-foreground" />;
	if (type === 'COMPOSE')
		return <Layers2 className="size-5 shrink-0 text-foreground" />;
	const kind = (dbKind || '').toLowerCase();
	const match = DB_ICONS.find(([needle]) => kind.includes(needle));
	if (match) return match[1]();
	return <DbIcon className="size-5 shrink-0 text-foreground" />;
}
