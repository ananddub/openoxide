import {useState, useEffect} from 'react';
import {toast} from 'sonner';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {
	Dialog,
	DialogContent,
	DialogDescription,
	DialogHeader,
	DialogTitle,
} from '#/components/ui/dialog';
import {Select, SelectContent, SelectItem, SelectTrigger, SelectValue} from '#/components/ui/select';
import {$api} from '#/api/query';
import {formatApiError} from '#/api/utils';
import {Server} from 'lucide-react';

interface CreateComposeDialogProps {
	isOpen: boolean;
	onClose: () => void;
	environmentId: number;
	onCreated: (compose: any) => void;
}

export function CreateComposeDialog({
	isOpen,
	onClose,
	environmentId,
	onCreated,
}: CreateComposeDialogProps) {
	const [name, setName] = useState('');
	const [composeType, setComposeType] = useState('DOCKER-COMPOSE');
	const [description, setDescription] = useState('');
	const [serverId, setServerId] = useState<string>('local');
	const [isSubmitting, setIsSubmitting] = useState(false);

	const createMutation = $api.useMutation('post', '/compose');
	const {data: rawServers = []} = $api.useQuery('get', '/remote-servers') as any;
	const serversList = Array.isArray(rawServers) ? rawServers : [];

	useEffect(() => {
		if (isOpen) {
			setName('');
			setComposeType('DOCKER-COMPOSE');
			setDescription('');
			setServerId('local');
		}
	}, [isOpen]);

	const handleSubmit = async (e: React.FormEvent) => {
		e.preventDefault();
		if (!name.trim()) {
			toast.error('Please specify stack name');
			return;
		}

		setIsSubmitting(true);
		try {
			const res = await createMutation.mutateAsync({
				body: {
					name: name.trim(),
					description: description.trim() || undefined,
					environment_id: environmentId,
					compose_file: '',
					compose_type: composeType,
					source_type: 'RAW',
					server_id: serverId && serverId !== 'local' ? Number(serverId) : undefined,
				},
			});
			toast.success('Compose stack created successfully');
			onCreated(res);
			onClose();
		} catch (err: any) {
			toast.error(formatApiError(err));
		} finally {
			setIsSubmitting(false);
		}
	};

	return (
		<Dialog open={isOpen} onOpenChange={open => !open && onClose()}>
			<DialogContent className="sm:max-w-md bg-card border border-border shadow-2xl p-6 flex flex-col gap-5 rounded-2xl">
				<DialogHeader className="space-y-1">
					<DialogTitle className="text-lg font-bold tracking-tight text-foreground">
						Create Compose Service
					</DialogTitle>
					<DialogDescription className="text-xs text-muted-foreground">
						Add a Docker Compose stack or Docker Swarm service under this environment.
					</DialogDescription>
				</DialogHeader>

				<form onSubmit={handleSubmit} className="flex flex-col gap-4">
					<div className="flex flex-col gap-4">
						{/* Name */}
						<div className="flex flex-col gap-1.5">
							<label className="text-[11px] font-bold text-muted-foreground uppercase tracking-wider">
								Name <span className="text-destructive">*</span>
							</label>
							<Input
								placeholder="e.g. redis-stack"
								value={name}
								onChange={e => setName(e.target.value)}
								required
								className="h-9 rounded-lg border border-border/80 bg-muted/20 px-3 text-xs shadow-inner focus:outline-none"
							/>
						</div>

						{/* Compose Type */}
						<div className="flex flex-col gap-1.5">
							<label className="text-[11px] font-bold text-muted-foreground uppercase tracking-wider">
								Compose Type <span className="text-destructive">*</span>
							</label>
							<Select value={composeType} onValueChange={val => setComposeType(val ?? 'DOCKER-COMPOSE')}>
								<SelectTrigger className="w-full h-9 text-xs rounded-lg border border-border/80 bg-muted/20 focus-visible:ring-0">
									<SelectValue placeholder="Select type" />
								</SelectTrigger>
								<SelectContent className="bg-card border-border">
									<SelectItem value="DOCKER-COMPOSE" className="text-xs">Docker Compose</SelectItem>
									<SelectItem value="STACK" className="text-xs">Stack (Docker Swarm)</SelectItem>
								</SelectContent>
							</Select>
						</div>

						{/* Target Server Selection */}
						<div className="flex flex-col gap-1.5">
							<label className="text-[11px] font-bold text-muted-foreground uppercase tracking-wider flex items-center gap-1">
								<Server className="size-3 text-muted-foreground" /> Target Server
							</label>
							<Select value={serverId} onValueChange={val => setServerId(val ?? 'local')}>
								<SelectTrigger className="!h-9 text-xs w-full"><SelectValue /></SelectTrigger>
								<SelectContent className="bg-card border-border">
									<SelectItem value="local" className="text-xs">Local Docker Engine</SelectItem>
									{serversList.map((srv: any) => (
										<SelectItem key={srv.id} value={String(srv.id)} className="text-xs">
											{srv.name || `Server #${srv.id}`}
										</SelectItem>
									))}
								</SelectContent>
							</Select>
						</div>

						{/* Description */}
						<div className="flex flex-col gap-1.5">
							<label className="text-[11px] font-bold text-muted-foreground uppercase tracking-wider">
								Description
							</label>
							<textarea
								rows={3}
								placeholder="Optional brief details about this stack..."
								value={description}
								onChange={e => setDescription(e.target.value)}
								className="flex w-full rounded-lg border border-input bg-transparent dark:bg-input/30 px-3 py-2 text-xs text-foreground placeholder:text-muted-foreground focus-visible:border-ring focus-visible:ring-2 focus-visible:ring-ring/50 outline-none resize-none leading-relaxed"
							/>
						</div>
					</div>

					{/* Modal Actions */}
					<div className="flex justify-end pt-3 border-t border-border/30">
						<Button
							type="submit"
							disabled={isSubmitting}
							className="bg-primary hover:bg-primary/95 text-primary-foreground text-xs h-9 px-5 font-semibold shadow-lg shadow-primary/10 rounded-lg">
							{isSubmitting ? 'Creating...' : 'Create Compose'}
						</Button>
					</div>
				</form>
			</DialogContent>
		</Dialog>
	);
}
