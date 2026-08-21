import { KeyRound } from "lucide-react";
import { Input } from "#/components/ui/input";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "#/components/ui/select";
import { Switch } from "#/components/ui/switch";
import { Textarea } from "#/components/ui/textarea";
import type { CertificateForm, RemoteServer } from "./certificate-types";

type Props = {
  editing: boolean;
  form: CertificateForm;
  servers: RemoteServer[];
  onChange: <K extends keyof CertificateForm>(
    key: K,
    value: CertificateForm[K],
  ) => void;
};

const slug = (value: string) =>
  value
    .toLowerCase()
    .trim()
    .replace(/[^a-z0-9]+/g, "-")
    .replace(/^-|-$/g, "");

export function CertificateFormFields({
  editing,
  form,
  servers,
  onChange,
}: Props) {
  return (
    <div className="grid gap-4">
      <div className="grid gap-3 sm:grid-cols-2">
        <div className="space-y-1.5">
          <label className="text-xs font-medium">Name</label>
          <Input
            value={form.name}
            onChange={(event) => {
              onChange("name", event.target.value);
              if (!editing)
                onChange("certificate_path", slug(event.target.value));
            }}
            placeholder="Production wildcard"
          />
        </div>
        <div className="space-y-1.5">
          <label className="text-xs font-medium">Directory</label>
          <Input
            value={form.certificate_path}
            onChange={(event) =>
              onChange("certificate_path", slug(event.target.value))
            }
            placeholder="production-wildcard"
            disabled={editing}
          />
        </div>
      </div>

      <div className="space-y-1.5">
        <label className="text-xs font-medium">Target server</label>
        <Select
          value={form.server_id}
          onValueChange={(value) => value && onChange("server_id", value)}
          disabled={editing}
        >
          <SelectTrigger className="w-full text-xs">
            <SelectValue placeholder="Select target server" />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="local">Openoxide (local)</SelectItem>
            {servers
              .filter((server) => server.server_type !== "BUILD")
              .map((server) => (
                <SelectItem key={server.id} value={String(server.id)}>
                  {server.name} ({server.ip_address})
                </SelectItem>
              ))}
          </SelectContent>
        </Select>
      </div>

      <div className="space-y-1.5">
        <label className="text-xs font-medium">Certificate chain</label>
        <Textarea
          className="min-h-40 resize-y font-mono text-xs"
          value={form.certificate_data}
          onChange={(event) => onChange("certificate_data", event.target.value)}
          placeholder="-----BEGIN CERTIFICATE-----"
        />
      </div>

      <div className="space-y-1.5">
        <label className="flex items-center gap-1.5 text-xs font-medium">
          <KeyRound className="size-3.5" />
          Private key
        </label>
        <Textarea
          className="min-h-40 resize-y font-mono text-xs"
          value={form.private_key}
          onChange={(event) => onChange("private_key", event.target.value)}
          placeholder={
            editing
              ? "Leave blank to keep the current private key"
              : "-----BEGIN PRIVATE KEY-----"
          }
        />
      </div>

      <div className="flex items-center justify-between rounded-md border px-3 py-2.5">
        <div>
          <p className="text-xs font-medium">Track manual renewals</p>
          <p className="text-[11px] text-muted-foreground">
            Store replacement history for this certificate.
          </p>
        </div>
        <Switch
          checked={form.auto_renew}
          onCheckedChange={(checked) => onChange("auto_renew", checked)}
        />
      </div>
    </div>
  );
}
