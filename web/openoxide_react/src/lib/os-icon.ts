import {
	siLinux,
	siUbuntu,
	siDebian,
	siFedora,
	siRedhat,
	siArchlinux,
	siAlpinelinux,
	siCentos,
	siOpensuse,
	siRockylinux,
	siAlmalinux,
	siApple,
	siFreebsd,
	siOpenwrt,
	siRaspberrypi,
} from 'simple-icons';

export type OsIcon = {
	/** Brand name for tooltips/labels. */
	name: string;
	paths: string[];
	/** Brand hex color, '#rrggbb'. */
	color: string;
};

/** simple-icons dropped Microsoft marks in v16, so the four-pane flag is inlined. */
const WINDOWS_ICON: OsIcon = {
	name: 'Windows',
	color: '#0078D4',
	paths: [
		'M3.6 5.1 11 4.15v7.35H3.6z',
		'M11.9 4.03 20.4 3v8.5h-8.5z',
		'M3.6 12.5H11v7.35L3.6 18.9z',
		'M11.9 12.5h8.5V19l-8.5-1.13z',
	],
};

const si = (icon: typeof siLinux): OsIcon => ({
	name: icon.title,
	paths: [icon.path],
	color: `#${icon.hex}`,
});

const RULES: Array<[RegExp, OsIcon]> = [
	[/ubuntu/, si(siUbuntu)],
	[/debian/, si(siDebian)],
	[/raspbian|raspberry/, si(siRaspberrypi)],
	[/fedora/, si(siFedora)],
	[/red\s*hat|rhel/, si(siRedhat)],
	[/rocky/, si(siRockylinux)],
	[/alma/, si(siAlmalinux)],
	[/centos/, si(siCentos)],
	[/suse/, si(siOpensuse)],
	[/arch/, si(siArchlinux)],
	[/alpine/, si(siAlpinelinux)],
	[/openwrt/, si(siOpenwrt)],
	[/windows|win32|mingw/, WINDOWS_ICON],
	[/mac ?os|darwin/, si(siApple)],
	[/freebsd/, si(siFreebsd)],
	[/linux|kernel/, si(siLinux)],
];

/** Detect the OS brand from agent-reported strings (distro → os → kernel). */
export function matchOsIcon(
	...hints: Array<string | undefined | null>
): OsIcon {
	const hay = hints.filter(Boolean).join(' ').toLowerCase();
	for (const [re, icon] of RULES) {
		if (re.test(hay)) return icon;
	}
	return si(siLinux);
}
