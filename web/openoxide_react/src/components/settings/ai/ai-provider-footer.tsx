import {CheckCircle2, Loader2, Save} from 'lucide-react';
import {Button} from '#/components/ui/button';
import type {AiForm} from './ai-types';

/** Cancel / Test / Save row at the bottom of the AI provider dialog. */
export function AiProviderFooter({
	form,
	testing,
	saving,
	onCancel,
	onTest,
	onSave,
}: {
	form: AiForm;
	testing: boolean;
	saving: boolean;
	onCancel: () => void;
	onTest: () => void;
	onSave: () => void;
}) {
	return (
		<div className="flex items-center justify-between border-t  pt-4">
			<Button
				variant="outline"
				size="sm"
				onClick={onCancel}
				className="h-9 px-4 text-xs font-semibold">
				Cancel
			</Button>
			<div className="flex gap-2">
				<Button
					variant="outline"
					size="sm"
					onClick={onTest}
					disabled={testing || !form.model}
					className="h-9 gap-1.5 px-3 text-xs font-semibold">
					{testing ? (
						<Loader2 className="size-3.5 animate-spin" />
					) : (
						<CheckCircle2 className="size-3.5 text-emerald-500" />
					)}{' '}
					Test
				</Button>
				<Button
					onClick={onSave}
					disabled={saving || !form.name || !form.api_url || !form.model}
					className="h-9 gap-1.5 rounded-lg bg-primary px-4 text-xs font-semibold text-primary-foreground">
					{saving ? (
						<Loader2 className="size-3.5 animate-spin" />
					) : (
						<Save className="size-3.5" />
					)}{' '}
					Save
				</Button>
			</div>
		</div>
	);
}
