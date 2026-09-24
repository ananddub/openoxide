import {useState} from 'react';
import {
	Import,
	Pencil,
	RefreshCw,
	Trash2,
	Users,
	CheckCircle2,
	AlertCircle,
	Link2,
} from 'lucide-react';
import {
	BitbucketIcon,
	GiteaIcon,
	GithubIcon,
	GitlabIcon,
} from '#/components/icons/data-tools-icons';
import {Button} from '#/components/ui/button';
import {
	ListCard,
	ListRow,
	type ListView,
	RowTile,
	ViewCard,
	ViewGrid,
} from '#/components/settings/list-card';
import {
	type GitProviderKind,
	GIT_PROVIDER_LABELS,
} from './git-provider-types';

interface GitProviderListProps {
	/** Already filtered by the page toolbar. */
	providers: any[];
	view: ListView;
	onAuthorize: (id: number) => void;
	onTest: (id: number) => Promise<void> | void;
	onEdit: (provider: any) => void;
	onDelete: (id: number) => void;
}

function KindIcon({kind}: {kind: GitProviderKind}) {
	switch (kind) {
		case 'gitlab':
			return <GitlabIcon className="size-4" />;
		case 'bitbucket':
			return <BitbucketIcon className="size-4" />;
		case 'gitea':
			return <GiteaIcon className="size-4" />;
		default:
			return <GithubIcon className="size-4" />;
	}
}

function ProviderTile({kind}: {kind: GitProviderKind}) {
	return (
		<RowTile>
			<KindIcon kind={kind} />
		</RowTile>
	);
}

function KindChip({kind}: {kind: GitProviderKind}) {
	return (
		<span className="shrink-0 rounded-md bg-foreground/[0.05] px-1.5 py-0.5 text-[10px] font-medium text-muted-foreground">
			{GIT_PROVIDER_LABELS[kind] || kind}
		</span>
	);
}

function StatusChip({configured}: {configured: boolean}) {
	return configured ? (
		<span className="flex shrink-0 items-center gap-1 rounded-md bg-emerald-500/10 px-1.5 py-0.5 text-[10px] font-medium text-emerald-500">
			<CheckCircle2 className="size-3" /> Connected
		</span>
	) : (
		<span className="flex shrink-0 items-center gap-1 rounded-md bg-amber-500/10 px-1.5 py-0.5 text-[10px] font-medium text-amber-600 dark:text-amber-400">
			<AlertCircle className="size-3" /> Action required
		</span>
	);
}

function SharedChip() {
	return (
		<span className="flex shrink-0 items-center gap-1 rounded-md bg-foreground/[0.05] px-1.5 py-0.5 text-[10px] font-medium text-muted-foreground">
			<Users className="size-3" /> Shared org
		</span>
	);
}

function hostLine(provider: any) {
	return (
		provider.config?.url ||
		(provider.config?.app_name
			? `App: ${provider.config.app_name}`
			: 'Ready')
	);
}

function Actions({
	provider,
	testing,
	onAuthorize,
	onTest,
	onEdit,
	onDelete,
}: {
	provider: any;
	testing: boolean;
	onAuthorize: (id: number) => void;
	onTest: () => void;
	onEdit: (provider: any) => void;
	onDelete: (id: number) => void;
}) {
	return (
		<>
			{!provider.configured && (
				<Button
					variant="outline"
					size="sm"
					title="Authorize this provider"
					onClick={() => onAuthorize(provider.id)}
					className="h-7 gap-1 rounded-lg px-2 text-[11px] font-medium">
					<Import className="size-3" />
					Authorize
				</Button>
			)}
			<Button
				variant="ghost"
				size="icon"
				title={testing ? 'Testing…' : 'Test connection'}
				disabled={testing}
				onClick={onTest}
				className="size-7 rounded-lg text-muted-foreground hover:text-foreground">
				<RefreshCw
					className={`size-3.5 ${testing ? 'animate-spin' : ''}`}
				/>
			</Button>
			<Button
				variant="ghost"
				size="icon"
				title="Edit provider"
				onClick={() => onEdit(provider)}
				className="size-7 rounded-lg text-muted-foreground hover:text-foreground">
				<Pencil className="size-3.5" />
			</Button>
			<Button
				variant="ghost"
				size="icon"
				title="Delete provider"
				onClick={() => onDelete(provider.id)}
				className="size-7 rounded-lg text-muted-foreground hover:bg-destructive/10 hover:text-destructive">
				<Trash2 className="size-3.5" />
			</Button>
		</>
	);
}

