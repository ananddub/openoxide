import {useState} from 'react';
import {
	Dialog,
	DialogContent,
	DialogDescription,
	DialogFooter,
	DialogHeader,
	DialogTitle,
} from '#/components/ui/dialog';
import {Button} from '#/components/ui/button';

import {Checkbox} from '#/components/ui/checkbox';
import {Box, Database, ArrowRight, ShieldCheck, Zap} from 'lucide-react';
import {toast} from 'sonner';

interface ServiceLinkerModalProps {
	isOpen: boolean;
	onClose: () => void;
	sourceNode?: {id: number; name: string; type: string};
	targetNode?: {id: number; name: string; type: string; dbType?: string; port?: number};
	onConfirmLink: (injectedEnvs: Record<string, string>) => Promise<void>;
}

export function ServiceLinkerModal({
	isOpen,
	onClose,
	sourceNode,
	targetNode,
	onConfirmLink,
}: ServiceLinkerModalProps) {
	const [isSubmitting, setIsSubmitting] = useState(false);

	const dbName = targetNode?.name || 'database';
	const dbType = (targetNode?.dbType || 'postgres').toLowerCase();
	const port = targetNode?.port || (dbType === 'redis' ? 6379 : dbType === 'mongodb' ? 27017 : 5432);

	// Propose template environment variables based on connected database type
	const defaultTemplates = (() => {
		const prefix = dbName.toUpperCase().replace(/[^A-Z0-9]/g, '_');
		if (dbType.includes('postgres')) {
			return [
				{
					key: 'DATABASE_URL',
					value: `postgres://postgres:password@${dbName.toLowerCase()}:${port}/${dbName.toLowerCase()}`,
					checked: true,
				},
				{key: `${prefix}_HOST`, value: dbName.toLowerCase(), checked: true},
				{key: `${prefix}_PORT`, value: String(port), checked: true},
			];
		}
		if (dbType.includes('redis')) {
			return [
				{
					key: 'REDIS_URL',
					value: `redis://${dbName.toLowerCase()}:${port}`,
					checked: true,
				},
				{key: `${prefix}_HOST`, value: dbName.toLowerCase(), checked: true},
				{key: `${prefix}_PORT`, value: String(port), checked: true},
			];
		}
		return [
			{key: `${prefix}_URL`, value: `${dbType}://${dbName.toLowerCase()}:${port}`, checked: true},
			{key: `${prefix}_HOST`, value: dbName.toLowerCase(), checked: true},
			{key: `${prefix}_PORT`, value: String(port), checked: true},
		];
	})();

	const [envs, setEnvs] = useState(defaultTemplates);

	const toggleCheck = (idx: number) => {
		setEnvs(prev =>
			prev.map((item, i) => (i === idx ? {...item, checked: !item.checked} : item)),
		);
	};

	const handleApply = async () => {
		setIsSubmitting(true);
		try {
			const selected: Record<string, string> = {};
			envs.filter(e => e.checked).forEach(e => {
				selected[e.key] = e.value;
			});

			await onConfirmLink(selected);
			toast.success(`Successfully linked "${sourceNode?.name}" to "${targetNode?.name}"!`);
			onClose();
		} catch {
			toast.error('Failed to inject environment variables');
		} finally {
			setIsSubmitting(false);
		}
	};

	return (
		<Dialog open={isOpen} onOpenChange={open => !open && onClose()}>
			<DialogContent className="sm:max-w-md bg-card border-border shadow-xl">
				<DialogHeader>
					<div className="flex items-center gap-2 text-primary font-bold text-xs uppercase tracking-wider mb-1">
						<Zap className="w-4 h-4 text-amber-500" />
						Visual Railway Linker
					</div>
					<DialogTitle className="text-base font-bold text-foreground">
						Connect Service &amp; Inject Variables
					</DialogTitle>
					<DialogDescription className="text-xs text-muted-foreground">
						Inject automatic environment variables into target application when connected visually.
					</DialogDescription>
				</DialogHeader>

				{/* Visual Connection Badge Row */}
				<div className="flex items-center justify-between gap-3 p-3 bg-muted/30 border border-border/40 rounded-xl my-2">
					<div className="flex items-center gap-2 min-w-0">
						<div className="w-7 h-7 rounded-lg bg-primary/10 border border-primary/20 flex items-center justify-center shrink-0">
							<Box className="w-3.5 h-3.5 text-primary" />
						</div>
						<span className="text-xs font-bold text-foreground truncate">
							{sourceNode?.name || 'Application'}
						</span>
					</div>

					<ArrowRight className="w-4 h-4 text-muted-foreground shrink-0" />

					<div className="flex items-center gap-2 min-w-0">
						<div className="w-7 h-7 rounded-lg bg-emerald-500/10 border border-emerald-500/20 flex items-center justify-center shrink-0">
							<Database className="w-3.5 h-3.5 text-emerald-500" />
						</div>
						<span className="text-xs font-bold text-foreground truncate">
							{targetNode?.name || 'Database'}
						</span>
					</div>
				</div>

				{/* Environment Variables Selection List */}
				<div className="flex flex-col gap-2 my-2">
					<span className="text-xs font-semibold text-foreground flex items-center gap-1.5">
						<ShieldCheck className="w-3.5 h-3.5 text-emerald-500" />
						Auto-Generated Environment Variables:
					</span>

					<div className="border border-border rounded-lg divide-y divide-border/60 bg-muted/20">
						{envs.map((env, idx) => (
							<div
								key={env.key}
								className="flex items-center gap-3 p-2.5 text-xs font-mono hover:bg-accent/30 transition-colors"
							>
								<Checkbox
									id={`env-${idx}`}
									checked={env.checked}
									onCheckedChange={() => toggleCheck(idx)}
								/>
								<div className="flex-1 min-w-0">
									<label
										htmlFor={`env-${idx}`}
										className="font-bold text-foreground cursor-pointer block truncate"
									>
										{env.key}
									</label>
									<span className="text-[11px] text-muted-foreground truncate block">
										{env.value}
									</span>
								</div>
							</div>
						))}
					</div>
				</div>

				<DialogFooter className="gap-2 sm:gap-0 mt-3">
					<Button variant="outline" size="sm" onClick={onClose} className="h-8 text-xs cursor-pointer">
						Cancel
					</Button>
					<Button
						size="sm"
						onClick={handleApply}
						disabled={isSubmitting}
						className="h-8 text-xs font-bold gap-1.5 cursor-pointer"
					>
						<Zap className="w-3.5 h-3.5 text-amber-400 fill-amber-400" />
						<span>Inject &amp; Connect</span>
					</Button>
				</DialogFooter>
			</DialogContent>
		</Dialog>
	);
}
