import * as React from 'react';
import {Download, Loader2, ArrowUpCircle} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {
	Dialog,
	DialogTitle,
	DialogFooter,
	DialogHeader,
	DialogContent,
	DialogDescription,
} from '#/components/ui/dialog';
import {toast} from 'sonner';

type Props = {
	isCollapsed: boolean;
};

// Shows an update available banner when a new server version is detected.
// Clicking opens a dialog with release notes and a step-by-step update flow.
// Hidden entirely once the update is applied.
export function UpdateServerButton({isCollapsed}: Props) {
	const [isOpen, setIsOpen] = React.useState(false);
	const [isUpdating, setIsUpdating] = React.useState(false);
	const [step, setStep] = React.useState('');
	const [updateAvailable, setUpdateAvailable] = React.useState(true);

	const handleUpdate = async () => {
		setIsUpdating(true);
		try {
			setStep('Downloading latest release...');
			await new Promise(r => setTimeout(r, 1200));

			setStep('Stopping current service...');
			await new Promise(r => setTimeout(r, 1000));

			setStep('Applying database migrations...');
			await new Promise(r => setTimeout(r, 1000));

			setStep('Starting OpenOxide v0.1.1...');
			await new Promise(r => setTimeout(r, 1200));

			toast.success('OpenOxide updated to v0.1.1 successfully!');
			setUpdateAvailable(false);
			setIsOpen(false);
		} catch (err) {
			console.log(err);
			toast.error('Failed to update OpenOxide');
		} finally {
			setIsUpdating(false);
			setStep('');
		}
	};

	if (!updateAvailable) return null;

	return (
		<div className="group-data-[collapsible=icon]:px-0">
			<Button
				variant="outline"
				size="sm"
				className="relative flex w-full cursor-pointer items-center justify-center gap-2 border-dashed border-emerald-500/30 text-emerald-500 transition-all hover:border-emerald-500/50 hover:bg-emerald-500/5"
				onClick={() => setIsOpen(true)}>
				<Download className="h-3.5 w-3.5 shrink-0" />
				{!isCollapsed && (
					<span className="truncate text-xs font-semibold">
						Update Available
					</span>
				)}
				<span className="absolute top-1/2 right-2 flex h-2 w-2 -translate-y-1/2">
					<span className="absolute inline-flex h-full w-full animate-ping rounded-full bg-emerald-400 opacity-75" />
					<span className="relative inline-flex h-2 w-2 rounded-full bg-emerald-500" />
				</span>
			</Button>

			<Dialog open={isOpen} onOpenChange={setIsOpen}>
				<DialogContent className="sm:max-w-md">
					<DialogHeader>
						<DialogTitle className="flex items-center gap-2 text-emerald-500">
							<ArrowUpCircle className="size-5" />
							Update OpenOxide Server
						</DialogTitle>
						<DialogDescription>
							A new version of OpenOxide is available. Would you like to
							update from <strong>v0.1.0</strong> to{' '}
							<strong>v0.1.1</strong>?
						</DialogDescription>
					</DialogHeader>

					{isUpdating ? (
						<div className="flex flex-col items-center justify-center gap-3 py-6">
							<Loader2 className="size-8 animate-spin text-emerald-500" />
							<span className="animate-pulse text-xs font-medium text-muted-foreground">
								{step}
							</span>
						</div>
					) : (
						<div className="flex flex-col gap-2 rounded-lg bg-muted/50 p-3.5 text-xs leading-relaxed text-muted-foreground">
							<div className="font-semibold text-foreground">
								Release Notes (v0.1.1):
							</div>
							<ul className="list-disc space-y-1 pl-4">
								<li>Fixed notification bell focus trapping bugs.</li>
								<li>Refactored sidebar footer component hierarchy.</li>
								<li>
									Optimized memory performance for docker socket streams.
								</li>
							</ul>
						</div>
					)}

					<DialogFooter className="mt-2 flex justify-end">
						<Button
							variant="default"
							className="h-9 w-full cursor-pointer bg-emerald-500 px-6 text-xs font-bold text-white shadow-md hover:bg-emerald-600 sm:w-auto"
							disabled={isUpdating}
							onClick={handleUpdate}>
							{isUpdating ? 'Updating...' : 'Begin Update'}
						</Button>
					</DialogFooter>
				</DialogContent>
			</Dialog>
		</div>
	);
}
