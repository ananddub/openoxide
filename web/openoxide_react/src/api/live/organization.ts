import {createServiceClient, useConnectMutation, useConnectQuery} from './query';
import {OrganizationService} from '#/gen/connect/openoxide/api/v1/organization_pb';
import {AuthService} from '#/gen/connect/openoxide/api/v1/auth_pb';
import type {Organization} from '#/gen/connect/openoxide/api/v1/organization_pb';

export type OrganizationPayload = {
	id: bigint;
	name: string;
	logo: string | null;
	slug: string;
	owner_id: bigint;
	created_at: bigint;
	updated_at: bigint;
	// Caller's effective role in this organization (ListOrganizations):
	// OWNER / ADMIN / MEMBER.
	role?: string;
};

const organizationClient = createServiceClient(OrganizationService);

/** Every organization the signed-in user owns or joined, with their role. */
export function useOrganizationListByOwner() {
	return useConnectQuery<OrganizationPayload[]>(
		['organizations', 'list'],
		async () => {
			const response = await organizationClient.listOrganizations({});
			return response.organizations.map(
				(organization: Organization): OrganizationPayload => ({
					id: organization.id,
					name: organization.name,
					logo: organization.logo ?? null,
					slug: organization.slug,
					owner_id: organization.ownerId,
					created_at: organization.createdAt,
					updated_at: organization.updatedAt,
					role: organization.role,
				}),
			);
		},
	);
}

/** Single organization (legacy live: useOrganizationGet). */
export function useOrganizationGet(id: bigint) {
	return useConnectQuery<OrganizationPayload>(
		['organizations', 'get', id.toString()],
		async () => {
			const organization = await organizationClient.getOrganization({id});
			return {
				id: organization.id,
				name: organization.name,
				logo: organization.logo ?? null,
				slug: organization.slug,
				owner_id: organization.ownerId,
				created_at: organization.createdAt,
				updated_at: organization.updatedAt,
			};
		},
		{enabled: id > 0n},
	);
}

/** Leave an organization the caller is a member of (owners must delete). */
export function useOrganizationLeave() {
	return useConnectMutation(
		(id: number | bigint) =>
			organizationClient.leaveOrganization({id: BigInt(id)}),
		{invalidate: [['organizations']]},
	);
}

/** Create an organization (legacy REST: POST /organizations). */
export function useOrganizationCreate() {
	return useConnectMutation(
		(input: {name: string; logo?: string; slug?: string}) =>
			organizationClient.createOrganization({
				name: input.name,
				logo: input.logo,
				slug: input.slug,
			}),
		{invalidate: [['organizations']]},
	);
}

/** Update an organization by id (legacy REST: PATCH /organizations/{id}). */
export function useOrganizationUpdate() {
	return useConnectMutation(
		(input: {id: number | bigint; name?: string; logo?: string; slug?: string}) =>
			organizationClient.updateOrganization({
				id: BigInt(input.id),
				name: input.name,
				logo: input.logo,
				slug: input.slug,
			}),
		{invalidate: [['organizations']]},
	);
}

/** Delete an organization by id (legacy REST: DELETE /organizations/{id}). */
export function useOrganizationDelete() {
	return useConnectMutation(
		(id: number | bigint) =>
			organizationClient.deleteOrganization({id: BigInt(id)}),
		{invalidate: [['organizations']]},
	);
}

// ---------------------------------------------------------------------------
// Profile (auth) updates. AuthService.UpdateUser carries the profile fields
// the settings page edits; the hook returns the legacy REST-era snake_case
// payload shape the stores read.
// ---------------------------------------------------------------------------

const authClient = createServiceClient(AuthService);

export type UserProfilePayload = {
	user_id: number;
	email: string;
	first_name: string;
	last_name: string;
	avatar: string;
};

/** Update the signed-in user's profile (legacy REST: PATCH /auth/user). */
export function useUserUpdate() {
	return useConnectMutation(
		(input: {
			email?: string;
			password?: string;
			first_name?: string;
			last_name?: string;
			avatar?: string;
		}) =>
			authClient.updateUser({
				email: input.email,
				password: input.password,
				firstName: input.first_name,
				lastName: input.last_name,
				avatar: input.avatar,
			}),
		// The profile query is keyed ['auth', 'profile'] — the same prefix
		// the table-watch USERS entry invalidates.
		{invalidate: [['auth']]},
	);
}

/** Legacy profile shape the former zustand `profile` slice carried. */
export type ProfilePayload = {
	id?: number;
	email?: string;
	name?: string;
	role?: string;
	avatar?: string;
};

/**
 * Signed-in user's profile (the former zustand `profile` slice data).
 * Key prefix `['auth']` matches the table-watch USERS entry, so profile
 * updates refetch automatically.
 */
export function useProfile() {
	return useConnectQuery<ProfilePayload>(['auth', 'profile'], async () => {
		const user = await authClient.whoAmI({});
		return {
			id: Number(user.id),
			email: user.email,
			name:
				`${user.firstName ?? ''} ${user.lastName ?? ''}`.trim() || undefined,
			avatar: user.avatar,
		};
	});
}
