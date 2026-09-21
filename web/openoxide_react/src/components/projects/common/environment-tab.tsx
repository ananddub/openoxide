import {Button} from '#/components/ui/button';
import {Save, Info, ShieldAlert, Check, Eye, EyeOff} from 'lucide-react';
import {useEnvEditor} from './use-env-editor';

interface EnvironmentTabProps {
	app: any;
	handleUpdate: (body: any) => Promise<void>;
}

export function EnvironmentTab({app, handleUpdate}: EnvironmentTabProps) {
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
		<div className="flex animate-in flex-col gap-6 duration-200 fade-in">
			{/* Info Warning Banner */}
			<div className="flex gap-3 rounded-xl bg-primary/5 p-4">
				<Info className="mt-0.5 h-4.5 w-4.5 shrink-0 text-primary" />
				<div className="flex flex-col gap-1">
					<span className="text-xs font-bold text-foreground">
						Rebuild Required
					</span>
					<p className="text-[11px] leading-normal text-muted-foreground">
						Changes to environment variables are injected at
						runtime/build-time. You must{' '}
						<strong>redeploy or rebuild</strong> the application to apply
						the changes.
					</p>
				</div>
			</div>

			{/* Main Editor Card */}
			<div className="flex flex-col gap-4 rounded-xl bg-card p-5">
				<div className="flex flex-col justify-between gap-4 sm:flex-row sm:items-center">
					<div className="flex flex-wrap items-center gap-3">
						<div>
							<h3 className="text-sm font-bold text-foreground">
								Environment Variables
							</h3>
							<p className="mt-1 text-[11px] text-muted-foreground">
								Define variables in{' '}
								<code className="rounded bg-muted/40 px-1 py-0.5 font-mono text-[10px]">
									KEY=VALUE
								</code>{' '}
								format.
							</p>
						</div>

						{isModified && (
							<span className="animate-pulse rounded-full bg-amber-500/10 px-2 py-0.5 text-[9px] font-semibold text-amber-500 select-none">
								Unsaved Changes
							</span>
						)}
					</div>

					<div className="flex items-center gap-2">
						<Button
							variant="outline"
							size="sm"
							onClick={() => setShowSecrets(!showSecrets)}
							className="flex h-8 items-center gap-1.5 rounded-lg  text-xs font-semibold text-foreground hover:bg-muted">
							{showSecrets ? (
								<EyeOff className="h-3.5 w-3.5" />
							) : (
								<Eye className="h-3.5 w-3.5 text-primary" />
							)}
							{showSecrets ? 'Hide Values' : 'Show Values'}
						</Button>

						<div className="hidden items-center gap-1.5 rounded-lg bg-muted/20 px-2 py-1 text-xs font-semibold text-muted-foreground md:flex">
							<ShieldAlert className="h-3.5 w-3.5 text-amber-500" />{' '}
							Secrets encrypted
						</div>
					</div>
				</div>

				{/* Notice when values are hidden */}
				{!showSecrets && (
					<div className="flex items-center justify-between rounded-lg bg-muted/30 px-3 py-2 text-[11px] text-muted-foreground">
						<span>
							Secret values are masked for security. Click{' '}
							<strong>Show Values</strong> (eye icon) to edit or view exact
							values.
						</span>
					</div>
				)}

				{/* Monaco Editor Container */}
				<div
					ref={containerRef}
					className={`relative overflow-hidden rounded-xl bg-zinc-950 shadow-inner transition-all duration-300 ${!showSecrets ? 'opacity-60 grayscale-[30%]' : 'opacity-100'}`}
					style={{height: '420px', width: '100%'}}
				/>

				<div className="mt-2 flex items-center justify-end gap-3">
					{saved && (
						<span className="flex animate-in items-center gap-1 text-xs font-bold text-emerald-500 duration-200 fade-in slide-in-from-right-2">
							<Check className="h-3.5 w-3.5" /> Saved!
						</span>
					)}

					<Button
						onClick={handleSave}
						disabled={saving || !showSecrets}
						title={
							!showSecrets
								? 'Click Show Values (eye icon) to edit and save'
								: ''
						}
						className="flex h-9 items-center gap-1.5 rounded-lg bg-primary text-xs font-semibold text-primary-foreground hover:bg-primary/95">
						<Save className="h-3.5 w-3.5" />{' '}
						{saving ? 'Saving...' : 'Save Environment'}
					</Button>
				</div>
			</div>
		</div>
	);
}
