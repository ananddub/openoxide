import {create} from 'zustand';
import type {components} from '#/types/api.d.ts';

type Organization = components['schemas']['OrganizationResponseDto'];

type OrganizationState = {
	organizations: Organization[];
	activeOrg: Organization | null;
	setOrganizations: (orgs: Organization[]) => void;
	setActiveOrg: (org: Organization | null) => void;
};

export const useOrganizationStore = create<OrganizationState>((set, get) => ({
	organizations: [],
	activeOrg: null,
	setOrganizations: orgs => {
		const currentActive = get().activeOrg;
		const currentOrgs = get().organizations;

		// If orgs array length and IDs match current, avoid re-setting
		if (
			currentOrgs.length === orgs.length &&
			currentOrgs.every((org, index) => org.id === orgs[index]?.id && org.name === orgs[index]?.name)
		) {
			return;
		}

		let nextActive = currentActive;
		if (orgs.length > 0) {
			const found = orgs.find(o => o.id === currentActive?.id);
			nextActive = found || orgs[0];
		} else {
			nextActive = null;
		}

		set({organizations: orgs, activeOrg: nextActive});
	},
	setActiveOrg: org => set({activeOrg: org}),
}));
