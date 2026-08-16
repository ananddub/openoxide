import React, {useState, useEffect} from 'react';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {Textarea} from '#/components/ui/textarea';
import {
	Dialog,
	DialogContent,
	DialogHeader,
	DialogTitle,
	DialogFooter,
} from '#/components/ui/dialog';
import {Select, SelectContent, SelectItem, SelectTrigger, SelectValue} from '#/components/ui/select';
import {$api} from '#/api/query';
import {useAppStore} from '#/stores/app-store';
import {formatApiError} from '#/api/utils';
import {Server} from 'lucide-react';

interface CreateComposeDialogProps {
	projectId: number;
	environmentId: number;
	isOpen: boolean;
	onOpenChange: (open: boolean) => void;
	onCreated?: () => void;
}

export function CreateComposeDialog({
	projectId,
	environmentId,
	isOpen,
	onOpenChange,
	onCreated,
}: CreateComposeDialogProps) {
	const [name, setName] = useState('');
	const [composeType, setComposeType] = useState('DOCKER-COMPOSE');
	const [description, setDescription] = useState('');
	const [serverId, setServerId] = useState<string>('local');
	const [isSubmitting, setIsSubmitting] = useState(false);

	const createMutation = $api.useMutation('post', '/compose');
	const serversList = useAppStore((state) => state.servers || []);

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
		if (!name.trim()) return;

		setIsSubmitting(true);
		try {
			await createMutation.mutateAsync({
				body: {
					name: name.trim(),
					compose_type: composeType,
					description: description.trim() || undefined,
					environment_id: environmentId,
					server_id: serverId === 'local' ? undefined : Number(serverId),
				},
			});
			onCreated?.();
			onOpenChange(false);
		} catch (err) {
			alert(formatApiError(err));
		} finally {
			setIsSubmitting(false);
		}
	};

	return (
		<Dialog open={isOpen} onOpenChange={onOpenChange}>
			<DialogContent className="sm:max-w-[440px]">
				<form onSubmit={handleSubmit}>
					<DialogHeader>
						<DialogTitle>Create Docker Compose Stack</DialogTitle>
					</DialogHeader>

					<div className="flex flex-col gap-4 py-4">
						<div className="flex flex-col gap-1.5">
							<label className="text-xs font-semibold text-foreground">Stack Name</label>
							<Input
								placeholder="my-compose-stack"
								value={name}
								onChange={(e) => setName(e.target.value)}
								autoFocus
							/>
						</div>

						<div className="flex flex-col gap-1.5">
							<label className="text-xs font-semibold text-foreground">Compose Type</label>
							<Select value={composeType} onValueChange={setComposeType}>
								<SelectTrigger className="w-full">
									<SelectValue placeholder="Select type" />
								</SelectTrigger>
								<SelectContent>
									<SelectItem value="DOCKER-COMPOSE">Docker Compose (YAML)</SelectItem>
									<SelectItem value="DOCKER-STACK">Docker Swarm Stack</SelectItem>
								</SelectContent>
							</Select>
						</div>

						<div className="flex flex-col gap-1.5">
							<label className="text-xs font-semibold text-foreground">Target Server</label>
							<Select value={serverId} onValueChange={setServerId}>
								<SelectTrigger className="w-full">
									<SelectValue placeholder="Select server">
										{(() => {
											if (serverId === 'local') return 'Localhost (Default)';
											const found = serversList.find((s: any) => String(s.id) === String(serverId));
											return found ? `${found.name} (${found.ip_address || found.ip || ''})` : serverId;
										})()}
									</SelectValue>
								</SelectTrigger>
								<SelectContent>
									<SelectItem value="local">Localhost (Default)</SelectItem>
									{serversList.map((srv: any) => (
										<SelectItem key={srv.id} value={String(srv.id)}>
											<div className="flex items-center gap-2">
												<Server className="size-3.5 text-muted-foreground" />
												<span>{srv.name} ({srv.ip_address || srv.ip || ''})</span>
											</div>
										</SelectItem>
									))}
								</SelectContent>
							</Select>
						</div>

						<div className="flex flex-col gap-1.5">
							<label className="text-xs font-semibold text-foreground">Description (Optional)</label>
							<Textarea
								placeholder="Brief description of this compose stack..."
								value={description}
								onChange={(e) => setDescription(e.target.value)}
								rows={2}
							/>
						</div>
					</div>

					<DialogFooter>
						<Button
							type="button"
							variant="outline"
							onClick={() => onOpenChange(false)}
							disabled={isSubmitting}
						>
							Cancel
						</Button>
						<Button type="submit" disabled={!name.trim() || isSubmitting}>
							{isSubmitting ? 'Creating...' : 'Create Stack'}
						</Button>
					</DialogFooter>
				</form>
			</DialogContent>
		</Dialog>
	);
}
