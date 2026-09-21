import {useState} from 'react';
import {TerminalCard} from '#/components/shared/terminal-card';
import {ComposeDeployCard} from './compose-deploy-card';
import {ComposeSourceCard} from './cards/compose-source-card';
import {useComposeSource} from './use-compose-source';

interface GeneralTabProps {
	compose: any;
	onAction: (
		action: 'deploy' | 'reload' | 'rebuild' | 'start' | 'cancel',
	) => Promise<void>;
	onUpdated: () => void;
}

export function ComposeGeneralTab({compose, onUpdated}: GeneralTabProps) {
	const [showTerminal, setShowTerminal] = useState(false);
	const {source, onPatch, savingSource, handleSaveSource} =
		useComposeSource(compose, onUpdated);

	return (
		<div className="flex flex-col gap-6">
			{/* Deploy Settings Card Component */}
			<ComposeDeployCard
				compose={compose}
				onUpdated={onUpdated}
				onOpenTerminal={() => setShowTerminal(true)}
			/>

			{/* Source Settings Card Component */}
			<ComposeSourceCard
				source={source}
				onPatch={onPatch}
				savingSource={savingSource}
				onSave={handleSaveSource}
			/>

			{/* Interactive terminal for compose containers — inline card */}
			{showTerminal && (
				<TerminalCard
					app={compose}
					enabled={true}
					onClose={() => setShowTerminal(false)}
				/>
			)}
		</div>
	);
}
