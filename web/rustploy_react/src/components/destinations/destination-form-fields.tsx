import {Input} from '#/components/ui/input';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';

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
		<div className="flex flex-col gap-4 mt-3">
			{/* Name Field (Full Width) */}
			<div className="flex flex-col gap-1.5">
				<label className="text-xs font-semibold text-foreground">Name</label>
				<Input
					value={name}
					onChange={e => setName(e.target.value)}
					placeholder="AWS Production Storage"
					className="h-9 text-xs bg-muted/20 border-border/70"
				/>
			</div>

			{/* Access Key & Secret Key (2-Column Grid Dokploy Style) */}
			<div className="grid grid-cols-2 gap-3">
				<div className="flex flex-col gap-1.5">
					<label className="text-xs font-semibold text-foreground">Access Key ID</label>
					<Input
						value={accessKey}
						onChange={e => setAccessKey(e.target.value)}
						placeholder="AKIAIOSFODNN7EXAMPLE"
						className="h-9 text-xs font-mono bg-muted/20 border-border/70"
					/>
				</div>
				<div className="flex flex-col gap-1.5">
					<label className="text-xs font-semibold text-foreground">Secret Access Key</label>
					<Input
						type="password"
						value={secretKey}
						onChange={e => setSecretKey(e.target.value)}
						placeholder="••••••••••••••••••••••••"
						className="h-9 text-xs font-mono bg-muted/20 border-border/70"
					/>
				</div>
			</div>

			{/* Bucket & Region (2-Column Grid Dokploy Style) */}
			<div className="grid grid-cols-2 gap-3">
				<div className="flex flex-col gap-1.5">
					<label className="text-xs font-semibold text-foreground">Bucket</label>
					<Input
						value={bucket}
						onChange={e => setBucket(e.target.value)}
						placeholder="my-backups-bucket"
						className="h-9 text-xs font-mono bg-muted/20 border-border/70"
					/>
				</div>
				<div className="flex flex-col gap-1.5">
					<label className="text-xs font-semibold text-foreground">Region</label>
					<Input
						value={region}
						onChange={e => setRegion(e.target.value)}
						placeholder="us-east-1"
						className="h-9 text-xs font-mono bg-muted/20 border-border/70"
					/>
				</div>
			</div>

			{/* Endpoint URL (Full Width Dokploy Style) */}
			<div className="flex flex-col gap-1.5">
				<label className="text-xs font-semibold text-foreground">Endpoint</label>
				<Input
					value={endpoint}
					onChange={e => setEndpoint(e.target.value)}
					placeholder="https://s3.amazonaws.com"
					className="h-9 text-xs font-mono bg-muted/20 border-border/70"
				/>
			</div>

			{/* Provider Preset Dropdown */}
			<div className="flex flex-col gap-1.5">
				<label className="text-xs font-semibold text-foreground">Provider Preset</label>
				<Select value={provider} onValueChange={onProviderChange}>
					<SelectTrigger className="h-9 text-xs font-sans bg-muted/20 border-border/70 w-full">
						<SelectValue placeholder="Select Provider Preset" />
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
	);
}
