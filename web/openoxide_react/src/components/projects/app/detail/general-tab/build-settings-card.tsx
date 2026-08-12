import {useState, useEffect} from 'react';
import {Save, Info} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {Select, SelectContent, SelectItem, SelectTrigger, SelectValue} from '#/components/ui/select';
import {toast} from 'sonner';
import {$api} from '#/api/query';
import {formatApiError} from '#/api/utils';
import {RadioGroup, RadioGroupItem} from '#/components/ui/radio-group';

interface BuildSettingsCardProps {
	app: any;
	onUpdated: () => void;
}

export const RAILPACK_VERSIONS = [
	"0.15.4",
	"0.15.3",
	"0.15.2",
	"0.15.1",
	"0.15.0",
	"0.14.0",
	"0.13.0",
	"0.12.0",
	"0.11.0",
	"0.10.0",
	"0.9.2",
	"0.9.1",
	"0.9.0",
	"0.8.0",
	"0.7.0",
	"0.6.0",
	"0.5.0",
	"0.4.0",
	"0.3.0",
	"0.2.2",
] as const;

export function BuildSettingsCard({app, onUpdated}: BuildSettingsCardProps) {
	const BUILD_TYPES: { id: string; label: string; isNew?: boolean }[] = [
		{id: 'DOCKERFILE', label: 'Dockerfile'},
		{id: 'RAILPACK', label: 'Railpack', isNew: true},
		{id: 'NIXPACKS', label: 'Nixpacks'},
		{id: 'HEROKU_BUILDPACKS', label: 'Heroku Buildpacks'},
		{id: 'PAKETO_BUILDPACKS', label: 'Paketo Buildpacks'},
		{id: 'STATIC', label: 'Static'},
	];

	const [buildType, setBuildType] = useState<string>(app.build_type || 'NIXPACKS');
	const [savingBuild, setSavingBuild] = useState(false);

	// Form field states
	const [dockerfile, setDockerfile] = useState(app.dockerfile || 'Dockerfile');
	const [dockerContextPath, setDockerContextPath] = useState(app.docker_context_path || '.');
	const [dockerBuildStage, setDockerBuildStage] = useState(app.docker_build_stage || '');
	const [herokuVersion, setHerokuVersion] = useState(app.heroku_version || '24');
	const [publishDirectory, setPublishDirectory] = useState(app.publish_directory || '');
	const [isStaticSpa, setIsStaticSpa] = useState(app.is_static_spa === 1);
	const [railpackVersion, setRailpackVersion] = useState(app.railpack_version || '0.15.4');
	const [isManualRailpack, setIsManualRailpack] = useState(!RAILPACK_VERSIONS.includes(app.railpack_version as any) && !!app.railpack_version);

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
			setIsManualRailpack(!RAILPACK_VERSIONS.includes(app.railpack_version as any) && !!app.railpack_version);
		}
	}, [app]);

	const patchBuildType = $api.useMutation('patch', '/applications/{id}');
	const patchBuildConfig = $api.useMutation('patch', '/applications/{id}/build');

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
		<section className="bg-card border border-border rounded-xl p-5 flex flex-col gap-4">
			<div>
				<h3 className="text-sm font-bold text-foreground">Build Type</h3>
				<p className="text-xs text-muted-foreground mt-1">Select the way of building your code</p>
			</div>

			{/* Resource alert block */}
			<div className="bg-amber-500/5 border border-amber-500/10 rounded-xl p-4 flex gap-3 text-xs leading-relaxed text-muted-foreground">
				<Info className="w-4 h-4 text-amber-500 shrink-0 mt-0.5" />
				<div>
					Builders can consume significant memory and CPU resources (recommended: 4+ GB RAM and 2+ CPU cores).
				</div>
			</div>

			{/* Radio list in vertical layout */}
			<RadioGroup
				value={buildType}
				onValueChange={(val) => setBuildType(val)}
				className="flex flex-col gap-1 mt-2"
			>
				{BUILD_TYPES.map(bt => (
					<label
						key={bt.id}
						className="flex items-center gap-3 cursor-pointer select-none py-1.5 px-2 rounded-lg hover:bg-muted/20 transition-colors w-full text-left"
					>
						<RadioGroupItem value={bt.id} />
						<span className={`text-xs flex items-center gap-2 transition-colors ${
							buildType === bt.id 
								? 'text-foreground font-extrabold' 
								: 'text-muted-foreground hover:text-foreground font-medium'
						}`}>
							{bt.label}
							{bt.isNew && (
								<span className="text-[9px] font-extrabold px-1.5 py-0.5 rounded bg-[#3b82f6] text-white select-none leading-none">
									New
								</span>
							)}
						</span>
					</label>
				))}
			</RadioGroup>

			{/* Dynamic Form Sections */}
			<div className="border-t border-border/40 pt-4 flex flex-col gap-4">
				{buildType === 'DOCKERFILE' && (
					<div className="grid grid-cols-1 sm:grid-cols-2 gap-4">
						<div className="flex flex-col gap-1.5">
							<span className="text-xs font-bold text-muted-foreground">Dockerfile Path</span>
							<Input placeholder="Dockerfile" value={dockerfile} onChange={e => setDockerfile(e.target.value)} className="bg-card border-border text-xs font-mono" />
						</div>
						<div className="flex flex-col gap-1.5">
							<span className="text-xs font-bold text-muted-foreground">Docker Context Path</span>
							<Input placeholder="." value={dockerContextPath} onChange={e => setDockerContextPath(e.target.value)} className="bg-card border-border text-xs font-mono" />
						</div>
						<div className="flex flex-col gap-1.5 sm:col-span-2">
							<span className="text-xs font-bold text-muted-foreground">Docker Build Stage (Optional)</span>
							<Input placeholder="E.g. production" value={dockerBuildStage} onChange={e => setDockerBuildStage(e.target.value)} className="bg-card border-border text-xs" />
						</div>
					</div>
				)}

				{buildType === 'HEROKU_BUILDPACKS' && (
					<div className="flex flex-col gap-1.5">
						<span className="text-xs font-bold text-muted-foreground">Heroku Version / Stack (Optional)</span>
						<Input placeholder="Default: 24" value={herokuVersion} onChange={e => setHerokuVersion(e.target.value)} className="bg-card border-border text-xs" />
					</div>
				)}

				{buildType === 'NIXPACKS' && (
					<div className="flex flex-col gap-1.5">
						<span className="text-xs font-bold text-muted-foreground">Publish Directory (Optional)</span>
						<Input placeholder="e.g. dist" value={publishDirectory} onChange={e => setPublishDirectory(e.target.value)} className="bg-card border-border text-xs" />
					</div>
				)}

				{buildType === 'STATIC' && (
					<div className="flex items-center gap-3 bg-muted/20 border border-border/40 p-3 rounded-lg">
						<button
							type="button"
							onClick={() => setIsStaticSpa(!isStaticSpa)}
							className={`relative inline-flex h-4.5 w-8 shrink-0 cursor-pointer items-center rounded-full transition-colors ${isStaticSpa ? 'bg-primary' : 'bg-muted'}`}
						>
							<span className={`pointer-events-none block w-3 h-3 rounded-full bg-background shadow-lg transition-transform ${isStaticSpa ? 'translate-x-4.5' : 'translate-x-0.5'}`} />
						</button>
						<div className="flex flex-col gap-0.5">
							<span className="text-xs font-bold text-foreground">Single Page Application (SPA)</span>
							<span className="text-[10px] text-muted-foreground">Redirect all requests to index.html for clientside routers</span>
						</div>
					</div>
				)}

				{buildType === 'RAILPACK' && (
					<div className="flex flex-col gap-3">
						<span className="text-xs font-bold text-muted-foreground">Railpack Version</span>
						{isManualRailpack ? (
							<div className="flex flex-col gap-2">
								<Input placeholder="Enter custom version (e.g. 0.15.4)" value={railpackVersion} onChange={e => setRailpackVersion(e.target.value)} className="bg-card border-border text-xs font-mono" />
								<Button type="button" variant="outline" size="sm" onClick={() => { setIsManualRailpack(false); setRailpackVersion('0.15.4'); }} className="text-[10px] h-7 w-fit border-border font-semibold">
									Use predefined versions
								</Button>
							</div>
						) : (
							<div className="flex flex-col gap-2">
								<Select
									value={railpackVersion}
									onValueChange={(val) => {
										if (val === 'manual') {
											setIsManualRailpack(true);
											setRailpackVersion('');
										} else {
											setRailpackVersion(val);
										}
									}}
								>
									<SelectTrigger className="w-full bg-card border-border text-xs h-9">
										<SelectValue placeholder="Select Railpack version" />
									</SelectTrigger>
									<SelectContent className="bg-popover border border-border">
										<SelectItem value="manual" className="text-xs cursor-pointer hover:bg-muted font-bold text-primary">
											✏️ Custom Version (Manual)
										</SelectItem>
										{RAILPACK_VERSIONS.map(v => (
											<SelectItem key={v} value={v} className="text-xs cursor-pointer hover:bg-muted font-mono">
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

			<div className="flex justify-end mt-2 border-t border-border/20 pt-4">
				<Button onClick={handleSaveBuildType} disabled={savingBuild} className="bg-primary hover:bg-primary/95 text-primary-foreground font-semibold flex items-center gap-1.5 h-9 rounded-lg text-xs">
					<Save className="w-3.5 h-3.5" /> {savingBuild ? 'Saving...' : 'Save Build Settings'}
				</Button>
			</div>
		</section>
	);
}
