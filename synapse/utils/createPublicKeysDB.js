import { initializeOrbitDB } from '../config/orbitdb-service.js';

const db = await initializeOrbitDB().then(async (ob) => {
    const db = await ob.open('meNexus-publicKeys', {
        type: 'documents',
        indexBy: '_id',
        accessController: {
            type: 'orbitdb',
            write: ['*']          // 👈 open ACL so any Synapse identity may append
        }
    });
    //await db.load();// force manifest/head write
    console.log('✔️  New DB address:', db.address.toString());
    process.exit(0);
});
