let accessToken = null;

export function setAccessToken(token) {
    accessToken = token;
}

export function getAccessToken() {
    return accessToken;
}

export function clearAccessToken() {
    accessToken = null;
}

export async function apiFetch(url, opts={}) {
    const headers = { ...(opts.headers || {}) };
    const t = getAccessToken();
    if (t) headers.Authorization = `Bearer ${t}`;

    let res = await fetch(url, { ...opts, headers, credentials: 'include' }); // include for refresh cookie
    if (res.status === 401) {
        const r = await fetch('/api/auth/refresh', { method: 'POST', credentials: 'include' });
        if (r.ok) {
            const j = await r.json();
            setAccessToken(j.accessToken);
            res = await fetch(url, { ...opts, headers: { ...headers, Authorization: `Bearer ${j.accessToken}` }, credentials: 'include' });
        }
    }
    return res;
}