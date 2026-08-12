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
	const [testingMap, setTestingMap] = useState<Record<string, 'testing' | 'success' | 'failed' | undefined>>({});

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
			<div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-3 py-3">
				{[1, 2, 3, 4].map(i => (
					<div key={i} className="h-16 bg-muted/40 animate-pulse rounded-xl border border-border/60" />
				))}
			</div>
		);
	}

	if (!registries || registries.length === 0) {
		return (
			<Card className="bg-card border-border shadow-sm p-8 text-center flex flex-col items-center justify-center rounded-xl my-4">
				<Database className="w-6 h-6 text-muted-foreground mb-2" />
				<h3 className="text-xs font-bold text-foreground">No Registries Found</h3>
				<p className="text-xs text-muted-foreground mt-1">Add a Docker registry to pull or push container images.</p>
			</Card>
		);
	}

	return (
		<div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-3 py-3 w-full">
			{registries.map(rawItem => {
				const item = rawItem as any;
				const status = testingMap[String(item.id)];
				const isTesting = status === 'testing';

				const getDotColor = () => {
					if (isTesting) return 'bg-amber-400 animate-pulse shadow-sm shadow-amber-400/50';
					if (status === 'success') return 'bg-emerald-500 shadow-sm shadow-emerald-500/50';
					if (status === 'failed') return 'bg-rose-500 shadow-sm shadow-rose-500/50';
					return 'bg-zinc-400/80';
				};

				return (
					<Card key={item.id} className="bg-card border-border hover:border-border/80 transition-all rounded-xl shadow-sm">
						<CardContent className="p-3 flex items-center justify-between gap-2">
							{/* Left: Status Dot, Registry Name & URL */}
							<div className="flex items-center gap-2 min-w-0 flex-1">
								<span className={`block w-2.5 h-2.5 rounded-full shrink-0 ${getDotColor()}`} />

								<div className="flex flex-col min-w-0 flex-1">
									<div className="flex items-center gap-1.5 min-w-0">
										<h4 className="text-xs font-bold text-foreground truncate">{item.name || item.registry_name}</h4>
										{item.is_default && (
											<span className="text-[9px] font-semibold px-1.5 py-0.5 rounded bg-primary/10 text-primary border border-primary/20 shrink-0">
												Default
											</span>
										)}
									</div>
									<p className="text-[11px] font-mono text-muted-foreground truncate mt-0.5">
										{item.registry_url || 'docker.io'}
									</p>
								</div>
							</div>

							{/* Right: Test Plug Icon Button & Clean Text 3-Dots Dropdown */}
							<div className="flex items-center gap-1 shrink-0">
								<Button
									variant="ghost"
									size="icon"
									disabled={isTesting}
									onClick={() => handleTestConnection(item.id)}
									title="Test Registry Connection"
									className="h-7 w-7 text-muted-foreground hover:text-foreground"
								>
									{isTesting ? (
										<RefreshCw className="w-3.5 h-3.5 animate-spin text-amber-500" />
									) : status === 'success' ? (
										<Check className="w-3.5 h-3.5 text-emerald-500" />
									) : status === 'failed' ? (
										<X className="w-3.5 h-3.5 text-rose-500" />
									) : (
										<Plug className="w-3.5 h-3.5" />
									)}
								</Button>

								<DropdownMenu>
									<DropdownMenuTrigger
										render={
											<Button variant="ghost" size="icon" className="h-7 w-7 text-muted-foreground hover:text-foreground shrink-0" />
										}
									>
										<MoreVertical className="w-4 h-4" />
									</DropdownMenuTrigger>
									<DropdownMenuContent align="end" className="w-40 bg-card border-border shadow-xl rounded-xl p-1 text-xs z-50">
										<DropdownMenuItem
											className="flex cursor-pointer items-center px-2.5 py-1.5 rounded-lg hover:bg-muted text-xs font-medium"
											onClick={() => onEdit(rawItem)}
										>
											Edit Registry
										</DropdownMenuItem>

										{!item.is_default && (
											<DropdownMenuItem
												className="flex cursor-pointer items-center px-2.5 py-1.5 rounded-lg hover:bg-muted text-xs font-medium"
												onClick={() => handleSetDefault(item.id)}
											>
												Set as Default
											</DropdownMenuItem>
										)}

										<DropdownMenuSeparator className="my-1 border-border/50" />

										<DropdownMenuItem
											className="flex cursor-pointer items-center px-2.5 py-1.5 rounded-lg hover:bg-muted/80 text-rose-500 text-xs font-medium"
											onClick={() => onDelete(Number(item.id))}
										>
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
