import { useEffect, useState } from "react";
import { Clock3, Loader2, RefreshCw } from "lucide-react";
import { toast } from "sonner";
import { Button } from "#/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from "#/components/ui/dialog";
import { Textarea } from "#/components/ui/textarea";
import { listCertificateRenewals, renewCertificate } from "./certificate-api";
import type { Certificate, CertificateRenewal } from "./certificate-types";

type Props = {
  certificate: Certificate | null;
  open: boolean;
  onOpenChange: (open: boolean) => void;
  onRenewed: () => Promise<void>;
};

const date = (timestamp?: number) =>
  timestamp ? new Date(timestamp * 1000).toLocaleString() : "Not available";

export function CertificateHistoryDialog({
  certificate,
  open,
  onOpenChange,
  onRenewed,
}: Props) {
  const [history, setHistory] = useState<CertificateRenewal[]>([]);
  const [certificateData, setCertificateData] = useState("");
  const [privateKey, setPrivateKey] = useState("");
  const [loading, setLoading] = useState(false);
  const [renewing, setRenewing] = useState(false);

  useEffect(() => {
    if (!open || !certificate) return;
    setCertificateData("");
    setPrivateKey("");
    setLoading(true);
    listCertificateRenewals(Number(certificate.id))
      .then((items) => setHistory(items as CertificateRenewal[]))
      .catch((error) => toast.error(error.message))
      .finally(() => setLoading(false));
  }, [certificate, open]);

  const renew = async () => {
    if (!certificate) return;
    try {
      setRenewing(true);
      await renewCertificate(Number(certificate.id), {
        certificate_data: certificateData,
        private_key: privateKey,
      });
      toast.success("Certificate replaced successfully");
      onOpenChange(false);
      await onRenewed();
    } catch (error) {
      toast.error(error instanceof Error ? error.message : "Renewal failed");
    } finally {
      setRenewing(false);
    }
  };

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-h-[92vh] overflow-y-auto sm:max-w-2xl">
        <DialogHeader>
          <DialogTitle className="flex items-center gap-2">
            <RefreshCw className="size-4" />
            Replace certificate
          </DialogTitle>
          <DialogDescription>
            Install a renewed PEM pair and keep an audit trail of previous
            attempts.
          </DialogDescription>
        </DialogHeader>
        <div className="grid gap-3 sm:grid-cols-2">
          <Textarea
            className="min-h-32 font-mono text-xs"
            value={certificateData}
            onChange={(event) => setCertificateData(event.target.value)}
            placeholder="New certificate chain"
          />
          <Textarea
            className="min-h-32 font-mono text-xs"
            value={privateKey}
            onChange={(event) => setPrivateKey(event.target.value)}
            placeholder="New private key"
          />
        </div>
        <div className="flex justify-end">
          <Button
            onClick={renew}
            disabled={renewing || !certificateData.trim() || !privateKey.trim()}
          >
            {renewing ? <Loader2 className="animate-spin" /> : <RefreshCw />}
            Replace
          </Button>
        </div>
        <div className="border-t pt-4">
          <p className="mb-2 flex items-center gap-1.5 text-xs font-semibold">
            <Clock3 className="size-3.5" />
            Renewal history
          </p>
          {loading ? (
            <Loader2 className="mx-auto my-6 size-5 animate-spin" />
          ) : history.length === 0 ? (
            <p className="py-5 text-center text-xs text-muted-foreground">
              No renewal attempts yet.
            </p>
          ) : (
            <div className="space-y-2">
              {history.map((item) => (
                <div
                  key={item.id}
                  className="rounded-md border px-3 py-2 text-xs"
                >
                  <div className="flex justify-between">
                    <span
                      className={
                        item.status === "SUCCEEDED"
                          ? "font-semibold text-emerald-500"
                          : item.status === "FAILED"
                            ? "font-semibold text-destructive"
                            : "font-semibold text-amber-500"
                      }
                    >
                      {item.status}
                    </span>
                    <span className="text-muted-foreground">
                      {date(item.started_at)}
                    </span>
                  </div>
                  {item.new_expires_at && (
                    <p className="mt-1 text-muted-foreground">
                      Expires: {date(item.new_expires_at)}
                    </p>
                  )}
                  {item.error && (
                    <p className="mt-1 text-destructive">{item.error}</p>
                  )}
                </div>
              ))}
            </div>
          )}
        </div>
      </DialogContent>
    </Dialog>
  );
}
