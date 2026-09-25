// Dev harness — not part of the app. Drives a running panel/spa, screenshots
// the rail and asserts the row geometry numerically.
//
//	 CHROME_PATH=/usr/bin/chromium EMAIL=a@b.c PASS=x bun scripts/sidebar-shot.mjs
//
// Env: BASE (default http://localhost:3001), CHROME_PATH (required).
import {mkdir} from 'node:fs/promises';
import puppeteer from 'puppeteer-core';

const BASE = process.env.BASE ?? 'http://localhost:3001';
const OUT = '.artifacts';
const RAIL = '[data-sidebar="sidebar"]';
/** Rail rows are the links/buttons that own an icon cell. */
const ROWS = `${RAIL} :is(a,button):has(> [class*="place-content-center"])`;
const CELL = '[class*="place-content-center"]';
const wait = ms => new Promise(r => setTimeout(r, ms));

if (!process.env.CHROME_PATH) {
	console.error('Set CHROME_PATH to a Chrome/Chromium binary.');
	process.exit(1);
}

/** Reads the rail from the page: every row's height and its icon-cell centre. */
const geometry = () => {
	const rail = document.querySelector('[data-sidebar="sidebar"]');
	if (!rail) return {railWidth: 0, rows: [], active: 0};
	const box = rail.getBoundingClientRect();
	const rows = [...rail.querySelectorAll(
		':is(a,button):has(> [class*="place-content-center"])',
	)]
		.slice(0, 12)
		.map(row => {
			const cell = row.querySelector('[class*="place-content-center"]');
			return {
				text: (row.textContent ?? '').trim().slice(0, 12),
				rowH: Math.round(row.getBoundingClientRect().height),
				cellCx: +(
					cell.getBoundingClientRect().left -
					box.left +
					cell.getBoundingClientRect().width / 2
				).toFixed(1),
				fill: getComputedStyle(row).backgroundColor,
				selected: row.classList.contains('bg-sidebar-accent'),
			};
		});
	return {
		railWidth: Math.round(box.width),
		active: rows.filter(r => !/rgba\(0, 0, 0, 0\)|transparent/.test(r.fill))
			.length,
		rows,
	};
};

/** Cell centre: 26px in the 240px rail, 35px in the 70px one. Selected row must carry a fill. */
const verdict = (label, g) => {
	const want = g.railWidth <= 80 ? 35 : 26;
	const off = g.rows.filter(r => Math.abs(r.cellCx - want) > 0.6);
	const h = g.rows.filter(r => r.rowH !== 36);
	const lit = g.rows.filter(r => r.selected);
	const ok = g.rows.length && !off.length && !h.length && lit.length;
	console.log(
		`${ok ? 'PASS' : 'FAIL'} ${label} rail=${g.railWidth}px cell=${want}px` +
			` rows=${g.rows.length} filled=${lit.length}` +
			` [${lit.map(r => `${r.text}:${r.fill}`).join(', ')}]` +
			(off.length ? ` offCentre=${JSON.stringify(off)}` : '') +
			(h.length ? ` badHeight=${JSON.stringify(h)}` : ''),
	);
};

const browser = await puppeteer.launch({
	executablePath: process.env.CHROME_PATH,
	headless: true,
	args: ['--no-sandbox'],
});
await mkdir(OUT, {recursive: true});
const page = await browser.newPage();
await page.setViewport({width: 1440, height: 900, deviceScaleFactor: 2});
page.on('pageerror', e => console.log('PAGEERROR', e.message));

const railShot = async name => {
	const clip = await page.$eval(RAIL, el => {
		const r = el.getBoundingClientRect();
		return {x: 0, y: 0, width: Math.ceil(r.width), height: Math.ceil(r.height)};
	});
	await page.screenshot({path: `${OUT}/${name}.png`, clip});
};

const toggleRail = async () => {
	await page.keyboard.down('Control');
	await page.keyboard.press('KeyB');
	await page.keyboard.up('Control');
	await wait(700);
};

await page.goto(`${BASE}/singin`, {waitUntil: 'networkidle2'});
if (await page.evaluate(() => !!document.querySelector('input[type=password]'))) {
	await page.type('input[type=email]', process.env.EMAIL ?? '');
	await page.type('input[type=password]', process.env.PASS ?? '');
	await page.click('button[type=submit]');
	await page.waitForFunction(() => !location.pathname.includes('singin'), {
		timeout: 15000,
	});
}

await page.goto(`${BASE}/`, {waitUntil: 'networkidle2'});
await page.waitForSelector(ROWS);
await wait(600);
verdict('expanded /', await page.evaluate(geometry));
await railShot('01-rail-expanded');

await toggleRail();
verdict('collapsed /', await page.evaluate(geometry));
await railShot('02-rail-collapsed');

await toggleRail();
await page.goto(`${BASE}/settings`, {waitUntil: 'networkidle2'});
await page.waitForSelector(ROWS);
await wait(600);
verdict('expanded /settings', await page.evaluate(geometry));
await railShot('03-rail-context');

await browser.close();
console.log(`screenshots → ${OUT}/`);
