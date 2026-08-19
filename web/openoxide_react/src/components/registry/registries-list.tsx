import {useState} from 'react';
import {Button} from '#/components/ui/button';
import {Card, CardContent} from '#/components/ui/card';
import {
	DropdownMenu,
	DropdownMenuContent,
	DropdownMenuItem,
	DropdownMenuSeparator,
	DropdownMenuTrigger,
} from '#/components/ui/dropdown';
import {$api} from '#/api/query';
import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';
import {
	Database,
	Plug,
	RefreshCw,
	Check,
	X,
	MoreVertical,
} from 'lucide-react';

import type {RegistryResponse} from '#/types/api-helpers';

interface RegistriesListProps {
	registries: RegistryResponse[];
	isLoading: boolean;
	onEdit: (item: RegistryResponse) => void;
	onDelete: (id: number) => void;
	onRefresh: () => void;
}

export function RegistriesList({
	registries,
	isLoading,
	onEdit,
	onDelete,
	onRefresh,
}: RegistriesListProps) {
	const [testingMap, setTestingMap] = useState<
		Record<string, 'testing' | 'success' | 'failed' | undefined>
	>({});

	const testMutation = $api.useMutation('post', '/registries/{id}/test');
	const patchMutation = $api.useMutation('patch', '/registries/{id}');

	const handleTestConnection = async (id: number | string) => {
		const key = String(id);
		setTestingMap(prev => ({...prev, [key]: 'testing'}));
		try {
			await testMutation.mutateAsync({params: {path: {id: Number(id)}}});
			setTestingMap(prev => ({...prev, [key]: 'success'}));
			toast.success('Registry connection test passed');
			setTimeout(() => {
				setTestingMap(prev => ({...prev, [key]: undefined}));
			}, 3000);
		} catch (err: unknown) {
			setTestingMap(prev => ({...prev, [key]: 'failed'}));
			toast.error(formatApiError(err));
		}
	};

	const handleSetDefault = async (id: number | string) => {
		try {
			await patchMutation.mutateAsync({
				params: {path: {id: Number(id)}},
				body: {registry_name: undefined},
			});
			toast.success('Set as default registry');
			onRefresh();
		} catch (err: unknown) {
			toast.error(formatApiError(err));
		}
	};

	if (isLoading) {
		return (
			<div className="grid grid-cols-1 gap-3 py-3 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4">
				{[1, 2, 3, 4].map(i => (
					<div
						key={i}
						className="h-16 animate-pulse rounded-xl border border-border/60 bg-muted/40"
					/>
				))}
			</div>
		);
	}

	if (!registries || registries.length === 0) {
		return (
			<Card className="my-4 flex flex-col items-center justify-center rounded-xl border-border bg-card p-8 text-center shadow-sm">
				<Database className="mb-2 h-6 w-6 text-muted-foreground" />
				<h3 className="text-xs font-bold text-foreground">
					No Registries Found
				</h3>
				<p className="mt-1 text-xs text-muted-foreground">
					Add a Docker registry to pull or push container images.
				</p>
			</Card>
		);
	}

	return (
		<div className="grid w-full grid-cols-1 gap-3 py-3 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4">
			{registries.map(rawItem => {
				const item = rawItem as any;
				const status = testingMap[String(item.id)];
				const isTesting = status === 'testing';

				const getDotColor = () => {
					if (isTesting)
						return 'bg-amber-400 animate-pulse shadow-sm shadow-amber-400/50';
					if (status === 'success')
						return 'bg-emerald-500 shadow-sm shadow-emerald-500/50';
					if (status === 'failed')
						return 'bg-rose-500 shadow-sm shadow-rose-500/50';
					return 'bg-zinc-400/80';
				};

				return (
					<Card
						key={item.id}
						className="rounded-xl border-border bg-card shadow-sm transition-all hover:border-border/80">
						<CardContent className="flex items-center justify-between gap-2 p-3">
							{/* Left: Status Dot, Registry Name & URL */}
							<div className="flex min-w-0 flex-1 items-center gap-2">
								<span
									className={`block h-2.5 w-2.5 shrink-0 rounded-full ${getDotColor()}`}
								/>

								<div className="flex min-w-0 flex-1 flex-col">
									<div className="flex min-w-0 items-center gap-1.5">
										<h4 className="truncate text-xs font-bold text-foreground">
											{item.name || item.registry_name}
										</h4>
										{item.is_default && (
											<span className="shrink-0 rounded border border-primary/20 bg-primary/10 px-1.5 py-0.5 text-[9px] font-semibold text-primary">
												Default
											</span>
										)}
									</div>
									<p className="mt-0.5 truncate font-mono text-[11px] text-muted-foreground">
										{item.registry_url || 'docker.io'}
									</p>
								</div>
							</div>

							{/* Right: Test Plug Icon Button & Clean Text 3-Dots Dropdown */}
							<div className="flex shrink-0 items-center gap-1">
								<Button
									variant="ghost"
									size="icon"
									disabled={isTesting}
									onClick={() => handleTestConnection(item.id)}
									title="Test Registry Connection"
									className="h-7 w-7 text-muted-foreground hover:text-foreground">
									{isTesting ? (
										<RefreshCw className="h-3.5 w-3.5 animate-spin text-amber-500" />
									) : status === 'success' ? (
										<Check className="h-3.5 w-3.5 text-emerald-500" />
									) : status === 'failed' ? (
										<X className="h-3.5 w-3.5 text-rose-500" />
									) : (
										<Plug className="h-3.5 w-3.5" />
									)}
								</Button>

								<DropdownMenu>
									<DropdownMenuTrigger
										render={
											<Button
												variant="ghost"
												size="icon"
												className="h-7 w-7 shrink-0 text-muted-foreground hover:text-foreground"
											/>
										}>
										<MoreVertical className="h-4 w-4" />
									</DropdownMenuTrigger>
									<DropdownMenuContent
										align="end"
										className="z-50 w-40 rounded-xl border-border bg-card p-1 text-xs shadow-xl">
										<DropdownMenuItem
											className="flex cursor-pointer items-center rounded-lg px-2.5 py-1.5 text-xs font-medium hover:bg-muted"
											onClick={() => onEdit(rawItem)}>
											Edit Registry
										</DropdownMenuItem>

										{!item.is_default && (
											<DropdownMenuItem
												className="flex cursor-pointer items-center rounded-lg px-2.5 py-1.5 text-xs font-medium hover:bg-muted"
												onClick={() => handleSetDefault(item.id)}>
												Set as Default
											</DropdownMenuItem>
										)}

										<DropdownMenuSeparator className="my-1 border-border/50" />

										<DropdownMenuItem
											className="flex cursor-pointer items-center rounded-lg px-2.5 py-1.5 text-xs font-medium text-rose-500 hover:bg-muted/80"
											onClick={() => onDelete(Number(item.id))}>
											Delete Registry
										</DropdownMenuItem>
									</DropdownMenuContent>
								</DropdownMenu>
							</div>
						</CardContent>
					</Card>
				);
			})}
		</div>
	);
}
