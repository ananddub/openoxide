import {useMemo, useState} from 'react';
import type {ReactNode} from 'react';
import {ArrowUpDown, Globe, Plus, Search, ShieldCheck} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import {
	ListCard,
	ListEmpty,
	ListSkeleton,
	SectionHeader,
	Toolbar,
	ToolbarSearch,
	ViewToggle,
} from '#/components/settings/list-card';
import {DomainPreviewModal} from '#/components/projects/common/domain-preview-modal';
import {buildDomainUrl, DomainCard, DomainRow} from './domain-item';

export type DomainsView = 'grid' | 'list';

type TlsFilter = 'all' | 'https' | 'http';

interface DomainsSectionProps {
	domains: any[];
	isLoading?: boolean;
	onCreate: () => void;
	onEdit: (domain: any) => void;
	onDelete: (id: number) => void | Promise<void>;
	description?: ReactNode;
	headerAction?: ReactNode;
	createLabel?: string;
	previewTitle?: string;
}

/**
 * Domains surface — same grammar as Schedules / SSH Keys: `SectionHeader`
 * carrying the title and the create action, a quiet `Toolbar` (search + TLS
 * filter + grid/list switch) under it, then the list card or card grid.
 */
export function DomainsSection({
	domains,
	isLoading,
	onCreate,
	onEdit,
	onDelete,
	description,
	headerAction,
	createLabel = 'Add Domain',
	previewTitle = 'Domain preview',
}: DomainsSectionProps) {
	const [query, setQuery] = useState('');
	const [tls, setTls] = useState<TlsFilter>('all');
	const [view, setView] = useState<DomainsView>('grid');
	const [previewUrl, setPreviewUrl] = useState<string | null>(null);
	const [deletingId, setDeletingId] = useState<number | null>(null);

	const safe = Array.isArray(domains) ? domains : [];
	const q = query.trim().toLowerCase();

	const visible = useMemo(
		() =>
			safe.filter((d: any) => {
				if (tls === 'https' && !d.https) return false;
				if (tls === 'http' && d.https) return false;
				if (!q) return true;
				return [
					d.host,
					d.domain,
					d.path,
					d.internal_path,
					d.service_name,
					d.port,
				]
					.filter(v => v !== undefined && v !== null)
					.join(' ')
					.toLowerCase()
					.includes(q);
			}),
		[safe, q, tls],
	);

	const handleDelete = async (id: number) => {
		setDeletingId(id);
		try {
			await onDelete(id);
		} finally {
			setDeletingId(null);
		}
	};

	const clearFilters = () => {
		setQuery('');
		setTls('all');
	};

	const createButton = (
		<Button onClick={onCreate} className="h-9 gap-1.5">
			<Plus className="size-3.5" /> {createLabel}
		</Button>
	);

	return (
		<div className="flex w-full flex-col gap-4">
			<SectionHeader
				title="Domains"
				description={description}
				action={
					<>
						{headerAction}
						{createButton}
					</>
				}
			/>

			<Toolbar>
				<ToolbarSearch
					value={query}
					onChange={setQuery}
					placeholder="Search domains…"
					className="min-w-[200px] flex-1 sm:max-w-none"
				/>

				<Select
					value={tls}
					onValueChange={val => val && setTls(val as TlsFilter)}>
					<SelectTrigger className="h-9 w-auto rounded-lg bg-card">
						<div className="flex min-w-0 items-center gap-2">
							<ShieldCheck className="size-3.5 shrink-0 text-muted-foreground" />
							<SelectValue>
								{tls === 'all'
									? 'All TLS'
									: tls === 'https'
										? 'HTTPS only'
										: 'HTTP only'}
							</SelectValue>
						</div>
					</SelectTrigger>
					<SelectContent className="bg-card">
						<SelectItem value="all">TLS: All</SelectItem>
						<SelectItem value="https">TLS: HTTPS only</SelectItem>
						<SelectItem value="http">TLS: HTTP only</SelectItem>
					</SelectContent>
				</Select>

				<ViewToggle view={view} onChange={setView} />
			</Toolbar>

			{isLoading && safe.length === 0 ? (
				<ListSkeleton />
			) : safe.length === 0 ? (
				<ListEmpty
					icon={Globe}
					title="No domain routes yet"
					description="Add a host to expose this service on your own domain."
					action={createButton}
				/>
			) : visible.length === 0 ? (
				<ListEmpty
					icon={Search}
					title="No domains match your filters"
					description="Try a different host, path or service — or clear the filters."
					action={
						<Button
							variant="ghost"
							onClick={clearFilters}
							className="gap-1.5 text-xs font-semibold">
							<ArrowUpDown className="size-3.5" /> Clear filters
						</Button>
					}
				/>
			) : view === 'grid' ? (
				<div className="grid w-full grid-cols-1 gap-3 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
					{visible.map((d: any) => (
						<DomainCard
							key={d.id}
							domain={d}
							deleting={deletingId === d.id}
							onPreview={() => setPreviewUrl(buildDomainUrl(d))}
							onEdit={() => onEdit(d)}
							onDelete={() => handleDelete(d.id)}
						/>
					))}
				</div>
			) : (
				<ListCard className="divide-border/60">
					{visible.map((d: any) => (
						<DomainRow
							key={d.id}
							domain={d}
							deleting={deletingId === d.id}
							onPreview={() => setPreviewUrl(buildDomainUrl(d))}
							onEdit={() => onEdit(d)}
							onDelete={() => handleDelete(d.id)}
						/>
					))}
				</ListCard>
			)}

			<DomainPreviewModal
				open={!!previewUrl}
				onClose={() => setPreviewUrl(null)}
				url={previewUrl || ''}
				title={previewTitle}
			/>
		</div>
	);
}
