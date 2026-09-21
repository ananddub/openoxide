import {ToggleSwitch} from '#/components/shared/toggle-switch';
import {LabeledInput} from '../source-settings-card/labeled-input';
import type {BuildFields} from './build-types';

const MUTED = 'text-muted-foreground';

/** Dynamic per-build-type fields: Dockerfile / Heroku / Nixpacks / Static. */
export function BuildTypeFields({
	buildType,
	fields,
	isStaticSpa,
	onStaticSpaChange,
	onPatch,
}: {
	buildType: string;
	fields: BuildFields;
	isStaticSpa: boolean;
	onStaticSpaChange: (next: boolean) => void;
	onPatch: (patch: Partial<BuildFields>) => void;
}) {
	if (buildType === 'DOCKERFILE') {
		return (
			<div className="grid grid-cols-1 gap-4 sm:grid-cols-2">
				<LabeledInput
					label="Dockerfile Path"
					labelClassName={MUTED}
					placeholder="Dockerfile"
					mono
					inputClassName=""
					value={fields.dockerfile}
					onValueChange={v => onPatch({dockerfile: v})}
				/>
				<LabeledInput
					label="Docker Context Path"
					labelClassName={MUTED}
					placeholder="."
					mono
					inputClassName=""
					value={fields.dockerContextPath}
					onValueChange={v => onPatch({dockerContextPath: v})}
				/>
				<LabeledInput
					label="Docker Build Stage (Optional)"
					labelClassName={MUTED}
					placeholder="E.g. production"
					inputClassName=""
					className="sm:col-span-2"
					value={fields.dockerBuildStage}
					onValueChange={v => onPatch({dockerBuildStage: v})}
				/>
			</div>
		);
	}

	if (buildType === 'HEROKU') {
		return (
			<LabeledInput
				label="Heroku Version / Stack (Optional)"
				labelClassName={MUTED}
				placeholder="Default: 24"
				inputClassName=""
				value={fields.herokuVersion}
				onValueChange={v => onPatch({herokuVersion: v})}
			/>
		);
	}

	if (buildType === 'NIXPACKS') {
		return (
			<LabeledInput
				label="Publish Directory (Optional)"
				labelClassName={MUTED}
				placeholder="e.g. dist"
				inputClassName=""
				value={fields.publishDirectory}
				onValueChange={v => onPatch({publishDirectory: v})}
			/>
		);
	}

	if (buildType === 'STATIC') {
		return (
			<div className="flex items-center gap-3 rounded-lg bg-muted/20 p-3">
				<ToggleSwitch
					checked={isStaticSpa}
					onCheckedChange={onStaticSpaChange}
				/>
				<div className="flex flex-col gap-0.5">
					<span className="text-xs font-bold text-foreground">
						Single Page Application (SPA)
					</span>
					<span className="text-[10px] text-muted-foreground">
						Redirect all requests to index.html for clientside routers
					</span>
				</div>
			</div>
		);
	}

	return null;
}
