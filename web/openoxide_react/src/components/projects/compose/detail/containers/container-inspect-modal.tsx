import {useState} from 'react';
import {X, Check, Copy} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {toast} from 'sonner';
import {
	ContainerInspectBody,
	inspectIcon,
	type InspectType,
} from './container-inspect-body';

export interface ContainerItem {
	id: string;
	name: string;
	service: string;
	image: string;
	status: 'running' | 'stopped' | 'restarting';
	statusText: string;
	ports: string;
	networks: string[];
	mounts: {source: string; destination: string; mode: string}[];
	env: Record<string, string>;
}

interface ContainerInspectModalProps {
	activeModal: {
		type: InspectType;
		container: ContainerItem;
	} | null;
	onClose: () => void;
	logsStream: string[];
}

export function ContainerInspectModal({
	activeModal,
	onClose,
	logsStream,
}: ContainerInspectModalProps) {
	const [copied, setCopied] = useState(false);

	if (!activeModal) return null;

	const handleCopy = (text: string) => {
		navigator.clipboard.writeText(text);
		setCopied(true);
		toast.success('Copied to clipboard');
		setTimeout(() => setCopied(false), 2000);
	};

	return (
		<div className="fixed inset-0 z-50 flex items-center justify-center bg-black/60 p-4 backdrop-blur-sm">
			<div className="flex max-h-[85vh] w-full max-w-3xl animate-in flex-col overflow-hidden rounded-xl bg-card shadow-lg duration-150 fade-in">
				{/* Modal Header */}
				<div className="border- flex items-center justify-between border-b bg-muted/30 p-4">
					<div className="flex items-center gap-2">
						{inspectIcon(activeModal.type)}
						<h3 className="text-xs font-bold text-foreground capitalize">
							Container {activeModal.type} — {activeModal.container.name}
						</h3>
					</div>
					<div className="flex items-center gap-2">
						<Button
							variant="outline"
							size="sm"
							onClick={() =>
								handleCopy(JSON.stringify(activeModal.container, null, 2))
							}
							className="flex h-7 items-center gap-1  px-2 text-xs font-semibold">
							{copied ? (
								<Check className="h-3 w-3 text-emerald-500" />
							) : (
								<Copy className="h-3 w-3" />
							)}
							{copied ? 'Copied' : 'Copy Data'}
						</Button>
						<Button
							variant="ghost"
							size="icon"
							onClick={onClose}
							className="h-7 w-7 text-muted-foreground">
							<X className="h-4 w-4" />
						</Button>
					</div>
				</div>

				{/* Modal Body Content */}
				<div className="overflow-y-auto p-4">
					<ContainerInspectBody
						type={activeModal.type}
						container={activeModal.container}
						logsStream={logsStream}
					/>
				</div>
			</div>
		</div>
	);
}
