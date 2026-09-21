import {DialogTitle, DialogDescription} from '#/components/ui/dialog';
import type {AuditLogPayload} from '#/api/live/audit';
import {cn} from '#/api/utils';
import {
	TONE_DOT,
	sentenceFor,
	resourceLabel,
	parseMeta,
	timeAgo,
	absoluteTime,
} from './audit-format';

/** Click-to-open details sheet content: who / what / when / metadata. */
export function AuditDetails({event}: {event: AuditLogPayload}) {
	const info = sentenceFor(event.action);
	const meta = parseMeta(event);
	return (
		<div className="flex flex-col gap-5">
			<div>
				<div className="flex items-center gap-2">
					<span
						className={cn(
							'size-2 shrink-0 rounded-full',
							TONE_DOT[info.tone],
						)}
					/>
					<DialogTitle className="text-lg font-semibold text-foreground">
						{event.action.replace(/_/g, ' ').toLowerCase()}
					</DialogTitle>
				</div>
				<p className="mt-1.5 text-sm text-foreground/80">
					<span className="font-medium">
						{event.user_email || 'System'}
					</span>{' '}
					{info.verb}
					<span className="font-medium"> {resourceLabel(event)}</span>
				</p>
				<DialogDescription className="mt-0.5 text-xs text-muted-foreground">
					{timeAgo(event.created_at)} · {absoluteTime(event.created_at)}
				</DialogDescription>
			</div>

			<section className="rounded-xl bg-muted/20 p-4">
				<DetailRow label="Who" value={event.user_email || 'System'} />
				<DetailRow label="Role" value={event.user_role || '—'} />
				<DetailRow label="Action" value={event.action} mono />
				<DetailRow
					label="Resource"
					value={`${event.resource_type}${event.resource_id ? ` #${event.resource_id}` : ''}`}
				/>
			</section>

			{Object.keys(meta).length > 0 && (
				<section className="overflow-hidden rounded-xl ">
					<div className="border-b  px-4 py-2.5">
						<span className="text-xs font-semibold tracking-wide text-muted-foreground uppercase">
							Metadata
						</span>
					</div>
					<pre className="max-h-96 overflow-auto p-4 font-mono text-xs leading-relaxed break-words whitespace-pre-wrap">
						{JSON.stringify(meta, null, 2)}
					</pre>
				</section>
			)}
		</div>
	);
}

function DetailRow({
	label,
	value,
	mono,
}: {
	label: string;
	value: string;
	mono?: boolean;
}) {
	return (
		<div className="flex items-start gap-3 py-1.5">
			<span className="w-24 shrink-0 text-xs text-muted-foreground">
				{label}
			</span>
			<span
				className={cn(
					'flex-1 text-sm break-all text-foreground',
					mono && 'font-mono text-xs',
				)}>
				{value}
			</span>
		</div>
	);
}
