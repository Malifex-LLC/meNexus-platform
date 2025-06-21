import { createLibp2p } from 'libp2p';
import { tcp } from '@libp2p/tcp';
import { webSockets } from '@libp2p/websockets';
import { noise } from '@chainsafe/libp2p-noise';
import { mplex } from '@libp2p/mplex';
import { gossipsub } from '@chainsafe/libp2p-gossipsub';
import { bootstrap } from '@libp2p/bootstrap';
import { mdns } from '@libp2p/mdns';
import { identify } from '@libp2p/identify';

// Configure libp2p2Instance for use by Synapse
export const createLibp2pInstance = async () => {
    return await createLibp2p({
        addresses: {
            listen: [
                '/ip4/0.0.0.0/tcp/4001',
                //'/ip4/0.0.0.0/ws'
            ],
        },
        transports: [
            tcp(),
            //webSockets()
        ],
        connectionEncrypters: [noise()],
        streamMuxers: [mplex()],
        services: {
            pubsub: gossipsub({
                emitSelf: false,
                gossipInterval: 2000,
                heartbeatInterval: 1000,
            }),
            identify: identify(),
        },
        peerDiscovery: [
            bootstrap({
                list: [
                    '/ip4/192.168.1.60/tcp/4001/p2p/12D3KooWMgmyd1zh7pB4srggFNiKVb1BbzWzKfBtUnEzbZds8iSe',
                    '/ip4/192.168.1.188/tcp/4001/p2p/12D3KooWPPHrMDJBJHwxcKgkWSLwphZS9V3X3wDAthpYe1CiDo6Z',
                ],
            }),
            mdns({
                interval: 2000,
            }),
        ],
        connectionManager: {
            minConnections: 5,
            maxConnections: 20,
        },
        dialer: {
            maxParallelDials: 10,
            maxDialsPerPeer: 2,
            dialTimeout: 30000,
        },
        relay: {
            enabled: true,
            hop: {
                enabled: true,
            },
        },
        nat: {
            enabled: true,
        },
    });
};
