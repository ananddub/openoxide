import {Download} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {downloadKeyFile} from '#/utils/ssh-key-utils';

interface SshKeyDownloadButtonsProps {
	fileName: string;
	publicKey?: string;
	privateKey?: string;
}

/** Download pub / private key buttons (rendered only when present). */
export function SshKeyDownloadButtons({
	fileName,
	publicKey,
	privateKey,
}: SshKeyDownloadButtonsProps) {
	if (!publicKey && !privateKey) return null;
	return (
		<div className="flex w-full items-center gap-2 sm:w-auto">
			{publicKey && (
				<Button
					type="button"
					variant="outline"
					size="sm"
					onClick={() => downloadKeyFile(`${fileName}.pub`, publicKey)}
					className="h-9 gap-1.5 text-xs font-medium">
					<Download className="h-3.5 w-3.5" />
					Download Pub
				</Button>
			)}
			{privateKey && (
				<Button
					type="button"
					variant="outline"
					size="sm"
					onClick={() => downloadKeyFile(`${fileName}.pem`, privateKey)}
					className="h-9 gap-1.5 text-xs font-medium">
					<Download className="h-3.5 w-3.5 text-amber-500" />
					Download Private
				</Button>
			)}
		</div>
	);
}
