import {
	createServiceClient,
	useConnectMutation,
	useConnectQuery,
} from './query';
import {
	SshKeyService,
	SshKeyType,
	type SshKey,
} from '#/gen/connect/openoxide/api/v1/ssh_key_pb';

export type SshKeyPayload = {
	id: number;
	name: string;
	description?: string;
	public_key: string;
	private_key?: string;
	has_private_key: boolean;
	last_used_at?: number;
	created_at: number;
	updated_at: number;
};

const sshKeyClient = createServiceClient(SshKeyService);

const sshKeyTypeByName = SshKeyType as unknown as Record<
	string,
	SshKeyType
>;

/** Maps the typed `SshKey` message to the payload shape the UI reads. */
function toSshKeyPayload(key: SshKey): SshKeyPayload {
	return {
		id: Number(key.id),
		name: key.name,
		description: key.description,
		public_key: key.publicKey,
		private_key: key.privateKey,
		has_private_key: key.hasPrivateKey,
		last_used_at:
			key.lastUsedAt !== undefined ? Number(key.lastUsedAt) : undefined,
		created_at: Number(key.createdAt),
		updated_at: Number(key.updatedAt),
	};
}

/** SSH keys (legacy live: useSshKeyList). */
export function useSshKeyList() {
	return useConnectQuery<SshKeyPayload[]>(
		['ssh_keys', 'list'],
		async () => {
			const response = await sshKeyClient.listKeys({});
			return response.keys.map(toSshKeyPayload);
		},
	);
}

/** SSH keys under the table-watch key prefix `['ssh_keys']` — the former zustand `sshKeys` slice data with the same payload shape. */
export function useSshKeyItems() {
	return useConnectQuery<SshKeyPayload[]>(['ssh_keys'], async () => {
		const response = await sshKeyClient.listKeys({});
		return response.keys.map(toSshKeyPayload);
	});
}

/** Single SSH key (legacy live: useSshKeyGet). */
export function useSshKeyGet(id: bigint) {
	return useConnectQuery<SshKeyPayload>(
		['ssh_keys', 'get', id.toString()],
		async () => {
			const response = await sshKeyClient.getKey({id});
			if (!response.key) throw new Error('ssh key not found');
			return toSshKeyPayload(response.key);
		},
		{enabled: id > 0n},
	);
}

/** Create an SSH key from provided key material. */
export function useSshKeyCreate() {
	return useConnectMutation(
		(input: {
			name: string;
			description?: string;
			public_key: string;
			private_key?: string;
		}) =>
			sshKeyClient.createKey({
				name: input.name,
				description: input.description,
				publicKey: input.public_key,
				privateKey: input.private_key ?? '',
			}),
		// The watch stream also invalidates this prefix, but refetching on mutation success keeps the actor's own list deterministic even while the stream is reconnecting (delete/insert latency must not depend on a live subscription).
		{invalidate: [['ssh_keys']]},
	);
}

/** Generate and store a new key pair server-side (`key_type`: 'ed25519' | 'rsa'). */
export function useSshKeyGenerate() {
	return useConnectMutation(
		(input: {name: string; description?: string; key_type?: string}) =>
			sshKeyClient.generateKey({
				name: input.name,
				description: input.description,
				keyType:
					(input.key_type
						? sshKeyTypeByName[input.key_type.toUpperCase()]
						: undefined) ?? SshKeyType.UNSPECIFIED,
			}),
		{invalidate: [['ssh_keys']]},
	);
}

/** Generate an in-memory key pair without storing it (`key_type`: 'ed25519' | 'rsa'; omitted uses the server default). The typed response carries `privateKey` / `publicKey` directly. */
export function useSshKeyGeneratePair() {
	return useConnectMutation((input: {key_type?: string}) =>
		sshKeyClient.generatePair({
			keyType: input.key_type
				? sshKeyTypeByName[input.key_type.toUpperCase()]
				: undefined,
		}),
	);
}

/** Update an SSH key by id (PATCH semantics: absent fields unchanged). */
export function useSshKeyUpdate() {
	return useConnectMutation(
		(input: {
			id: number | bigint;
			name?: string;
			description?: string;
			public_key?: string;
			private_key?: string;
		}) =>
			sshKeyClient.updateKey({
				id: BigInt(input.id),
				name: input.name,
				description: input.description,
				publicKey: input.public_key,
				privateKey: input.private_key,
			}),
		{invalidate: [['ssh_keys']]},
	);
}

/** Delete an SSH key by id. */
export function useSshKeyDelete() {
	return useConnectMutation(
		(id: number | bigint) => sshKeyClient.deleteKey({id: BigInt(id)}),
		// Delete CDC events fan out globally (the org lookup runs after the row is gone), so the stream does deliver them — but the deleting user should never wait on a 150ms coalesced stream tick to see their own row disappear.
		{invalidate: [['ssh_keys']]},
	);
}
