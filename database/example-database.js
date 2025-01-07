import { initializeOrbitDB } from './orbitdb-service.js';

(async () => {
    const orbitdb = await initializeOrbitDB();

    // Create or open a database
    const db = await orbitdb.open('meNexus-decentralized-database', { type: 'keyvalue' });
    console.log('Database address:', db.address.toString());

    // Add an entry
    await db.put('publicKey', 'example-key');
    console.log('Added entry: publicKey = example-key');

    // Retrieve the entry
    const value = await db.get('publicKey');
    console.log('Retrieved value:', value);

    // Listen for updates
    db.events.on('update', async (entry) => {
        console.log('Database updated:', entry);
        const all = db.all();
        console.log('All entries:', all);
    });

    // Clean up
    await db.close();
})();
