import {Input} from '#/components/ui/input';
import {cn} from '#/api/utils';
import {DB_KINDS} from './constants';
import {Field} from './field';
import type {useCreateService} from './use-create-service';

type Form = ReturnType<typeof useCreateService>;

/** Chip selector for the database engine (kind) at the top of the form. */
export function EngineField({form}: {form: Form}) {
	const {dbKind, setDbKind, setDockerImage} = form;
	return (
		<Field label="Database Engine">
			<div className="flex flex-wrap gap-2">
				{DB_KINDS.map(k => (
					<button
						key={k.id}
						type="button"
						onClick={() => {
							setDbKind(k.id);
							setDockerImage(k.defaultImage);
						}}
						className={cn(
							'inline-flex h-8 items-center rounded-lg border px-3 text-xs font-medium transition-colors',
							dbKind === k.id
								? 'border-primary/40 bg-primary/10 text-primary'
								: 'border-border/50 text-muted-foreground hover:bg-muted/40 hover:text-foreground',
						)}>
						{k.label}
					</button>
				))}
			</div>
		</Field>
	);
}

/** Per-engine credential + image fields shown under the main inputs. */
export function DbDetailFields({form}: {form: Form}) {
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
	return (
		<div className="grid grid-cols-1 gap-5 sm:grid-cols-2">
			{showNameUser && (
				<Field label="Database Name">
					<Input
						placeholder="app_db"
						value={dbName}
						onChange={e => setDbName(e.target.value)}
						className="h-10 rounded-xl border-border/50 bg-background text-sm"
					/>
				</Field>
			)}
			{showNameUser && (
				<Field label="Database User">
					<Input
						placeholder="admin"
						value={dbUser}
						onChange={e => setDbUser(e.target.value)}
						className="h-10 rounded-xl border-border/50 bg-background text-sm"
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
						className="h-10 rounded-xl border-border/50 bg-background text-sm"
					/>
				</Field>
			)}
			<Field label="Docker Image">
				<Input
					placeholder={dockerImage}
					value={dockerImage}
					onChange={e => setDockerImage(e.target.value)}
					className="h-10 rounded-xl border-border/50 bg-background font-mono text-sm"
				/>
			</Field>
		</div>
	);
}
