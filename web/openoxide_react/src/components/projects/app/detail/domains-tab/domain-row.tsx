import {
	Edit2,
	Eye,
	ExternalLink,
	Globe,
	Lock,
	LockOpen,
	Trash2,
	RefreshCw,
} from 'lucide-react';
import {Badge} from '#/components/ui/badge';
import {Button} from '#/components/ui/button';
import {TableCell, TableRow} from '#/components/ui/table';

export const buildDomainUrl = (domain: any): string =>
	`${domain.https ? 'https' : 'http'}://${domain.host}${domain.path && domain.path !== '/' ? domain.path : ''}`;

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

/** Presentational row for the domains table. */
export function DomainRow({
	domain,
	deleting,
	onPreview,
	onEdit,
	onDelete,
}: {
	domain: any;
	deleting: boolean;
	onPreview: () => void;
	onEdit: () => void;
	onDelete: () => void;
}) {
	const url = buildDomainUrl(domain);
	return (
		<TableRow>
			<TableCell>
				<a
					href={url}
					target="_blank"
					rel="noreferrer"
					className="flex items-center gap-2 font-mono text-xs font-semibold hover:text-primary">
					<Globe className="size-4 text-primary" />
					{domain.host}
					<ExternalLink className="size-3" />
				</a>
			</TableCell>
			<TableCell className="font-mono text-xs text-muted-foreground">
				{domain.path || '/'} → {domain.internal_path || '/'} ·{' '}
				{domain.port || 80}
			</TableCell>
			<TableCell>
				{domain.https ? (
					<Badge variant="outline" className="text-emerald-500">
						<Lock className="mr-1 size-3" />
						HTTPS
					</Badge>
				) : (
					<Badge variant="secondary">
						<LockOpen className="mr-1 size-3" />
						HTTP
					</Badge>
				)}
			</TableCell>
			<TableCell>
				<Badge variant="outline" className="text-[10px]">
					{certificateLabel(domain)}
				</Badge>
			</TableCell>
			<TableCell className="text-right">
				<Button
					variant="ghost"
					size="icon"
					title="Preview in browser"
					onClick={onPreview}>
					<Eye className="size-4" />
				</Button>
				<Button variant="ghost" size="icon" onClick={onEdit}>
					<Edit2 className="size-4" />
				</Button>
				<Button
					variant="ghost"
					size="icon"
					disabled={deleting}
					onClick={onDelete}
					className="text-muted-foreground hover:text-destructive">
					{deleting ? (
						<RefreshCw className="size-4 animate-spin" />
					) : (
						<Trash2 className="size-4" />
					)}
				</Button>
			</TableCell>
		</TableRow>
	);
}
