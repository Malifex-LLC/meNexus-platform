// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

console.log('Initializing peerState...');
// console.log('peerState loaded from', import.meta.url);
// console.log(
//     'peerState loaded in PID', process.pid,
//     'from', import.meta.url,
//     'at', new Date().toISOString()
// );


export const discoveredPeers = new Map();
export const connectedPeers = new Map();