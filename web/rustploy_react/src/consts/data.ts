type Organizations = {
	name: string;
	id: number;
	logo: string;
	isDefault: boolean;
};

export const organizations: Organizations[] = [
	{
		id: 1,
		name: 'Organization 1',
		logo: 'https://api.dicebear.com/9.x/shapes/svg?seed=org1',
		isDefault: true,
	},
	{
		id: 2,
		name: 'Organization 2',
		logo: 'https://api.dicebear.com/9.x/shapes/svg?seed=org2',
		isDefault: false,
	},
];
