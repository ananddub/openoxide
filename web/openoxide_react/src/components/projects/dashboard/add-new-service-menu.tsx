import {useEffect, useRef, useState} from 'react';
import {Plus, ChevronDown} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {
	DropdownMenu,
	DropdownMenuContent,
	DropdownMenuItem,
	DropdownMenuTrigger,
} from '#/components/ui/dropdown';
import {Badge} from '#/components/ui/badge';
import {
	OPTIONS,
	type ServiceKind,
} from '#/components/projects/create/constants';
import {CreateServiceSheet} from '#/components/projects/create/create-service-sheet';
import {cn} from '#/api/utils';

// ─── Vercel-style "Add New" dropdown: opens a right side-panel per type ──────

export function AddNewServiceMenu({
	projectId,
	envId,
	onCreated,
}: {
	projectId: number;
	envId?: number;
	onCreated?: () => void;
}) {
	const [sheetKind, setSheetKind] = useState<ServiceKind | null>(null);
	// Kept separate from kind: closing only flips `open` so the sheet can
	// play its exit animation instead of unmounting mid-slide. Opening is
	// deferred one paint so base-ui sees a real closed→open transition
	// (a Root mounted with open=true renders without the enter animation).
	const [sheetOpen, setSheetOpen] = useState(false);
	// Bumped on every open: remounts the sheet with fresh form state.
	const [session, setSession] = useState(0);
	const disabled = !envId;

	// Auto-open only for a fresh session (an explicit "Add New" click) —
	// never right after a close, which would fight the exit animation and
	// leave the modal overlay trapping the page.
	const openedSession = useRef(0);

	useEffect(() => {
		if (!sheetKind || sheetOpen || openedSession.current === session)
			return;
		openedSession.current = session;
		let inner = 0;
		const outer = requestAnimationFrame(() => {
			inner = requestAnimationFrame(() => setSheetOpen(true));
		});
		return () => {
			cancelAnimationFrame(outer);
			cancelAnimationFrame(inner);
		};
	}, [sheetKind, sheetOpen, session]);

	const openSheet = (kind: ServiceKind) => {
		setSheetKind(kind);
		setSheetOpen(false);
		setSession(s => s + 1);
	};

	return (
		<>
			<DropdownMenu>
				<DropdownMenuTrigger
					disabled={disabled}
					render={
						<Button className="h-9 gap-1.5 px-3 text-sm font-medium">
							<Plus className="size-3.5" />
							Add New
							<ChevronDown className="size-3.5 opacity-60" />
						</Button>
					} />
				<DropdownMenuContent align="end" className="min-w-[340px] p-1.5">
					<div className="grid grid-cols-2 gap-1">
						{OPTIONS.map(o => (
							<DropdownMenuItem
								key={o.id}
								disabled={o.soon}
								onClick={() => openSheet(o.id)}
								className={cn(
									'flex-col items-start gap-0.5 rounded-lg px-2.5 py-2',
									o.soon && 'opacity-60',
								)}>
								<span className="flex items-center gap-1.5">
									<o.icon className="size-4 text-muted-foreground" />
									<span className="text-[13px] font-medium text-foreground">
										{o.title}
									</span>
									{o.soon && (
										<Badge
											variant="secondary"
											className="px-1 py-0 text-[9px] font-semibold">
											SOON
										</Badge>
									)}
								</span>
								<span className="ps-[22px] text-[11px] leading-snug text-muted-foreground">
									{o.description}
								</span>
							</DropdownMenuItem>
						))}
					</div>
				</DropdownMenuContent>
			</DropdownMenu>

			{sheetKind && (
				<CreateServiceSheet
					key={`${sheetKind}-${session}`}
					open={sheetOpen}
					projectId={projectId}
					envId={envId}
					kind={sheetKind}
					onClose={() => setSheetOpen(false)}
					onCreated={onCreated ?? (() => {})}
				/>
			)}
		</>
	);
}
