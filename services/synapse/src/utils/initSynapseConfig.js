// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import fs from 'fs/promises';
import path from 'path';
import os from 'os';
import { fileURLToPath } from 'url';
import { generateCryptoKeysUtil } from '#utils/cryptoUtils.js';
import { createSecp256k1PeerId } from '@libp2p/peer-id-factory';
import { base64pad } from 'multiformats/bases/base64';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

function env(name, fallback) {
    return process.env[name] ?? fallback;
}

function toWs(url) {
    // http -> ws, https -> wss; append /ws if needed
    if (!url) return null;
    const u = new URL(url);
    u.protocol = (u.protocol === 'https:') ? 'wss:' : 'ws:';
    if (!u.pathname.endsWith('/ws')) u.pathname = (u.pathname.replace(/\/+$/,'') + '/ws');
    return u.toString();
}

function announceAddrs(ip, tcpPort, peerId) {
    if (!ip || !tcpPort || !peerId) return [];
    return [`/ip4/${ip}/tcp/${tcpPort}/p2p/${peerId}`];
}

async function ensureDir(p) {
    await fs.mkdir(path.dirname(p), { recursive: true });
}

async function writeSecure(file, contents) {
    await ensureDir(file);
    await fs.writeFile(file, contents, { mode: 0o600 });
}

async function writeJsonSecure(file, data) {
    await fs.mkdir(path.dirname(file), { recursive: true });
    await fs.writeFile(file, JSON.stringify(data, null, 2), { mode: 0o600 });
}

async function main() {
    const outFlagIndex = process.argv.indexOf('--out');
    const outPath = outFlagIndex > -1
        ? process.argv[outFlagIndex + 1]
        : '/app/services/synapse/src/config/synapse-config.json';

    const EXPRESS_PORT = parseInt(env('EXPRESS_PORT', '3001'), 10);
    const LIBP2P_TCP   = parseInt(env('LIBP2P_TCP', '4001'), 10);
    const ORBITDB_TCP  = parseInt(env('ORBITDB_TCP', '4002'), 10);

    // If provided, we can fully populate announce addrs
    const EXTERNAL_IP  = env('EXTERNAL_IP', '');
    const PUBLIC_URL   = env('PUBLIC_URL', '');
    const NAME         = env('SYNAPSE_NAME', os.hostname());
    const DESC         = env('SYNAPSE_DESC', 'Synapse instance');

    const LIBP2P_BOOTSTRAP = (env('LIBP2P_BOOTSTRAP', '') || '')
        .split(',').map(s => s.trim()).filter(Boolean);
    const ORBITDB_BOOTSTRAP  = (env('ORBITDB_BOOTSTRAP', '') || '')
        .split(',').map(s => s.trim()).filter(Boolean);

    // Generate identity
    const { publicKey, privateKey } = await generateCryptoKeysUtil();
    const peerId = await createSecp256k1PeerId();
    const peerIdJson = {
        id: peerId.toString(),
        type: 'secp256k1',
        publicKey: base64pad.encode(peerId.publicKey),
        privateKey: base64pad.encode(peerId.privateKey)
    };

    // Export Synapse private key to secret
    const KEY_FILE = env('SYNAPSE_PRIVATE_KEY_FILE', '/data/secrets/synapse_private_key');
    await writeSecure(KEY_FILE, privateKey + '\n');
    console.log(`[generator] Wrote Synapse private key to ${KEY_FILE}`);

    // If PUBLIC_URL set, use it; otherwise default to localhost
    const synapseUrl   = PUBLIC_URL || `http://localhost:${EXPRESS_PORT}`;
    const webSocketUrl = toWs(synapseUrl);

    const cfg = {
        version: 1, // bump on breaking changes to support future migrations
        identity: {
            publicKey,
            peerId: peerIdJson,
            synapseUrl,
            webSocketUrl
        },
        libp2p: {
            multiaddrs: [
                `/ip4/0.0.0.0/tcp/${LIBP2P_TCP}`
            ],
            announce: announceAddrs(EXTERNAL_IP, LIBP2P_TCP, peerId.toString()),
            bootstrapPeers: LIBP2P_BOOTSTRAP
        },
        orbitdb: {
            multiaddrs: [
                `/ip4/0.0.0.0/tcp/${ORBITDB_TCP}`,
                `/ip4/0.0.0.0/udp/0/quic-v1`
            ],
            announce: announceAddrs(EXTERNAL_IP, ORBITDB_TCP, peerId.toString()),
            bootstrapPeers: ORBITDB_BOOTSTRAP
        },
        api: { port: String(EXPRESS_PORT) },
        metadata: {
            name: NAME,
            description: DESC
        },
        boards: (env('SYNAPSE_BOARDS', '') ? env('SYNAPSE_BOARDS','').split(',').map(s=>s.trim()) :
            ['Welcome', 'General', 'Announcements']),
        channels: (env('SYNAPSE_CHANNELS','') ? env('SYNAPSE_CHANNELS','').split(',').map(s=>s.trim()) :
            ['Global','Random','Support'])
    };

    await writeJsonSecure(outPath, cfg);

    // Print safe summary
    console.log('Generated synapse-config.json at', outPath);
    console.log(JSON.stringify({
        identity: {
            peerId: cfg.identity.peerId.id,
            publicKey: cfg.identity.publicKey.slice(0,10) + '…'
        },
        libp2p: { announce: cfg.libp2p.announce },
        orbitdb: { announce: cfg.orbitdb.announce },
        api: cfg.api,
        metadata: cfg.metadata
    }, null, 2));
}

main().catch(err => {
    console.error('init-config failed:', err);
    process.exit(1);
});
