import {RefreshCw, Plug, Check, X} from 'lucide-react';
import type {TestStatus} from '#/components/destinations/use-destination-form';

const TEST_LABEL: Record<TestStatus, string> = {
	idle: 'Test Connection',
	testing: 'Testing S3 connection…',
	success: 'Connection passed',
	failed: 'Connection failed',
};

const TEST_CLASS: Record<TestStatus, string> = {
	idle: 'border-border/60 text-muted-foreground hover:bg-muted/30 hover:text-foreground',
	testing: 'border-border/60 text-muted-foreground',
	success:
		'border-emerald-500/40 bg-emerald-500/10 text-emerald-500 hover:bg-emerald-500/10',
	failed:
		'border-rose-500/40 bg-rose-500/10 text-rose-500 hover:bg-rose-500/10',
};

/** Full-width labeled control that reflects the S3 connection-test status. */
export function DestinationTestButton({
	status,
	onClick,
}: {
	status: TestStatus;
	onClick: () => void;
}) {
	return (
		<button
			type="button"
			onClick={onClick}
			disabled={status === 'testing'}
			className={`flex w-full items-center justify-center gap-2 rounded-lg border px-4 py-2.5 text-[13px] font-medium transition-colors disabled:cursor-not-allowed ${TEST_CLASS[status]}`}>
			{status === 'testing' ? (
				<RefreshCw className="size-3.5 animate-spin text-amber-500" />
			) : status === 'success' ? (
				<Check className="size-3.5" />
			) : status === 'failed' ? (
				<X className="size-3.5" />
			) : (
				<Plug className="size-3.5" />
			)}
			{TEST_LABEL[status]}
		</button>
	);
}
