import {CheckCircle2, Loader2, RefreshCw} from 'lucide-react';
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
import {AI_PRESETS, EMPTY_AI_FORM} from './ai-types';
import type {AiForm, AiSetting} from './ai-types';

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
	const discover = async () => {
		try {
			const data = await aiRequest('/ai/models/discover', {
				method: 'POST',
				body: JSON.stringify({
					api_url: form.api_url,
					api_key: form.api_key,
				}),
			});
			setModels(data.models || []);
			if (data.models?.[0] && !form.model) set('model', data.models[0]);
			toast.success(`${data.models?.length || 0} models found`);
		} catch (error) {
			toast.error(
				error instanceof Error
					? error.message
					: 'Could not discover models',
			);
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
			toast.success('AI connection successful');
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
					<DialogTitle>{editing ? 'Edit AI' : 'Add AI'}</DialogTitle>
					<DialogDescription>
						Configure an AI provider, discover its models, and test the
						connection.
					</DialogDescription>
				</DialogHeader>
				<div className="grid gap-3">
					<select
						className="h-9 rounded-md border bg-background px-3 text-sm"
						value={form.api_url}
						onChange={event => {
							const preset = AI_PRESETS.find(
								item => item[1] === event.target.value,
							);
							setForm(current => ({
								...current,
								api_url: event.target.value,
								name: preset?.[0] || current.name,
								model: '',
							}));
						}}>
						<option value="">Custom provider</option>
						{AI_PRESETS.map(([name, url]) => (
							<option key={url} value={url}>
								{name}
							</option>
						))}
					</select>
					<Input
						value={form.name}
						onChange={event => set('name', event.target.value)}
						placeholder="Configuration name"
					/>
					<Input
						value={form.api_url}
						onChange={event => set('api_url', event.target.value)}
						placeholder="API URL"
					/>
					<Input
						type="password"
						value={form.api_key}
						onChange={event => set('api_key', event.target.value)}
						placeholder={
							editing?.has_api_key
								? 'Leave blank to keep existing key'
								: 'API key'
						}
					/>
					<div className="flex gap-2">
						<Input
							value={form.model}
							onChange={event => set('model', event.target.value)}
							placeholder="Model name"
						/>
						<Button variant="outline" onClick={discover}>
							<RefreshCw className="mr-2 size-4" /> Discover
						</Button>
					</div>
					{models.length > 0 && (
						<select
							className="h-9 rounded-md border bg-background px-3 text-sm"
							value={form.model}
							onChange={event => set('model', event.target.value)}>
							<option value="">Select discovered model</option>
							{models.map(model => (
								<option key={model}>{model}</option>
							))}
						</select>
					)}
					<label className="flex items-center gap-2 text-sm">
						<input
							type="checkbox"
							checked={form.is_enabled}
							onChange={event => set('is_enabled', event.target.checked)}
						/>{' '}
						Enable this AI configuration
					</label>
					<div className="flex justify-between border-t pt-4">
						<Button
							variant="secondary"
							onClick={() => onOpenChange(false)}>
							Cancel
						</Button>
						<div className="flex gap-2">
							<Button
								variant="outline"
								onClick={test}
								disabled={testing || !form.model}>
								{testing ? (
									<Loader2 className="mr-2 size-4 animate-spin" />
								) : (
									<CheckCircle2 className="mr-2 size-4" />
								)}{' '}
								Test
							</Button>
							<Button
								onClick={save}
								disabled={
									saving || !form.name || !form.api_url || !form.model
								}>
								{saving ? (
									<Loader2 className="mr-2 size-4 animate-spin" />
								) : null}{' '}
								Save
							</Button>
						</div>
					</div>
				</div>
			</DialogContent>
		</Dialog>
	);
}
