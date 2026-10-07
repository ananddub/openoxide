/** Adds a `docker system df` display size ("1.2GB", "0B") in bytes; unparseable rows count as zero. */
export function diskBytes(value: string): number {
	const match = /^([\d.]+)\s*([kMGT]?)[B]?$/i.exec(value.trim());
	if (!match) return 0;
	const units = {k: 1e3, m: 1e6, g: 1e9, t: 1e12};
	const unit = match[2].toLowerCase() as keyof typeof units;
	return Number(match[1]) * (units[unit] ?? 1);
}

export function formatBytes(bytes: number): string {
	if (bytes >= 1e9) return `${(bytes / 1e9).toFixed(1)} GB`;
	if (bytes >= 1e6) return `${(bytes / 1e6).toFixed(1)} MB`;
	return `${Math.round(bytes / 1e3)} kB`;
}
