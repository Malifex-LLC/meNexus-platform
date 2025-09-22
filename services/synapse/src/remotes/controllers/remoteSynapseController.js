// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

// TODO Need to fix so that the privateKey is not included in the Synapse Metadata returned
import { getGlobalUsersDB } from "#src/orbitdb/globalUsers.js";
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

export const fetchRemoteSynapseMembers = async (req, res) => {
    const synapsePublicKey = req.query.synapsePublicKey;
    if (!synapsePublicKey) {
        return res.status(401).json({error: 'No Synapse publicKey provided.'});
    }
    console.log('fetchRemoteSynapseMembers for synapsePublicKey: ', synapsePublicKey)

    const peer = peerStateManager.getPeerByPublicKey(synapsePublicKey);
    if (!peer || !peer.peerId) {
        return res.status(401).json({ error: 'No peerId returned from peerStateManager.' });
    }
    const { peerId } = peer;
    const synapseMembersRequest = createMessage(
        MESSAGE_TYPES.DATA.REQUEST,
        ACTION_TYPES.DATA.QUERY,
        RESOURCE_TYPES.SYNAPSE_MEMBERS,
        {},
        {sender: process.env.PUBLIC_KEY}
    );
    try {
        const response = await sendMessageWithResponse(peerId, synapseMembersRequest);
        res.status(200).json(response.payload);
    } catch (err) {
        console.error('Error fetching Synapse members:', err);
        res.status(500).json({error: 'Failed to fetch members from the synapse.'});
    }
}

export const fetchRemoteSynapseAllActivities = async (req, res) => {
    const synapsePublicKey = req.query.synapsePublicKey;
    if (!synapsePublicKey) {
        return res.status(401).json({error: 'No Synapse publicKey provided.'});
    }
    console.log('fetchRemoteSynapseAllActivities for synapsePublicKey: ', synapsePublicKey)

    const peer = peerStateManager.getPeerByPublicKey(synapsePublicKey);
    if (!peer || !peer.peerId) {
        return res.status(401).json({ error: 'No peerId returned from peerStateManager.' });
    }
    const { peerId } = peer;
    const synapseActivityRequest = createMessage(
        MESSAGE_TYPES.DATA.REQUEST,
        ACTION_TYPES.DATA.QUERY,
        RESOURCE_TYPES.SYNAPSE_ACTIVITIES,
        {},
        {sender: process.env.PUBLIC_KEY}
    );
    try {
        const response = await sendMessageWithResponse(peerId, synapseActivityRequest);
        res.status(200).json(response.payload);
    } catch (err) {
        console.error('Error fetching Synapse activity:', err);
        res.status(500).json({error: 'Failed to fetch activity from the synapse.'});
    }
}

export const fetchRemoteSynapseUserActivities = async (req, res) => {
    const synapsePublicKey = req.query.synapsePublicKey;
    const publicKey = req.query.publicKey;
    if (!synapsePublicKey || !publicKey) {
        return res.status(401).json({error: 'No Synapse publicKey or user publicKey provided.'});
    }
    console.log('fetchRemoteSynapseUserActivities for synapsePublicKey: ', synapsePublicKey)

    const peer = peerStateManager.getPeerByPublicKey(synapsePublicKey);
    if (!peer || !peer.peerId) {
        return res.status(401).json({ error: 'No peerId returned from peerStateManager.' });
    }
    const { peerId } = peer;
    const synapseActivityRequest = createMessage(
        MESSAGE_TYPES.DATA.REQUEST,
        ACTION_TYPES.DATA.QUERY,
        RESOURCE_TYPES.SYNAPSE_USER_ACTIVITIES,
        {publicKey},
        {sender: process.env.PUBLIC_KEY}
    );
    try {
        const response = await sendMessageWithResponse(peerId, synapseActivityRequest);
        res.status(200).json(response.payload);
    } catch (err) {
        console.error('Error fetching Synapse activity:', err);
        res.status(500).json({error: 'Failed to fetch activity from the synapse.'});
    }
}

export const fetchRemoteSynapsePostBoards = async (req, res) => {
    const synapsePublicKey = req.query.synapsePublicKey;
    if (!synapsePublicKey) {
        return res.status(401).json({error: 'No Synapse publicKey provided.'});
    }
    console.log('fetchRemoteSynapsePostBoards for synapsePublicKey: ', synapsePublicKey)

    const peer = peerStateManager.getPeerByPublicKey(synapsePublicKey);
    if (!peer || !peer.peerId) {
        return res.status(401).json({ error: 'No peerId returned from peerStateManager.' });
    }
    const { peerId } = peer;
    const synapsePostBoardsRequest = createMessage(
        MESSAGE_TYPES.DATA.REQUEST,
        ACTION_TYPES.DATA.QUERY,
        RESOURCE_TYPES.SYNAPSE_POST_BOARDS,
        {},
        {sender: process.env.PUBLIC_KEY}
    );
    try {
        const response = await sendMessageWithResponse(peerId, synapsePostBoardsRequest);
        res.status(200).json(response.payload);
    } catch (err) {
        console.error('Error fetching Synapse post boards:', err);
        res.status(500).json({error: 'Failed to fetch post boards from the synapse.'});
    }
}

