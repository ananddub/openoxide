import {Database} from 'lucide-react';
import {
	PostgresqlIcon,
	MysqlIcon,
	MariadbIcon,
	MongodbIcon,
	RedisIcon,
	LibsqlIcon,
} from '#/components/icons/db-icons';

/** Provider mark for the rail's identity tile, by detected db kind. */
export function kindIcon(kind: string) {
	const k = kind.toLowerCase();
	if (k.includes('postgres')) return <PostgresqlIcon className="size-5" />;
	if (k.includes('mysql')) return <MysqlIcon className="size-5" />;
	if (k.includes('mariadb')) return <MariadbIcon className="size-5" />;
	if (k.includes('mongo')) return <MongodbIcon className="size-5" />;
	if (k.includes('redis')) return <RedisIcon className="size-5" />;
	if (k.includes('libsql')) return <LibsqlIcon className="size-5" />;
	return <Database className="size-4 text-muted-foreground" />;
}
