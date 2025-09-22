import Synapse from '../models/synapse.js'
import activityController from "#api/controllers/activityController.js";
import broadcastController from "#api/controllers/broadcastController.js";
import { loadConfig, saveConfig } from '#utils/configUtils.js';
import path from 'path';
import { fileURLToPath } from 'url';
// Get __dirname equivalent in ESM
const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const CONFIG_FILE = path.resolve(__dirname, '../../config/synapse-config.json');

export const joinSynapse = async (publicKey) => {
    const metadata = await loadConfig(CONFIG_FILE);
    if (!metadata) {
        console.error(`No metadata found in joinSynapse`);
    }
    const result = await Synapse.addSynapseMember(publicKey);
    const activity = await activityController.createJoinSynapseActivity(publicKey, metadata.identity.publicKey);
    broadcastController.broadcastActivity(activity);
    return result;
}
export const leaveSynapse = async (publicKey) => {
    const metadata = await loadConfig(CONFIG_FILE);
    if (!metadata) {
        console.error(`No metadata found in leaveSynapse`);
    }
    const result =await Synapse.removeSynapseMember(publicKey);
    const activity = await activityController.createLeaveSynapseActivity(publicKey, metadata.identity.publicKey);
    broadcastController.broadcastActivity(activity);
    return result;
}

export default {
    joinSynapse,
    leaveSynapse,
}