import {useRef} from 'react';
import {Camera, Upload, Palette} from 'lucide-react';
import {toast} from 'sonner';
import {isSolidColorAvatar} from '#/lib/avatar-utils';

const PRESET_AVATARS = [
	'https://api.dicebear.com/7.x/bottts/svg?seed=OpenOxide1',
	'https://api.dicebear.com/7.x/bottts/svg?seed=OpenOxide2',
	'https://api.dicebear.com/7.x/bottts/svg?seed=OpenOxide3',
	'https://api.dicebear.com/7.x/bottts/svg?seed=OpenOxide4',
	'https://api.dicebear.com/7.x/avataaars/svg?seed=Alex',
	'https://api.dicebear.com/7.x/avataaars/svg?seed=Sarah',
	'https://api.dicebear.com/7.x/avataaars/svg?seed=Elena',
	'https://api.dicebear.com/7.x/avataaars/svg?seed=David',
	'https://api.dicebear.com/7.x/shapes/svg?seed=Shape1',
	'https://api.dicebear.com/7.x/shapes/svg?seed=Shape2',
];

/** Avatar chooser: initials / image upload / solid color / preset gallery. */
export function AvatarPicker({
	avatarValue,
	initials,
	onChange,
}: {
	avatarValue: string;
	initials: string;
	onChange: (value: string) => void;
}) {
	const colorInputRef = useRef<HTMLInputElement>(null);
	const fileInputRef = useRef<HTMLInputElement>(null);

	const handleImageFileUpload = (
		e: React.ChangeEvent<HTMLInputElement>,
	) => {
		const file = e.target.files?.[0];
		if (!file) return;
		if (file.size > 2 * 1024 * 1024) {
			toast.error('Image size must be less than 2MB');
			return;
		}
		const reader = new FileReader();
		reader.onload = event => {
			onChange(event.target?.result as string);
			toast.success('Avatar image loaded');
		};
		reader.readAsDataURL(file);
	};

	return (
		<div className="rounded-xl bg-muted/20 p-4">
			<div className="mb-3 flex items-center justify-between gap-3">
				<p className="text-[11px] font-semibold tracking-wide text-muted-foreground uppercase">
					Choose avatar
				</p>
				<p className="flex items-center gap-1.5 text-[11px] text-muted-foreground">
					<Camera className="size-3" />
					Upload up to 2 MB, or pick a color.
				</p>
			</div>
			<div className="flex flex-wrap items-center gap-2.5">
				<button
					type="button"
					onClick={() => onChange('')}
					aria-label="Use initials"
					className={`flex size-10 cursor-pointer items-center justify-center rounded-full text-[11px] font-bold uppercase transition-all ${
						!avatarValue
							? 'bg-foreground/[0.08] text-foreground ring-2 ring-primary'
							: 'bg-foreground/[0.05] text-muted-foreground hover:scale-105'
					}`}>
					{initials}
				</button>

				<button
					type="button"
					onClick={() => fileInputRef.current?.click()}
					aria-label="Upload image"
					className={`relative flex size-10 cursor-pointer items-center justify-center overflow-hidden rounded-full bg-muted/60 transition-all hover:scale-105 ${
						(avatarValue || '').startsWith('data:')
							? 'ring-2 ring-primary'
							: ''
					}`}>
					{(avatarValue || '').startsWith('data:') ? (
						<img
							src={avatarValue}
							alt="Custom avatar"
							className="size-full rounded-full object-cover"
						/>
					) : (
						<Upload className="size-4 text-muted-foreground" />
					)}
				</button>
				<input
					ref={fileInputRef}
					type="file"
					accept="image/*"
					className="hidden"
					onChange={handleImageFileUpload}
				/>

				<button
					type="button"
					onClick={() => colorInputRef.current?.click()}
					aria-label="Pick color"
					className={`relative flex size-10 cursor-pointer items-center justify-center overflow-hidden rounded-full bg-muted/60 transition-all hover:scale-105 ${
						isSolidColorAvatar(avatarValue) ? 'ring-2 ring-primary' : ''
					}`}
					style={{
						backgroundColor: isSolidColorAvatar(avatarValue)
							? avatarValue
							: undefined,
					}}>
					{!isSolidColorAvatar(avatarValue) && (
						<Palette className="size-4 text-muted-foreground" />
					)}
				</button>
				<input
					ref={colorInputRef}
					type="color"
					className="pointer-events-none absolute size-10 opacity-0"
					value={isSolidColorAvatar(avatarValue) ? avatarValue : '#3b82f6'}
					onChange={e => onChange(e.target.value)}
				/>

				<span className="mx-1 h-5 w-px bg-border/60" aria-hidden />

				{PRESET_AVATARS.map((imgUrl, idx) => (
					<button
						type="button"
						key={idx}
						onClick={() => onChange(imgUrl)}
						aria-label={`Preset avatar ${idx + 1}`}
						className={`size-10 cursor-pointer overflow-hidden rounded-full transition-all hover:scale-105 ${
							avatarValue === imgUrl ? 'ring-2 ring-primary' : ''
						}`}>
						<img
							src={imgUrl}
							alt=""
							className="size-full rounded-full object-cover"
						/>
					</button>
				))}
			</div>
		</div>
	);
}
