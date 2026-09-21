import {useEffect, useState} from 'react';
import type {
	RemoteServerResponse,
	SshKeyResponse,
} from '#/types/api-helpers';

/** Seed + hold the six remote-server form fields for the modal. */
export function useServerForm({
	isOpen,
	editingServer,
	sshKeys,
}: {
	isOpen: boolean;
	editingServer?: RemoteServerResponse | null;
	sshKeys: SshKeyResponse[];
}) {
	const [name, setName] = useState('');
	const [ipAddress, setIpAddress] = useState('');
	const [port, setPort] = useState('22');
	const [username, setUsername] = useState('root');
	const [sshKeyId, setSshKeyId] = useState<string>('');
	const [description, setDescription] = useState('');

	useEffect(() => {
		if (editingServer) {
			setName(editingServer.name || '');
			setIpAddress(editingServer.ip_address || '');
			setPort(String(editingServer.port || 22));
			setUsername(editingServer.username || 'root');
			setSshKeyId(
				editingServer.ssh_key_id ? String(editingServer.ssh_key_id) : '',
			);
			setDescription(editingServer.description || '');
		} else {
			setName('');
			setIpAddress('');
			setPort('22');
			setUsername('root');
			setSshKeyId(
				sshKeys && sshKeys.length > 0 ? String(sshKeys[0].id) : '',
			);
			setDescription('');
		}
	}, [editingServer, isOpen, sshKeys]);

	return {
		name,
		setName,
		ipAddress,
		setIpAddress,
		port,
		setPort,
		username,
		setUsername,
		sshKeyId,
		setSshKeyId,
		description,
		setDescription,
	};
}
