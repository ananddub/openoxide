import { createFileRoute } from "@tanstack/react-router";
import { Plus, ShieldAlert, ShieldCheck } from "lucide-react";
import { useState } from "react";
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
} from "#/components/settings/certificates/certificate-api";
import type {
  Certificate,
  RemoteServer,
} from "#/components/settings/certificates/certificate-types";
import { useCertificateItems } from "#/api/live/certificates";
import { useRemoteServerList } from "#/api/live/servers";
import { useConnectMutation } from "#/api/live/query";

export const Route = createFileRoute("/_app/settings/certificates")({
  component: CertificatesPage,
});

function CertificatesPage() {
  const itemsQuery = useCertificateItems();
  const serversQuery = useRemoteServerList();
  const items = (itemsQuery.data ?? []) as unknown as Certificate[];
  const servers = (serversQuery.data ?? []) as unknown as RemoteServer[];
  const [dialogOpen, setDialogOpen] = useState(false);
  const [historyOpen, setHistoryOpen] = useState(false);
  const [selected, setSelected] = useState<Certificate | null>(null);
  const [deleting, setDeleting] = useState<Certificate | null>(null);

  // Cache invalidation comes from the table-watch stream (certificates
  // table → refetch while this page is mounted); the mutation only needs
  // error handling here.
  const removeMutation = useConnectMutation(
    async (certificate: Certificate) => {
      await deleteCertificate(Number(certificate.id));
      return certificate;
    },
    {
      onSuccess: (certificate) => {
        toast.success(`Certificate ${certificate.name} deleted`);
      },
      onError: (error: unknown) => {
        toast.error(
          error instanceof Error
            ? error.message
            : "Certificate could not be deleted",
        );
      },
    },
  );

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
        loading={itemsQuery.isLoading}
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
        onSaved={async () => {
          /* table-watch invalidates ['certificates'] → refetch */
        }}
      />
      <CertificateHistoryDialog
        certificate={selected}
        open={historyOpen}
        onOpenChange={setHistoryOpen}
        onRenewed={async () => {
          /* table-watch invalidates ['certificates'] → refetch */
        }}
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
                if (deleting) await removeMutation.mutateAsync(deleting);
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
