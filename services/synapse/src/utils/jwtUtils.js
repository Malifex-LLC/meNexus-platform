// utils/jwtUtils.js
import { SignJWT, jwtVerify } from 'jose';

const ISS = process.env.PUBLIC_URL;
const AUD = 'menexus-client';

// Secrets must be Uint8Array for jose
const ACCESS_SECRET  = new TextEncoder().encode(process.env.JWT_SECRET);
const REFRESH_SECRET = new TextEncoder().encode(process.env.JWT_REFRESH_SECRET);

// Access: short-lived (e.g., 10m)
export async function mintAccessToken({ userPk, scopes = [] }) {
    return await new SignJWT({ sub: userPk, pubkey: userPk, scopes })
        .setProtectedHeader({ alg: 'HS256' })
        .setIssuer(ISS).setAudience(AUD)
        .setIssuedAt().setExpirationTime('10m')
        .sign(ACCESS_SECRET);
}

export async function verifyAccessToken(token) {
    const { payload } = await jwtVerify(token, ACCESS_SECRET, {
        issuer: ISS, audience: AUD, clockTolerance: '90s',
    });
    return payload; // { sub, pubkey, scopes, iat, exp }
}

// Refresh: long-lived (e.g., 7d)
export async function mintRefreshToken({ userPk, scopes = [] }) {
    return await new SignJWT({ sub: userPk, pubkey: userPk, scopes })
        .setProtectedHeader({ alg: 'HS256' })
        .setIssuer(ISS).setAudience(AUD)
        .setIssuedAt().setExpirationTime('7d')
        .sign(REFRESH_SECRET);
}

export async function verifyRefreshToken(token) {
    const { payload } = await jwtVerify(token, REFRESH_SECRET, {
        issuer: ISS, audience: AUD, clockTolerance: '90s',
    });
    return payload; // { sub, pubkey, scopes, ... }
}
