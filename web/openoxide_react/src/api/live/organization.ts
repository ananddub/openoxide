import {createServiceClient, useConnectQuery} from './query';
import {OrganizationService} from '#/gen/connect/openoxide/api/v1/organization_pb';
import type {Organization} from '#/gen/connect/openoxide/api/v1/organization_pb';

export type OrganizationPayload = {
	id: bigint;
	name: string;
	logo: string | null;
	slug: string;
	owner_id: bigint;
	created_at: bigint;
	updated_at: bigint;
};

const organizationClient = createServiceClient(OrganizationService);

/** Organizations for the current owner (legacy live: useOrganizationListByOwner). */
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
