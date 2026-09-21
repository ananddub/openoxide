/**
 * Permission payloads mirror the legacy live shape (snake_case JSON payloads
 * of the identity DTOs). The organization context travels in the
 * `X-Organization-Id` header attached by the transport.
 */
export type OrganizationMemberPayload = {
	id: bigint | null;
	user_id: bigint;
	email: string;
	avatar: string | null;
	organization_id: bigint;
	role: string | null;
	created_at: bigint;
	updated_at: bigint;
};

export type OrganizationInvitePayload = {
	id: bigint | null;
	email: string;
	role: string | null;
	group_id: bigint;
	organization_id: bigint;
	invited_by: bigint;
	expired_at: bigint;
	created_at: bigint;
};

export type PermissionGroupPayload = {
	id: bigint;
	name: string;
	is_system: boolean;
	organization_id: bigint | null;
	actions: string[];
	created_at: bigint;
	updated_at: bigint;
};

/** Legacy member shape the former zustand `members` slice carried. */
export type MemberItem = {
	id: string;
	user_id?: number;
	email: string;
	name?: string;
	role: string;
	avatar?: string;
	banned?: boolean;
	created_at?: number;
};

/** Legacy invite shape the former zustand `invites` slice carried. */
export type InviteItem = {
	id: string;
	email: string;
	role: string;
	created_at?: number;
};

export type MyPermissionsPayload = {
	role: string;
	organization_id: bigint;
	actions: string[];
};

export type PermissionPolicyPayload = {
	id: number;
	action: string;
};
