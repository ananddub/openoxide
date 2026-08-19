import {useState, useEffect} from 'react';
import {Save, Info} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import {toast} from 'sonner';
import {$api} from '#/api/query';
import {formatApiError} from '#/api/utils';
import {RadioGroup, RadioGroupItem} from '#/components/ui/radio-group';

interface BuildSettingsCardProps {
	app: any;
	onUpdated: () => void;
}

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

export function BuildSettingsCard({
	app,
	onUpdated,
}: BuildSettingsCardProps) {
	const BUILD_TYPES: {id: string; label: string; isNew?: boolean}[] = [
		{id: 'DOCKERFILE', label: 'Dockerfile'},
		{id: 'RAILPACK', label: 'Railpack', isNew: true},
		{id: 'NIXPACKS', label: 'Nixpacks'},
		{id: 'HEROKU_BUILDPACKS', label: 'Heroku Buildpacks'},
		{id: 'PAKETO_BUILDPACKS', label: 'Paketo Buildpacks'},
		{id: 'STATIC', label: 'Static'},
	];

	const [buildType, setBuildType] = useState<string>(
		app.build_type || 'NIXPACKS',
	);
	const [savingBuild, setSavingBuild] = useState(false);

	// Form field states
	const [dockerfile, setDockerfile] = useState(
		app.dockerfile || 'Dockerfile',
	);
	const [dockerContextPath, setDockerContextPath] = useState(
		app.docker_context_path || '.',
	);
	const [dockerBuildStage, setDockerBuildStage] = useState(
		app.docker_build_stage || '',
	);
	const [herokuVersion, setHerokuVersion] = useState(
		app.heroku_version || '24',
	);
	const [publishDirectory, setPublishDirectory] = useState(
		app.publish_directory || '',
	);
	const [isStaticSpa, setIsStaticSpa] = useState(app.is_static_spa === 1);
	const [railpackVersion, setRailpackVersion] = useState(
		app.railpack_version || '0.15.4',
	);
	const [isManualRailpack, setIsManualRailpack] = useState(
		!RAILPACK_VERSIONS.includes(app.railpack_version as any) &&
			!!app.railpack_version,
	);

	useEffect(() => {
		if (app) {
			setBuildType(app.build_type || 'NIXPACKS');
			setDockerfile(app.dockerfile || 'Dockerfile');
			setDockerContextPath(app.docker_context_path || '.');
			setDockerBuildStage(app.docker_build_stage || '');
			setHerokuVersion(app.heroku_version || '24');
			setPublishDirectory(app.publish_directory || '');
			setIsStaticSpa(app.is_static_spa === 1);
			setRailpackVersion(app.railpack_version || '0.15.4');
			setIsManualRailpack(
				!RAILPACK_VERSIONS.includes(app.railpack_version as any) &&
					!!app.railpack_version,
			);
		}
	}, [app]);

	const patchBuildType = $api.useMutation('patch', '/applications/{id}');
	const patchBuildConfig = $api.useMutation(
		'patch',
		'/applications/{id}/build',
	);

	const handleSaveBuildType = async () => {
		setSavingBuild(true);
		try {
			// Update general build type
			await patchBuildType.mutateAsync({
				params: {path: {id: app.id}},
				body: {
					build_type: buildType,
				},
			});

			// Update build-specific configuration details
			await patchBuildConfig.mutateAsync({
				params: {path: {id: app.id}},
				body: {
					dockerfile,
					docker_context_path: dockerContextPath,
					docker_build_stage: dockerBuildStage || undefined,
					heroku_version: herokuVersion || undefined,
					publish_directory: publishDirectory || undefined,
					is_static_spa: isStaticSpa ? 1 : 0,
					railpack_version: railpackVersion || undefined,
				},
			});

			toast.success('Build settings saved successfully');
			onUpdated();
		} catch (err: any) {
			toast.error(formatApiError(err));
		} finally {
			setSavingBuild(false);
		}
	};

	return (
		<section className="flex flex-col gap-4 rounded-xl border border-border bg-card p-5">
			<div>
				<h3 className="text-sm font-bold text-foreground">Build Type</h3>
				<p className="mt-1 text-xs text-muted-foreground">
					Select the way of building your code
				</p>
			</div>

			{/* Resource alert block */}
			<div className="flex gap-3 rounded-xl border border-amber-500/10 bg-amber-500/5 p-4 text-xs leading-relaxed text-muted-foreground">
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
									? 'font-extrabold text-foreground'
									: 'font-medium text-muted-foreground hover:text-foreground'
							}`}>
							{bt.label}
							{bt.isNew && (
								<span className="rounded bg-[#3b82f6] px-1.5 py-0.5 text-[9px] leading-none font-extrabold text-white select-none">
									New
								</span>
							)}
						</span>
					</label>
				))}
			</RadioGroup>

			{/* Dynamic Form Sections */}
			<div className="flex flex-col gap-4 border-t border-border/40 pt-4">
				{buildType === 'DOCKERFILE' && (
					<div className="grid grid-cols-1 gap-4 sm:grid-cols-2">
						<div className="flex flex-col gap-1.5">
							<span className="text-xs font-bold text-muted-foreground">
								Dockerfile Path
							</span>
							<Input
								placeholder="Dockerfile"
								value={dockerfile}
								onChange={e => setDockerfile(e.target.value)}
								className="border-border bg-card font-mono text-xs"
							/>
						</div>
						<div className="flex flex-col gap-1.5">
							<span className="text-xs font-bold text-muted-foreground">
								Docker Context Path
							</span>
							<Input
								placeholder="."
								value={dockerContextPath}
								onChange={e => setDockerContextPath(e.target.value)}
								className="border-border bg-card font-mono text-xs"
							/>
						</div>
						<div className="flex flex-col gap-1.5 sm:col-span-2">
							<span className="text-xs font-bold text-muted-foreground">
								Docker Build Stage (Optional)
							</span>
							<Input
								placeholder="E.g. production"
								value={dockerBuildStage}
								onChange={e => setDockerBuildStage(e.target.value)}
								className="border-border bg-card text-xs"
							/>
						</div>
					</div>
				)}

				{buildType === 'HEROKU_BUILDPACKS' && (
					<div className="flex flex-col gap-1.5">
						<span className="text-xs font-bold text-muted-foreground">
							Heroku Version / Stack (Optional)
						</span>
						<Input
							placeholder="Default: 24"
							value={herokuVersion}
							onChange={e => setHerokuVersion(e.target.value)}
							className="border-border bg-card text-xs"
						/>
					</div>
				)}

				{buildType === 'NIXPACKS' && (
					<div className="flex flex-col gap-1.5">
						<span className="text-xs font-bold text-muted-foreground">
							Publish Directory (Optional)
						</span>
						<Input
							placeholder="e.g. dist"
							value={publishDirectory}
							onChange={e => setPublishDirectory(e.target.value)}
							className="border-border bg-card text-xs"
						/>
					</div>
				)}

				{buildType === 'STATIC' && (
					<div className="flex items-center gap-3 rounded-lg border border-border/40 bg-muted/20 p-3">
						<button
							type="button"
							onClick={() => setIsStaticSpa(!isStaticSpa)}
							className={`relative inline-flex h-4.5 w-8 shrink-0 cursor-pointer items-center rounded-full transition-colors ${isStaticSpa ? 'bg-primary' : 'bg-muted'}`}>
							<span
								className={`pointer-events-none block h-3 w-3 rounded-full bg-background shadow-lg transition-transform ${isStaticSpa ? 'translate-x-4.5' : 'translate-x-0.5'}`}
							/>
						</button>
						<div className="flex flex-col gap-0.5">
							<span className="text-xs font-bold text-foreground">
								Single Page Application (SPA)
							</span>
							<span className="text-[10px] text-muted-foreground">
								Redirect all requests to index.html for clientside routers
							</span>
						</div>
					</div>
				)}

				{buildType === 'RAILPACK' && (
					<div className="flex flex-col gap-3">
						<span className="text-xs font-bold text-muted-foreground">
							Railpack Version
						</span>
						{isManualRailpack ? (
							<div className="flex flex-col gap-2">
								<Input
									placeholder="Enter custom version (e.g. 0.15.4)"
									value={railpackVersion}
									onChange={e => setRailpackVersion(e.target.value)}
									className="border-border bg-card font-mono text-xs"
								/>
								<Button
									type="button"
									variant="outline"
									size="sm"
									onClick={() => {
										setIsManualRailpack(false);
										setRailpackVersion('0.15.4');
									}}
									className="h-7 w-fit border-border text-[10px] font-semibold">
									Use predefined versions
								</Button>
							</div>
						) : (
							<div className="flex flex-col gap-2">
								<Select
									value={railpackVersion}
									onValueChange={val => {
										if (val === 'manual') {
											setIsManualRailpack(true);
											setRailpackVersion('');
										} else {
											setRailpackVersion(val);
										}
									}}>
									<SelectTrigger className="h-9 w-full border-border bg-card text-xs">
										<SelectValue placeholder="Select Railpack version" />
									</SelectTrigger>
									<SelectContent className="border border-border bg-popover">
										<SelectItem
											value="manual"
											className="cursor-pointer text-xs font-bold text-primary hover:bg-muted">
											✏️ Custom Version (Manual)
										</SelectItem>
										{RAILPACK_VERSIONS.map(v => (
											<SelectItem
												key={v}
												value={v}
												className="cursor-pointer font-mono text-xs hover:bg-muted">
												v{v}
											</SelectItem>
										))}
									</SelectContent>
								</Select>
							</div>
						)}
					</div>
				)}
			</div>

			<div className="mt-2 flex justify-end border-t border-border/20 pt-4">
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
