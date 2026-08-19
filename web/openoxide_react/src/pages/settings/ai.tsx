import {createFileRoute} from '@tanstack/react-router';
import {Bot, Plus, Sparkles} from 'lucide-react';
import {useEffect, useState} from 'react';
import {toast} from 'sonner';
import {Button} from '#/components/ui/button';
import {AiProviderDialog} from '#/components/settings/ai/ai-provider-dialog';
import {aiRequest} from '#/components/settings/ai/ai-api';
import {AiSettingsList} from '#/components/settings/ai/ai-settings-list';
import type {AiSetting} from '#/components/settings/ai/ai-types';

export const Route = createFileRoute('/_app/settings/ai')({
	component: AiSettingsPage,
});

function AiSettingsPage() {
	const [items, setItems] = useState<AiSetting[]>([]);
	const [loading, setLoading] = useState(true);
	const [open, setOpen] = useState(false);
	const [editing, setEditing] = useState<AiSetting | null>(null);

	const load = async () => {
		try {
			setLoading(true);
			const data = (await aiRequest('/ai/settings')) || [];
			setItems(data);
		} catch (error) {
			toast.error(
				error instanceof Error
					? error.message
					: 'Failed to load AI settings',
			);
		} finally {
			setLoading(false);
		}
	};

	useEffect(() => {
		void load();
	}, []);

	const create = () => {
		setEditing(null);
		setOpen(true);
	};

	const edit = (item: AiSetting) => {
		setEditing(item);
		setOpen(true);
	};

	const remove = async (id: number) => {
		if (!confirm('Delete this AI configuration?')) return;
		try {
			await aiRequest(`/ai/settings/${id}`, {method: 'DELETE'});
			toast.success('AI configuration deleted');
			await load();
		} catch (error) {
			toast.error(
				error instanceof Error
					? error.message
					: 'Failed to delete AI configuration',
			);
		}
	};

	return (
		<div className="mx-auto flex w-full max-w-5xl animate-in flex-col gap-6 p-6 duration-200 fade-in">
			{/* Header Banner */}
			<div className="flex flex-col justify-between gap-4 sm:flex-row sm:items-center">
				<div>
					<h1 className="flex items-center gap-2.5 text-2xl font-bold tracking-tight text-foreground">
						<div className="flex size-9 items-center justify-center rounded-xl border border-primary/20 bg-primary/10 text-primary">
							<Bot className="size-5" />
						</div>
						AI Settings
					</h1>
					<p className="mt-1 text-xs text-muted-foreground">
						Configure LLM providers (OpenAI, Anthropic, Gemini, Ollama) for
						AI container logs analysis & template generation.
					</p>
				</div>

				<div className="flex items-center gap-3">
					<div className="flex items-center gap-1.5 rounded-xl border border-border/60 bg-muted/40 px-3 py-1.5 text-xs font-semibold text-muted-foreground">
						<Sparkles className="size-3.5 text-amber-500" />
						<span>{items.filter(i => i.is_enabled).length} Active</span>
					</div>

					<Button
						onClick={create}
						className="h-9 gap-1.5 rounded-lg bg-primary text-xs font-semibold text-primary-foreground">
						<Plus className="size-4" /> Add AI Provider
					</Button>
				</div>
			</div>

			{/* AI Providers List */}
			<section className="flex flex-col gap-3">
				<h3 className="text-xs font-bold tracking-wider text-muted-foreground uppercase">
					Configured AI Models & Gateways
				</h3>

				<AiSettingsList
					items={items}
					loading={loading}
					onCreate={create}
					onEdit={edit}
					onDelete={remove}
				/>
			</section>

			{/* Add/Edit Modal */}
			<AiProviderDialog
				editing={editing}
				open={open}
				onOpenChange={setOpen}
				onSaved={load}
			/>
		</div>
	);
}
