import {createFileRoute} from '@tanstack/react-router';
import {Bot, Plus} from 'lucide-react';
import {useEffect, useState} from 'react';
import {toast} from 'sonner';
import {Button} from '#/components/ui/button';
import {Card} from '#/components/ui/card';
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
			setItems((await aiRequest('/ai/settings')) || []);
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
		<div className="mx-auto flex w-full max-w-5xl flex-col gap-6 p-6">
			<Card className="p-2.5">
				<div className="rounded-xl bg-background shadow-sm">
					<div className="flex items-center justify-between p-6">
						<div>
							<h1 className="flex items-center gap-2 text-xl font-bold">
								<Bot className="size-5 text-muted-foreground" /> AI
								Settings
							</h1>
							<p className="mt-1 text-sm text-muted-foreground">
								Manage AI providers used for template generation and log
								analysis.
							</p>
						</div>
						<Button onClick={create}>
							<Plus className="mr-2 size-4" /> Add AI
						</Button>
					</div>
					<div className="border-t p-6">
						<AiSettingsList
							items={items}
							loading={loading}
							onCreate={create}
							onEdit={edit}
							onDelete={remove}
						/>
					</div>
				</div>
			</Card>
			<AiProviderDialog
				editing={editing}
				open={open}
				onOpenChange={setOpen}
				onSaved={load}
			/>
		</div>
	);
}