function useTest(provider: any, onTest: GitProviderListProps['onTest']) {
	const [testing, setTesting] = useState(false);
	const run = async () => {
		setTesting(true);
		try {
			await onTest(provider.id);
		} finally {
			setTesting(false);
		}
	};
	return {testing, run};
}

/** Git providers — grid cards or one bordered list card, same grammar. */
export function GitProviderList({
	providers,
	view,
	onAuthorize,
	onTest,
	onEdit,
	onDelete,
}: GitProviderListProps) {
	if (view === 'grid') {
		return (
			<ViewGrid>
				{providers.map(provider => (
					<GridCard
						key={provider.id}
						provider={provider}
						onAuthorize={onAuthorize}
						onTest={onTest}
						onEdit={onEdit}
						onDelete={onDelete}
					/>
				))}
			</ViewGrid>
		);
	}

	return (
		<ListCard>
			{providers.map(provider => (
				<Row
					key={provider.id}
					provider={provider}
					onAuthorize={onAuthorize}
					onTest={onTest}
					onEdit={onEdit}
					onDelete={onDelete}
				/>
			))}
		</ListCard>
	);
}

function GridCard({
	provider,
	onAuthorize,
	onTest,
	onEdit,
	onDelete,
}: {
	provider: any;
	onAuthorize: (id: number) => void;
	onTest: (id: number) => void;
	onEdit: (provider: any) => void;
	onDelete: (id: number) => void;
}) {
	const kind = (provider.provider_type || 'github') as GitProviderKind;
	const {testing, run} = useTest(provider, onTest);

	return (
		<ViewCard>
			<div className="flex min-w-0 items-start gap-3">
				<ProviderTile kind={kind} />
				<div className="min-w-0 flex-1">
					<span className="block truncate text-sm font-medium text-foreground">
						{provider.name}
					</span>
					<p className="mt-0.5 flex min-w-0 items-center gap-1.5 truncate text-[11px] text-muted-foreground">
						<Link2 className="size-3 shrink-0 opacity-60" />
						<span className="truncate">{hostLine(provider)}</span>
					</p>
				</div>
			</div>
			<div className="flex items-end justify-between gap-2">
				<div className="flex min-w-0 flex-wrap items-center gap-1.5">
					<KindChip kind={kind} />
					<StatusChip configured={!!provider.configured} />
					{provider.shared && <SharedChip />}
				</div>
				<div className="flex shrink-0 items-center gap-1 opacity-0 transition-opacity group-focus-within:opacity-100 group-hover:opacity-100">
					<Actions
						provider={provider}
						testing={testing}
						onAuthorize={onAuthorize}
						onTest={() => void run()}
						onEdit={onEdit}
						onDelete={onDelete}
					/>
				</div>
			</div>
		</ViewCard>
	);
}

function Row({
	provider,
	onAuthorize,
	onTest,
	onEdit,
	onDelete,
}: {
	provider: any;
	onAuthorize: (id: number) => void;
	onTest: (id: number) => void;
	onEdit: (provider: any) => void;
	onDelete: (id: number) => void;
}) {
	const kind = (provider.provider_type || 'github') as GitProviderKind;
	const {testing, run} = useTest(provider, onTest);

	return (
		<ListRow
			icon={<ProviderTile kind={kind} />}
			title={provider.name}
			badges={
				<>
					<KindChip kind={kind} />
					<StatusChip configured={!!provider.configured} />
					{provider.shared && <SharedChip />}
				</>
			}
			subtitle={
				<span className="flex items-center gap-1.5">
					<Link2 className="size-3 shrink-0 opacity-60" />
					{hostLine(provider)}
				</span>
			}>
			<Actions
				provider={provider}
				testing={testing}
				onAuthorize={onAuthorize}
				onTest={() => void run()}
				onEdit={onEdit}
				onDelete={onDelete}
			/>
		</ListRow>
	);
}
