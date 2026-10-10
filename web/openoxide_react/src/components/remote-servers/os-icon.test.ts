import {describe, expect, it} from 'vitest';
import {matchOsIcon} from '#/lib/os-icon';

describe('matchOsIcon', () => {
	it('matches specific distros before the generic linux glyph', () => {
		expect(matchOsIcon('Ubuntu 24.04.1 LTS').name).toBe('Ubuntu');
		expect(matchOsIcon('Fedora Linux 42 (Workstation Edition)').name).toBe(
			'Fedora',
		);
		expect(matchOsIcon('Debian GNU/Linux 12 (bookworm)').name).toBe(
			'Debian',
		);
		expect(matchOsIcon('ARCH_LINUX').name).toBe('Arch Linux');
		expect(matchOsIcon('AlmaLinux 9.5').name).toBe('AlmaLinux');
	});

	it('maps os/kernel strings for non-linux and bare linux hosts', () => {
		expect(matchOsIcon(undefined, 'Windows 11 Pro').name).toBe('Windows');
		expect(matchOsIcon(undefined, 'Darwin', '24.0.0').name).toBe('Apple');
		expect(matchOsIcon(undefined, 'Linux', '7.2.7').name).toBe('Linux');
		expect(matchOsIcon('').name).toBe('Linux');
	});

	it('carries a brand color for every match', () => {
		expect(matchOsIcon('Ubuntu 24.04').color).toBe('#E95420');
		expect(matchOsIcon('Windows Server 2022').color).toBe('#0078D4');
	});
});
