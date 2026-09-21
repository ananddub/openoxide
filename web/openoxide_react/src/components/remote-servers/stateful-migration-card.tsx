import {RefreshCw} from 'lucide-react';
import type {
	MigrationObjectPayload,
	StatefulMigrationPayload,
} from '#/api/live';
import {
	formatBytes,
	PhaseStrip,
	statusMeta,
} from './stateful-migration-parts';

const OBJ_STATUS_CLS: Record<string, string> = {
	VERIFIED: 'text-emerald-500',
	APPLIED: 'text-emerald-500',
	FAILED: 'text-rose-500',
};

/** Live run card: status chip, progress bar, phases, synced objects. */
export function StatefulMigrationCard({
	latest,
	polling,
	objects,
}: {
	latest: StatefulMigrationPayload;
	polling: boolean;
	objects: MigrationObjectPayload[] | undefined;
}) {
	const meta = statusMeta(latest.status);
	return (
		<div className="flex flex-col gap-3 rounded-xl border p-3.5">
			<div className="flex items-center justify-between">
				<span className="font-mono text-[11px] text-muted-foreground">
					{latest.id.slice(0, 12)}…
				</span>
				<span
					className={`flex items-center gap-1.5 text-xs font-semibold ${meta.cls}`}>
					{meta.icon} {latest.status} · {latest.phase} · {latest.progress}%
				</span>
			</div>
			<div className="h-1.5 w-full overflow-hidden rounded-full bg-muted">
				<div
					className="h-full rounded-full bg-primary transition-all"
					style={{
						width: `${Math.min(100, Math.max(0, latest.progress))}%`,
					}}
				/>
			</div>
			<PhaseStrip phase={latest.phase} />
			{latest.error && (
				<div className="rounded-xl bg-rose-500/10 p-3 text-[11px] text-rose-500">
					{latest.error}
				</div>
			)}
			{objects && objects.length > 0 && (
				<div className="max-h-40 overflow-y-auto rounded-xl bg-muted/20 text-[11px]">
					<table className="w-full">
						<tbody>
							{objects.map(object => (
								<tr
									key={object.obj_key}
									className="border-b last:border-b-0">
									<td className="px-2.5 py-1.5 font-mono text-muted-foreground">
										{object.kind}
									</td>
									<td
										className="max-w-40 truncate px-1 py-1.5"
										title={object.source_ref}>
										{object.source_ref}
									</td>
									<td className="px-1 py-1.5">
										<span
											className={
												OBJ_STATUS_CLS[object.status] ??
												'text-muted-foreground'
											}>
											{object.status}
										</span>
									</td>
									<td className="px-2.5 py-1.5 text-right font-mono">
										{object.bytes > 0 ? formatBytes(object.bytes) : ''}
									</td>
								</tr>
							))}
						</tbody>
					</table>
				</div>
			)}
			{polling && (
				<p className="flex items-center gap-1.5 text-[11px] text-muted-foreground">
					<RefreshCw className="size-3 animate-spin" /> Running — you can
					close this dialog; the migration keeps going in the background.
				</p>
			)}
		</div>
	);
}
