import {useMemo, useState} from 'react';
import {ChevronLeft, ExternalLink, Rocket, Search} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Badge} from '#/components/ui/badge';
import {Input} from '#/components/ui/input';
import {
	Sheet,
	SheetContent,
	SheetDescription,
	SheetFooter,
	SheetHeader,
	SheetTitle,
} from '#/components/ui/sheet';
import {
	Tabs,
	TabsContent,
	TabsList,
	TabsTrigger,
} from '#/components/ui/tabs';
import {TemplateCard} from './template-card';
import {TemplateListItem} from './template-list-item';
import {TemplateLogo} from './template-logo';
import {TemplateFilePreview} from './template-file-preview';
import {TemplateDeployDialog} from './template-deploy-dialog';
import {
	ListCard,
	ViewToggle,
	useListView,
} from '#/components/settings/list-card';
import {
	useTemplateFile,
	useTemplateList,
	type TemplateMeta,
} from '#/api/live';

/**
 * Template gallery as a wide right side panel — the Template entry point
 * on the project services/create pages. One panel, in-panel navigation:
 * the card grid is the root view, clicking a card swaps the body to that
 * template's detail (back link up top), Deploy opens its form on top.
 */
export function TemplateGallerySheet({
	open,
	onClose,
}: {
	open: boolean;
	onClose: () => void;
}) {
	const [search, setSearch] = useState('');
	const [selected, setSelected] = useState<TemplateMeta | null>(null);
	const [deployOpen, setDeployOpen] = useState(false);
	const [view, setView] = useListView('templates-gallery-view');

	const templatesQuery = useTemplateList();
	const templates: TemplateMeta[] = useMemo(
		() => (Array.isArray(templatesQuery.data) ? templatesQuery.data : []),
		[templatesQuery.data],
	);

	const filtered = useMemo(() => {
		const q = search.trim().toLowerCase();
		if (!q) return templates;
		return templates.filter(
			t =>
				t.name.toLowerCase().includes(q) ||
				t.description?.toLowerCase().includes(q) ||
				t.tags?.some(tag => tag.toLowerCase().includes(q)),
		);
	}, [templates, search]);

	const compose = useTemplateFile(
		selected?.id ?? '',
		'docker-compose.yml',
	);
	const toml = useTemplateFile(selected?.id ?? '', 'template.toml');
	const links = selected ? Object.entries(selected.links ?? {}) : [];

	const closeAll = () => {
		setSelected(null);
		setDeployOpen(false);
		onClose();
	};

	return (
		<>
			<Sheet open={open} onOpenChange={next => !next && closeAll()}>
				<SheetContent className="sm:data-[side=right]:w-[860px]">
					<SheetHeader>
						<SheetTitle>
							{selected ? selected.name : 'Templates'}
						</SheetTitle>
						<SheetDescription className="text-xs">
							{selected
								? selected.description
								: 'Pick a ready-made blueprint — it installs here as a compose stack with its variables pre-filled.'}
						</SheetDescription>
					</SheetHeader>

					<div className="flex min-h-0 flex-1 flex-col gap-4 px-6 pt-4 pb-6">
						{selected ? (
							<div
								key={selected.id}
								className="flex min-h-0 flex-1 animate-in flex-col gap-4 overflow-y-auto duration-200 ease-out fade-in-0 slide-in-from-right-4">
								<div className="flex shrink-0 items-center gap-3">
									<TemplateLogo
										template={selected}
										className="size-12 shrink-0"
									/>
									<Badge
										variant="outline"
										className="shrink-0 font-mono text-[10px] font-normal">
										{selected.version}
									</Badge>
									{selected.tags?.length > 0 && (
										<div className="flex min-w-0 flex-wrap gap-1">
											{selected.tags.map(tag => (
												<Badge
													key={tag}
													variant="secondary"
													className="font-mono text-[10px] font-normal">
													{tag}
												</Badge>
											))}
										</div>
									)}
								</div>

								{links.length > 0 && (
									<div className="flex shrink-0 flex-wrap gap-4">
										{links.map(([label, href]) => (
											<a
												key={label}
												href={href}
												target="_blank"
												rel="noreferrer noopener"
												className="inline-flex items-center gap-1 text-xs font-medium text-primary hover:underline">
												<ExternalLink className="size-3" />
												{label}
											</a>
										))}
									</div>
								)}

								<Tabs defaultValue="compose" className="shrink-0">
									<TabsList>
										<TabsTrigger value="compose">
											docker-compose.yml
										</TabsTrigger>
										<TabsTrigger value="toml">
											template.toml
										</TabsTrigger>
									</TabsList>
									<TabsContent value="compose">
										<TemplateFilePreview
											filename="docker-compose.yml"
											language="yaml"
											data={compose.data}
											isLoading={compose.isLoading}
										/>
									</TabsContent>
									<TabsContent value="toml">
										<TemplateFilePreview
											filename="template.toml"
											language="ini"
											data={toml.data}
											isLoading={toml.isLoading}
										/>
									</TabsContent>
								</Tabs>
							</div>
						) : (
							<div
								key="list"
								className="flex min-h-0 flex-1 animate-in flex-col gap-4 duration-200 ease-out fade-in-0 slide-in-from-left-4">
								<div className="flex shrink-0 items-center gap-2">
									<div className="relative flex-1">
										<Search className="pointer-events-none absolute top-1/2 left-3 size-4 -translate-y-1/2 text-muted-foreground/70" />
										<Input
											placeholder="Search templates"
											value={search}
											onChange={e =>
												setSearch(e.target.value)
											}
											className="h-9 rounded-lg pl-9 text-sm transition-colors duration-150 hover:border-foreground/25"
										/>
									</div>
									<ViewToggle view={view} onChange={setView} />
								</div>

								<div className="min-h-0 flex-1 overflow-y-auto">
									{templatesQuery.isLoading ? (
										<div className="grid grid-cols-1 gap-3 md:grid-cols-2">
											{[...Array(4)].map((_, i) => (
												<div
													key={i}
													className="min-h-[124px] animate-pulse rounded-lg border bg-card"
												/>
											))}
										</div>
									) : templatesQuery.isError ? (
										<div className="flex flex-col items-center justify-center gap-3 rounded-xl border bg-card p-12 text-center">
											<p className="text-sm text-muted-foreground">
												Template registry
												unreachable — check your
												network connection.
											</p>
											<Button
												variant="outline"
												onClick={() =>
													templatesQuery.refetch()
												}
												className="h-9 text-xs font-semibold">
												Retry
											</Button>
										</div>
									) : filtered.length === 0 ? (
										<div className="flex flex-col items-center justify-center gap-2 rounded-xl border bg-card p-12 text-center">
											<Search className="size-8 text-muted-foreground/40" />
											<p className="text-sm text-muted-foreground">
												No templates match
												&ldquo;{search}&rdquo;
											</p>
										</div>
									) : view === 'grid' ? (
										<div className="grid grid-cols-1 gap-3 md:grid-cols-2">
											{filtered.map(template => (
												<TemplateCard
													key={template.id}
													template={template}
													onOpen={setSelected}
												/>
											))}
										</div>
									) : (
										<ListCard>
											{filtered.map(template => (
												<TemplateListItem
													key={template.id}
													template={template}
													onOpen={setSelected}
												/>
											))}
										</ListCard>
									)}
									</div>
							</div>
						)}
					</div>

					{selected && (
						<SheetFooter>
							<Button
								variant="outline"
								onClick={() => setSelected(null)}
								className="mr-auto h-9 gap-1.5 text-sm">
								<ChevronLeft className="size-3.5" />
								Back
							</Button>
							<Button
								onClick={() => setDeployOpen(true)}
								className="h-9 gap-1.5">
								<Rocket className="size-3.5" />
								Deploy
							</Button>
						</SheetFooter>
					)}
				</SheetContent>
			</Sheet>

			<TemplateDeployDialog
				template={selected}
				open={deployOpen}
				onOpenChange={setDeployOpen}
			/>
		</>
	);
}
