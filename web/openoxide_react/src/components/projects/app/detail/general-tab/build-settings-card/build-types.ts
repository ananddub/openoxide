export const RAILPACK_VERSIONS = [
	'0.15.4',
	'0.15.3',
	'0.15.2',
	'0.15.1',
	'0.15.0',
	'0.14.0',
	'0.13.0',
	'0.12.0',
	'0.11.0',
	'0.10.0',
	'0.9.2',
	'0.9.1',
	'0.9.0',
	'0.8.0',
	'0.7.0',
	'0.6.0',
	'0.5.0',
	'0.4.0',
	'0.3.0',
	'0.2.2',
] as const;

// Ids are the wire enum names (ApplicationBuildType) so the value read
// back from GetApplication selects the same radio after a save.
export const BUILD_TYPES: {id: string; label: string; isNew?: boolean}[] =
	[
		{id: 'DOCKERFILE', label: 'Dockerfile'},
		{id: 'RAILPACK', label: 'Railpack', isNew: true},
		{id: 'NIXPACKS', label: 'Nixpacks'},
		{id: 'HEROKU', label: 'Heroku Buildpacks'},
		{id: 'PAKETO', label: 'Paketo Buildpacks'},
		{id: 'STATIC', label: 'Static'},
	];

/** Subset of the build-form fields relevant to the dynamic sections. */
export interface BuildFields {
	dockerfile: string;
	dockerContextPath: string;
	dockerBuildStage: string;
	herokuVersion: string;
	publishDirectory: string;
}
