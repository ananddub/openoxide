import {useEffect, useState} from 'react';
import {Clock3, Loader2} from 'lucide-react';
import {toast} from 'sonner';
import {Button} from '#/components/ui/button';
import {Textarea} from '#/components/ui/textarea';
import {
	Sheet,
	SheetContent,
	SheetDescription,
	SheetFooter,
	SheetHeader,
	SheetTitle,
} from '#/components/ui/sheet';
import {ListCard} from '#/components/settings/list-card';
import {WizardField} from '#/components/shared/sheet-wizard';
import {
	listCertificateRenewals,
	renewCertificate,
} from './certificate-api';
import type {Certificate, CertificateRenewal} from './certificate-types';

type Props = {
	certificate: Certificate | null;
	open: boolean;
	onOpenChange: (open: boolean) => void;
	onRenewed: () => Promise<void>;
};

const date = (timestamp?: number | bigint) => {
	// Proto int64 fields arrive as bigint at runtime — coerce first.
	const seconds = Number(timestamp ?? 0);
	return seconds
		? new Date(seconds * 1000).toLocaleString()
		: 'Not available';
};

const statusTone = (status: string) =>
	status === 'SUCCEEDED'
		? 'text-emerald-500'
		: status === 'FAILED'
			? 'text-destructive'
			: 'text-amber-500';

/** Replace a certificate with a renewed PEM pair + its audit trail. */
export function CertificateRenewSheet({
	certificate,
	open,
	onOpenChange,
	onRenewed,
}: Props) {
	const [history, setHistory] = useState<CertificateRenewal[]>([]);
	const [certificateData, setCertificateData] = useState('');
	const [privateKey, setPrivateKey] = useState('');
	const [loading, setLoading] = useState(false);
	const [renewing, setRenewing] = useState(false);

	useEffect(() => {
		if (!open || !certificate) return;
		setCertificateData('');
		setPrivateKey('');
		setLoading(true);
		listCertificateRenewals(Number(certificate.id))
			.then(items => setHistory(items as CertificateRenewal[]))
			.catch(error => toast.error(error.message))
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
			toast.success('Certificate replaced successfully');
			onOpenChange(false);
			await onRenewed();
		} catch (error) {
			toast.error(
				error instanceof Error ? error.message : 'Renewal failed',
			);
		} finally {
			setRenewing(false);
		}
	};

	return (
		<Sheet open={open} onOpenChange={onOpenChange}>
			<SheetContent className="p-0">
				<SheetHeader>
					<SheetTitle>Replace certificate</SheetTitle>
					<SheetDescription>
						Install a renewed PEM pair and keep an audit trail of previous
						attempts.
					</SheetDescription>
				</SheetHeader>

				<div className="flex min-h-0 flex-1 flex-col gap-5 overflow-y-auto px-6 py-6">
					<WizardField label="New certificate chain" required>
						<Textarea
							className="min-h-32 resize-y rounded-lg bg-background font-mono text-xs"
							value={certificateData}
							onChange={event => setCertificateData(event.target.value)}
							placeholder="-----BEGIN CERTIFICATE-----"
						/>
					</WizardField>

					<WizardField label="New private key" required>
						<Textarea
							className="min-h-32 resize-y rounded-lg bg-background font-mono text-xs"
							value={privateKey}
							onChange={event => setPrivateKey(event.target.value)}
							placeholder="-----BEGIN PRIVATE KEY-----"
						/>
					</WizardField>

					<div className="flex flex-col gap-2">
						<p className="flex items-center gap-1.5 text-[13px] font-medium text-foreground">
							<Clock3 className="size-3.5 text-muted-foreground" />
							Renewal history
						</p>
						{loading ? (
							<div className="flex items-center gap-2 rounded-xl border bg-card px-4 py-3 text-[13px] text-muted-foreground">
								<Loader2 className="size-3.5 animate-spin" />
								Loading attempts…
							</div>
						) : history.length === 0 ? (
							<p className="rounded-xl border bg-card px-4 py-6 text-center text-[13px] text-muted-foreground">
								No renewal attempts yet.
							</p>
						) : (
							<ListCard>
								{history.map(item => (
									<div
										key={String(item.id)}
										className="flex items-start gap-3.5 px-4 py-3">
										<div className="min-w-0 flex-1">
											<p
												className={`text-sm font-medium ${statusTone(item.status)}`}>
												{item.status}
											</p>
											<p className="mt-0.5 truncate text-[13px] text-muted-foreground">
												{date(item.started_at)}
											</p>
											{item.new_expires_at && (
												<p className="mt-0.5 truncate text-[13px] text-muted-foreground">
													Expires {date(item.new_expires_at)}
												</p>
											)}
											{item.error && (
												<p className="mt-0.5 break-all text-[13px] text-destructive">
													{item.error}
												</p>
											)}
										</div>
									</div>
								))}
							</ListCard>
						)}
					</div>
				</div>

				<SheetFooter>
					<Button
						onClick={() => void renew()}
						disabled={
							renewing || !certificateData.trim() || !privateKey.trim()
						}
						className="h-10 gap-1.5 px-5">
						{renewing && <Loader2 className="size-3.5 animate-spin" />}
						Replace certificate
					</Button>
				</SheetFooter>
			</SheetContent>
		</Sheet>
	);
}
