import {
  Clock3,
  HardDrive,
  KeyRound,
  Loader2,
  MoreHorizontal,
  Pencil,
  RefreshCw,
  ShieldCheck,
  Trash2,
} from "lucide-react";
import { Button } from "#/components/ui/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "#/components/ui/dropdown";
import type { Certificate, RemoteServer } from "./certificate-types";

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
    if (!match) return { count: 0 };
    return {
      count: (pem.match(/-----BEGIN CERTIFICATE-----/g) || []).length,
    };
  } catch {
    return { count: 0 };
  }
}

export function CertificateList({
  items,
  servers,
  loading,
  onCreate,
  onEdit,
  onHistory,
  onDelete,
}: Props) {
  if (loading)
    return (
      <div className="flex items-center justify-center gap-2 py-20 text-xs text-muted-foreground">
        <Loader2 className="size-5 animate-spin" />
        Loading certificates...
      </div>
    );
  if (!items.length)
    return (
      <div className="flex flex-col items-center gap-3 rounded-lg border border-dashed py-16 text-center">
        <ShieldCheck className="size-9 text-muted-foreground" />
        <div>
          <p className="text-sm font-semibold">No custom certificates</p>
          <p className="text-xs text-muted-foreground">
            Install a PEM certificate chain for Traefik.
          </p>
        </div>
        <Button size="sm" onClick={onCreate}>
          Install certificate
        </Button>
      </div>
    );
  return (
    <div className="space-y-2">
      {items.map((item) => {
        const server = servers.find(
          (value) => String(value.id) === item.server_id,
        );
        const info = certificateInfo(item.certificate_data);
        return (
          <div
            key={item.id}
            className="flex flex-col gap-3 rounded-lg border bg-card px-4 py-3 sm:flex-row sm:items-center sm:justify-between"
          >
            <div className="flex min-w-0 items-center gap-3">
              <div className="flex size-9 shrink-0 items-center justify-center rounded-md bg-emerald-500/10 text-emerald-500">
                <ShieldCheck className="size-4.5" />
              </div>
              <div className="min-w-0">
                <div className="flex flex-wrap items-center gap-2">
                  <p className="truncate text-sm font-semibold">{item.name}</p>
                  <span className="rounded bg-muted px-1.5 py-0.5 text-[10px] text-muted-foreground">
                    {info.count > 1 ? `${info.count} cert chain` : "PEM"}
                  </span>
                  {item.auto_renew === 1 && (
                    <span className="flex items-center gap-1 text-[10px] font-medium text-emerald-500">
                      <RefreshCw className="size-3" />
                      Renewal tracking
                    </span>
                  )}
                </div>
                <div className="mt-1 flex flex-wrap gap-x-3 gap-y-1 text-[11px] text-muted-foreground">
                  <span className="flex items-center gap-1">
                    <HardDrive className="size-3" />
                    {server
                      ? `${server.name} (${server.ip_address})`
                      : "Openoxide local"}
                  </span>
                  <span className="flex items-center gap-1">
                    <KeyRound className="size-3" />
                    {item.has_private_key
                      ? "Private key stored"
                      : "Missing key"}
                  </span>
                  <span className="font-mono">{item.certificate_path}</span>
                </div>
              </div>
            </div>
            <DropdownMenu>
              <DropdownMenuTrigger
                render={
                  <Button variant="ghost" size="icon" className="size-8" />
                }
              >
                <MoreHorizontal className="size-4" />
              </DropdownMenuTrigger>
              <DropdownMenuContent align="end" className="w-48">
                <DropdownMenuItem
                  onClick={() => onHistory(item)}
                  className="gap-2 text-xs"
                >
                  <Clock3 className="size-3.5" /> Renewal history
                </DropdownMenuItem>
                <DropdownMenuItem
                  onClick={() => onEdit(item)}
                  className="gap-2 text-xs"
                >
                  <Pencil className="size-3.5" /> Edit certificate
                </DropdownMenuItem>
                <DropdownMenuSeparator />
                <DropdownMenuItem
                  onClick={() => onDelete(item)}
                  variant="destructive"
                  className="gap-2 text-xs"
                >
                  <Trash2 className="size-3.5" /> Delete certificate
                </DropdownMenuItem>
              </DropdownMenuContent>
            </DropdownMenu>
          </div>
        );
      })}
    </div>
  );
}
