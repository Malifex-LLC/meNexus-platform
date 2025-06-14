console.log('Initializing peerState...');
// console.log('peerState loaded from', import.meta.url);
// console.log(
//     'peerState loaded in PID', process.pid,
//     'from', import.meta.url,
//     'at', new Date().toISOString()
// );


export const discoveredPeers = new Map();
export const connectedPeers = new Map();