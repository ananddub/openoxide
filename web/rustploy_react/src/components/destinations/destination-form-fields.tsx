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
		<div className="flex flex-col gap-5 py-1">
			{/* Name Field */}
			<div className="flex flex-col gap-2">
				<label className="text-xs font-semibold text-foreground">Name</label>
				<Input
					value={name}
					onChange={e => setName(e.target.value)}
					placeholder="My S3 Storage"
					className="h-10 text-xs bg-background border-border rounded-md px-3 font-sans"
				/>
			</div>

			{/* Access Key ID & Secret Access Key */}
			<div className="grid grid-cols-1 sm:grid-cols-2 gap-4">
				<div className="flex flex-col gap-2">
					<label className="text-xs font-semibold text-foreground">Access Key ID</label>
					<Input
						value={accessKey}
						onChange={e => setAccessKey(e.target.value)}
						placeholder="Access Key ID"
						className="h-10 text-xs font-mono bg-background border-border rounded-md px-3"
					/>
				</div>

				<div className="flex flex-col gap-2">
					<label className="text-xs font-semibold text-foreground">Secret Access Key</label>
					<Input
						type="password"
						value={secretKey}
						onChange={e => setSecretKey(e.target.value)}
						placeholder="Secret Access Key"
						className="h-10 text-xs font-mono bg-background border-border rounded-md px-3"
					/>
				</div>
			</div>

			{/* Bucket & Region */}
			<div className="grid grid-cols-1 sm:grid-cols-2 gap-4">
				<div className="flex flex-col gap-2">
					<label className="text-xs font-semibold text-foreground">Bucket</label>
					<Input
						value={bucket}
						onChange={e => setBucket(e.target.value)}
						placeholder="my-bucket-name"
						className="h-10 text-xs font-mono bg-background border-border rounded-md px-3"
					/>
				</div>

				<div className="flex flex-col gap-2">
					<label className="text-xs font-semibold text-foreground">Region</label>
					<Input
						value={region}
						onChange={e => setRegion(e.target.value)}
						placeholder="us-east-1"
						className="h-10 text-xs font-mono bg-background border-border rounded-md px-3"
					/>
				</div>
			</div>

			{/* Endpoint */}
			<div className="flex flex-col gap-2">
				<label className="text-xs font-semibold text-foreground">Endpoint</label>
				<Input
					value={endpoint}
					onChange={e => setEndpoint(e.target.value)}
					placeholder="https://s3.amazonaws.com"
					className="h-10 text-xs font-mono bg-background border-border rounded-md px-3"
				/>
			</div>

			{/* Provider Preset */}
			<div className="flex flex-col gap-2">
				<label className="text-xs font-semibold text-foreground">Provider Preset</label>
				<Select value={provider} onValueChange={onProviderChange}>
					<SelectTrigger className="h-10 text-xs font-sans bg-background border-border rounded-md w-full px-3">
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
	);
}
