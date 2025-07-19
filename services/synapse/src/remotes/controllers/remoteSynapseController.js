// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

// TODO Need to fix so that the privateKey is not included in the Synapse Metadata returned
import {getGlobalUsersDB} from "#src/orbitdb/globalUsers.js";
import { MESSAGE_TYPES, ACTION_TYPES, RESOURCE_TYPES } from "#protocols/snp/index.js";
import { createMessage } from '#protocols/snp/messageUtils.js'
import { sendMessageWithResponse } from "#core/messenger.js";
import { loadConfig, saveConfig } from '#utils/configUtils.js';
import * as peerStateManager from '#src/core/peerStateManager.js'

import path from 'path';
import { fileURLToPath } from 'url';

// Get __dirname equivalent in ESM
const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const CONFIG_FILE = path.resolve(__dirname, '../../config/synapse-config.json');

export const fetchRemoteSynapseMetadata = async (req, res) => {
    const synapsePublicKey = req.query.publicKey;
    if (!synapsePublicKey) {
        return res.status(401).json({error: 'No Synapse publicKey provided.'});
    }
    console.log('getSynapseMetadata for synapsePublicKey: ', synapsePublicKey)
    const localMetadata = await loadConfig(CONFIG_FILE);
    if (synapsePublicKey === localMetadata.identity.publicKey) {
        return res.status(200).json(localMetadata);
    }
    const peer = peerStateManager.getPeerByPublicKey(synapsePublicKey);
    if (!peer || !peer.peerId) {
        return res.status(401).json({ error: 'No peerId returned from peerStateManager.' });
    }
    const { peerId } = peer;
    const metadataRequest = createMessage(
        MESSAGE_TYPES.DATA.REQUEST,
        ACTION_TYPES.DATA.QUERY,
        RESOURCE_TYPES.SYNAPSE_METADATA,
        {},
        {sender: process.env.PUBLIC_KEY}
    );
    try {
        const response = await sendMessageWithResponse(peerId, metadataRequest);
        res.status(200).json(response.payload.localMetadata);
    } catch (err) {
        console.error('Error fetching Synapse metadata:', err);
        res.status(500).json({error: 'Failed to fetch metadata from the synapse.'});
    }
}

export const joinSynapse = async (req, res) => {
    const {publicKey, synapsePublicKey} = req.query;
    if (!publicKey || !synapsePublicKey) {
        return res.status(401).json({error: 'No user publicKey or Synapse publicKey provided.'});
    }
    const db = await getGlobalUsersDB();
    const [updatedUser] = await db.query(doc => doc._id === publicKey);
    if(updatedUser) {
        try {
            updatedUser.synapses.push(synapsePublicKey);
            await db.put(updatedUser);
            res.status(200).json(updatedUser);
        } catch (err) {
            console.error('Error joining Synapse: ', err);
        }
    }
}

export const leaveSynapse = async (req, res) => {
    const {publicKey, synapsePublicKey} = req.query;
    if (!publicKey || !synapsePublicKey) {
        return res.status(401).json({error: 'No user publicKey or Synapse publicKey provided.'});
    }
    const db = await getGlobalUsersDB();
    const [updatedUser] = await db.query(doc => doc._id === publicKey);
    if(updatedUser) {
        try {
            updatedUser.synapses = updatedUser.synapses.filter(synapse => synapse !== synapsePublicKey);
            await db.put(updatedUser);
            res.status(200).json(updatedUser);
        } catch (err) {
            console.error('Error leaving Synapse: ', err);
        }
    }
}

export default {
    fetchRemoteSynapseMetadata,
    joinSynapse,
    leaveSynapse,
}