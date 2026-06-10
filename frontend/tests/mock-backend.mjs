import http from 'node:http';

const json = (response, status, payload) => {
	response.writeHead(status, { 'content-type': 'application/json' });
	response.end(JSON.stringify(payload));
};

const users = {
	pending: {
		user_id: 'user-pending',
		telegram_id: 101,
		rating: 1000,
		agent_name: 'Pending Agent',
		agent_data: { codename: 'PENDING_AGENT' },
		is_admin: false,
		created_at: '1',
		updated_at: null
	},
	approved: {
		user_id: 'user-approved',
		telegram_id: 202,
		rating: 1300,
		agent_name: 'Approved Agent',
		agent_data: { codename: 'APPROVED_AGENT' },
		is_admin: false,
		created_at: '1',
		updated_at: null
	},
	rejected: {
		user_id: 'user-rejected',
		telegram_id: 303,
		rating: 990,
		agent_name: 'Rejected Agent',
		agent_data: { codename: 'REJECTED_AGENT' },
		is_admin: false,
		created_at: '1',
		updated_at: null
	},
	admin: {
		user_id: 'user-admin',
		telegram_id: 404,
		rating: 1700,
		agent_name: 'Admin Agent',
		agent_data: { codename: 'CONTROL_NODE' },
		is_admin: true,
		created_at: '1',
		updated_at: null
	}
};

const tokenUsers = new Map([
	['pending-token', users.pending],
	['approved-token', users.approved],
	['rejected-token', users.rejected],
	['admin-token', users.admin]
]);

const profileRequests = {
	pending: [
		{
			profile_request_id: 'request-pending',
			user_id: 'user-pending',
			requested_profile_data: { codename: 'PENDING_AGENT' },
			status: 'sent',
			reviewer_note: null,
			reviewed_at: null,
			created_at: '1710000000',
			updated_at: '1710000000'
		}
	],
	approved: [
		{
			profile_request_id: 'request-approved',
			user_id: 'user-approved',
			requested_profile_data: { codename: 'APPROVED_AGENT' },
			status: 'confirmed',
			reviewer_note: null,
			reviewed_at: '1710000100',
			created_at: '1710000000',
			updated_at: '1710000100'
		}
	],
	rejected: [
		{
			profile_request_id: 'request-rejected',
			user_id: 'user-rejected',
			requested_profile_data: { codename: 'REJECTED_AGENT' },
			status: 'rejected',
			reviewer_note: 'Need clearer identification image.',
			reviewed_at: '1710000200',
			created_at: '1710000000',
			updated_at: '1710000200'
		}
	]
};

const profileRequestsByUserId = new Map([
	[users.pending.user_id, structuredClone(profileRequests.pending)],
	[users.approved.user_id, structuredClone(profileRequests.approved)],
	[users.rejected.user_id, structuredClone(profileRequests.rejected)],
	[users.admin.user_id, []]
]);

let sequence = 3;

const nextUnix = () => String(1710000000 + sequence++);

const nextId = (prefix) => `${prefix}-${sequence++}`;

let targets = [
	{
		target_id: 'target-1',
		identifier: 'VAL-772_SYNDICATE_HEAD',
		last_known_location: 'CAMPUS_CENTRAL',
		status: 'active'
	},
	{
		target_id: 'target-2',
		identifier: 'RAZOR_WIND_ENFORCER',
		last_known_location: 'SECTOR_7',
		status: 'active'
	}
];

let killReports = [
	{
		kill_report_id: 'kill-report-1',
		reporter_user_id: users.approved.user_id,
		reporter_codename: users.approved.agent_data.codename,
		target_id: 'target-1',
		target_identifier: 'VAL-772_SYNDICATE_HEAD',
		modus_operandi: 'Confirmed elimination in clean room.',
		witness_present: false,
		status: 'confirmed',
		reviewer_note: 'Evidence chain intact.',
		created_at: '1710000400',
		updated_at: '1710000400',
		reviewed_at: '1710000400'
	}
];

