import { getDatabase} from '../../config/orbitdb-service.js';

const databaseAddress = '/orbitdb/zdpuAwHqN76o8y2qjGVRXb82kKYJbnSLpYaawj95aWLprNJY7'
let publicKeysDB = null;

export async function getPublicKeysDB() {
    if (!publicKeysDB) {
        console.log('Fetching orbitdb from getDatabase...');
        publicKeysDB = await getDatabase(databaseAddress, 'documents');
    }
    console.log('Database Address:', publicKeysDB.address);

    return publicKeysDB;
}

export async function storePublicKey(userId, publicKey) {
    const db = await getPublicKeysDB();

    console.log('storePublicKey called for user_id: ', userId, 'and publicKey: ', publicKey);

    await db.put({ _id: publicKey, userId});
    console.log(`Stored public key for ${userId}`);
}

export async function getUserIdByPublicKey(publicKey) {
    const db = await getPublicKeysDB();

    console.log(`Looking up user_id for publicKey: ${publicKey}`);

    // Fetch the document by public key
    const result = await db.get(publicKey);

    if (result && result.value.userId) {
        console.log(`Found user_id: ${result.value.userId} for publicKey: ${publicKey}`);
        return result.value.userId;
    } else {
        console.log(`No user_id found for publicKey: ${publicKey}`);
        return null;
    }
}

export async function getAllPublicKeys() {
    const db = await getPublicKeysDB();

    const allDocs = await db.all();
    console.log('Raw documents from OrbitDB:', allDocs);

    // Extract only the document values
    const publicKeys = allDocs.map(entry => entry.value);
    console.log('Processed public keys:', publicKeys);

    return publicKeys;
}

export async function deletePublicKey(userId) {
    const db = await getPublicKeysDB();
    await db.delete(userId);
}