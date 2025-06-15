// scripts/createPublicKeyDB.js
import { initializeOrbitDB } from '../config/orbitdb-service.js';
import { IPFSAccessController } from '@orbitdb/core';

try {
    const orbitdb = await initializeOrbitDB();

    // 1. Create (or open-if-already-there) with wide-open ACL
    console.log("Opening meNexus-publicKeys-v2")
    const db = await orbitdb.open('meNexus-publicKeys-v2', {
        type: 'documents',
        indexBy: '_id',
        AccessController: IPFSAccessController({ write: ['*'] }),
        create: true,
        overwrite: true,
    });
    console.log("meNexus-publicKeys-v2 opened")

    console.log('\n✔️  New DB address:', db.address.toString(), '\n');

    await db.close();        // flush head to disk
    await orbitdb.stop();    // clean shutdown
    process.exit(0);

} catch (err) {
    console.error('Failed to create public-keys DB:', err);
    process.exit(1);
}
