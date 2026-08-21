import { createFileRoute } from "@tanstack/react-router";
import { Plus, ShieldAlert, ShieldCheck } from "lucide-react";
import { useCallback, useEffect, useState } from "react";
import { toast } from "sonner";
import { Button } from "#/components/ui/button";
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
} from "#/components/ui/alert-dialog";
import { CertificateDialog } from "#/components/settings/certificates/certificate-dialog";
import { CertificateHistoryDialog } from "#/components/settings/certificates/certificate-history-dialog";
import { CertificateList } from "#/components/settings/certificates/certificate-list";
import {
  deleteCertificate,
  listRemoteServers,
} from "#/components/settings/certificates/certificate-api";
import type {
  Certificate,
  RemoteServer,
} from "#/components/settings/certificates/certificate-types";
import { useAppStore } from "#/stores/app-store";

export const Route = createFileRoute("/_app/settings/certificates")({
  component: CertificatesPage,
});

function CertificatesPage() {
  const items = useAppStore((state) => state.certificates) as Certificate[];
  const wsConnected = useAppStore((state) => state.isWsConnected);
  const [servers, setServers] = useState<RemoteServer[]>([]);
  const [dialogOpen, setDialogOpen] = useState(false);
  const [historyOpen, setHistoryOpen] = useState(false);
  const [selected, setSelected] = useState<Certificate | null>(null);
  const [deleting, setDeleting] = useState<Certificate | null>(null);

  const load = useCallback(async () => {
    try {
      const remoteServers = await listRemoteServers();
      setServers(remoteServers || []);
    } catch (error) {
      toast.error(
        error instanceof Error ? error.message : "Could not load certificates",
      );
    }
  }, []);

  useEffect(() => {
    void load();
  }, [load]);

  const create = () => {
    setSelected(null);
    setDialogOpen(true);
  };

  const edit = (certificate: Certificate) => {
    setSelected(certificate);
    setDialogOpen(true);
  };

  const history = (certificate: Certificate) => {
    setSelected(certificate);
    setHistoryOpen(true);
  };

  const remove = async (certificate: Certificate) => {
    try {
      await deleteCertificate(Number(certificate.id));
      toast.success("Certificate deleted");
      await load();
    } catch (error) {
      toast.error(
        error instanceof Error
          ? error.message
          : "Certificate could not be deleted",
      );
    }
  };

  return (
    <div className="mx-auto flex w-full max-w-5xl animate-in flex-col gap-6 p-6 duration-200 fade-in">
      <div className="flex flex-col justify-between gap-4 sm:flex-row sm:items-center">
        <div>
          <h1 className="flex items-center gap-2 text-2xl font-bold tracking-tight">
            <ShieldCheck className="size-6 text-emerald-500" />
            SSL / TLS Certificates
          </h1>
          <p className="mt-1 text-xs text-muted-foreground">
            Install custom PEM certificates on local or remote Traefik servers.
          </p>
        </div>
        <Button onClick={create} className="gap-1.5">
          <Plus className="size-4" />
          Install certificate
        </Button>
      </div>

      <div className="flex gap-3 rounded-lg border border-amber-500/20 bg-amber-500/5 px-4 py-3">
        <ShieldAlert className="mt-0.5 size-4 shrink-0 text-amber-500" />
        <p className="text-xs leading-5 text-muted-foreground">
          Openoxide validates that the certificate and private key match before
          writing them to Traefik. Invalid TLS files can interrupt HTTPS
          routing.
        </p>
      </div>

      <CertificateList
        items={items}
        servers={servers}
        loading={!wsConnected && items.length === 0}
        onCreate={create}
        onEdit={edit}
        onHistory={history}
        onDelete={setDeleting}
      />

      <CertificateDialog
        certificate={selected}
        servers={servers}
        open={dialogOpen}
        onOpenChange={setDialogOpen}
        onSaved={load}
      />
      <CertificateHistoryDialog
        certificate={selected}
        open={historyOpen}
        onOpenChange={setHistoryOpen}
        onRenewed={load}
      />
      <AlertDialog
        open={deleting !== null}
        onOpenChange={(open) => !open && setDeleting(null)}
      >
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>Delete certificate</AlertDialogTitle>
            <AlertDialogDescription>
              This removes the certificate files from Traefik and cannot be
              undone.
            </AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel onClick={() => setDeleting(null)}>
              Cancel
            </AlertDialogCancel>
            <AlertDialogAction
              className="bg-destructive text-destructive-foreground hover:bg-destructive/90"
              onClick={async () => {
                if (deleting) await remove(deleting);
                setDeleting(null);
              }}
            >
              Delete certificate
            </AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>
    </div>
  );
}
