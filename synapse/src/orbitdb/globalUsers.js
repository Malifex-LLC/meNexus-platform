import dotenv from 'dotenv';
import path from 'path';
import { fileURLToPath } from 'url';

// Get __dirname equivalent in ESM
const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

// Load the .env file from synapse/src/api/config/.env
dotenv.config({ path: path.resolve(__dirname, '../../.env') });


import { getDatabase} from '#config/orbitdb-service.js';

const databaseAddress = process.env.GLOBAL_USERS_DB_ADDR
let globalUsersDB = null;

export async function getGlobalUsersDB() {
    if (!globalUsersDB) {
        console.log('Fetching orbitdb from getDatabase...');
        globalUsersDB = await getDatabase(databaseAddress, 'documents');
    }
    console.log('Database Address:', globalUsersDB.address);

    return globalUsersDB;
}

// Create a new user (called when user registers)
export async function createGlobalUser(publicKey, handle, displayName,synapsePublicKey) {
    const db = await getGlobalUsersDB();

    const userDoc = {
        _id: publicKey,
        publicKey: publicKey,
        handle,
        displayName,
        profileName: displayName,
        bio: 'Update your bio!',
        location: 'Update your location',
        profilePicture: '/uploads/profile_pictures/default_avatar.jpeg',
        profileBanner: '/assets/default_profile_banner.jpg',
        followers: [],
        following: [],
        synapses: [synapsePublicKey], // Will append Synapse publicKeys here when user logs in to other Synapses
        timestamp: new Date().toISOString(),
        is_online: false,
    };

    await db.put(userDoc);
    console.log(`Created user for publicKey ${publicKey}`);
    return userDoc;
}

// Fetch full user by publicKey
export async function getUserByPublicKeyFromDB(publicKey) {
    const db = await getGlobalUsersDB();

    console.log(`Looking up user for publicKey: ${publicKey}`);

    const result = await db.get(publicKey);

    if (result && result.value) {
        return result.value;
    } else {
        console.log(`No user found for publicKey: ${publicKey}`);
        return null;
    }
}

// // Fetch full user by handle
export async function getUserByHandleFromDB(handle) {
    const db = await getGlobalUsersDB();

    console.log(`Looking up user for handle: ${handle}`);

    const result = await db.query((doc) => doc.handle === handle);

    if (result) {
        return result[0];
    } else {
        console.log(`No user found for handle: ${handle}`);
        return null;
    }
}

// Add Synapse publicKey to joinedSynapses array
export async function addSynapseToUser(publicKey, synapsePublicKey) {
    const db = await getGlobalUsersDB();

    const existingUser = await db.get(publicKey);
    if (!existingUser || !existingUser.value) {
        console.log(`No existing user found for publicKey: ${publicKey}`);
        return;
    }

    const user = existingUser.value;

    if (!user.synapses.includes(synapsePublicKey)) {
        user.synapses.push(synapsePublicKey);
        await db.put(user);
        console.log(`Added Synapse ${synapsePublicKey} to user ${publicKey}`);
    }
}

// Return all users in DB
export async function getAllUsersFromDB() {
    const db = await getGlobalUsersDB();

    const allDocs = await db.all();
    const users = allDocs.map(entry => entry.value);

    return users;
}

// Delete user
export async function deleteUser(publicKey) {
    const db = await getGlobalUsersDB();
    await db.delete(publicKey);
}