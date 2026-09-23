import {useState, useEffect} from 'react';
import {Save, Info} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {RadioGroup, RadioGroupItem} from '#/components/ui/radio-group';
import {toast} from 'sonner';
import {
	useApplicationUpdate,
	useApplicationUpdateBuildConfig,
} from '#/api/live';
import {ApplicationBuildType} from '#/gen/connect/openoxide/api/v1/application_pb';
import {cn, formatApiError} from '#/api/utils';
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
	embedded?: boolean;
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

export function BuildSettingsCard({
	app,
	onUpdated,
	embedded,
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
			// Build type rides UpdateApplication; the details ride the
			// dedicated UpdateBuildConfig RPC (UpdateApplication has no
			// dockerfile/publish-directory/version-pin fields at all).
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
		<section
			className={cn(
				'flex flex-col gap-4',
				!embedded && 'rounded-xl bg-card p-5',
			)}>
			{!embedded && (
				<div>
					<h3 className="text-sm font-bold text-foreground">
						Build Type
					</h3>
					<p className="mt-1 text-xs text-muted-foreground">
						Select the way of building your code
					</p>
				</div>
			)}

			{/* Resource alert block */}
			<div className="flex gap-3 rounded-xl bg-amber-500/5 p-4 text-xs leading-relaxed text-muted-foreground">
				<Info className="mt-0.5 h-4 w-4 shrink-0 text-amber-500" />
				<div>
					Builders can consume significant memory and CPU resources
					(recommended: 4+ GB RAM and 2+ CPU cores).
				</div>
			</div>

			{/* Radio list in vertical layout */}
			<RadioGroup
				value={buildType}
				onValueChange={val => setBuildType(val)}
				className="mt-2 flex flex-col gap-1">
				{BUILD_TYPES.map(bt => (
					<label
						key={bt.id}
						className="flex w-full cursor-pointer items-center gap-3 rounded-lg px-2 py-1.5 text-left transition-colors select-none hover:bg-muted/20">
						<RadioGroupItem value={bt.id} />
						<span
							className={`flex items-center gap-2 text-xs transition-colors ${
								buildType === bt.id
									? 'font-semibold text-foreground'
									: 'font-medium text-muted-foreground hover:text-foreground'
							}`}>
							{bt.label}
							{bt.isNew && (
								<span className="rounded bg-[#3b82f6] px-1.5 py-0.5 text-[9px] leading-none font-semibold text-white select-none">
									New
								</span>
							)}
						</span>
					</label>
				))}
			</RadioGroup>

			{/* Dynamic Form Sections */}
			<div className="flex flex-col gap-4 border-t  pt-4">
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

			<div className="mt-2 flex justify-end border-t  pt-4">
				<Button
					onClick={handleSaveBuildType}
					disabled={savingBuild}
					className="flex h-9 items-center gap-1.5 rounded-lg bg-primary text-xs font-semibold text-primary-foreground hover:bg-primary/95">
					<Save className="h-3.5 w-3.5" />{' '}
					{savingBuild ? 'Saving...' : 'Save Build Settings'}
				</Button>
			</div>
		</section>
	);
}
