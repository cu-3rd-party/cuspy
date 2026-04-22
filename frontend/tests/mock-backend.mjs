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
	}
};

const profileRequests = {
	pending: [
		{
			profile_creation_request_id: 'request-pending',
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
			profile_creation_request_id: 'request-approved',
			user_id: 'user-approved',
			requested_profile_data: { codename: 'APPROVED_AGENT' },
			status: 'confirmed',
			reviewer_note: null,
			reviewed_at: '1710000100',
			created_at: '1710000000',
			updated_at: '1710000100'
		}
	]
};

const server = http.createServer((request, response) => {
	const auth = request.headers.authorization;
	const token = auth?.replace(/^Bearer\s+/, '') ?? '';

	if (request.url === '/auth/me') {
		if (token === 'pending-token') return json(response, 200, users.pending);
		if (token === 'approved-token') return json(response, 200, users.approved);
		return json(response, 401, { error: 'unauthorized' });
	}

	if (request.url === '/profile-creation-requests') {
		if (token === 'pending-token') return json(response, 200, profileRequests.pending);
		if (token === 'approved-token') return json(response, 200, profileRequests.approved);
		return json(response, 200, []);
	}

	if (request.url === '/rankings') {
		return json(response, 200, [
			{ rank: 1, user_id: 'user-approved', agent_name: 'Approved Agent', rating: 1300, approved_kills: 4, approved_deaths: 1 },
			{ rank: 2, user_id: 'user-pending', agent_name: 'Pending Agent', rating: 1000, approved_kills: 1, approved_deaths: 0 }
		]);
	}

	json(response, 404, { error: 'not found' });
});

server.listen(3001, '127.0.0.1', () => {
	process.stdout.write('mock backend ready\n');
});
