import {Import, Pencil, RefreshCw, Trash2, Users} from 'lucide-react';
import {BitbucketIcon, GiteaIcon, GithubIcon, GitlabIcon} from '#/components/icons/data-tools-icons';
import {Badge} from '#/components/ui/badge';
import {Button} from '#/components/ui/button';
import {Card} from '#/components/ui/card';
import {type GitProviderKind, GIT_PROVIDER_LABELS} from './git-provider-types';

interface GitProviderCardProps {
	provider: any;
	onAuthorize: (id: number) => void;
	onTest: (id: number) => void;
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
	const kind = (provider.provider_type || 'github') as GitProviderKind;

	const renderIcon = (type: GitProviderKind) => {
		switch (type) {
			case 'github':
				return <GithubIcon className="size-5" />;
			case 'gitlab':
				return <GitlabIcon className="size-5" />;
			case 'bitbucket':
				return <BitbucketIcon className="size-5" />;
			case 'gitea':
				return <GiteaIcon className="size-5" />;
			default:
				return <GithubIcon className="size-5" />;
		}
	};

	return (
		<Card className="flex items-center justify-between p-4 hover:border-border/80 transition-colors">
			<div className="flex items-center gap-3">
				{renderIcon(kind)}
				<div>
					<div className="flex items-center gap-2">
						<span className="font-semibold">{provider.name}</span>
						<Badge variant="outline">{GIT_PROVIDER_LABELS[kind] || kind}</Badge>
						<Badge variant={provider.configured ? 'secondary' : 'destructive'}>
							{provider.configured ? 'Configured' : 'Action Required'}
						</Badge>
						{provider.shared && (
							<Badge variant="secondary">
								<Users className="mr-1 size-3" /> Shared
							</Badge>
						)}
					</div>
					<p className="text-xs text-muted-foreground mt-0.5">
						{provider.config?.url || (provider.config?.app_name ? 'GitHub App' : 'Credentials configured')}
					</p>
				</div>
			</div>

			<div className="flex gap-1">
				{!provider.configured && (
					<Button
						variant="ghost"
						size="icon"
						title="Authorize provider"
						onClick={() => onAuthorize(provider.id)}
					>
						<Import className="size-4 text-primary" />
					</Button>
				)}

				<Button
					variant="ghost"
					size="icon"
					title="Test connection"
					onClick={() => onTest(provider.id)}
				>
					<RefreshCw className="size-4" />
				</Button>

				<Button
					variant="ghost"
					size="icon"
					title="Edit"
					onClick={() => onEdit(provider)}
				>
					<Pencil className="size-4" />
				</Button>

				<Button
					variant="ghost"
					size="icon"
					title="Delete"
					onClick={() => onDelete(provider.id)}
				>
					<Trash2 className="size-4 text-destructive" />
				</Button>
			</div>
		</Card>
	);
}
