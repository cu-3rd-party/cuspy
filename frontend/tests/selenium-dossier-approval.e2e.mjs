import fs from 'node:fs';
import net from 'node:net';
import os from 'node:os';
import path from 'node:path';
import { spawn } from 'node:child_process';
import { setTimeout as sleep } from 'node:timers/promises';

import { chromium } from 'playwright';
import { Builder, By, until } from 'selenium-webdriver';
import chrome from 'selenium-webdriver/chrome.js';

const BASE_URL = 'http://127.0.0.1:4173';
const BACKEND_URL = 'http://127.0.0.1:3001';
const WORKDIR = process.cwd();
const TEST_IMAGE_BASE64 =
	'iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mP8/x8AAusB9WnRk6sAAAAASUVORK5CYII=';

const waitForPort = async (port, host = '127.0.0.1', timeoutMs = 30000) => {
	const start = Date.now();

	while (Date.now() - start < timeoutMs) {
		try {
			await new Promise((resolve, reject) => {
				const socket = net.connect({ port, host }, () => {
					socket.end();
					resolve();
				});
				socket.on('error', reject);
			});
			return;
		} catch {
			await sleep(250);
		}
	}

	throw new Error(`Timed out waiting for ${host}:${port}`);
};

const runCommand = (command, args, env = {}) =>
	new Promise((resolve, reject) => {
		const child = spawn(command, args, {
			cwd: WORKDIR,
			env: { ...process.env, ...env },
			stdio: 'inherit'
		});

		child.on('exit', (code) => {
			if (code === 0) {
				resolve();
				return;
			}

			reject(new Error(`${command} ${args.join(' ')} failed with code ${code}`));
		});
	});

const startProcess = (command, args, env = {}) =>
	spawn(command, args, {
		cwd: WORKDIR,
		env: { ...process.env, ...env },
		stdio: 'inherit',
		detached: true
	});

const stopProcess = async (child) => {
	if (!child || child.exitCode !== null || child.signalCode !== null) return;

	process.kill(-child.pid, 'SIGTERM');
	await Promise.race([
		new Promise((resolve) => child.once('exit', resolve)),
		sleep(5000).then(() => {
			if (child.exitCode === null && child.signalCode === null) {
				process.kill(-child.pid, 'SIGKILL');
			}
		})
	]);
};

const createTempImage = () => {
	const filePath = path.join(os.tmpdir(), `selenium-dossier-${Date.now()}.png`);
	fs.writeFileSync(filePath, Buffer.from(TEST_IMAGE_BASE64, 'base64'));
	return filePath;
};

const createDriver = () => {
	const options = new chrome.Options()
		.setChromeBinaryPath(chromium.executablePath())
		.addArguments(
			'--headless=new',
			'--no-sandbox',
			'--disable-dev-shm-usage',
			'--window-size=1440,1200'
		);

	return new Builder().forBrowser('chrome').setChromeOptions(options).build();
};

const waitVisible = async (driver, locator, timeout = 15000) => {
	const element = await driver.wait(until.elementLocated(locator), timeout);
	await driver.wait(until.elementIsVisible(element), timeout);
	return element;
};

const click = async (driver, locator, timeout) => {
	const element = await waitVisible(driver, locator, timeout);
	await driver.executeScript('arguments[0].click()', element);
	return element;
};

const forceClick = async (driver, locator, timeout = 15000) => {
	const element = await driver.wait(until.elementLocated(locator), timeout);
	await driver.executeScript('arguments[0].click()', element);
	return element;
};

const fill = async (driver, locator, value, timeout) => {
	const element = await waitVisible(driver, locator, timeout);
	await element.clear();
	await element.sendKeys(value);
	return element;
};

const waitForUrl = async (driver, fragment, timeout = 15000) => {
	await driver.wait(async () => (await driver.getCurrentUrl()).includes(fragment), timeout);
};

const waitForText = async (driver, text, timeout = 15000) => {
	await waitVisible(
		driver,
		By.xpath(`//*[contains(normalize-space(.), ${JSON.stringify(text)})]`),
		timeout
	);
};

const logStep = (message) => {
	process.stdout.write(`${message}\n`);
};

