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
import {Badge} from '#/components/ui/badge';
import {Button} from '#/components/ui/button';
import {
	type GitProviderKind,
	GIT_PROVIDER_LABELS,
} from './git-provider-types';

interface GitProviderCardProps {
	provider: any;
	onAuthorize: (id: number) => void;
	onTest: (id: number) => Promise<void> | void;
	onEdit: (provider: any) => void;
	onDelete: (id: number) => void;
}

export function GitProviderCard({
	provider,
	onAuthorize,
	onTest,
	onEdit,
	onDelete,
}: GitProviderCardProps) {
	const [testing, setTesting] = useState(false);
	const kind = (provider.provider_type || 'github') as GitProviderKind;

	const renderIcon = (type: GitProviderKind) => {
		switch (type) {
			case 'github':
				return <GithubIcon className="size-5 text-foreground" />;
			case 'gitlab':
				return <GitlabIcon className="size-5 text-orange-500" />;
			case 'bitbucket':
				return <BitbucketIcon className="size-5 text-blue-500" />;
			case 'gitea':
				return <GiteaIcon className="size-5 text-emerald-500" />;
			default:
				return <GithubIcon className="size-5 text-foreground" />;
		}
	};

	const handleTestClick = async () => {
		setTesting(true);
		try {
			await onTest(provider.id);
		} finally {
			setTesting(false);
		}
	};

	return (
		<div className="flex flex-col justify-between gap-4 rounded-2xl border border-border/80 bg-card p-4 transition-all hover:border-primary/40 hover:shadow-xs sm:flex-row sm:items-center">
			<div className="flex min-w-0 items-center gap-3.5">
				<div className="flex size-11 shrink-0 items-center justify-center rounded-xl border border-border/60 bg-muted/50">
					{renderIcon(kind)}
				</div>

				<div className="flex min-w-0 flex-col gap-1">
					<div className="flex flex-wrap items-center gap-2">
						<span className="truncate text-sm font-bold text-foreground">
							{provider.name}
						</span>
						<Badge variant="outline" className="text-[10px] font-semibold">
							{GIT_PROVIDER_LABELS[kind] || kind}
						</Badge>
						{provider.configured ? (
							<span className="flex items-center gap-1 rounded-full border border-emerald-500/20 bg-emerald-500/10 px-2 py-0.5 text-[10px] font-bold text-emerald-500">
								<CheckCircle2 className="size-3" /> Connected
							</span>
						) : (
							<span className="flex animate-pulse items-center gap-1 rounded-full border border-amber-500/20 bg-amber-500/10 px-2 py-0.5 text-[10px] font-bold text-amber-500">
								<AlertCircle className="size-3" /> Action Required
							</span>
						)}
						{provider.shared && (
							<Badge
								variant="secondary"
								className="gap-1 py-0.5 text-[10px]">
								<Users className="size-3 text-muted-foreground" /> Shared
								Org
							</Badge>
						)}
					</div>

					<div className="flex items-center gap-2 truncate text-xs text-muted-foreground">
						<Link2 className="size-3 shrink-0 opacity-70" />
						<span className="truncate">
							{provider.config?.url ||
								(provider.config?.app_name
									? `App: ${provider.config.app_name}`
									: 'Configured & Ready')}
						</span>
					</div>
				</div>
			</div>

			<div className="flex shrink-0 items-center gap-1.5 self-end sm:self-center">
				{!provider.configured && (
					<Button
						variant="outline"
						size="sm"
						onClick={() => onAuthorize(provider.id)}
						className="h-8 gap-1 rounded-lg border-primary/40 text-xs font-semibold text-primary hover:bg-primary/10">
						<Import className="size-3.5" /> Authorize
					</Button>
				)}

				<Button
					variant="outline"
					size="sm"
					onClick={handleTestClick}
					disabled={testing}
					className="h-8 gap-1.5 rounded-lg border-border/80 text-xs font-medium text-foreground hover:bg-muted">
					<RefreshCw
						className={`size-3.5 ${testing ? 'animate-spin' : ''}`}
					/>
					{testing ? 'Testing...' : 'Test'}
				</Button>

				<Button
					variant="ghost"
					size="icon"
					onClick={() => onEdit(provider)}
					className="size-8 rounded-lg text-muted-foreground hover:text-foreground">
					<Pencil className="size-3.5" />
				</Button>

				<Button
					variant="ghost"
					size="icon"
					onClick={() => onDelete(provider.id)}
					className="size-8 rounded-lg text-muted-foreground hover:bg-destructive/10 hover:text-destructive">
					<Trash2 className="size-3.5" />
				</Button>
			</div>
		</div>
	);
}
