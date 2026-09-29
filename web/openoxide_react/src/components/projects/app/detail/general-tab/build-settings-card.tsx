import {useState, useEffect} from 'react';
import {Check, Info} from 'lucide-react';
import {toast} from 'sonner';
import {Button} from '#/components/ui/button';
import {
	useApplicationUpdate,
	useApplicationUpdateBuildConfig,
} from '#/api/live';
import {ApplicationBuildType} from '#/gen/connect/openoxide/api/v1/application_pb';
import {cn, formatApiError} from '#/api/utils';
import {SheetGroup, SheetNote} from '#/components/shared/field';
import {SettingsSheet} from '#/components/shared/settings-sheet';
import {
	BUILD_TYPES,
	RAILPACK_VERSIONS,
	type BuildFields,
} from './build-settings-card/build-types';
import {BuildTypeFields} from './build-settings-card/build-type-fields';
import {RailpackVersionField} from './build-settings-card/railpack-version-field';

interface BuildSettingsCardProps {
	app: any;
	onUpdated: () => void;
	/** Header-icon trigger: the caller owns visibility, no strip is painted. */
	open?: boolean;
	onOpenChange?: (open: boolean) => void;
}

function fieldsFromApp(app: any): BuildFields {
	return {
		dockerfile: app.dockerfile || 'Dockerfile',
		dockerContextPath: app.docker_context_path || '.',
		dockerBuildStage: app.docker_build_stage || '',
		herokuVersion: app.heroku_version || '24',
		publishDirectory: app.publish_directory || '',
	};
}

/** One compact sheet: build-type chips on top, the selected type's options right below — same grammar as the Source & Provider sheet. */
export function BuildSettingsCard({
	app,
	onUpdated,
	open,
	onOpenChange,
}: BuildSettingsCardProps) {
	const [buildType, setBuildType] = useState<string>(
		app.build_type || 'NIXPACKS',
	);
	const [fields, setFields] = useState<BuildFields>(() =>
		fieldsFromApp(app),
	);
	const [isStaticSpa, setIsStaticSpa] = useState(!!app.is_static_spa);
	const [railpackVersion, setRailpackVersion] = useState(
		app.railpack_version || '0.15.4',
	);
	const [isManualRailpack, setIsManualRailpack] = useState(
		!RAILPACK_VERSIONS.includes(app.railpack_version as any) &&
			!!app.railpack_version,
	);
	const [savingBuild, setSavingBuild] = useState(false);

	useEffect(() => {
		if (app) {
			setBuildType(app.build_type || 'NIXPACKS');
			setFields(fieldsFromApp(app));
			setIsStaticSpa(!!app.is_static_spa);
			setRailpackVersion(app.railpack_version || '0.15.4');
			setIsManualRailpack(
				!RAILPACK_VERSIONS.includes(app.railpack_version as any) &&
					!!app.railpack_version,
			);
		}
	}, [app]);

	const patchBuildType = useApplicationUpdate();
	const patchBuildConfig = useApplicationUpdateBuildConfig();

	const onPatch = (patch: Partial<BuildFields>) =>
		setFields(f => ({...f, ...patch}));

	const handleSaveBuildType = async () => {
		setSavingBuild(true);
		try {
			// Build type rides UpdateApplication; the details ride the dedicated UpdateBuildConfig RPC (UpdateApplication has no dockerfile/publish-directory/version-pin fields at all).
			await patchBuildType.mutateAsync({
				id: app.id,
				build_type:
					ApplicationBuildType[
						buildType as keyof typeof ApplicationBuildType
					],
			});

			await patchBuildConfig.mutateAsync({
				id: app.id,
				dockerfile: fields.dockerfile,
				context_path: fields.dockerContextPath,
				build_stage: fields.dockerBuildStage || undefined,
				heroku_version:
					buildType === 'HEROKU'
						? fields.herokuVersion || undefined
						: undefined,
				publish_directory: fields.publishDirectory || undefined,
				static_spa: isStaticSpa,
				railpack_version:
					buildType === 'RAILPACK'
						? railpackVersion || undefined
						: undefined,
			});

			toast.success('Build settings saved successfully');
			onUpdated();
		} catch (err: unknown) {
			toast.error(formatApiError(err));
		} finally {
			setSavingBuild(false);
		}
	};

	return (
		<SettingsSheet
			title="Build Configuration"
			description="The builder that turns your code into an image, and its options."
			open={open}
			onOpenChange={onOpenChange}
			footer={
				<Button
					onClick={() => void handleSaveBuildType()}
					disabled={savingBuild}
					className="h-8 gap-1.5 px-4 text-xs">
					{savingBuild ? 'Saving…' : 'Save changes'}
				</Button>
			}>
			<SheetGroup
				title="Build type"
				description="Pick the builder that turns your repository into an image.">
				{/* Compact builder picker — chip grid, not radio rows. */}
				<div className="grid grid-cols-2 gap-2 sm:grid-cols-3">
					{BUILD_TYPES.map(bt => {
						const selected = buildType === bt.id;
						return (
							<button
								key={bt.id}
								type="button"
								onClick={() => setBuildType(bt.id)}
								aria-pressed={selected}
								className={cn(
									'flex cursor-pointer items-center gap-2 rounded-lg border px-3 py-2.5 text-sm font-medium transition-colors',
									selected
										? 'border-primary/50 bg-primary/[0.08] text-foreground'
										: 'border-border/60 text-muted-foreground hover:bg-muted/30 hover:text-foreground',
								)}>
								<span className="truncate">{bt.label}</span>
								{bt.isNew && (
									<span className="shrink-0 rounded bg-primary/10 px-1.5 py-0.5 text-[9px] leading-none font-semibold text-primary">
										New
									</span>
								)}
								{selected && (
									<Check className="ml-auto size-3.5 shrink-0 text-primary" />
								)}
							</button>
						);
					})}
				</div>

				<SheetNote icon={<Info className="size-4" />}>
					Builders can consume significant memory and CPU resources
					(recommended: 4+ GB RAM and 2+ CPU cores).
				</SheetNote>
			</SheetGroup>

			<SheetGroup
				title="Builder options"
				description="Settings for the selected builder — leave blank for defaults.">
				<div className="flex flex-col gap-5">
					<BuildTypeFields
						buildType={buildType}
						fields={fields}
						isStaticSpa={isStaticSpa}
						onStaticSpaChange={setIsStaticSpa}
						onPatch={onPatch}
					/>

					{buildType === 'RAILPACK' && (
						<RailpackVersionField
							version={railpackVersion}
							isManual={isManualRailpack}
							versions={RAILPACK_VERSIONS}
							onVersionChange={setRailpackVersion}
							onManualChange={setIsManualRailpack}
						/>
					)}
				</div>
			</SheetGroup>
		</SettingsSheet>
	);
}
