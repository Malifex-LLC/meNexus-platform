// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import * as peerStateManager from '#core/peerStateManager.js';
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

export const getAllDiscoveredPeers = async (req, res) => {
    const discoveredPeers = await peerStateManager.getAllDiscoveredPeers();
    const peersJSON = Object.fromEntries(discoveredPeers)
    console.log("Discovered Peers response:", peersJSON);
    res.status(200).json(peersJSON);
}

export const getSynapsePostBoards = async (req, res) => {
    try {
        const config = await loadConfig(CONFIG_FILE);
        const boards = config.boards;
        res.status(200).json(boards);
    } catch (err) {
        console.error('Error fetching Post Boards:', err);
        res.status(500).json({error: 'Failed to fetch Post Boards'});
    }
}

export const getSynapseChatChannels = async (req, res) => {
    try {
        const config = await loadConfig(CONFIG_FILE);
        const boards = config.channels;
        res.status(200).json(boards);
    } catch (err) {
        console.error('Error fetching chat channels:', err);
        res.status(500).json({error: 'Failed to fetch chat channels'});
    }
}

export default {
    getSynapseMetadata,
    getAllDiscoveredPeers,
    getSynapsePostBoards,
    getSynapseChatChannels
}