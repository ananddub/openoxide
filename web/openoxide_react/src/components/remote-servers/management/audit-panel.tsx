import {Loader2, Stethoscope} from 'lucide-react';
import {toast} from 'sonner';
import {Button} from '#/components/ui/button';
import {formatApiError} from '#/api/utils';
import {
	useServerAudit,
	type ServerAuditPayload,
} from '#/api/live/server-audit';
import {usePermissions} from '#/hooks/permissions/use-permissions';
import {DockerTable, type DockerColumn} from '../docker/docker-shell';
import {Panel, PanelPill} from '../server-panel';
import {
	auditCheckRows,
	auditIssues,
	auditToolRows,
	type AuditToolRow,
} from './audit-metrics';

/** One-shot toolchain and engine diagnostics, run over SSH only when asked. */
export function ServerAuditPanel({serverId}: {serverId: bigint}) {
	const {can} = usePermissions();
	const audit = useServerAudit();
	const run = () =>
		audit.mutate(serverId, {
			onError: error => toast.error(formatApiError(error)),
		});

	const columns: DockerColumn<AuditToolRow>[] = [
		{
			label: 'Tool',
			cell: row => (
				<span className="font-medium text-foreground">{row.label}</span>
			),
		},
		{
			label: 'Role',
			cell: row => (
				<span className="text-muted-foreground">{row.hint}</span>
			),
		},
		{
			label: 'Status',
			cell: row => (
				<PanelPill
					ok={row.installed}
					label={
						row.installed
							? 'installed'
							: row.required
								? 'missing'
								: 'not installed'
					}
					tone={row.required ? 'default' : 'muted'}
				/>
			),
		},
		{
			label: 'Version',
			align: 'right',
			cell: row => row.version || '—',
		},
	];

	const issues = audit.data ? auditIssues(audit.data) : [];

	return (
		<Panel
			title="Server Audit"
			caption="Checks the toolchain, Docker state and entrypoint ports over SSH">
			<div className="flex flex-col gap-4 p-4">
				<div className="flex flex-wrap items-center gap-3">
					<Button
						size="sm"
						variant="outline"
						disabled={!can('server:read') || audit.isPending}
						onClick={run}
						className="h-8 cursor-pointer gap-1.5 text-xs disabled:cursor-default">
						{audit.isPending ? (
							<Loader2 className="size-3.5 animate-spin" />
						) : (
							<Stethoscope className="size-3.5" />
						)}
						{audit.isPending ? 'Auditing…' : 'Run audit'}
					</Button>
					{audit.data && <AuditSummary audit={audit.data} />}
				</div>

				{audit.data ? (
					<>
						<AuditFindings issues={issues} />
						<DockerTable
							columns={columns}
							rows={auditToolRows(audit.data)}
							keyOf={row => row.key}
						/>
						<div className="flex flex-wrap gap-x-5 gap-y-2">
							{auditCheckRows(audit.data).map(row => (
								<span
									key={row.key}
									className="flex items-center gap-2 text-xs text-foreground/80">
									{row.label}
									<PanelPill ok={row.ok} label={row.detail} />
								</span>
							))}
						</div>
					</>
				) : (
					<p
						className={`py-6 text-center text-xs ${
							audit.isError ? 'text-rose-500' : 'text-muted-foreground'
						}`}>
						{audit.isError
							? formatApiError(audit.error)
							: audit.isPending
								? 'Opening SSH and running the checks…'
								: 'Nothing audited yet — the checks only run when you ask.'}
					</p>
				)}
			</div>
		</Panel>
	);
}

function AuditSummary({audit}: {audit: ServerAuditPayload}) {
	const ports = audit.ports
		.map(port => `${port.port} ${port.available ? 'free' : 'busy'}`)
		.join(', ');
	return (
		<span className="text-xs text-muted-foreground">
			{audit.osId} · {audit.architecture} · {ports}
		</span>
	);
}

/** What a failed check means for the next deployment, or the all-clear. */
function AuditFindings({issues}: {issues: string[]}) {
	return (
		<div
			className={`rounded-lg border p-3 text-xs ${
				issues.length > 0
					? 'border-rose-500/30 bg-rose-500/5 text-rose-600 dark:text-rose-400'
					: 'border-emerald-500/30 bg-emerald-500/5 text-emerald-600 dark:text-emerald-400'
			}`}>
			{issues.length > 0 ? (
				<ul className="flex list-disc flex-col gap-1 pl-4">
					{issues.map(issue => (
						<li key={issue}>{issue}</li>
					))}
				</ul>
			) : (
				<span>This server is ready to deploy to.</span>
			)}
		</div>
	);
}
