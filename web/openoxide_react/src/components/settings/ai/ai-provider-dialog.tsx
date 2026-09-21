import {RefreshCw, Bot} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {
	Sheet,
	SheetContent,
	SheetDescription,
	SheetHeader,
	SheetTitle,
} from '#/components/ui/sheet';
import {Input} from '#/components/ui/input';
import {AI_PRESETS, type AiSetting} from './ai-types';
import {useAiProviderForm} from './use-ai-provider-form';
import {AiProviderFooter} from './ai-provider-footer';

type Props = {
	editing: AiSetting | null;
	open: boolean;
	onOpenChange: (open: boolean) => void;
	onSaved: () => Promise<void>;
};

export function AiProviderDialog({
	editing,
	open,
	onOpenChange,
	onSaved,
}: Props) {
	const {
		form,
		set,
		models,
		selectPreset,
		discover,
		test,
		save,
		saving,
		testing,
		discovering,
	} = useAiProviderForm({open, editing, onOpenChange, onSaved});

	return (
		<Sheet open={open} onOpenChange={onOpenChange}>
			<SheetContent>
				<SheetHeader>
					<SheetTitle className="flex items-center gap-2">
						<Bot className="size-4 text-primary" />
						{editing ? 'Edit AI Configuration' : 'Add AI Configuration'}
					</SheetTitle>
					<SheetDescription>
						Connect an LLM provider for intelligent container log analysis
						and automatic deployment diagnostics.
					</SheetDescription>
				</SheetHeader>

				{/* Quick Presets */}
				<div className="flex flex-col gap-1.5 px-6 pt-5">
					<label className="text-[11px] font-bold tracking-wider text-muted-foreground uppercase">
						Quick Presets
					</label>
					<div className="flex flex-wrap gap-1.5">
						{AI_PRESETS.map(p => (
							<button
								key={p.name}
								type="button"
								onClick={() => selectPreset(p.name, p.url, p.defaultModel)}
								className={`rounded-lg  px-2.5 py-1 text-xs font-medium transition-colors ${
									form.api_url === p.url
										? ' bg-primary text-primary-foreground'
										: ' bg-muted/40 text-muted-foreground hover:bg-muted'
								}`}>
								{p.name}
							</button>
						))}
					</div>
				</div>

				<div className="grid flex-1 gap-3.5 overflow-y-auto px-6 pb-5">
					<div className="grid grid-cols-1 gap-3 md:grid-cols-2">
						<div className="space-y-1">
							<label className="text-xs font-semibold text-muted-foreground">
								Provider Name
							</label>
							<Input
								value={form.name}
								onChange={e => set('name', e.target.value)}
								placeholder="e.g. OpenAI Production"
								className="h-9 text-xs"
							/>
						</div>

						<div className="space-y-1">
							<label className="text-xs font-semibold text-muted-foreground">
								API Base URL
							</label>
							<Input
								value={form.api_url}
								onChange={e => set('api_url', e.target.value)}
								placeholder="https://api.openai.com/v1"
								className="h-9 font-mono text-xs"
							/>
						</div>
					</div>

					<div className="space-y-1">
						<label className="text-xs font-semibold text-muted-foreground">
							API Key / Token
						</label>
						<Input
							type="password"
							value={form.api_key}
							onChange={e => set('api_key', e.target.value)}
							placeholder={
								editing?.has_api_key
									? 'Leave blank to keep existing key'
									: 'sk-...'
							}
							className="h-9 font-mono text-xs"
						/>
					</div>

					<div className="space-y-1">
						<div className="flex items-center justify-between">
							<label className="text-xs font-semibold text-muted-foreground">
								Model Name
							</label>
							<Button
								type="button"
								variant="ghost"
								size="sm"
								onClick={() => void discover()}
								disabled={discovering}
								className="h-6 gap-1 px-2 text-[11px] text-primary">
								<RefreshCw
									className={`size-3 ${discovering ? 'animate-spin' : ''}`}
								/>{' '}
								Discover Models
							</Button>
						</div>
						<Input
							value={form.model}
							onChange={e => set('model', e.target.value)}
							placeholder="e.g. gpt-4o, claude-3-5-sonnet, llama3.2"
							className="h-9 font-mono text-xs"
						/>
					</div>

					{models.length > 0 && (
						<div className="space-y-1">
							<label className="text-[11px] font-semibold text-muted-foreground">
								Discovered Models
							</label>
							<select
								className="h-9 w-full rounded-lg bg-background px-3 font-mono text-xs text-foreground"
								value={form.model}
								onChange={e => set('model', e.target.value)}>
								<option value="">
									-- Choose from {models.length} discovered models --
								</option>
								{models.map(m => (
									<option key={m} value={m}>
										{m}
									</option>
								))}
							</select>
						</div>
					)}

					<label className="flex cursor-pointer items-center gap-2 pt-1 text-xs font-semibold text-foreground">
						<input
							type="checkbox"
							checked={form.is_enabled}
							onChange={e => set('is_enabled', e.target.checked)}
							className="size-4 rounded  accent-primary"
						/>
						Enable this AI configuration for log diagnosis & assistant
					</label>

					<AiProviderFooter
						form={form}
						testing={testing}
						saving={saving}
						onCancel={() => onOpenChange(false)}
						onTest={() => void test()}
						onSave={() => void save()}
					/>
				</div>
			</SheetContent>
		</Sheet>
	);
}
