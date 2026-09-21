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
	ListEmpty,
	RowTile,
} from '#/components/settings/list-card';
import type {Certificate, RemoteServer} from './certificate-types';

type Props = {
	items: Certificate[];
	servers: RemoteServer[];
	loading: boolean;
	onCreate: () => void;
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

/** OpenShip's list-card rows: shield tile, name + chain/server/key facts, kebab. */
export function CertificateList({
	items,
	servers,
	loading,
	onCreate,
	onEdit,
	onHistory,
	onDelete,
}: Props) {
	return (
		<ListCard
			title={`${items.length} ${items.length === 1 ? 'certificate' : 'certificates'}`}
			description="Installed on local or remote Traefik servers.">
			{loading ? (
				<div className="space-y-2 p-5">
					{[...Array(2)].map((_, i) => (
						<div
							key={i}
							className="h-14 animate-pulse rounded-xl bg-muted/30"
						/>
					))}
				</div>
			) : items.length === 0 ? (
				<ListEmpty
					icon={ShieldCheck}
					title="No custom certificates"
					description="Install a PEM certificate chain for Traefik to serve HTTPS with your own cert."
					action={
						<Button size="sm" onClick={onCreate} className="gap-1.5">
							<Pencil className="size-3.5" /> Install certificate
						</Button>
					}
				/>
			) : (
				<div className="divide-y divide-border/40">
					{items.map(item => {
						const server = servers.find(
							value => String(value.id) === item.server_id,
						);
						const info = certificateInfo(item.certificate_data);
						return (
							<ListRow
								key={item.id}
								icon={
									<RowTile className="bg-emerald-500/10 text-emerald-500">
										<ShieldCheck className="size-4" />
									</RowTile>
								}
								title={item.name}
								badges={
									<>
										<span className="rounded-md bg-muted/50 px-1.5 py-0.5 text-[10px] font-medium text-muted-foreground">
											{info.count > 1 ? `${info.count} cert chain` : 'PEM'}
										</span>
										{item.auto_renew === 1 && (
											<span className="flex items-center gap-1 text-[10px] font-medium text-emerald-500">
												<RefreshCw className="size-3" />
												Renewal tracking
											</span>
										)}
									</>
								}
								subtitle={
									<span className="flex flex-wrap gap-x-3 gap-y-1 text-[11px]">
										<span className="flex items-center gap-1">
											<HardDrive className="size-3" />
											{server
												? `${server.name} (${server.ip_address})`
												: 'Openoxide local'}
										</span>
										<span className="flex items-center gap-1">
											<KeyRound className="size-3" />
											{item.has_private_key
												? 'Private key stored'
												: 'Missing key'}
										</span>
										<span className="font-mono">
											{item.certificate_path}
										</span>
									</span>
								}>
								<DropdownMenu>
									<DropdownMenuTrigger
										render={
											<Button
												variant="ghost"
												size="icon"
												className="size-7 rounded-lg text-muted-foreground hover:text-foreground"
											/>
										}>
										<MoreHorizontal className="size-4" />
									</DropdownMenuTrigger>
									<DropdownMenuContent align="end" className="w-48">
										<DropdownMenuItem
											onClick={() => onHistory(item)}
											className="gap-2 text-xs">
											<Clock3 className="size-3.5" /> Renewal history
										</DropdownMenuItem>
										<DropdownMenuItem
											onClick={() => onEdit(item)}
											className="gap-2 text-xs">
											<Pencil className="size-3.5" /> Edit certificate
										</DropdownMenuItem>
										<DropdownMenuSeparator />
										<DropdownMenuItem
											onClick={() => onDelete(item)}
											variant="destructive"
											className="gap-2 text-xs">
											<Trash2 className="size-3.5" /> Delete certificate
										</DropdownMenuItem>
									</DropdownMenuContent>
								</DropdownMenu>
							</ListRow>
						);
					})}
				</div>
			)}
		</ListCard>
	);
}
