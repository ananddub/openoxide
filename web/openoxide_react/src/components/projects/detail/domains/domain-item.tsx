import {useState} from 'react';
import type {ReactNode} from 'react';
import {
	Check,
	Copy,
	Edit2,
	Eye,
	Globe,
	MoreHorizontal,
	RefreshCw,
	Trash2,
} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {ListRow, RowTile} from '#/components/settings/list-card';
import {
	DropdownMenu,
	DropdownMenuContent,
	DropdownMenuItem,
	DropdownMenuSeparator,
	DropdownMenuTrigger,
} from '#/components/ui/dropdown';

export const buildDomainUrl = (domain: any): string =>
	`${domain.https ? 'https' : 'http'}://${domain.host || domain.domain || ''}${domain.path && domain.path !== '/' ? domain.path : ''}`;

export const certificateLabel = (domain: any): string => {
	const certificate = String(
		domain.certificate_type || 'NONE',
	).toUpperCase();
	return certificate === 'LETSENCRYPT'
		? "Let's Encrypt"
		: certificate === 'CUSTOM'
			? 'Custom resolver'
			: 'Manual/default TLS';
};

/** Route summary: `/app → / · port 3000`. */
export const domainRoute = (domain: any): string =>
	`${domain.path || '/'} → ${domain.internal_path || '/'} · port ${domain.port || domain.container_port || 80}`;

export function Chip({
	children,
	mono,
}: {
	children: ReactNode;
	mono?: boolean;
}) {
	return (
		<span
			className={`shrink-0 rounded-md bg-foreground/[0.05] px-1.5 py-0.5 text-[10px] font-medium text-muted-foreground ${
				mono ? 'font-mono' : ''
			}`}>
			{children}
		</span>
	);
}

export function TlsChip({https}: {https: boolean}) {
	return (
		<Chip>
			<span
				className={https ? 'text-emerald-500' : 'text-muted-foreground'}>
				{https ? 'HTTPS' : 'HTTP'}
			</span>
		</Chip>
	);
}

export function DomainChips({domain}: {domain: any}) {
	return (
		<>
			{domain.service_name && <Chip>{domain.service_name}</Chip>}
			<TlsChip https={!!domain.https} />
			{domain.https && <Chip>{certificateLabel(domain)}</Chip>}
		</>
	);
}

interface DomainItemProps {
	domain: any;
	deleting: boolean;
	onPreview: () => void;
	onEdit: () => void;
	onDelete: () => void;
}

/** List row: host link, route summary, TLS/certificate chips. */
export function DomainRow({
	domain,
	deleting,
	onPreview,
	onEdit,
	onDelete,
}: DomainItemProps) {
	const url = buildDomainUrl(domain);
	return (
		<ListRow
			icon={
				<RowTile>
					<Globe className="size-4" />
				</RowTile>
			}
			title={
				<a
					href={url}
					target="_blank"
					rel="noreferrer"
					className="font-mono hover:underline">
					{domain.host || domain.domain}
				</a>
			}
			subtitle={<span className="font-mono">{domainRoute(domain)}</span>}
			badges={<DomainChips domain={domain} />}>
			<Button
				variant="ghost"
				size="icon"
				title="Preview in browser"
				onClick={onPreview}
				className="size-8 text-muted-foreground hover:text-foreground">
				<Eye className="size-4" />
			</Button>
			<Button
				variant="ghost"
				size="icon"
				title="Edit"
				onClick={onEdit}
				className="size-8 text-muted-foreground hover:text-foreground">
				<Edit2 className="size-4" />
			</Button>
			<Button
				variant="ghost"
				size="icon"
				title="Delete"
				disabled={deleting}
				onClick={onDelete}
				className="size-8 text-muted-foreground hover:bg-destructive/10 hover:text-destructive">
				{deleting ? (
					<RefreshCw className="size-4 animate-spin" />
				) : (
					<Trash2 className="size-4" />
				)}
			</Button>
		</ListRow>
	);
}

/** Grid card: the row's counterpart — flat hairline card + kebab actions. */
export function DomainCard({
	domain,
	deleting,
	onPreview,
	onEdit,
	onDelete,
}: DomainItemProps) {
	const url = buildDomainUrl(domain);
	const [copied, setCopied] = useState(false);

	const copyUrl = () => {
		navigator.clipboard.writeText(url);
		setCopied(true);
		setTimeout(() => setCopied(false), 2000);
	};

	return (
		<div className="group flex min-h-[124px] flex-col justify-between gap-3.5 rounded-lg border bg-card p-4 transition-colors hover:bg-accent/60">
			<div className="flex items-start justify-between gap-2.5">
				<div className="flex min-w-0 items-center gap-3">
					<div className="flex size-8 shrink-0 items-center justify-center rounded-md border">
						<Globe className="size-4 text-muted-foreground" />
					</div>
					<div className="flex min-w-0 flex-col">
						<a
							href={url}
							target="_blank"
							rel="noreferrer"
							className="truncate font-mono text-sm font-semibold text-foreground hover:underline">
							{domain.host || domain.domain}
						</a>
						<span className="mt-0.5 truncate font-mono text-xs text-muted-foreground">
							{domainRoute(domain)}
						</span>
					</div>
				</div>

				<DropdownMenu>
					<DropdownMenuTrigger
						render={
							<Button
								variant="ghost"
								size="icon"
								aria-label={`Domain ${domain.host || domain.domain} actions`}
								className="size-7 rounded-md text-muted-foreground hover:bg-muted/80 hover:text-foreground focus-visible:ring-0">
								<MoreHorizontal className="size-4" />
							</Button>
						}
					/>
					<DropdownMenuContent align="end" className="w-44">
						<DropdownMenuItem
							className="cursor-pointer gap-2"
							onClick={onPreview}>
							<Eye className="size-3.5" /> Preview
						</DropdownMenuItem>
						<DropdownMenuItem
							className="cursor-pointer gap-2"
							onClick={copyUrl}>
							{copied ? (
								<Check className="size-3.5 text-emerald-500" />
							) : (
								<Copy className="size-3.5" />
							)}
							{copied ? 'Copied!' : 'Copy URL'}
						</DropdownMenuItem>
						<DropdownMenuItem
							className="cursor-pointer gap-2"
							onClick={onEdit}>
							<Edit2 className="size-3.5" /> Edit route
						</DropdownMenuItem>
						<DropdownMenuSeparator />
						<DropdownMenuItem
							className="cursor-pointer gap-2 text-destructive focus:text-destructive"
							onClick={onDelete}>
							{deleting ? (
								<RefreshCw className="size-3.5 animate-spin" />
							) : (
								<Trash2 className="size-3.5" />
							)}
							Delete domain
						</DropdownMenuItem>
					</DropdownMenuContent>
				</DropdownMenu>
			</div>

			<div className="flex flex-wrap items-center gap-1.5">
				<DomainChips domain={domain} />
			</div>
		</div>
	);
}
