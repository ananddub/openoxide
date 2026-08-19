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
	targetNode?: {
		id: number;
		name: string;
		type: string;
		dbType?: string;
		port?: number;
	};
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
	const port =
		targetNode?.port ||
		(dbType === 'redis' ? 6379 : dbType === 'mongodb' ? 27017 : 5432);

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
				{
					key: `${prefix}_HOST`,
					value: dbName.toLowerCase(),
					checked: true,
				},
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
				{
					key: `${prefix}_HOST`,
					value: dbName.toLowerCase(),
					checked: true,
				},
				{key: `${prefix}_PORT`, value: String(port), checked: true},
			];
		}
		return [
			{
				key: `${prefix}_URL`,
				value: `${dbType}://${dbName.toLowerCase()}:${port}`,
				checked: true,
			},
			{key: `${prefix}_HOST`, value: dbName.toLowerCase(), checked: true},
			{key: `${prefix}_PORT`, value: String(port), checked: true},
		];
	})();

	const [envs, setEnvs] = useState(defaultTemplates);

	const toggleCheck = (idx: number) => {
		setEnvs(prev =>
			prev.map((item, i) =>
				i === idx ? {...item, checked: !item.checked} : item,
			),
		);
	};

	const handleApply = async () => {
		setIsSubmitting(true);
		try {
			const selected: Record<string, string> = {};
			envs
				.filter(e => e.checked)
				.forEach(e => {
					selected[e.key] = e.value;
				});

			await onConfirmLink(selected);
			toast.success(
				`Successfully linked "${sourceNode?.name}" to "${targetNode?.name}"!`,
			);
			onClose();
		} catch {
			toast.error('Failed to inject environment variables');
		} finally {
			setIsSubmitting(false);
		}
	};

	return (
		<Dialog open={isOpen} onOpenChange={open => !open && onClose()}>
			<DialogContent className="border-border bg-card shadow-xl sm:max-w-md">
				<DialogHeader>
					<div className="mb-1 flex items-center gap-2 text-xs font-bold tracking-wider text-primary uppercase">
						<Zap className="h-4 w-4 text-amber-500" />
						Visual Railway Linker
					</div>
					<DialogTitle className="text-base font-bold text-foreground">
						Connect Service &amp; Inject Variables
					</DialogTitle>
					<DialogDescription className="text-xs text-muted-foreground">
						Inject automatic environment variables into target application
						when connected visually.
					</DialogDescription>
				</DialogHeader>

				{/* Visual Connection Badge Row */}
				<div className="my-2 flex items-center justify-between gap-3 rounded-xl border border-border/40 bg-muted/30 p-3">
					<div className="flex min-w-0 items-center gap-2">
						<div className="flex h-7 w-7 shrink-0 items-center justify-center rounded-lg border border-primary/20 bg-primary/10">
							<Box className="h-3.5 w-3.5 text-primary" />
						</div>
						<span className="truncate text-xs font-bold text-foreground">
							{sourceNode?.name || 'Application'}
						</span>
					</div>

					<ArrowRight className="h-4 w-4 shrink-0 text-muted-foreground" />

					<div className="flex min-w-0 items-center gap-2">
						<div className="flex h-7 w-7 shrink-0 items-center justify-center rounded-lg border border-emerald-500/20 bg-emerald-500/10">
							<Database className="h-3.5 w-3.5 text-emerald-500" />
						</div>
						<span className="truncate text-xs font-bold text-foreground">
							{targetNode?.name || 'Database'}
						</span>
					</div>
				</div>

				{/* Environment Variables Selection List */}
				<div className="my-2 flex flex-col gap-2">
					<span className="flex items-center gap-1.5 text-xs font-semibold text-foreground">
						<ShieldCheck className="h-3.5 w-3.5 text-emerald-500" />
						Auto-Generated Environment Variables:
					</span>

					<div className="divide-y divide-border/60 rounded-lg border border-border bg-muted/20">
						{envs.map((env, idx) => (
							<div
								key={env.key}
								className="flex items-center gap-3 p-2.5 font-mono text-xs transition-colors hover:bg-accent/30">
								<Checkbox
									id={`env-${idx}`}
									checked={env.checked}
									onCheckedChange={() => toggleCheck(idx)}
								/>
								<div className="min-w-0 flex-1">
									<label
										htmlFor={`env-${idx}`}
										className="block cursor-pointer truncate font-bold text-foreground">
										{env.key}
									</label>
									<span className="block truncate text-[11px] text-muted-foreground">
										{env.value}
									</span>
								</div>
							</div>
						))}
					</div>
				</div>

				<DialogFooter className="mt-3 gap-2 sm:gap-0">
					<Button
						variant="outline"
						size="sm"
						onClick={onClose}
						className="h-8 cursor-pointer text-xs">
						Cancel
					</Button>
					<Button
						size="sm"
						onClick={handleApply}
						disabled={isSubmitting}
						className="h-8 cursor-pointer gap-1.5 text-xs font-bold">
						<Zap className="h-3.5 w-3.5 fill-amber-400 text-amber-400" />
						<span>Inject &amp; Connect</span>
					</Button>
				</DialogFooter>
			</DialogContent>
		</Dialog>
	);
}
