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
		// Keep current active if it still exists in the new list, otherwise fallback to first
		let nextActive = null;
		if (orgs.length > 0) {
			nextActive = orgs.find(o => o.id === currentActive?.id) || orgs[0];
		}
		set({organizations: orgs, activeOrg: nextActive});
	},
	setActiveOrg: org => set({activeOrg: org}),
}));
