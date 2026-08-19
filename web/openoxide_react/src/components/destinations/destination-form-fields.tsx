import {useState} from 'react';
import {Input} from '#/components/ui/input';
import {Button} from '#/components/ui/button';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import {Eye, EyeOff} from 'lucide-react';

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
	const [showSecret, setShowSecret] = useState(false);

	const selectedProviderObj = providers.find(
		p =>
			p.id === provider ||
			p.name.toLowerCase() === provider?.toLowerCase(),
	);

	return (
		<div className="flex flex-col gap-4 py-1">
			{/* 1. Name Field (Single Full-Width Input) */}
			<div className="flex flex-col gap-1.5">
				<label className="text-xs font-semibold text-foreground">
					Name
				</label>
				<Input
					value={name}
					onChange={e => setName(e.target.value)}
					placeholder="My S3 Storage"
					className="h-10 w-full rounded-md border-border bg-background px-3 font-sans text-xs"
				/>
			</div>

			{/* 2. Access Key ID (Single Full-Width Input) */}
			<div className="flex flex-col gap-1.5">
				<label className="text-xs font-semibold text-foreground">
					Access Key ID
				</label>
				<Input
					value={accessKey}
					onChange={e => setAccessKey(e.target.value)}
					placeholder="AKIAIOSFODNN7EXAMPLE"
					className="h-10 w-full rounded-md border-border bg-background px-3 font-mono text-xs"
				/>
			</div>

			{/* 3. Secret Access Key with Show/Hide Toggle */}
			<div className="flex flex-col gap-1.5">
				<label className="text-xs font-semibold text-foreground">
					Secret Access Key
				</label>
				<div className="relative flex w-full items-center">
					<Input
						type={showSecret ? 'text' : 'password'}
						value={secretKey}
						onChange={e => setSecretKey(e.target.value)}
						placeholder="Secret Access Key"
						className="h-10 w-full rounded-md border-border bg-background pr-10 pl-3 font-mono text-xs"
					/>
					<Button
						type="button"
						variant="ghost"
						size="icon"
						onClick={() => setShowSecret(!showSecret)}
						className="absolute right-1 h-8 w-8 shrink-0 text-muted-foreground hover:text-foreground">
						{showSecret ? (
							<EyeOff className="h-4 w-4" />
						) : (
							<Eye className="h-4 w-4" />
						)}
					</Button>
				</div>
			</div>

			{/* 4. Bucket Name (Single Full-Width Input) */}
			<div className="flex flex-col gap-1.5">
				<label className="text-xs font-semibold text-foreground">
					Bucket
				</label>
				<Input
					value={bucket}
					onChange={e => setBucket(e.target.value)}
					placeholder="my-bucket-name"
					className="h-10 w-full rounded-md border-border bg-background px-3 font-mono text-xs"
				/>
			</div>

			{/* 5. Region (Single Full-Width Input) */}
			<div className="flex flex-col gap-1.5">
				<label className="text-xs font-semibold text-foreground">
					Region
				</label>
				<Input
					value={region}
					onChange={e => setRegion(e.target.value)}
					placeholder="us-east-1"
					className="h-10 w-full rounded-md border-border bg-background px-3 font-mono text-xs"
				/>
			</div>

			{/* 6. Endpoint URL (Single Full-Width Input) */}
			<div className="flex flex-col gap-1.5">
				<label className="text-xs font-semibold text-foreground">
					Endpoint
				</label>
				<Input
					value={endpoint}
					onChange={e => setEndpoint(e.target.value)}
					placeholder="https://s3.amazonaws.com"
					className="h-10 w-full rounded-md border-border bg-background px-3 font-mono text-xs"
				/>
			</div>

			{/* 7. Provider Preset (Single Full-Width Dropdown) */}
			<div className="flex flex-col gap-1.5">
				<label className="text-xs font-semibold text-foreground">
					Provider Preset
				</label>
				<Select
					value={selectedProviderObj?.id || provider || 'aws'}
					onValueChange={v => v && onProviderChange(v)}>
					<SelectTrigger className="flex !h-10 w-full items-center justify-between rounded-md border-border bg-background px-3 font-sans text-xs">
						<SelectValue placeholder="Select Provider">
							{selectedProviderObj?.name || provider || 'Select Provider'}
						</SelectValue>
					</SelectTrigger>
					<SelectContent className="z-50 border-border bg-card text-xs">
						{providers.map(p => (
							<SelectItem
								key={p.id}
								value={p.id}
								className="font-sans text-xs">
								{p.name}
							</SelectItem>
						))}
					</SelectContent>
				</Select>
			</div>
		</div>
	);
}
