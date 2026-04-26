import http from 'k6/http';
import { check, sleep } from 'k6';
import { Rate } from 'k6/metrics';

const errorRate = new Rate('errors');

export const options = {
  scenarios: {
    smoke: {
      executor: 'constant-vus',
      vus: 10,
      duration: '30s',
    },
    load: {
      executor: 'ramping-vus',
      startVUs: 0,
      stages: [
        { duration: '30s', target: 100 },
        { duration: '1m', target: 100 },
        { duration: '30s', target: 0 },
      ],
    },
  },
  thresholds: {
    http_req_duration: ['p(95)<500', 'p(99)<1000'],
    http_req_failed: ['rate<0.01'],
  },
};

const BASE_URL = __ENV.BASE_URL || 'http://localhost:8080';
const EMAIL = __ENV.EMAIL || 'test@example.com';
const PASSWORD = __ENV.PASSWORD || 'testpassword123';

let token = '';
let userId = '';

export function setup() {
  const loginRes = http.post(`${BASE_URL}/auth/login`, JSON.stringify({
    email: EMAIL,
    password: PASSWORD,
  }), {
    headers: { 'Content-Type': 'application/json' },
  });

  if (loginRes.status === 200) {
    const body = JSON.parse(loginRes.body);
    token = body.data.token;
  }

  return { token };
}

export default function (data) {
  const params = {
    headers: {
      'Content-Type': 'application/json',
      'Authorization': `Bearer ${data.token}`,
    },
  };

  // Test list users
  const listRes = http.get(`${BASE_URL}/users?limit=20`, params);
  check(listRes, {
    'list users status is 200': (r) => r.status === 200,
    'list users has data': (r) => JSON.parse(r.body).data !== undefined,
  }) || errorRate.add(1);
  sleep(0.1);

  // Test get user by id (if we have a user id)
  if (userId) {
    const getRes = http.get(`${BASE_URL}/users/${userId}`, params);
    check(getRes, {
      'get user status is 200': (r) => r.status === 200,
    }) || errorRate.add(1);
  }
  sleep(0.1);

  // Test health endpoint
  const healthRes = http.get(`${BASE_URL}/health`);
  check(healthRes, {
    'health status is 200': (r) => r.status === 200,
  }) || errorRate.add(1);
  sleep(0.1);
}

export function handleSummary(data) {
  return {
    stdout: textSummary(data, { indent: ' ', produce: true }),
    './summary.json': JSON.stringify(data, null, 2),
  };
}

function textSummary(data, options) {
  const summary = {
    'Test Duration': data.metrics.duration.values['count'],
    'Requests': data.metrics.http_reqs.values['count'],
    'RPS': Math.round(data.metrics.http_reqs.values['rate']),
    'Avg Response Time': Math.round(data.metrics.http_req_duration.values['avg']) + 'ms',
    'P95 Response Time': Math.round(data.metrics.http_req_duration.values['p(95)']) + 'ms',
    'P99 Response Time': Math.round(data.metrics.http_req_duration.values['p(99)']) + 'ms',
    'Error Rate': Math.round(data.metrics.errors.values['rate'] * 100) + '%',
  };
  return JSON.stringify(summary, null, options.indent);
}