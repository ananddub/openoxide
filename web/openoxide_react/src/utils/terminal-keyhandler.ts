import type {Terminal} from '@xterm/xterm';

/** Ctrl/Cmd key combos translated to their raw control-byte sequence. */
const CONTROL_KEY_MAP: Record<string, string> = {
	l: '\x0c',
	c: '\x03',
	d: '\x04',
	z: '\x1a',
	u: '\x15',
	a: '\x01',
	e: '\x05',
	k: '\x0b',
	w: '\x17',
};

/** xterm `attachCustomKeyEventHandler` logic: clipboard copy/paste and control-key passthrough. `sendInput` writes raw text to the session. Returns false when the event was consumed (xterm should ignore it). */
export function handleTerminalKey(
	event: KeyboardEvent,
	term: Terminal,
	sendInput: (text: string) => void,
): boolean {
	if (!(event.ctrlKey || event.metaKey)) return true;
	const k = event.key.toLowerCase();

	if (k === 'c' && term.hasSelection()) {
		if (event.type === 'keydown') {
			navigator.clipboard.writeText(term.getSelection());
		}
		event.preventDefault();
		return false;
	}

	if (k === 'v') {
		if (event.type === 'keydown') {
			navigator.clipboard
				.readText()
				.then(text => {
					if (text) sendInput(text);
				})
				.catch(() => {});
		}
		event.preventDefault();
		return false;
	}

	if (CONTROL_KEY_MAP[k]) {
		if (event.type === 'keydown') {
			if (k === 'l') {
				term.clear();
				sendInput('clear\r');
			} else {
				sendInput(CONTROL_KEY_MAP[k]);
			}
		}
		event.preventDefault();
		return false;
	}

	return true;
}
