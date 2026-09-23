import {Check, Eye, EyeOff} from 'lucide-react';
import {useEnvEditor} from './use-env-editor';
import {Button} from '#/components/ui/button';
import {
	Sheet,
	SheetContent,
	SheetDescription,
	SheetFooter,
	SheetHeader,
	SheetTitle,
} from '#/components/ui/sheet';

interface EnvironmentSheetProps {
	open: boolean;
	onOpenChange: (open: boolean) => void;
	app: any;
	handleUpdate: (body: any) => Promise<void>;
}

/** Slug for the `.env` label above the editor. */
const envFileName = (app: any) =>
	`${
		String(app?.name || app?.app_name || 'service')
			.toLowerCase()
			.replace(/[^a-z0-9]+/g, '-')
			.replace(/^-|-$/g, '') || 'service'
	}.env`;

/**
 * Panel body. Rendered only while the sheet is open — Monaco attaches to the
 * container ref once on init and never retries, so mounting it against a
 * closed sheet left an editor that could not be typed into.
 */
function EnvironmentSheetBody({
	app,
	handleUpdate,
}: {
	app: any;
	handleUpdate: (body: any) => Promise<void>;
}) {
	const {
		showSecrets,
		setShowSecrets,
		saved,
		saving,
		isModified,
		containerRef,
		handleSave,
	} = useEnvEditor(app, handleUpdate);

	return (
		<SheetContent className="sm:data-[side=right]:w-[640px]">
			<SheetHeader className="px-5 pt-5 pb-4">
				<SheetTitle>Environment variables</SheetTitle>
				<SheetDescription>
					KEY=VALUE pairs injected at build and runtime — redeploy or
					rebuild this service to apply them.
				</SheetDescription>
			</SheetHeader>

			<div className="flex min-h-0 flex-1 flex-col gap-2.5 px-5 pt-4 pb-5">
				<div className="flex items-center justify-between gap-3">
					<div className="flex min-w-0 items-center gap-2">
						<span className="truncate font-mono text-[10px] font-medium tracking-wider text-muted-foreground uppercase">
							{envFileName(app)}
						</span>
						{isModified && (
							<span className="shrink-0 rounded-md bg-foreground/[0.05] px-1.5 py-0.5 text-[10px] font-medium text-muted-foreground">
								Unsaved changes
							</span>
						)}
					</div>

					<Button
						variant="outline"
						size="sm"
						onClick={() => setShowSecrets(!showSecrets)}
						className="h-7 shrink-0 gap-1.5 rounded-lg text-xs">
						{showSecrets ? (
							<EyeOff className="size-3.5" />
						) : (
							<Eye className="size-3.5" />
						)}
						{showSecrets ? 'Hide values' : 'Show values'}
					</Button>
				</div>

				{/* Editor fills the panel — no fixed height slab */}
				<div
					ref={containerRef}
					className="min-h-[280px] flex-1 overflow-hidden rounded-xl border border-border bg-zinc-950"
				/>

				<p className="text-[11px] leading-normal text-muted-foreground">
					{showSecrets
						? 'Values are stored encrypted on the panel.'
						: 'Values are masked to read-only. Show them to keep editing.'}
				</p>
			</div>

			<SheetFooter className="px-5 py-4">
				{saved && (
					<span className="mr-auto flex animate-in items-center gap-1 text-xs font-medium text-emerald-500 duration-200 fade-in">
						<Check className="size-3.5" /> Saved
					</span>
				)}
				<Button
					onClick={() => void handleSave()}
					disabled={saving || !showSecrets}
					title={!showSecrets ? 'Show values to edit and save' : undefined}
					className="h-8 px-4 text-xs">
					{saving ? 'Saving…' : 'Save'}
				</Button>
			</SheetFooter>
		</SheetContent>
	);
}

/**
 * Service environment variables — the key icon in a General tab header opens
 * this; the rail no longer carries an Environment entry per service.
 */
export function EnvironmentSheet({
	open,
	onOpenChange,
	app,
	handleUpdate,
}: EnvironmentSheetProps) {
	return (
		<Sheet open={open} onOpenChange={onOpenChange}>
			{open && (
				<EnvironmentSheetBody app={app} handleUpdate={handleUpdate} />
			)}
		</Sheet>
	);
}
