import { verifyAccessToken } from '#utils/jwtUtils.js';

export const requireJwt = (scopes = []) => async (req, res, next) => {
    try {
        console.log('requireJwt authorization headers: ', req.headers.authorization);
        const auth = req.headers.authorization || '';
        if (!auth.startsWith('Bearer ')) {
            return res.status(401).json({ error: 'Missing bearer token' });
        }
        const token = auth.slice(7);
        const payload = await verifyAccessToken(token);
        console.log('requireJwt payload: ', payload);

        if (scopes.length && !scopes.every(s => payload.scopes?.includes(s))) {
            return res.status(403).json({ error: 'forbidden' });
        }

        // Attach identity from token
        req.user = { publicKey: payload.pubkey, sub: payload.sub, scopes: payload.scopes || [] };
        next();
    } catch (e) {
        return res.status(401).json({ error: 'invalid or expired token' });
    }
};
