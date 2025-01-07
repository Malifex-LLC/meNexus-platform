import { createOrbitDB } from '@orbitdb/core';
import { createHelia } from 'helia';
import { createLibp2p } from 'libp2p';
import { LevelBlockstore } from 'blockstore-level'
import { bitswap } from '@helia/block-brokers'
import { gossipsub } from '@chainsafe/libp2p-gossipsub';
import { Identities, KeyStore } from '@orbitdb/core'
import { identify } from '@libp2p/identify';

let orbitdbInstance = null; // To hold the OrbitDB instance
let ipfsInstance = null;
let databases = {}; // To store references to opened databases

// Initialize OrbitDB and Libp2p/Helia
export async function initializeOrbitDB() {
    if (orbitdbInstance) {
        console.log('OrbitDB already initialized');
        return orbitdbInstance;
    }

    console.log('Initializing OrbitDB...');
    const libp2p = await createLibp2p({
        services: {
            pubsub: gossipsub({
                allowPublishToZeroTopicPeers: true, // Allow single peer for testing
            }),
            identify: identify(),
        },
    });

    const directory = '../../database'
    const blockstore = new LevelBlockstore(`${directory}/ipfs/blocks`)

    const keystorePath = '../../database/keystore';
    const keystore = await KeyStore({ path: keystorePath });

    const id = 'meNexus-orbitdb'
    const identities = await Identities({ keystore });
    const identity = identities.createIdentity({ id })

    ipfsInstance = await createHelia({ libp2p, blockstore, blockBrokers: [bitswap()] })

    orbitdbInstance = await createOrbitDB({
        ipfs: ipfsInstance,
        identities: identities,
        id: id,
        directory: directory,
    });

    console.log('OrbitDB initialized');

    return orbitdbInstance;
}

export function getIPFSInstance() {
    if (!ipfsInstance) {
        throw new Error('IPFS instance is not initialized. Call initializeOrbitDB first.');
    }
    return ipfsInstance;
}

// Open or create a database
export async function getDatabase(name, type = 'documents', options = {}) {
    if (!orbitdbInstance) {
        throw new Error('OrbitDB is not initialized. Call initializeOrbitDB first.');
    }

    if (databases[name]) {
        console.log(`Database "${name}" already opened.`);
        return databases[name];
    }

    console.log(`Opening database "${name}" of type "${type}"...`);
    const db = await orbitdbInstance.open(name, { type, ...options });
    databases[name] = db; // Store reference to the database
    console.log('Databases object:', databases);
    return db;
}

// Close a specific database
export async function closeDatabase(name) {
    if (databases[name]) {
        console.log(`Closing database "${name}"...`);
        await databases[name].close();
        delete databases[name];
    }
}

// Close all databases and OrbitDB
export async function closeOrbitDB() {
    console.log('Closing all databases and OrbitDB...');
    for (const name in databases) {
        await databases[name].close();
    }
    databases = {};

    if (orbitdbInstance) {
        await orbitdbInstance.stop();
        orbitdbInstance = null;
        console.log('OrbitDB stopped');
    }
}