export const fetchRemoteSynapseChatChannels = async (req, res) => {
    const synapsePublicKey = req.query.synapsePublicKey;
    if (!synapsePublicKey) {
        return res.status(401).json({error: 'No Synapse publicKey provided.'});
    }
    console.log('fetchRemoteSynapseChatChannels for synapsePublicKey: ', synapsePublicKey)

    const peer = peerStateManager.getPeerByPublicKey(synapsePublicKey);
    if (!peer || !peer.peerId) {
        return res.status(401).json({ error: 'No peerId returned from peerStateManager.' });
    }
    const { peerId } = peer;
    const synapsePostBoardsRequest = createMessage(
        MESSAGE_TYPES.DATA.REQUEST,
        ACTION_TYPES.DATA.QUERY,
        RESOURCE_TYPES.SYNAPSE_CHAT_CHANNELS,
        {},
        {sender: process.env.PUBLIC_KEY}
    );
    try {
        const response = await sendMessageWithResponse(peerId, synapsePostBoardsRequest);
        res.status(200).json(response.payload);
    } catch (err) {
        console.error('Error fetching Synapse post boards:', err);
        res.status(500).json({error: 'Failed to fetch post boards from the synapse.'});
    }
}

export const joinRemoteSynapse = async (req, res) => {
    if (!req.user) {
        console.log("User not authenticated or session missing");
        return res.status(401).json({ error: "User not authenticated" });
    }

    const publicKey = req.user?.publicKey;
    const {synapsePublicKey} = req.query;
    if (!publicKey || !synapsePublicKey) {
        return res.status(401).json({error: 'No user publicKey or Synapse publicKey provided.'});
    }
    const peer = peerStateManager.getPeerByPublicKey(synapsePublicKey);
    if (!peer || !peer.peerId) {
        return res.status(401).json({ error: 'No peerId returned from peerStateManager.' });
    }
    const { peerId } = peer;
    const joinRemoteSynapseRequest = createMessage(
        MESSAGE_TYPES.DATA.REQUEST,
        ACTION_TYPES.RESOURCE.UPDATE,
        RESOURCE_TYPES.SYNAPSE_MEMBERS,
        { publicKey },
        {sender: process.env.PUBLIC_KEY}
    );
    try {
        const response = await sendMessageWithResponse(peerId, joinRemoteSynapseRequest);
        res.status(200).json(response.payload);
    } catch (err) {
        console.error('Error joining remote Synapse :', err);
        res.status(500).json({error: 'Failed to join remote Synapse.'});
    }

}

export const leaveRemoteSynapse = async (req, res) => {
    if (!req.user) {
        console.log("User not authenticated or session missing");
        return res.status(401).json({ error: "User not authenticated" });
    }

    const publicKey = req.user?.publicKey;
    const {synapsePublicKey} = req.query;
    console.log('leaveRemoteSynapse request publicKey: ', publicKey, ' synapsePublicKey: ', synapsePublicKey);
    if (!publicKey || !synapsePublicKey) {
        return res.status(401).json({error: 'No user publicKey or Synapse publicKey provided.'});
    }
    const peer = peerStateManager.getPeerByPublicKey(synapsePublicKey);
    if (!peer || !peer.peerId) {
        return res.status(401).json({ error: 'No peerId returned from peerStateManager.' });
    }
    const { peerId } = peer;
    const leaveRemoteSynapseRequest = createMessage(
        MESSAGE_TYPES.DATA.REQUEST,
        ACTION_TYPES.RESOURCE.DELETE,
        RESOURCE_TYPES.SYNAPSE_MEMBERS,
        { publicKey },
        {sender: process.env.PUBLIC_KEY}
    );
    try {
        const response = await sendMessageWithResponse(peerId, leaveRemoteSynapseRequest);
        res.status(200).json(response.payload);
    } catch (err) {
        console.error('Error leaving remote Synapse :', err);
        res.status(500).json({error: 'Failed to leave remote Synapse.'});
    }

}

export default {
    fetchRemoteSynapseMetadata,
    fetchRemoteSynapseMembers,
    fetchRemoteSynapseAllActivities,
    fetchRemoteSynapseUserActivities,
    fetchRemoteSynapsePostBoards,
    fetchRemoteSynapseChatChannels,
    joinRemoteSynapse,
    leaveRemoteSynapse,
}