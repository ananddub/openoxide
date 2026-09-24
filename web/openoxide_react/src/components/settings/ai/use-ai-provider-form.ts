import {useEffect, useState} from 'react';
import {toast} from 'sonner';
import {
	createAiSetting,
	discoverAiModels,
	discoverSettingModels,
	testAiConnection,
	testAiSetting,
	updateAiSetting,
} from './ai-api';
import {EMPTY_AI_FORM, type AiForm, type AiSetting} from './ai-types';

/** All state + actions behind the AI provider dialog (form, discover, test, save). */
export function useAiProviderForm({
	open,
	editing,
	onOpenChange,
	onSaved,
}: {
	open: boolean;
	editing: AiSetting | null;
	onOpenChange: (open: boolean) => void;
	onSaved: () => Promise<void>;
}) {
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

	useEffect(() => {
		if (open) reset();
		// Reset when the parent switches between create and edit while mounted. eslint-disable-next-line react-hooks/exhaustive-deps
	}, [open, editing]);

	const set = (key: keyof AiForm, value: string | boolean) =>
		setForm(current => ({...current, [key]: value}));

	const selectPreset = (name: string, url: string, defaultModel: string) =>
		setForm(current => ({
			...current,
			name,
			api_url: url,
			model: defaultModel || current.model,
		}));

	const discover = async () => {
		if (!form.api_url)
			return toast.error('API URL is required to discover models');
		try {
			setDiscovering(true);
			const canUseSavedKey =
				editing?.has_api_key &&
				!form.api_key &&
				form.api_url === editing.api_url;
			const data = canUseSavedKey
				? await discoverSettingModels(editing.id)
				: await discoverAiModels({
						api_url: form.api_url,
						api_key: form.api_key,
					});
			const discovered = data.map(model =>
				String(model.model ?? model.name ?? ''),
			);
			setModels(discovered);
			if (discovered[0] && !form.model) set('model', discovered[0]);
			toast.success(`${discovered.length} models discovered`);
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
			const canUseSavedKey =
				editing?.has_api_key &&
				!form.api_key &&
				form.api_url === editing.api_url &&
				form.model === editing.model;
			if (canUseSavedKey) {
				await testAiSetting(editing.id);
			} else {
				await testAiConnection({
					api_url: form.api_url,
					api_key: form.api_key,
					model: form.model,
				});
			}
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
			const body = {
				...form,
				api_key: editing ? form.api_key || undefined : form.api_key,
			};
			if (editing) await updateAiSetting(editing.id, body);
			else await createAiSetting({...body, api_key: form.api_key});
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

	return {
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
	};
}