let adminRequests = [
	{
		profile_request_id: 'queue-1',
		user_id: 'user-pending',
		requested_profile_data: {
			codename: 'PENDING_AGENT',
			academicGroup: 'B21-DS-01',
			identificationName: 'pending-id.png',
			identificationImage: 'data:image/png;base64,AA=='
		},
		status: 'sent',
		reviewer_note: null,
		reviewed_at: null,
		created_at: '1710000000',
		updated_at: '1710000000'
	},
	{
		profile_request_id: 'queue-2',
		user_id: 'user-rejected',
		requested_profile_data: {
			codename: 'REJECTED_AGENT',
			academicGroup: 'M11-AI-02',
			identificationName: 'rejected-id.png'
		},
		status: 'rejected',
		reviewer_note: 'Need clearer identification image.',
		reviewed_at: '1710000200',
		created_at: '1710000100',
		updated_at: '1710000200'
	}
];

const requestForToken = (token) => tokenUsers.get(token) ?? null;

const server = http.createServer((request, response) => {
	const auth = request.headers.authorization;
	const token = auth?.replace(/^Bearer\s+/, '') ?? '';

	if (request.url === '/auth/me') {
		const user = requestForToken(token);
		if (user) return json(response, 200, user);
		return json(response, 401, { error: 'unauthorized' });
	}

	if (request.url === '/auth/register' && request.method === 'POST') {
		let body = '';
		request.on('data', (chunk) => {
			body += chunk;
		});
		request.on('end', () => {
			const payload = JSON.parse(body || '{}');
			const accessToken = `generated-token-${sequence}`;
			const userId = nextId('user-generated');
			const user = {
				user_id: userId,
				telegram_id: payload.telegram_id ?? 0,
				rating: payload.rating ?? 0,
				agent_name: payload.agent_name ?? 'Generated Agent',
				agent_data: payload.agent_data ?? {},
				is_admin: false,
				created_at: nextUnix(),
				updated_at: null
			};

			tokenUsers.set(accessToken, user);
			profileRequestsByUserId.set(userId, []);

			json(response, 200, { access_token: accessToken, user });
		});
		return;
	}

	if (request.url === '/profile-creation-requests' && request.method === 'GET') {
		const user = requestForToken(token);
		if (!user) return json(response, 200, []);
		return json(response, 200, profileRequestsByUserId.get(user.user_id) ?? []);
	}

	if (request.url === '/profile-creation-requests' && request.method === 'POST') {
		const user = requestForToken(token);
		if (!user) return json(response, 401, { error: 'unauthorized' });

		let body = '';
		request.on('data', (chunk) => {
			body += chunk;
		});
		request.on('end', () => {
			const payload = JSON.parse(body || '{}');
			const timestamp = nextUnix();
			const requestedProfileData = payload.requested_profile_data ?? {};
			const profileRequest = {
				profile_request_id: nextId('request-generated'),
				user_id: user.user_id,
				requested_profile_data: requestedProfileData,
				status: 'sent',
				reviewer_note: null,
				reviewed_at: null,
				created_at: timestamp,
				updated_at: timestamp
			};

			profileRequestsByUserId.set(user.user_id, [profileRequest]);
			user.agent_data = requestedProfileData;
			user.agent_name = requestedProfileData.codename ?? user.agent_name;

			adminRequests = [
				{
					...profileRequest,
					requested_profile_data: {
						academicGroup: requestedProfileData.academicGroup ?? 'AUTO-GROUP',
						...requestedProfileData
					}
				},
				...adminRequests
			];

			json(response, 200, { ok: true, user });
		});
		return;
	}

	if (request.url === '/targets' && request.method === 'GET') {
		if (!requestForToken(token)) return json(response, 401, { error: 'unauthorized' });
		return json(response, 200, targets);
	}

	if (request.url === '/kill-reports' && request.method === 'POST') {
		const user = requestForToken(token);
		if (!user) return json(response, 401, { error: 'unauthorized' });

		let body = '';
		request.on('data', (chunk) => {
			body += chunk;
		});
		request.on('end', () => {
			const payload = JSON.parse(body || '{}');
			const target = targets.find((entry) => entry.target_id === payload.target_id);
			if (!target) return json(response, 400, { error: 'Unknown target' });

			const timestamp = nextUnix();
			const report = {
				kill_report_id: nextId('kill-report'),
				reporter_user_id: user.user_id,
				reporter_codename: user.agent_data?.codename ?? user.agent_name ?? 'AGENT',
				target_id: target.target_id,
				target_identifier: target.identifier,
				modus_operandi: payload.modus_operandi ?? '',
				witness_present: Boolean(payload.witness_present),
				status: 'pending',
				reviewer_note: null,
				created_at: timestamp,
				updated_at: timestamp,
				reviewed_at: null
			};

			killReports = [report, ...killReports];
			json(response, 200, report);
		});
		return;
	}

	if (request.url === '/admin/kill-reports' && request.method === 'GET') {
		if (token !== 'admin-token') return json(response, 403, { error: 'forbidden' });
		return json(response, 200, killReports);
	}

	if (request.url?.startsWith('/admin/kill-reports/') && request.method === 'PATCH') {
		if (token !== 'admin-token') return json(response, 403, { error: 'forbidden' });

		let body = '';
		request.on('data', (chunk) => {
			body += chunk;
		});
		request.on('end', () => {
			const payload = JSON.parse(body || '{}');
			const reportId = request.url.split('/').pop();
			killReports = killReports.map((entry) =>
				entry.kill_report_id === reportId
					? {
						...entry,
						status: payload.status ?? entry.status,
						reviewer_note: payload.reviewer_note ?? entry.reviewer_note,
						reviewed_at: '1710000500',
						updated_at: '1710000500'
					}
					: entry
			);

			const updated = killReports.find((entry) => entry.kill_report_id === reportId);
			if (updated?.status === 'confirmed') {
				targets = targets.map((entry) =>
					entry.target_id === updated.target_id ? { ...entry, status: 'eliminated' } : entry
				);
			}

			json(response, 200, updated ?? { error: 'not found' });
		});
		return;
	}

	if (request.url === '/admin/profile-creation-requests' && request.method === 'GET') {
		if (token !== 'admin-token') return json(response, 403, { error: 'forbidden' });
		return json(response, 200, adminRequests);
	}

	if (request.url?.startsWith('/admin/profile-creation-requests/') && request.method === 'PATCH') {
		if (token !== 'admin-token') return json(response, 403, { error: 'forbidden' });

		let body = '';
		request.on('data', (chunk) => {
			body += chunk;
		});
		request.on('end', () => {
			const payload = JSON.parse(body || '{}');
			const requestId = request.url.split('/').pop();
			adminRequests = adminRequests.map((entry) =>
				entry.profile_request_id === requestId
					? {
						...entry,
						status: payload.status ?? entry.status,
						reviewer_note: payload.reviewer_note ?? entry.reviewer_note,
						reviewed_at: '1710000300',
						updated_at: '1710000300'
					}
					: entry
			);

			const updated = adminRequests.find((entry) => entry.profile_request_id === requestId);
			if (updated) {
				profileRequestsByUserId.set(updated.user_id, [updated]);
			}
			json(response, 200, updated ?? { error: 'not found' });
		});
		return;
	}

	if (request.url === '/rankings') {
		const rankingUsers = [users.approved, users.pending]
			.map((user) => ({
				user_id: user.user_id,
				agent_name: user.agent_name,
				rating: user.rating,
				approved_kills: killReports.filter(
					(report) => report.reporter_user_id === user.user_id && report.status === 'confirmed'
				).length,
				approved_deaths: 0
			}))
			.sort((left, right) => right.approved_kills - left.approved_kills || right.rating - left.rating)
			.map((entry, index) => ({ rank: index + 1, ...entry }));

		return json(response, 200, rankingUsers);
	}

	json(response, 404, { error: 'not found' });
});

server.listen(3001, '127.0.0.1', () => {
	process.stdout.write('mock backend ready\n');
});
