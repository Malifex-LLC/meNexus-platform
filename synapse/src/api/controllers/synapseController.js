import { loadConfig, saveConfig } from '#utils/configUtils.js';
import path from 'path';
import { fileURLToPath } from 'url';

// Get __dirname equivalent in ESM
const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const CONFIG_FILE = path.resolve(__dirname, '../../config/synapse-config.json');

// console.log('peerStateManager instance:', import.meta.url);
// console.log('peerStateManager instance in messenger:', peerStateManager);

export const getSynapseMetadata = async (req, res) => {
    try {
        const metadata = await loadConfig(CONFIG_FILE);
        res.status(200).json(metadata);
    } catch (err) {
        console.error('Error fetching Local Synapse Metadata:', err);
        res.status(500).json({error: 'Failed to fetch Metadata from the Synapse '});
    }
}

export default {
    getSynapseMetadata,
}