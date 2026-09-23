import {RefreshCw} from 'lucide-react';
import {Button} from '#/components/ui/button';

/**
 * Dialog form footer with a left "Test Connection" button and a right
 * Create/Update submit button (DNS & Vault provider forms layout).
 */
export function TestSaveFooter({
	isTesting,
	isSubmitting,
	editing,
	onTest,
}: {
	isTesting: boolean;
	isSubmitting: boolean;
	editing: boolean;
	onTest: () => void;
}) {
	return (
		<div className="flex w-full shrink-0 items-center justify-between gap-3 border-t border-border px-6 py-5">
			<Button
				type="button"
				variant="secondary"
				disabled={isTesting}
				onClick={onTest}
				className="h-10 gap-1.5 bg-secondary px-4 text-xs font-semibold text-secondary-foreground hover:bg-secondary/80">
				{isTesting && <RefreshCw className="size-3.5 animate-spin" />}
				Test Connection
			</Button>
			<Button
				type="submit"
				disabled={isSubmitting}
				className="h-10 gap-1.5 px-6 text-xs font-semibold">
				{isSubmitting && <RefreshCw className="size-3.5 animate-spin" />}
				{editing ? 'Update' : 'Save'}
			</Button>
		</div>
	);
}
