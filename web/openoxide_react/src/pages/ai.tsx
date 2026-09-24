import {createFileRoute} from '@tanstack/react-router';
import {Bot, Plus, Search} from 'lucide-react';
import {useMemo, useState} from 'react';
import {toast} from 'sonner';
import {Button} from '#/components/ui/button';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import {AiProviderSheet} from '#/components/settings/ai/ai-provider-sheet';
import {deleteAiSetting} from '#/components/settings/ai/ai-api';
import {AiSettingsList} from '#/components/settings/ai/ai-settings-list';
import {ConfirmDeleteDialog} from '#/components/settings/confirm-delete-dialog';
import {
	GridSkeleton,
	ListEmpty,
	ListSkeleton,
	SectionHeader,
	Toolbar,
	ToolbarSearch,
	ViewToggle,
	useListView,
} from '#/components/settings/list-card';
import type {AiSetting} from '#/components/settings/ai/ai-types';
import {useAiSettingItems, useConnectMutation} from '#/api/live';

export const Route = createFileRoute('/_app/ai')({
	component: AiSettingsPage,
});

const STATUS_FILTERS = ['All', 'Enabled', 'Disabled'] as const;
type StatusFilter = (typeof STATUS_FILTERS)[number];

export function AiSettingsPage() {
	const itemsQuery = useAiSettingItems();
	const items = (itemsQuery.data ?? []) as AiSetting[];
	const [open, setOpen] = useState(false);
	const [editing, setEditing] = useState<AiSetting | null>(null);
	const [deleting, setDeleting] = useState<AiSetting | null>(null);
	const [search, setSearch] = useState('');
	const [status, setStatus] = useState<StatusFilter>('All');
	const [view, setView] = useListView('settings-ai-view');

	const q = search.trim().toLowerCase();
	const filtered = useMemo(
		() =>
			items.filter(item => {
				if (status === 'Enabled' && !item.is_enabled) return false;
				if (status === 'Disabled' && item.is_enabled) return false;
				if (!q) return true;
				return [item.name, item.provider, item.model, item.api_url]
					.filter(Boolean)
					.join(' ')
					.toLowerCase()
					.includes(q);
			}),
		[items, q, status],
	);
	const hasFilters = q !== '' || status !== 'All';

	// Cache invalidation comes from the table-watch stream (ai_settings table → refetch while this page is mounted).
	const deleteMutation = useConnectMutation(
		async (id: number) => {
			await deleteAiSetting(id);
			return id;
		},
		{
			onSuccess: () => {
				toast.success('AI configuration deleted');
			},
			onError: (error: unknown) => {
				toast.error(
					error instanceof Error
						? error.message
						: 'Failed to delete AI configuration',
				);
			},
		},
	);

	const create = () => {
		setEditing(null);
		setOpen(true);
	};

	const edit = (item: AiSetting) => {
		setEditing(item);
		setOpen(true);
	};

	const remove = async () => {
		if (!deleting) return;
		await deleteMutation.mutateAsync(Number(deleting.id));
		setDeleting(null);
	};

	return (
		<div className="mx-auto flex w-full max-w-[1920px] animate-in flex-col gap-5 px-4 pt-4 pb-6 duration-200 fade-in">
			<SectionHeader
				title="AI Providers"
				description="Model credentials used for log diagnosis and stack generation."
				action={
					<Button onClick={create} className="h-9 gap-1.5">
						<Plus className="size-3.5" />
						New provider
					</Button>
				}>
				<span className="shrink-0 rounded-md bg-foreground/[0.05] px-1.5 py-0.5 text-[10px] font-medium text-muted-foreground">
					{items.filter(i => i.is_enabled).length} of {items.length}{' '}
					enabled
				</span>
			</SectionHeader>

			<Toolbar
				action={
					<>
						<Select
							value={status}
							onValueChange={v => v && setStatus(v as StatusFilter)}>
							<SelectTrigger className="h-9 w-auto rounded-lg bg-card">
								<SelectValue>
									{status === 'All'
										? 'All Providers'
										: status === 'Enabled'
											? 'Enabled'
											: 'Disabled'}
								</SelectValue>
							</SelectTrigger>
							<SelectContent className="bg-card">
								{STATUS_FILTERS.map(f => (
									<SelectItem key={f} value={f}>
										{f === 'All' ? 'All Providers' : f}
									</SelectItem>
								))}
							</SelectContent>
						</Select>
						<ViewToggle view={view} onChange={setView} />
					</>
				}>
				<ToolbarSearch
					value={search}
					onChange={setSearch}
					placeholder="Search by name, provider or model…"
					className="min-w-[200px] flex-1 sm:max-w-none"
				/>
			</Toolbar>

			{itemsQuery.isLoading ? (
				view === 'grid' ? (
					<GridSkeleton />
				) : (
					<ListSkeleton />
				)
			) : items.length === 0 ? (
				<ListEmpty
					icon={Bot}
					title="No AI providers yet"
					description="Connect an OpenAI, Anthropic, Gemini or local Ollama provider for log diagnosis and stack generation."
					action={
						<Button
							onClick={create}
							className="h-9 gap-1.5 text-xs font-semibold">
							<Plus className="size-3.5" /> New provider
						</Button>
					}
				/>
			) : filtered.length === 0 ? (
				<ListEmpty
					icon={Search}
					title="No providers match your filters"
					description="Try a different name, provider or model — or clear the filters."
					action={
						hasFilters && (
							<Button
								variant="ghost"
								onClick={() => {
									setSearch('');
									setStatus('All');
								}}
								className="text-xs font-semibold text-primary">
								Clear All Filters
							</Button>
						)
					}
				/>
			) : (
				<AiSettingsList
					items={filtered}
					view={view}
					onEdit={edit}
					onDelete={setDeleting}
				/>
			)}

			<ConfirmDeleteDialog
				open={deleting !== null}
				onOpenChange={o => !o && setDeleting(null)}
				title="Delete provider"
				description={
					<span>
						<span className="font-medium text-foreground">
							{deleting?.name}
						</span>{' '}
						will stop being available for log diagnosis and template
						generation. This cannot be undone.
					</span>
				}
				confirmLabel="Delete provider"
				busy={deleteMutation.isPending}
				onConfirm={() => void remove()}
			/>

			{/* Add/Edit Modal */}
			<AiProviderSheet
				editing={editing}
				open={open}
				onOpenChange={setOpen}
				onSaved={async () => {
					/* table-watch invalidates ['ai_settings'] → refetch */
				}}
			/>
		</div>
	);
}
