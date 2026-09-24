import {Fragment, useMemo, useState} from 'react';
import {
	ChevronDown,
	Rocket,
	Boxes,
	Database,
	AlertCircle,
} from 'lucide-react';
import type {Deployment} from '#/hooks/deployments/use-deployments';
import {usePermissionGroupMembers} from '#/api/live';
import {getAvatarInitials, isSolidColorAvatar} from '#/lib/avatar-utils';
import {timeAgo} from '#/components/activity/activity-feed';
import {cn} from '#/api/utils';
import {
	Table,
	TableHeader,
	TableBody,
	TableHead,
	TableRow,
	TableCell,
} from '#/components/ui/table';
import {isBuildActive} from '#/components/projects/detail/deployments/build-status';
import {DeploymentInlineLogs} from '#/components/projects/detail/deployments/deployment-inline-logs';
import {useAutoExpandBuildLogs} from '#/components/projects/detail/deployments/use-auto-expand-build-logs';
import {
	deploymentType,
	formatDeploymentDuration,
} from './deployment-utils';
import {DeploymentStatusBadge} from './deployment-status-badge';

interface DeploymentActivityListProps {
	deployments: Deployment[];
}

const KIND_ICON = {
	Application: Rocket,
	Compose: Boxes,
	Database: Database,
	Generic: Rocket,
} as const;

const HEAD_CLS =
	'px-4 py-1.5 text-[11px] font-semibold tracking-wider text-muted-foreground uppercase';

/** Who pushed the button — the deployer's avatar as a round tile (the sidebar/member-row grammar): image, solid color, initials, and the kind icon only when the deployer is unknown (System). A build in flight gets an amber ring. */
function DeployerTile({
	avatar,
	name,
	icon: Icon,
	busy,
}: {
	avatar: string;
	name: string;
	icon: typeof Rocket;
	busy: boolean;
}) {
	return (
		<span
			className={cn(
				'flex size-6 shrink-0 items-center justify-center overflow-hidden rounded-full bg-foreground/[0.06] text-[9px] font-semibold text-foreground uppercase',
				busy && 'ring-1 ring-amber-500/50',
			)}>
			{isSolidColorAvatar(avatar) ? (
				<span
					className="size-full ring-1 ring-border/50 ring-inset"
					style={{backgroundColor: avatar}}
				/>
			) : avatar ? (
				<img
					src={avatar}
					alt={name}
					className="size-full bg-muted object-cover"
				/>
			) : name ? (
				getAvatarInitials(name)
			) : (
				<Icon className="size-3.5 text-muted-foreground" />
			)}
		</span>
	);
}

/** Global deployments table in the house table grammar (Overview services / Traefik requests): sticky uppercase column heads, one bordered card, rows = deployer avatar tile + title/meta, status badge, mono duration + time. Clicking a row opens the streaming log panel inline under it. */
export function DeploymentActivityList({
	deployments,
}: DeploymentActivityListProps) {
	const [expandedId, setExpandedId] = useState<number | null>(null);
	useAutoExpandBuildLogs(deployments as any[], setExpandedId);

	// user_id → org member, so each row can show the deployer's avatar.
	const {data: membersData} = usePermissionGroupMembers();
	const usersById = useMemo(() => {
		const map = new Map<
			number,
			{email?: string; name?: string; avatar?: string | null}
		>();
		const list = Array.isArray(membersData) ? membersData : [];
		for (const m of list as any[]) {
			map.set(Number(m.user_id), m);
		}
		return map;
	}, [membersData]);

	return (
		<div className="w-full overflow-hidden rounded-xl border bg-card">
			<Table>
				<TableHeader>
					<TableRow className="border-b hover:bg-transparent">
						<TableHead className={HEAD_CLS}>Deployment</TableHead>
						<TableHead className={HEAD_CLS}>Status</TableHead>
						<TableHead className={cn(HEAD_CLS, 'hidden md:table-cell')}>
							Duration
						</TableHead>
						<TableHead className={cn(HEAD_CLS, 'hidden sm:table-cell')}>
							When
						</TableHead>
						<TableHead className={cn(HEAD_CLS, 'w-10')} />
					</TableRow>
				</TableHeader>
				<TableBody>
					{deployments.map(d => {
						const kind = deploymentType(d);
						const Icon =
							KIND_ICON[kind as keyof typeof KIND_ICON] ?? Rocket;
						const member = usersById.get(Number((d as any).user_id ?? 0));
						const email =
							member?.email || String((d as any).user_email || '');
						const displayName =
							member?.name?.trim() || (email ? email.split('@')[0] : '');
						const avatar = member?.avatar || '';
						const createdAtSec = Number(d.created_at ?? 0);
						const finishedSec = Number((d as any).finished_at ?? 0);
						const busy = isBuildActive(d as any);
						const id = Number(d.id);
						const isOpen = expandedId === id;
						return (
							<Fragment key={String(d.id)}>
								<TableRow
									onClick={() =>
										setExpandedId(prev => (prev === id ? null : id))
									}
									className="group cursor-pointer border-b border-border transition-colors hover:bg-accent/30">
									<TableCell className="max-w-0 px-4 py-1.5">
										<div className="flex items-center gap-3">
											<DeployerTile
												avatar={avatar}
												name={displayName}
												icon={Icon}
												busy={busy}
											/>
											<div className="min-w-0">
												<span className="block truncate text-[13px] leading-tight font-medium text-foreground">
													{d.title || `Deployment #${d.id}`}
												</span>
												<p className="truncate text-xs text-muted-foreground">
													{kind}
													<span className="mx-1.5 text-muted-foreground/40">
														·
													</span>
													{displayName || 'System'}
													<span className="mx-1.5 text-muted-foreground/40">
														·
													</span>
													<span className="font-mono">#{d.id}</span>
												</p>
												{d.error_message && (
													<p className="mt-0.5 flex max-w-md items-center gap-1.5 truncate text-xs font-medium text-danger">
														<AlertCircle className="size-3 shrink-0" />
														<span className="truncate">
															{d.error_message}
														</span>
													</p>
												)}
											</div>
										</div>
									</TableCell>
									<TableCell className="px-4 py-1.5 whitespace-nowrap">
										<DeploymentStatusBadge
											status={String(d.status || '')}
										/>
									</TableCell>
									<TableCell className="hidden px-4 py-1.5 font-mono text-xs whitespace-nowrap text-muted-foreground tabular-nums md:table-cell">
										{finishedSec > 0
											? formatDeploymentDuration(createdAtSec, finishedSec)
											: busy
												? 'running…'
												: '—'}
									</TableCell>
									<TableCell className="hidden px-4 py-1.5 font-mono text-xs whitespace-nowrap text-muted-foreground tabular-nums sm:table-cell">
										{timeAgo(createdAtSec)}
									</TableCell>
									<TableCell className="px-2 py-1.5">
										<ChevronDown
											className={cn(
												'size-3.5 text-muted-foreground/60 transition-transform duration-200 group-hover:text-muted-foreground',
												isOpen && 'rotate-180',
											)}
										/>
									</TableCell>
								</TableRow>
								{isOpen && (
									<tr>
										<td colSpan={5} className="p-0">
											<DeploymentInlineLogs deployment={d} />
										</td>
									</tr>
								)}
							</Fragment>
						);
					})}
				</TableBody>
			</Table>
		</div>
	);
}
