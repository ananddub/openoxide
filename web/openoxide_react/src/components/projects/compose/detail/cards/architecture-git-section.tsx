import {useState} from 'react';
import {Link} from '@tanstack/react-router';
import {ChevronDown, KeyRound} from 'lucide-react';
import {toast} from 'sonner';
import {cn, formatApiError} from '#/api/utils';
import {
	Collapsible,
	CollapsibleTrigger,
	CollapsibleContent,
} from '#/components/ui/collapsible';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
} from '#/components/ui/select';
import {useComposeSetCustomGitSource} from '#/api/live/compose/sources';
import {useApplicationSetCustomGitSource} from '#/api/live/applications/sources';
import {useSshKeyItems} from '#/api/live/ssh_keys';

const GIT_PROVIDERS = ['GITHUB', 'GITLAB', 'GITEA', 'BITBUCKET', 'GIT'];

function Field({label, value}: {label: string; value: string}) {
	return (
		<div className="min-w-0">
			<p className="text-[10px] font-bold tracking-wider text-muted-foreground uppercase">
				{label}
			</p>
			<p className="mt-1 truncate font-mono text-[12px] text-foreground">
				{value || '—'}
			</p>
		</div>
	);
}

interface ArchitectureGitSectionProps {
	entity: any;
	kind: 'compose' | 'app';
	onUpdated?: () => void;
}

/**
 * Vercel-style collapsible integration strip inside the architecture card:
 * provider/repo summary in the trigger, full source details plus the deploy
 * key dropdown in the panel (custom-Git sources only).
 */
export function ArchitectureGitSection({
	entity,
	kind,
	onUpdated,
}: ArchitectureGitSectionProps) {
	const [open, setOpen] = useState(false);
	const [saving, setSaving] = useState(false);

	const provider = String(entity?.source_type || '').toUpperCase();
	const isGit = GIT_PROVIDERS.includes(provider);

	const repoUrl = String(
		entity?.custom_git_url || entity?.repository || '',
	);
	const owner = String(
		entity?.owner ||
			entity?.gitlab_owner ||
			entity?.gitea_owner ||
			entity?.bitbucket_owner ||
			'',
	);
	const repo = String(
		entity?.repository ||
			entity?.gitlab_repository ||
			entity?.gitea_repository ||
			entity?.bitbucket_repository ||
			'',
	);
	const branch = String(
		entity?.branch || entity?.custom_git_branch || 'main',
	);
	const buildPath = String(
		entity?.build_path ||
			entity?.custom_git_build_path ||
			(kind === 'compose' ? 'docker-compose.yml' : ''),
	);
	const repoSummary =
		provider === 'GIT'
			? repoUrl
			: owner && repo
				? `${owner}/${repo}`
				: repo;

	const {data: keys = []} = useSshKeyItems();
	const patchComposeGit = useComposeSetCustomGitSource();
	const patchAppGit = useApplicationSetCustomGitSource();

	if (!isGit || !entity?.id) return null;

	const currentKeyId = Number(
		entity?.custom_git_ssh_key_id ?? entity?.ssh_key_id ?? 0,
	);
	const currentKey = keys.find(k => k.id === currentKeyId);

	const changeCredential = async (value: string) => {
		const keyId = Number(value) || 0;
		setSaving(true);
		try {
			if (kind === 'compose') {
				await patchComposeGit.mutateAsync({
					id: entity.id,
					url: repoUrl,
					branch,
					build_path: buildPath,
					ssh_key_id: keyId,
				});
			} else {
				await patchAppGit.mutateAsync({
					id: entity.id,
					url: repoUrl,
					repository: repo,
					owner,
					branch,
					build_path: buildPath,
					ssh_key_id: keyId,
				});
			}
			toast.success('Deploy key updated');
			onUpdated?.();
		} catch (err) {
			toast.error(formatApiError(err));
		} finally {
			setSaving(false);
		}
	};

	return (
		<Collapsible
			open={open}
			onOpenChange={setOpen}
			className="border-t border-border">
			<CollapsibleTrigger
				className={cn(
					'group flex w-full cursor-pointer items-center gap-3 px-5 py-3.5 text-left transition-colors hover:bg-muted/30',
					open && 'bg-muted/30',
				)}>
				<ChevronDown
					className={cn(
						'size-4 shrink-0 text-muted-foreground transition-transform duration-200',
						!open && '-rotate-90',
					)}
				/>
				<span className="shrink-0 text-sm font-medium text-foreground">
					Git Integration
				</span>
				<span className="shrink-0 rounded-full border border-border bg-background px-2 py-0.5 font-mono text-[10px] font-bold text-foreground">
					{provider}
				</span>
				<span className="min-w-0 truncate font-mono text-[11px] text-muted-foreground">
					{repoSummary}
					{branch ? ` · ${branch}` : ''}
				</span>
			</CollapsibleTrigger>

			<CollapsibleContent>
				<div className="grid gap-x-8 gap-y-4 px-5 pt-1 pb-5 sm:grid-cols-2 lg:grid-cols-4">
					<Field
						label="Repository"
						value={
							provider === 'GIT'
								? repoUrl
								: repoSummary || repo
						}
					/>
					<Field label="Branch" value={branch} />
					<Field label="Build path" value={buildPath} />

					{provider === 'GIT' ? (
						<div className="min-w-0">
							<p className="text-[10px] font-bold tracking-wider text-muted-foreground uppercase">
								Deploy key
							</p>
							<div className="mt-1 flex items-center gap-1.5">
								<Select
									value={
										currentKeyId
											? String(currentKeyId)
											: 'none'
									}
									onValueChange={v => {
										if (v) changeCredential(v);
									}}>
									<SelectTrigger
										disabled={saving}
										className="h-8 min-w-0 flex-1 cursor-pointer rounded-lg bg-card text-xs">
										<span className="truncate">
											{currentKey?.name ??
												'No key attached'}
										</span>
									</SelectTrigger>
									<SelectContent className="bg-card">
										<SelectItem value="none">
											No key attached
										</SelectItem>
										{keys.map(k => (
											<SelectItem
												key={k.id}
												value={String(k.id)}>
												{k.name}
											</SelectItem>
										))}
									</SelectContent>
								</Select>
								<Link
									to={'/ssh-keys' as any}
									title="Add a new credential"
									className="flex size-8 shrink-0 items-center justify-center rounded-lg border border-border bg-card text-muted-foreground transition-colors hover:bg-accent hover:text-foreground">
									<KeyRound className="size-3.5" />
								</Link>
							</div>
						</div>
					) : (
						<Field
							label="Auth"
							value={`${provider.toLowerCase()} app / OAuth provider`}
						/>
					)}
				</div>
			</CollapsibleContent>
		</Collapsible>
	);
}
