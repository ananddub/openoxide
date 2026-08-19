import {CheckCircle2, Loader2, RefreshCw, Bot, Save} from 'lucide-react';
import {useState} from 'react';
import {toast} from 'sonner';
import {Button} from '#/components/ui/button';
import {
	Dialog,
	DialogContent,
	DialogDescription,
	DialogHeader,
	DialogTitle,
} from '#/components/ui/dialog';
import {Input} from '#/components/ui/input';
import {aiRequest} from './ai-api';
import {
	AI_PRESETS,
	EMPTY_AI_FORM,
	type AiForm,
	type AiSetting,
} from './ai-types';

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
	const [form, setForm] = useState<AiForm>(EMPTY_AI_FORM);
	const [models, setModels] = useState<string[]>([]);
	const [saving, setSaving] = useState(false);
	const [testing, setTesting] = useState(false);
	const [discovering, setDiscovering] = useState(false);

	const reset = () => {
		setModels([]);
		setForm(
			editing
				? {
						name: editing.name,
						api_url: editing.api_url,
						api_key: '',
						model: editing.model,
						is_enabled: editing.is_enabled,
					}
				: EMPTY_AI_FORM,
		);
	};

	const set = (key: keyof AiForm, value: string | boolean) =>
		setForm(current => ({...current, [key]: value}));

	const handleSelectPreset = (
		name: string,
		url: string,
		defaultModel: string,
	) => {
		setForm(current => ({
			...current,
			name,
			api_url: url,
			model: defaultModel || current.model,
		}));
	};

	const discover = async () => {
		if (!form.api_url)
			return toast.error('API URL is required to discover models');
		try {
			setDiscovering(true);
			const data = await aiRequest('/ai/models/discover', {
				method: 'POST',
				body: JSON.stringify({
					api_url: form.api_url,
					api_key: form.api_key,
				}),
			});
			setModels(data.models || []);
			if (data.models?.[0] && !form.model) set('model', data.models[0]);
			toast.success(`${data.models?.length || 0} models discovered`);
		} catch (error) {
			toast.error(
				error instanceof Error
					? error.message
					: 'Could not discover models',
			);
		} finally {
			setDiscovering(false);
		}
	};

	const test = async () => {
		try {
			setTesting(true);
			await aiRequest('/ai/connection/test', {
				method: 'POST',
				body: JSON.stringify({
					api_url: form.api_url,
					api_key: form.api_key,
					model: form.model,
				}),
			});
			toast.success('AI connection test successful');
		} catch (error) {
			toast.error(
				error instanceof Error ? error.message : 'AI connection failed',
			);
		} finally {
			setTesting(false);
		}
	};

	const save = async () => {
		try {
			setSaving(true);
			await aiRequest(
				editing ? `/ai/settings/${editing.id}` : '/ai/settings',
				{
					method: editing ? 'PUT' : 'POST',
					body: JSON.stringify({
						...form,
						api_key: form.api_key || undefined,
					}),
				},
			);
			toast.success(
				editing ? 'AI settings updated' : 'AI settings created',
			);
			onOpenChange(false);
			await onSaved();
		} catch (error) {
			toast.error(
				error instanceof Error
					? error.message
					: 'Failed to save AI settings',
			);
		} finally {
			setSaving(false);
		}
	};

	return (
		<Dialog
			open={open}
			onOpenChange={value => {
				onOpenChange(value);
				if (value) reset();
			}}>
			<DialogContent className="max-h-[90vh] overflow-y-auto sm:max-w-lg">
				<DialogHeader>
					<DialogTitle className="flex items-center gap-2.5 text-base font-bold">
						<div className="flex size-8 items-center justify-center rounded-lg border border-primary/20 bg-primary/10 text-primary">
							<Bot className="size-4" />
						</div>
						{editing ? 'Edit AI Configuration' : 'Add AI Configuration'}
					</DialogTitle>
					<DialogDescription className="text-xs">
						Connect an LLM provider for intelligent container log analysis
						and automatic deployment diagnostics.
					</DialogDescription>
				</DialogHeader>

				{/* Quick Presets */}
				<div className="flex flex-col gap-1.5 pt-1">
					<label className="text-[11px] font-bold tracking-wider text-muted-foreground uppercase">
						Quick Presets
					</label>
					<div className="flex flex-wrap gap-1.5">
						{AI_PRESETS.map(p => (
							<button
								key={p.name}
								type="button"
								onClick={() =>
									handleSelectPreset(p.name, p.url, p.defaultModel)
								}
								className={`rounded-lg border px-2.5 py-1 text-xs font-medium transition-colors ${
									form.api_url === p.url
										? 'border-primary bg-primary text-primary-foreground'
										: 'border-border/60 bg-muted/40 text-muted-foreground hover:bg-muted'
								}`}>
								{p.name}
							</button>
						))}
					</div>
				</div>

				<div className="grid gap-3.5 pt-2">
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
								onClick={discover}
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
								className="h-9 w-full rounded-lg border border-border bg-background px-3 font-mono text-xs text-foreground"
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
							className="size-4 rounded border-border accent-primary"
						/>
						Enable this AI configuration for log diagnosis & assistant
					</label>

					<div className="flex items-center justify-between border-t border-border/40 pt-4">
						<Button
							variant="outline"
							size="sm"
							onClick={() => onOpenChange(false)}
							className="h-9 px-4 text-xs font-semibold">
							Cancel
						</Button>
						<div className="flex gap-2">
							<Button
								variant="outline"
								size="sm"
								onClick={test}
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
								onClick={save}
								disabled={
									saving || !form.name || !form.api_url || !form.model
								}
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
				</div>
			</DialogContent>
		</Dialog>
	);
}
