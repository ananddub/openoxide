import {
	siPostgresql,
	siMysql,
	siRedis,
	siMongodb,
	siNginx,
	siDocker,
	siNodedotjs,
} from 'simple-icons';
import type {StackServiceData} from './stack-service-card';

export interface Template {
	key: string;
	label: string;
	sub: string;
	icon: string;
	color: string;
	data: Partial<StackServiceData>;
}

// ─── Preset templates ─────────────────────────────────────────────────────────

export const TEMPLATES: Template[] = [
	{
		key: 'nginx',
		label: 'Nginx',
		sub: 'Web server / proxy',
		icon: siNginx.path,
		color: '#009639',
		data: {
			image: 'nginx:alpine',
			volumes: ['/etc/nginx/conf.d'],
			envVars: [],
		},
	},
	{
		key: 'node',
		label: 'Node.js',
		sub: 'Custom app',
		icon: siNodedotjs.path,
		color: '#339933',
		data: {
			image: 'node:20-alpine',
			volumes: [],
			envVars: [{key: 'NODE_ENV', value: 'production'}],
		},
	},
	{
		key: 'postgres',
		label: 'PostgreSQL',
		sub: 'Managed database',
		icon: siPostgresql.path,
		color: '#336791',
		data: {
			image: 'postgres:16-alpine',
			volumes: ['/var/lib/postgresql/data'],
			envVars: [
				{key: 'POSTGRES_DB', value: 'app'},
				{key: 'POSTGRES_USER', value: 'postgres'},
				{key: 'POSTGRES_PASSWORD', value: 'secret'},
			],
		},
	},
	{
		key: 'mysql',
		label: 'MySQL',
		sub: 'Managed database',
		icon: siMysql.path,
		color: '#4479A1',
		data: {
			image: 'mysql:8',
			volumes: ['/var/lib/mysql'],
			envVars: [
				{key: 'MYSQL_DATABASE', value: 'app'},
				{key: 'MYSQL_ROOT_PASSWORD', value: 'secret'},
			],
		},
	},
	{
		key: 'redis',
		label: 'Redis',
		sub: 'Cache / queue',
		icon: siRedis.path,
		color: '#FF4438',
		data: {image: 'redis:7-alpine', volumes: ['/data'], envVars: []},
	},
	{
		key: 'mongodb',
		label: 'MongoDB',
		sub: 'Document database',
		icon: siMongodb.path,
		color: '#47A248',
		data: {
			image: 'mongo:7',
			volumes: ['/data/db'],
			envVars: [{key: 'MONGO_INITDB_DATABASE', value: 'app'}],
		},
	},
	{
		key: 'custom',
		label: 'Custom',
		sub: 'Any Docker image',
		icon: siDocker.path,
		color: '#1e90ff',
		data: {image: 'your-image:latest', volumes: [], envVars: []},
	},
];
