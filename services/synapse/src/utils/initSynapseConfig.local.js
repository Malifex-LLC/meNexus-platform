// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import fs from 'fs/promises';
import path from 'path';
import os from 'os';
import { fileURLToPath } from 'url';
import dotenv from 'dotenv';
import { generateCryptoKeysUtil } from '#utils/cryptoUtils.js';
import { createSecp256k1PeerId } from '@libp2p/peer-id-factory';
import { base64pad } from 'multiformats/bases/base64';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

// Load local dev .env if present
const LOCAL_ENV = path.resolve(__dirname, '../../.env'); // services/synapse/.env
dotenv.config({ path: LOCAL_ENV });

/** Helpers */
function env(name, fallback) {
    return process.env[name] ?? fallback;
}

function toWs(url) {
    if (!url) return null;
    const u = new URL(url);
    u.protocol = (u.protocol === 'https:') ? 'wss:' : 'ws:';
    if (!u.pathname.endsWith('/ws')) u.pathname = (u.pathname.replace(/\/+$/, '') + '/ws');
    return u.toString();
}

async function exists(p) {
    try { await fs.access(p); return true; } catch { return false; }
}

async function ensureDirFor(filePath) {
    await fs.mkdir(path.dirname(filePath), { recursive: true });
}

async function writeSecure(filePath, contents) {
    await ensureDirFor(filePath);
    await fs.writeFile(filePath, contents, { mode: 0o600 });
}

async function writeJsonSecure(filePath, data) {
    await ensureDirFor(filePath);
    await fs.writeFile(filePath, JSON.stringify(data, null, 2), { mode: 0o600 });
}

function parseArgs() {
    const args = process.argv.slice(2);
    const outIdx = args.indexOf('--out');
    const nameIdx = args.indexOf('--name');
    const descIdx = args.indexOf('--desc');
    return {
        out: outIdx > -1 ? args[outIdx + 1] : path.resolve(__dirname, '../config/synapse-config.json'),
        force: args.includes('--force'),
        name: nameIdx > -1 ? args[nameIdx + 1] : undefined,
        desc: descIdx > -1 ? args[descIdx + 1] : undefined,
    };
}

async function main() {
    const { out, force, name, desc } = parseArgs();

    const EXPRESS_PORT = parseInt(env('EXPRESS_PORT', '3001'), 10);
    const LIBP2P_TCP   = parseInt(env('LIBP2P_TCP', '4001'), 10);
    const ORBITDB_TCP  = parseInt(env('ORBITDB_TCP', '4002'), 10);

    // Local-first defaults
    const PUBLIC_URL   = env('PUBLIC_URL', `http://localhost:${EXPRESS_PORT}`);
    const NAME         = name ?? env('SYNAPSE_NAME', `${os.hostname()}'s Synapse`);
    const DESC         = desc ?? env('SYNAPSE_DESC', 'Local development Synapse');

    const LIBP2P_BOOTSTRAP = (env('LIBP2P_BOOTSTRAP', '') || '')
        .split(',').map(s => s.trim()).filter(Boolean);
    const ORBITDB_BOOTSTRAP = (env('ORBITDB_BOOTSTRAP', '') || '')
        .split(',').map(s => s.trim()).filter(Boolean);

    // Respect existing config unless --force
    if (!force && await exists(out)) {
        console.log(`[local-init] Config already exists at: ${out}`);
        console.log(`Use --force to overwrite, or --out <path> to write elsewhere.`);
        return;
    }

    // Generate identity (kept local for dev)
    const { publicKey, privateKey } = await generateCryptoKeysUtil();
    const peerId = await createSecp256k1PeerId();
    const peerIdJson = {
        id: peerId.toString(),
        type: 'secp256k1',
        publicKey: base64pad.encode(peerId.publicKey),
        privateKey: base64pad.encode(peerId.privateKey),
    };

    // Store private key to a local, gitignored secrets file
    const DEFAULT_SECRET_FILE = path.resolve(__dirname, '../../secrets/synapse_private_key'); // services/synapse/secrets/...
    const KEY_FILE = env('SYNAPSE_PRIVATE_KEY_FILE', DEFAULT_SECRET_FILE);
    await writeSecure(KEY_FILE, privateKey + '\n');

    const synapseUrl   = PUBLIC_URL;
    const webSocketUrl = toWs(synapseUrl);

    // Bind to loopback by default for local dev (safer than 0.0.0.0)
    const cfg = {
        version: 1,
        identity: {
            publicKey,
            peerId: peerIdJson,
            synapseUrl,
            webSocketUrl,
        },
        libp2p: {
            multiaddrs: [
                `/ip4/127.0.0.1/tcp/${LIBP2P_TCP}`
            ],
            announce: [],                 // no public announce for local dev
            bootstrapPeers: LIBP2P_BOOTSTRAP
        },
        orbitdb: {
            multiaddrs: [
                `/ip4/127.0.0.1/tcp/${ORBITDB_TCP}`,
                `/ip4/127.0.0.1/udp/0/quic-v1`
            ],
            announce: [],                 // no public announce for local dev
            bootstrapPeers: ORBITDB_BOOTSTRAP
        },
        api: { port: String(EXPRESS_PORT) },
        metadata: {
            name: NAME,
            description: DESC
        },
        boards: (env('SYNAPSE_BOARDS','') ? env('SYNAPSE_BOARDS','').split(',').map(s=>s.trim())
            : ['Welcome','General','Announcements']),
        channels: (env('SYNAPSE_CHANNELS','') ? env('SYNAPSE_CHANNELS','').split(',').map(s=>s.trim())
            : ['Global','Random','Support'])
    };

    await writeJsonSecure(out, cfg);

    console.log(`[local-init] Wrote synapse-config.json → ${out}`);
    console.log(`[local-init] Wrote local private key   → ${KEY_FILE}`);
    console.log(JSON.stringify({
        identity: {
            peerId: cfg.identity.peerId.id,
            publicKey: cfg.identity.publicKey.slice(0, 10) + '…'
        },
        api: cfg.api,
        libp2p: { multiaddrs: cfg.libp2p.multiaddrs },
        orbitdb: { multiaddrs: cfg.orbitdb.multiaddrs },
        metadata: cfg.metadata
    }, null, 2));
}

main().catch(err => {
    console.error('initSynapseConfig.local failed:', err);
    process.exit(1);
});
