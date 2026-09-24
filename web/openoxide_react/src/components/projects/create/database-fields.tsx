import type {ComponentType} from 'react';
import {Check} from 'lucide-react';
import {Input} from '#/components/ui/input';
import {cn} from '#/api/utils';
import {
	PostgresqlIcon,
	MysqlIcon,
	MariadbIcon,
	MongodbIcon,
	RedisIcon,
	LibsqlIcon,
} from '#/components/icons/db-icons';
import {DB_KINDS} from './constants';
import {Field} from './field';
import type {useCreateService} from './use-create-service';

type Form = ReturnType<typeof useCreateService>;
type DbKindOption = (typeof DB_KINDS)[number];

const DB_ICONS: Record<string, ComponentType<{className?: string}>> = {
	postgres: PostgresqlIcon,
	mysql: MysqlIcon,
	mariadb: MariadbIcon,
	mongodb: MongodbIcon,
	redis: RedisIcon,
	libsql: LibsqlIcon,
};

/** Vertical provider list — Vercel "Browse Storage" pattern: full-width rows with logo + name + description, filled blue check on the selected row. */
export function DbEngineList({
	selected,
	onSelect,
	view = 'list',
}: {
	selected: string;
	onSelect: (k: DbKindOption) => void;
	view?: 'list' | 'grid';
}) {
	return (
		<div
			className={cn(
				view === 'list' ? 'flex flex-col gap-2' : 'grid grid-cols-2 gap-2',
			)}>
			{DB_KINDS.map(k => {
				const Icon = DB_ICONS[k.id];
				const isSelected = selected === k.id;
				return (
					<button
						key={k.id}
						type="button"
						onClick={() => onSelect(k)}
						className={cn(
							'flex w-full items-center gap-3.5 rounded-xl border px-4 py-3.5 text-start transition-all duration-150',
							view === 'grid' && 'flex-col items-start gap-2.5',
							isSelected
								? 'border-primary/40 bg-primary/10'
								: 'border-border/50 hover:border-border hover:bg-muted/30',
						)}>
						<span className="flex size-8 shrink-0 items-center justify-center">
							{Icon && <Icon className="size-7" />}
						</span>
						<span className="min-w-0 flex-1">
							<span className="block text-sm font-medium text-foreground">
								{k.label}
							</span>
							<span
								className={cn(
									'block text-[13px] text-muted-foreground',
									view === 'list' ? 'truncate' : 'line-clamp-2',
								)}>
								{k.description}
							</span>
						</span>
						{isSelected && (
							<span className="flex size-5 shrink-0 animate-in items-center justify-center rounded-full bg-primary text-primary-foreground duration-150 zoom-in-75">
								<Check className="size-3" />
							</span>
						)}
					</button>
				);
			})}
		</div>
	);
}

/** Summary card of the chosen engine shown at the top of the configure step. */
export function DbEngineCard({kind}: {kind: string}) {
	const k = DB_KINDS.find(x => x.id === kind);
	if (!k) return null;
	const Icon = DB_ICONS[k.id];
	return (
		<div className="flex items-center gap-3.5 rounded-xl border border-border/50 bg-muted/20 p-4">
			<span className="flex size-8 shrink-0 items-center justify-center">
				{Icon && <Icon className="size-7" />}
			</span>
			<span className="min-w-0">
				<span className="block text-sm font-medium text-foreground">
					{k.label}
				</span>
				<span className="block truncate text-[13px] text-muted-foreground">
					{k.description}
				</span>
			</span>
		</div>
	);
}

/** Engine picker used inside the page-mode form. */
export function EngineField({form}: {form: Form}) {
	const {dbKind, setDbKind, setDockerImage} = form;
	return (
		<Field label="Database Engine">
			<DbEngineList
				selected={dbKind}
				onSelect={k => {
					setDbKind(k.id);
					setDockerImage(k.defaultImage);
				}}
			/>
		</Field>
	);
}

/** Per-engine credential + image fields, each as a full-width row. */
export function DbDetailFields({
	form,
	sheet,
}: {
	form: Form;
	sheet?: boolean;
}) {
	const {
		dbKind,
		dbName,
		setDbName,
		dbUser,
		setDbUser,
		dbPassword,
		setDbPassword,
		dockerImage,
		setDockerImage,
	} = form;
	const showNameUser = dbKind !== 'redis' && dbKind !== 'libsql';
	const KindIcon = DB_ICONS[dbKind];
	return (
		<div className={cn('grid gap-4', !sheet && 'sm:grid-cols-2')}>
			{showNameUser && (
				<Field label="Database Name">
					<Input
						placeholder="app_db"
						value={dbName}
						onChange={e => setDbName(e.target.value)}
					/>
				</Field>
			)}
			{showNameUser && (
				<Field label="Database User">
					<Input
						placeholder="admin"
						value={dbUser}
						onChange={e => setDbUser(e.target.value)}
					/>
				</Field>
			)}
			{dbKind !== 'libsql' && (
				<Field label="Password">
					<Input
						type="password"
						placeholder="••••••••"
						value={dbPassword}
						onChange={e => setDbPassword(e.target.value)}
					/>
				</Field>
			)}
			<Field label="Docker Image">
				<div className="relative">
					{KindIcon && (
						<KindIcon className="pointer-events-none absolute top-1/2 left-3 size-4 -translate-y-1/2" />
					)}
					<Input
						placeholder={dockerImage}
						value={dockerImage}
						onChange={e => setDockerImage(e.target.value)}
						className={cn('font-mono', KindIcon && 'pl-9')}
					/>
				</div>
			</Field>
		</div>
	);
}