const registerClient = async (driver, client, imagePath) => {
	logStep(`register start ${client.codename}`);
	await driver.get(`${BASE_URL}/agent-id`);

	await fill(driver, By.css('input[placeholder]'), client.codename);
	await driver.wait(until.elementLocated(By.css('input[type="file"]')), 15000);
	await driver.findElement(By.css('input[type="file"]')).sendKeys(imagePath);
	await forceClick(driver, By.css('input[name="academicLevel"][value="bachelor"]'));
	await waitVisible(driver, By.css('select'));
	await driver.findElement(By.css('select')).sendKeys(client.courseNumber);
	await forceClick(driver, By.css(`input[name="bachelorTrack"][value="${client.bachelorTrack}"]`));
	await click(driver, By.css('form button[type="submit"]'));

	await waitForUrl(driver, '/operational-boundaries');

	const token = await driver.executeScript(
		'return window.localStorage.getItem("backend-access-token")'
	);
	if (!token) throw new Error(`Missing localStorage token for ${client.codename}`);

	await driver.manage().addCookie({
		name: 'backend-access-token',
		value: token,
		path: '/',
		domain: '127.0.0.1'
	});

	const actionButtons = await driver.findElements(By.css('button[type="button"]'));
	await driver.executeScript('arguments[0].click()', actionButtons.at(-1));
	await waitForUrl(driver, '/dossier-verification');
	await click(driver, By.css('button[type="button"]'));
	await driver.wait(
		async () => !(await driver.getCurrentUrl()).includes('/dossier-verification'),
		15000
	);

	await driver.get(`${BASE_URL}/`);
	await waitForUrl(driver, '/waiting-clearance');
	await waitForText(driver, 'Dossier in command queue');
	logStep(`register done ${client.codename}`);
};

const approveClient = async (driver, codename) => {
	logStep(`approve start ${codename}`);
	await click(driver, By.xpath(`//button[contains(., ${JSON.stringify(codename)})]`));
	await fill(driver, By.id('reviewer-note'), `Approved ${codename}.`);
	await click(driver, By.css('button[name="decision"][value="confirmed"]'));
	await waitVisible(
		driver,
		By.xpath(
			`//button[contains(., ${JSON.stringify(codename)})]//*[contains(normalize-space(.), "confirmed")]`
		),
		15000
	);
	logStep(`approve done ${codename}`);
};

const verifyApprovedClient = async (driver) => {
	await driver.get(`${BASE_URL}/`);
	await waitForUrl(driver, '/target-intel');
	await waitForText(driver, 'Current objective');
};

const run = async () => {
	const backend = startProcess('node', ['tests/mock-backend.mjs']);
	let preview;
	const drivers = [];
	const imagePath = createTempImage();

	try {
		await waitForPort(3001);
		await runCommand('npm', ['run', 'build'], { BACKEND_URL });
		preview = startProcess(
			'npm',
			['run', 'preview', '--', '--host', '127.0.0.1', '--port', '4173'],
			{
				BACKEND_URL
			}
		);
		await waitForPort(4173);

		const [clientA, clientB, admin] = await Promise.all([
			createDriver(),
			createDriver(),
			createDriver()
		]);
		drivers.push(clientA, clientB, admin);

		const clients = [
			{ codename: 'SELENIUM_AGENT_ALPHA', courseNumber: '2', bachelorTrack: 'ai' },
			{ codename: 'SELENIUM_AGENT_BRAVO', courseNumber: '3', bachelorTrack: 'development' }
		];

		await registerClient(clientA, clients[0], imagePath);
		await registerClient(clientB, clients[1], imagePath);

		await admin.get(BASE_URL);
		await admin.manage().addCookie({
			name: 'backend-access-token',
			value: 'admin-token',
			path: '/',
			domain: '127.0.0.1'
		});
		await admin.get(`${BASE_URL}/admin/moderation`);
		await waitForText(admin, 'PROFILE_CLEARANCE_QUEUE');

		await approveClient(admin, clients[0].codename);
		await approveClient(admin, clients[1].codename);

		await verifyApprovedClient(clientA);
		await verifyApprovedClient(clientB);

		process.stdout.write('Selenium dossier approval flow passed\n');
	} finally {
		await Promise.all(drivers.map((driver) => driver.quit().catch(() => {})));
		await stopProcess(preview);
		await stopProcess(backend);
		fs.rmSync(imagePath, { force: true });
	}
};

run()
	.then(() => {
		process.exit(0);
	})
	.catch((error) => {
		console.error(error);
		process.exit(1);
	});
