import {Input} from '#/components/ui/input';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import {KeyRound, HardDrive, Globe, Server} from 'lucide-react';

interface DestinationFormFieldsProps {
	name: string;
	setName: (v: string) => void;
	provider: string;
	onProviderChange: (v: string) => void;
	bucket: string;
	setBucket: (v: string) => void;
	region: string;
	setRegion: (v: string) => void;
	endpoint: string;
	setEndpoint: (v: string) => void;
	accessKey: string;
	setAccessKey: (v: string) => void;
	secretKey: string;
	setSecretKey: (v: string) => void;
	providers: {id: string; name: string}[];
}

export function DestinationFormFields({
	name,
	setName,
	provider,
	onProviderChange,
	bucket,
	setBucket,
	region,
	setRegion,
	endpoint,
	setEndpoint,
	accessKey,
	setAccessKey,
	secretKey,
	setSecretKey,
	providers,
}: DestinationFormFieldsProps) {
	return (
		<div className="flex flex-col gap-6 py-2">
			{/* Section 1: General Info */}
			<div className="flex flex-col gap-4 bg-muted/20 border border-border/50 rounded-xl p-4">
				<div className="flex items-center gap-2 text-xs font-bold text-foreground">
					<Server className="w-4 h-4 text-primary" />
					<span>General Configuration</span>
				</div>
				<div className="grid grid-cols-1 sm:grid-cols-2 gap-4">
					<div className="flex flex-col gap-2">
						<label className="text-xs font-semibold text-foreground">Destination Name</label>
						<Input
							value={name}
							onChange={e => setName(e.target.value)}
							placeholder="e.g. AWS Production Storage"
							className="h-10 text-xs bg-background border-border/80 rounded-lg px-3.5 focus-visible:ring-1"
						/>
					</div>

					<div className="flex flex-col gap-2">
						<label className="text-xs font-semibold text-foreground">Provider Preset</label>
						<Select value={provider} onValueChange={onProviderChange}>
							<SelectTrigger className="h-10 text-xs font-sans bg-background border-border/80 rounded-lg w-full px-3.5">
								<SelectValue placeholder="Select Provider" />
							</SelectTrigger>
							<SelectContent>
								{providers.map(p => (
									<SelectItem key={p.id} value={p.id} className="text-xs font-sans">
										{p.name}
									</SelectItem>
								))}
							</SelectContent>
						</Select>
					</div>
				</div>
			</div>

			{/* Section 2: Access Credentials */}
			<div className="flex flex-col gap-4 bg-muted/20 border border-border/50 rounded-xl p-4">
				<div className="flex items-center gap-2 text-xs font-bold text-foreground">
					<KeyRound className="w-4 h-4 text-primary" />
					<span>Access Credentials</span>
				</div>
				<div className="grid grid-cols-1 sm:grid-cols-2 gap-4">
					<div className="flex flex-col gap-2">
						<label className="text-xs font-semibold text-foreground">Access Key ID</label>
						<Input
							value={accessKey}
							onChange={e => setAccessKey(e.target.value)}
							placeholder="AKIAIOSFODNN7EXAMPLE"
							className="h-10 text-xs font-mono bg-background border-border/80 rounded-lg px-3.5"
						/>
					</div>

					<div className="flex flex-col gap-2">
						<label className="text-xs font-semibold text-foreground">Secret Access Key</label>
						<Input
							type="password"
							value={secretKey}
							onChange={e => setSecretKey(e.target.value)}
							placeholder="••••••••••••••••••••••••"
							className="h-10 text-xs font-mono bg-background border-border/80 rounded-lg px-3.5"
						/>
					</div>
				</div>
			</div>

			{/* Section 3: Bucket & Endpoint Details */}
			<div className="flex flex-col gap-4 bg-muted/20 border border-border/50 rounded-xl p-4">
				<div className="flex items-center gap-2 text-xs font-bold text-foreground">
					<HardDrive className="w-4 h-4 text-primary" />
					<span>Bucket Details & Endpoint</span>
				</div>
				<div className="grid grid-cols-1 sm:grid-cols-2 gap-4">
					<div className="flex flex-col gap-2">
						<label className="text-xs font-semibold text-foreground">Bucket Name</label>
						<Input
							value={bucket}
							onChange={e => setBucket(e.target.value)}
							placeholder="my-backups-bucket"
							className="h-10 text-xs font-mono bg-background border-border/80 rounded-lg px-3.5"
						/>
					</div>

					<div className="flex flex-col gap-2">
						<label className="text-xs font-semibold text-foreground">Region</label>
						<Input
							value={region}
							onChange={e => setRegion(e.target.value)}
							placeholder="us-east-1"
							className="h-10 text-xs font-mono bg-background border-border/80 rounded-lg px-3.5"
						/>
					</div>
				</div>

				<div className="flex flex-col gap-2 mt-1">
					<label className="text-xs font-semibold text-foreground">Endpoint URL</label>
					<Input
						value={endpoint}
						onChange={e => setEndpoint(e.target.value)}
						placeholder="https://s3.amazonaws.com"
						className="h-10 text-xs font-mono bg-background border-border/80 rounded-lg px-3.5"
					/>
				</div>
			</div>
		</div>
	);
}
