import {toast} from 'sonner';

/**
 * Save text out of the browser as a file — the metric and log exports are text
 * payloads the panel already has in memory, so no server-side download URL is
 * involved. An empty payload is refused instead of dropping a 0-byte file.
 */
export function downloadTextFile(
	filename: string,
	content: string,
	mimeType = 'text/plain;charset=utf-8',
): boolean {
	if (!content) {
		toast.error('Nothing to download yet');
		return false;
	}
	const url = URL.createObjectURL(new Blob([content], {type: mimeType}));
	const link = document.createElement('a');
	link.href = url;
	link.download = filename;
	link.click();
	URL.revokeObjectURL(url);
	return true;
}
