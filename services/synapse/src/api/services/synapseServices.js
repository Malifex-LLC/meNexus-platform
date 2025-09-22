import Synapse from '../models/synapse.js'
import activityController from "#api/controllers/activityController.js";
import broadcastController from "#api/controllers/broadcastController.js";
import { loadConfig, saveConfig } from '#utils/configUtils.js';
import path from 'path';
import { fileURLToPath } from 'url';
import {getGlobalUsersDB} from "#src/orbitdb/globalUsers.js";
// Get __dirname equivalent in ESM
const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const CONFIG_FILE = path.resolve(__dirname, '../../config/synapse-config.json');

export const joinSynapse = async (publicKey) => {
    const metadata = await loadConfig(CONFIG_FILE);
    if (!metadata) {
        console.error(`No metadata found in joinSynapse`);
    }
    const db = await getGlobalUsersDB();
    const [updatedUser] = await db.query(doc => doc._id === publicKey);
    if(updatedUser) {
        try {
            updatedUser.synapses.push(metadata.identity.publicKey);
            await db.put(updatedUser);
            const result = await Synapse.addSynapseMember(publicKey);
            const activity = await activityController.createJoinSynapseActivity(publicKey, metadata.identity.publicKey);
            broadcastController.broadcastActivity(activity);
            return result;
        } catch (err) {
            console.error('Error joining Synapse: ', err);
        }
    }

}
export const leaveSynapse = async (publicKey) => {
    const metadata = await loadConfig(CONFIG_FILE);
    if (!metadata) {
        console.error(`No metadata found in leaveSynapse`);
    }

    const db = await getGlobalUsersDB();
    const [updatedUser] = await db.query(doc => doc._id === publicKey);
    if(updatedUser) {
        try {
            updatedUser.synapses = updatedUser.synapses.filter(synapse => synapse !== metadata.identity.publicKey);
            await db.put(updatedUser);
            const result =await Synapse.removeSynapseMember(publicKey);
            const activity = await activityController.createLeaveSynapseActivity(publicKey, metadata.identity.publicKey);
            broadcastController.broadcastActivity(activity);
            return result;
        } catch (err) {
            console.error('Error leaving Synapse: ', err);
        }
    }

}

export default {
    joinSynapse,
    leaveSynapse,
}