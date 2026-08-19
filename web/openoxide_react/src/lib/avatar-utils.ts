/**
 * Checks if the given avatar value represents a solid color in hexadecimal format or color prefix.
 */
export function isSolidColorAvatar(value?: string | null): boolean {
	if (!value) return false;
	return (
		(value.startsWith('#') && /^#[0-9A-Fa-f]{6}$/.test(value)) ||
		value.startsWith('color:') ||
		false
	);
}

/**
 * Checks if avatar is an image URL or data URL.
 */
export function isImageAvatar(value?: string | null): boolean {
	if (!value) return false;
	return (
		value.startsWith('data:image/') ||
		value.startsWith('http://') ||
		value.startsWith('https://') ||
		value.startsWith('/avatars/')
	);
}

/**
 * Gets the avatar type for selection.
 */
export function getAvatarType(value?: string | null): string {
	if (!value) return '';

	if (isImageAvatar(value)) return 'image';
	if (isSolidColorAvatar(value)) return 'color';

	return value;
}

/**
 * Helper to compute fallback avatar initials from name or email.
 */
export function getAvatarInitials(nameOrEmail?: string | null): string {
	if (!nameOrEmail) return 'US';
	const clean = nameOrEmail.trim();
	if (clean.includes('@')) {
		return clean.substring(0, 2).toUpperCase();
	}
	const parts = clean.split(' ').filter(Boolean);
	if (parts.length >= 2) {
		return `${parts[0][0]}${parts[1][0]}`.toUpperCase();
	}
	return clean.substring(0, 2).toUpperCase();
}
