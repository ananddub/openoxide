import {useEffect, useState} from 'react';
import {toast} from 'sonner';
import {useSettingsGet, useSettingsUpdate} from '#/api/live';
import {usePermissions} from '#/hooks/permissions/use-permissions';

export type CertType = 'None' | 'Letsencrypt' | 'Custom';

export type SettingsForm = {
	server_ip: string;
	host: string;
	https: boolean;
	certificate_type: CertType;
	lets_encrypt_email: string;
	custom_cert_resolver: string;
	enable_docker_cleanup: boolean;
	log_cleanup_cron: string;
};

const EMPTY_FORM: SettingsForm = {
	server_ip: '',
	host: '',
	https: false,
	certificate_type: 'None',
	lets_encrypt_email: '',
	custom_cert_resolver: '',
	enable_docker_cleanup: false,
	log_cleanup_cron: '',
};

export function isIpAddress(value: string): boolean {
	const ipv4 =
		/^\d{1,3}(\.\d{1,3}){3}$/.test(value) &&
		value.split('.').every(part => Number(part) <= 255);
	return ipv4 || value.includes(':');
}

/** Host-configuration form: load, dirty-tracking, validate and save. */
export function useHostSettingsForm() {
	const {can} = usePermissions();
	const canManage = can('org:write');
	const settingsQuery = useSettingsGet({enabled: canManage});
	const updateMutation = useSettingsUpdate();

	const [form, setForm] = useState<SettingsForm>(EMPTY_FORM);
	const [dirty, setDirty] = useState(false);

	// Seed the form once settings land (and on realtime refreshes while the form is untouched, so external edits don't get silently overwritten).
	useEffect(() => {
		const s = settingsQuery.data;
		if (!s || dirty) return;
		setForm({
			server_ip: s.server_ip ?? '',
			host: s.host ?? '',
			https: s.https,
			certificate_type:
				s.certificate_type === 'LETSENCRYPT'
					? 'Letsencrypt'
					: s.certificate_type === 'CUSTOM'
						? 'Custom'
						: 'None',
			lets_encrypt_email: s.lets_encrypt_email ?? '',
			custom_cert_resolver: s.custom_cert_resolver ?? '',
			enable_docker_cleanup: s.enable_docker_cleanup,
			log_cleanup_cron: s.log_cleanup_cron ?? '',
		});
	}, [settingsQuery.data, dirty]);

	const set = <K extends keyof SettingsForm>(
		key: K,
		value: SettingsForm[K],
	) => {
		setDirty(true);
		setForm(prev => ({...prev, [key]: value}));
	};

	const validate = () => {
		const trimmedIp = form.server_ip.trim();
		if (trimmedIp && !isIpAddress(trimmedIp)) {
			toast.error('Server IP must be a valid IPv4/IPv6 address');
			return false;
		}
		if (form.host.trim().match(/\s/)) {
			toast.error('Host must not contain whitespace');
			return false;
		}
		if (
			form.certificate_type === 'Letsencrypt' &&
			!form.lets_encrypt_email.includes('@')
		) {
			toast.error('Letsencrypt requires a valid email');
			return false;
		}
		if (
			form.certificate_type === 'Custom' &&
			!form.custom_cert_resolver.trim()
		) {
			toast.error('Custom certificate requires a resolver name');
			return false;
		}
		return true;
	};

	const handleSave = () => {
		if (!validate()) return;
		updateMutation.mutate(
			{
				// Backend patch semantics: null keeps the stored value, "" clears it.
				server_ip: form.server_ip.trim(),
				host: form.host.trim(),
				https: form.https,
				certificate_type: form.certificate_type,
				lets_encrypt_email:
					form.certificate_type === 'Letsencrypt'
						? form.lets_encrypt_email.trim()
						: null,
				custom_cert_resolver:
					form.certificate_type === 'Custom'
						? form.custom_cert_resolver.trim()
						: null,
				enable_docker_cleanup: form.enable_docker_cleanup,
				log_cleanup_cron: form.log_cleanup_cron.trim(),
			},
			{
				onSuccess: () => {
					setDirty(false);
					toast.success('Web server settings saved');
				},
				onError: error => toast.error(error.message),
			},
		);
	};

	return {
		canManage,
		form,
		set,
		handleSave,
		isSaving: updateMutation.isPending,
		isLoading: settingsQuery.isLoading,
	};
}
