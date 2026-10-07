import {describe, expect, it} from 'vitest';
import {
	buildSettingsPatch,
	cleanupProblem,
	settingsProblem,
	toSettingsForm,
} from './settings-form';
import type {RemoteServerPayload} from '#/api/live/servers';

const server: RemoteServerPayload = {
	id: 7,
	name: 'box-1',
	description: 'web',
	ip_address: '10.0.0.5',
	port: 22,
	username: 'root',
	app_name: 'srv-1',
	server_status: 'ACTIVE',
	server_type: 'BUILD',
	enable_docker_cleanup: 1,
	log_cleanup_cron: '0 2 * * *',
	command: '',
	metrics_config: '{}',
	ssh_key_id: 3,
	created_at: 0,
	updated_at: 0,
	build_memory_limit: '2g',
	build_cpu_limit: '2',
};

describe('toSettingsForm', () => {
	it('normalises the wire row into editable strings', () => {
		const form = toSettingsForm(server);
		expect(form).toMatchObject({
			port: '22',
			server_type: 'BUILD',
			ssh_key_id: '3',
			enable_docker_cleanup: true,
			log_cleanup_cron: '0 2 * * *',
		});
	});

	it('falls back to the defaults the backend applies on create', () => {
		const form = toSettingsForm({
			...server,
			port: 0,
			username: '',
			server_type: undefined as unknown as string,
			enable_docker_cleanup: 0,
		});
		expect(form.port).toBe('22');
		expect(form.username).toBe('root');
		expect(form.server_type).toBe('DEPLOY');
		expect(form.enable_docker_cleanup).toBe(false);
	});
});

describe('buildSettingsPatch', () => {
	it('sends nothing when the form matches the stored row', () => {
		const form = toSettingsForm(server);
		expect(buildSettingsPatch(form, form)).toEqual({});
	});

	it('sends only the changed fields, trimmed', () => {
		const initial = toSettingsForm(server);
		const patch = buildSettingsPatch(
			{...initial, name: ' box-2 ', port: '2222', description: '  '},
			initial,
		);
		expect(patch).toEqual({name: 'box-2', port: 2222, description: ''});
	});

	it('keeps an unchanged SSH key out of the patch and skips clearing one', () => {
		const initial = toSettingsForm(server);
		expect(
			buildSettingsPatch({...initial, ssh_key_id: '3'}, initial),
		).toEqual({});
		expect(
			buildSettingsPatch({...initial, ssh_key_id: '4'}, initial)
				.ssh_key_id,
		).toBe(4);
		expect(
			buildSettingsPatch({...initial, ssh_key_id: ''}, initial),
		).toEqual({});
	});

	it('treats the build limits as free text the builder parses', () => {
		const initial = toSettingsForm(server);
		expect(
			buildSettingsPatch(
				{...initial, build_memory_limit: '512m', build_cpu_limit: '0.5'},
				initial,
			),
		).toEqual({build_memory_limit: '512m', build_cpu_limit: '0.5'});
	});
});

describe('settingsProblem', () => {
	const ok = toSettingsForm(server);

	it('requires the three fields the create endpoint requires', () => {
		expect(settingsProblem(ok)).toBeNull();
		expect(settingsProblem({...ok, name: '  '})).toMatch(/name/i);
		expect(settingsProblem({...ok, ip_address: ''})).toMatch(/address/i);
		expect(settingsProblem({...ok, username: ''})).toMatch(/user/i);
	});

	it('rejects ports the SSH client could not connect to', () => {
		expect(settingsProblem({...ok, port: '0'})).toMatch(/port/i);
		expect(settingsProblem({...ok, port: '70000'})).toMatch(/port/i);
		expect(settingsProblem({...ok, port: '22.5'})).toMatch(/port/i);
	});
});

describe('cleanupProblem', () => {
	const off = {...toSettingsForm(server), enable_docker_cleanup: false};

	it('ignores the schedule while cleanup is disabled', () => {
		expect(cleanupProblem({...off, log_cleanup_cron: ''})).toBeNull();
	});

	it('needs a five-field expression when cleanup is enabled', () => {
		const on = toSettingsForm(server);
		expect(cleanupProblem(on)).toBeNull();
		expect(cleanupProblem({...on, log_cleanup_cron: '   '})).toMatch(
			/schedule/i,
		);
		expect(cleanupProblem({...on, log_cleanup_cron: '0 2 *'})).toMatch(
			/cron/i,
		);
	});
});
