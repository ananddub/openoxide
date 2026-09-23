import {useEffect, useState} from 'react';
import {Loader2, ShieldCheck} from 'lucide-react';
import {toast} from 'sonner';
import {Button} from '#/components/ui/button';
import {
	Sheet,
	SheetContent,
	SheetDescription,
	SheetFooter,
	SheetHeader,
	SheetTitle,
} from '#/components/ui/sheet';
import {createCertificate, updateCertificate} from './certificate-api';
import {CertificateFormFields} from './certificate-form-fields';
import {
	EMPTY_CERTIFICATE_FORM,
	type Certificate,
	type CertificateForm,
	type RemoteServer,
} from './certificate-types';

type Props = {
	certificate: Certificate | null;
	servers: RemoteServer[];
	open: boolean;
	onOpenChange: (open: boolean) => void;
	onSaved: () => Promise<void>;
};

export function CertificateSheet({
	certificate,
	servers,
	open,
	onOpenChange,
	onSaved,
}: Props) {
	const [form, setForm] = useState<CertificateForm>(
		EMPTY_CERTIFICATE_FORM,
	);
	const [saving, setSaving] = useState(false);

	useEffect(() => {
		if (!open) return;
		setForm(
			certificate
				? {
						name: certificate.name,
						certificate_data: certificate.certificate_data,
						private_key: '',
						certificate_path: certificate.certificate_path,
						auto_renew: certificate.auto_renew === 1,
						server_id: certificate.server_id || 'local',
					}
				: EMPTY_CERTIFICATE_FORM,
		);
	}, [certificate, open]);

	const set = <K extends keyof CertificateForm>(
		key: K,
		value: CertificateForm[K],
	) => setForm(current => ({...current, [key]: value}));

	const save = async () => {
		if (!certificate && !form.private_key.trim())
			return toast.error('Private key is required');
		try {
			setSaving(true);
			const body = {
				name: form.name.trim(),
				certificate_data: form.certificate_data.trim() || undefined,
				private_key: form.private_key.trim() || undefined,
				certificate_path: form.certificate_path.trim(),
				auto_renew: form.auto_renew ? 1 : 0,
				server_id: form.server_id === 'local' ? undefined : form.server_id,
			};
			if (certificate)
				await updateCertificate(Number(certificate.id), body);
			else
				await createCertificate({
					...body,
					certificate_data: form.certificate_data.trim(),
					private_key: form.private_key.trim(),
				});
			toast.success(
				certificate ? 'Certificate updated' : 'Certificate installed',
			);
			onOpenChange(false);
			await onSaved();
		} catch (error) {
			toast.error(
				error instanceof Error
					? error.message
					: 'Certificate could not be saved',
			);
		} finally {
			setSaving(false);
		}
	};

	return (
		<Sheet open={open} onOpenChange={onOpenChange}>
			<SheetContent className="p-0">
				<SheetHeader>
					<SheetTitle className="flex items-center gap-2">
						<ShieldCheck className="size-4 text-primary" />
						{certificate ? 'Edit certificate' : 'Install certificate'}
					</SheetTitle>
					<SheetDescription>
						Certificate chain and private key must be a matching PEM pair.
					</SheetDescription>
				</SheetHeader>
				<CertificateFormFields
					editing={Boolean(certificate)}
					form={form}
					servers={servers}
					onChange={set}
				/>
				<SheetFooter className="justify-between">
					<Button variant="outline" onClick={() => onOpenChange(false)}>
						Cancel
					</Button>
					<Button
						onClick={save}
						disabled={
							saving ||
							!form.name.trim() ||
							!form.certificate_data.trim() ||
							!form.certificate_path.trim() ||
							(!certificate && !form.private_key.trim())
						}
						className="h-10 gap-1.5 px-5">
						{saving && <Loader2 className="size-3.5 animate-spin" />}
						{certificate ? 'Save changes' : 'Install certificate'}
					</Button>
				</SheetFooter>
			</SheetContent>
		</Sheet>
	);
}
