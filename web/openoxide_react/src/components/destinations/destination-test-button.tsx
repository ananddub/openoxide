import {RefreshCw, Plug, Check, X} from 'lucide-react';
import {Button} from '#/components/ui/button';
import type {TestStatus} from '#/components/destinations/use-destination-form';

const TEST_TITLE: Record<TestStatus, string> = {
	idle: 'Test Connection',
	testing: 'Testing S3 Connection...',
	success: 'S3 Connection Passed!',
	failed: 'S3 Connection Failed!',
};

const TEST_CLASS: Record<TestStatus, string> = {
	idle: '',
	testing: '',
	success: 'border-emerald-500/50 bg-emerald-500/10 text-emerald-500',
	failed: 'border-rose-500/50 bg-rose-500/10 text-rose-500',
};

/** Icon button that reflects S3 connection-test status. */
export function DestinationTestButton({
	status,
	onClick,
}: {
	status: TestStatus;
	onClick: () => void;
}) {
	return (
		<Button
			type="button"
			variant="outline"
			size="icon"
			onClick={onClick}
			disabled={status === 'testing'}
			title={TEST_TITLE[status]}
			className={`h-9 w-9 shrink-0 transition-all ${TEST_CLASS[status]}`}>
			{status === 'testing' ? (
				<RefreshCw className="h-4 w-4 animate-spin text-amber-500" />
			) : status === 'success' ? (
				<Check className="h-4 w-4 text-emerald-500" />
			) : status === 'failed' ? (
				<X className="h-4 w-4 text-rose-500" />
			) : (
				<Plug className="h-4 w-4 text-muted-foreground hover:text-foreground" />
			)}
		</Button>
	);
}
