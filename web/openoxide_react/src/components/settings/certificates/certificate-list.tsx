import {
	Clock3,
	HardDrive,
	KeyRound,
	MoreHorizontal,
	Pencil,
	RefreshCw,
	ShieldCheck,
	Trash2,
} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {
	DropdownMenu,
	DropdownMenuContent,
	DropdownMenuItem,
	DropdownMenuSeparator,
	DropdownMenuTrigger,
} from '#/components/ui/dropdown';
import {
	ListCard,
	ListRow,
	type ListView,
	RowTile,
	ViewCard,
	ViewGrid,
} from '#/components/settings/list-card';
import type {Certificate, RemoteServer} from './certificate-types';

type Props = {
	items: Certificate[];
	servers: RemoteServer[];
	view: ListView;
	onEdit: (item: Certificate) => void;
	onHistory: (item: Certificate) => void;
	onDelete: (item: Certificate) => void;
};

function certificateInfo(pem: string) {
	try {
		const match = pem.match(
			/-----BEGIN CERTIFICATE-----([\s\S]*?)-----END CERTIFICATE-----/,
		);
		if (!match) return {count: 0};
		return {
			count: (pem.match(/-----BEGIN CERTIFICATE-----/g) || []).length,
		};
	} catch {
		return {count: 0};
	}
}

function CertificateTile() {
	return (
		<RowTile>
			<ShieldCheck className="size-4" />
		</RowTile>
	);
}

function ChainChip({count}: {count: number}) {
	return (
		<span className="shrink-0 rounded-md bg-foreground/[0.05] px-1.5 py-0.5 text-[10px] font-medium text-muted-foreground">
			{count > 1 ? `${count} cert chain` : 'PEM'}
		</span>
	);
}

function RenewChip() {
	return (
		<span className="flex shrink-0 items-center gap-1 rounded-md bg-emerald-500/10 px-1.5 py-0.5 text-[10px] font-medium text-emerald-500">
			<RefreshCw className="size-3" />
			Renewal tracking
		</span>
	);
}

function serverLine(item: Certificate, servers: RemoteServer[]) {
	const server = servers.find(
		value => String(value.id) === item.server_id,
	);
	return server
		? `${server.name} (${server.ip_address})`
		: 'Openoxide local';
}

function Menu({
	item,
	onEdit,
	onHistory,
	onDelete,
}: {
	item: Certificate;
	onEdit: (item: Certificate) => void;
	onHistory: (item: Certificate) => void;
	onDelete: (item: Certificate) => void;
}) {
	return (
		<DropdownMenu>
			<DropdownMenuTrigger
				render={
					<Button
						variant="ghost"
						size="icon"
						aria-label="Certificate actions"
						className="size-7 rounded-lg text-muted-foreground hover:text-foreground focus-visible:ring-0"
					/>
				}>
				<MoreHorizontal className="size-4" />
			</DropdownMenuTrigger>
			<DropdownMenuContent align="end" className="w-48">
				<DropdownMenuItem
					onClick={() => onHistory(item)}
					className="cursor-pointer gap-2 text-xs">
					<Clock3 className="size-3.5" /> Renewal history
				</DropdownMenuItem>
				<DropdownMenuItem
					onClick={() => onEdit(item)}
					className="cursor-pointer gap-2 text-xs">
					<Pencil className="size-3.5" /> Edit certificate
				</DropdownMenuItem>
				<DropdownMenuSeparator />
				<DropdownMenuItem
					onClick={() => onDelete(item)}
					variant="destructive"
					className="cursor-pointer gap-2 text-xs">
					<Trash2 className="size-3.5" /> Delete certificate
				</DropdownMenuItem>
			</DropdownMenuContent>
		</DropdownMenu>
	);
}

/** Certificates — grid cards or one bordered list card, same grammar. */
export function CertificateList({
	items,
	servers,
	view,
	onEdit,
	onHistory,
	onDelete,
}: Props) {
	if (view === 'grid') {
		return (
			<ViewGrid>
				{items.map(item => (
					<ViewCard key={item.id}>
						<div className="flex min-w-0 items-start gap-3">
							<CertificateTile />
							<div className="min-w-0 flex-1">
								<span className="block truncate text-sm font-medium text-foreground">
									{item.name}
								</span>
								<p className="mt-0.5 truncate text-[11px] text-muted-foreground">
									{serverLine(item, servers)}
								</p>
							</div>
						</div>
						<div className="flex items-end justify-between gap-2">
							<div className="flex min-w-0 flex-col gap-1.5">
								<div className="flex flex-wrap items-center gap-1.5">
									<ChainChip
										count={certificateInfo(item.certificate_data).count}
									/>
									{item.auto_renew === 1 && <RenewChip />}
								</div>
								<code className="truncate rounded-md bg-foreground/[0.04] px-2 py-0.5 font-mono text-[11px] text-primary">
									{item.certificate_path}
								</code>
							</div>
							<div className="shrink-0 opacity-0 transition-opacity group-focus-within:opacity-100 group-hover:opacity-100">
								<Menu
									item={item}
									onEdit={onEdit}
									onHistory={onHistory}
									onDelete={onDelete}
								/>
							</div>
						</div>
					</ViewCard>
				))}
			</ViewGrid>
		);
	}

	return (
		<ListCard>
			{items.map(item => {
				const info = certificateInfo(item.certificate_data);
				return (
					<ListRow
						key={item.id}
						icon={<CertificateTile />}
						title={item.name}
						badges={
							<>
								<ChainChip count={info.count} />
								{item.auto_renew === 1 && <RenewChip />}
							</>
						}
						subtitle={
							<span className="flex flex-wrap gap-x-3 gap-y-1 text-[11px]">
								<span className="flex items-center gap-1">
									<HardDrive className="size-3" />
									{serverLine(item, servers)}
								</span>
								<span className="flex items-center gap-1">
									<KeyRound className="size-3" />
									{item.has_private_key
										? 'Private key stored'
										: 'Missing key'}
								</span>
								<span className="font-mono">{item.certificate_path}</span>
							</span>
						}>
						<Menu
							item={item}
							onEdit={onEdit}
							onHistory={onHistory}
							onDelete={onDelete}
						/>
					</ListRow>
				);
			})}
		</ListCard>
	);
}
