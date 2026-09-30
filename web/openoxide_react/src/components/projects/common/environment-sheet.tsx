import {useEffect, useRef, useState} from 'react';
import {Check} from 'lucide-react';
import {toast} from 'sonner';
import {Button} from '#/components/ui/button';
import {
	Sheet,
	SheetContent,
	SheetDescription,
	SheetFooter,
	SheetHeader,
	SheetTitle,
} from '#/components/ui/sheet';
import {
	EnvRowsCard,
	parseEnv,
	serializeEnv,
	type EnvRow,
} from './env-rows-editor';

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

function EnvironmentSheetBody({
	app,
	handleUpdate,
}: {
	app: any;
	handleUpdate: (body: any) => Promise<void>;
}) {
	const original = String(app.env_var || '');
	const [rows, setRows] = useState<EnvRow[]>(() => parseEnv(original));
	const [saving, setSaving] = useState(false);
	const [saved, setSaved] = useState(false);

	const isModified =
		serializeEnv(rows) !== serializeEnv(parseEnv(original));

	const handleSave = async () => {
		setSaving(true);
		try {
			await handleUpdate({env_var: serializeEnv(rows)});
			toast.success('Environment variables saved successfully');
			setSaved(true);
			setTimeout(() => setSaved(false), 2000);
		} catch {
			toast.error('Failed to save environment variables');
		} finally {
			setSaving(false);
		}
	};

	// Ctrl/⌘+S saves without hunting for the button.
	const saveRef = useRef<() => void>(() => {});
	saveRef.current = () => {
		if (!saving && isModified) void handleSave();
	};
	useEffect(() => {
		const onKey = (e: KeyboardEvent) => {
			if ((e.metaKey || e.ctrlKey) && e.key.toLowerCase() === 's') {
				e.preventDefault();
				saveRef.current();
			}
		};
		window.addEventListener('keydown', onKey);
		return () => window.removeEventListener('keydown', onKey);
	}, []);

	return (
		<SheetContent className="gap-0 p-0 sm:data-[side=right]:w-[640px]">
			<SheetHeader className="px-5 pt-5 pb-4">
				<SheetTitle>Environment variables</SheetTitle>
				<SheetDescription>
					KEY=VALUE pairs injected at build and runtime — redeploy or
					rebuild this service to apply them.
				</SheetDescription>
			</SheetHeader>

			<div className="flex min-h-0 flex-1 flex-col px-5 pt-4 pb-5">
				<EnvRowsCard
					rows={rows}
					onRowsChange={setRows}
					fileName={envFileName(app)}
					isModified={isModified}
				/>
			</div>

			<SheetFooter className="px-5 py-4">
				{saved && (
					<span className="mr-auto flex animate-in items-center gap-1 text-xs font-medium text-emerald-500 duration-200 fade-in">
						<Check className="size-3.5" /> Saved
					</span>
				)}
				<Button
					onClick={() => void handleSave()}
					disabled={saving || !isModified}
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
