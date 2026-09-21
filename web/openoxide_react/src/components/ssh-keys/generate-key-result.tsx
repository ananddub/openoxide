import {Button} from '#/components/ui/button';
import {CheckCircle2} from 'lucide-react';
import {AuthorizeCommandBlock} from '#/components/ssh-keys/authorize-command-block';

interface GenerateKeyResultProps {
	name: string;
	publicKey?: string;
	saving: boolean;
	onSave: () => void;
}

/** Post-generation panel: authorize command + save action. */
export function GenerateKeyResult({
	name,
	publicKey,
	saving,
	onSave,
}: GenerateKeyResultProps) {
	return (
		<div className="flex flex-col gap-4 py-2">
			<div className="flex items-center gap-2.5 rounded-xl bg-emerald-500/10 p-3 text-xs font-bold text-emerald-500">
				<CheckCircle2 className="h-4 w-4 shrink-0" />
				<span>Key Pair "{name}" Generated in Memory</span>
			</div>

			<AuthorizeCommandBlock publicKey={publicKey || ''} />

			<div className="flex items-center justify-end border-t  pt-3">
				<Button
					onClick={onSave}
					disabled={saving}
					className="h-9 w-full bg-primary px-6 text-xs font-bold text-primary-foreground shadow-md hover:bg-primary/90 sm:w-auto">
					{saving ? 'Saving...' : 'Save Key Pair'}
				</Button>
			</div>
		</div>
	);
}
