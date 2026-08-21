export type Certificate = {
	id: string;
	name: string;
	certificate_data: string;
	has_private_key: boolean;
	certificate_path: string;
	auto_renew: number;
	server_id?: string;
	created_at: number;
	updated_at: number;
};

export type CertificateRenewal = {
	id: number;
	status: string;
	previous_expires_at?: number;
	new_expires_at?: number;
	error?: string;
	started_at: number;
	finished_at?: number;
};

export type CertificateForm = {
	name: string;
	certificate_data: string;
	private_key: string;
	certificate_path: string;
	auto_renew: boolean;
	server_id: string;
};

export type RemoteServer = {
	id: number;
	name: string;
	ip_address: string;
	server_type: string;
};

export const EMPTY_CERTIFICATE_FORM: CertificateForm = {
	name: '',
	certificate_data: '',
	private_key: '',
	certificate_path: '',
	auto_renew: false,
	server_id: 'local',
};
