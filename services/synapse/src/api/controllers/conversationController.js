// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

// Import Conversation model
import Conversation from '../models/conversation.js';

export const getConversations = async (req, res) => {
    if (!req.session || !req.session.user) {
        console.log("User not authenticated or session missing");
        return res.status(401).json({ error: "User not authenticated" });
    }
    const { publicKey } = req.session.user;
    console.log('/getConversations called for publicKey: ', publicKey);

    if(!publicKey) {
        return res.status(401).json({ error: 'User not authenticated' });
    }

    const results = await Conversation.getConversations(publicKey);
    res.status(200).json(results);
}

export const createConversation = async (req, res) => {
    if (!req.session || !req.session.user) {
        console.log("User not authenticated or session missing");
        return res.status(401).json({ error: "User not authenticated" });
    }
    const { publicKey } = req.session.user;
    console.log("/createConversation called for participants");

    const result = await Conversation.createConversation(publicKey);

    if(result.affectedRows > 0) {
        res.status(500).json({error: "Failed to create conversation"});
    }

    return res.status(200).json(result);
}

export const updateConversationParticipants = async (req, res) => {
    const newParticipantsHandle = req.body.participants;
    const conversation_id = req.body.conversation_id;
    console.log("/updateConversationParticipants called for participant: ", newParticipantsHandle, ' and conversation_id: ', conversation_id);

    try {
        const result = await Conversation.updateConversationParticipants(conversation_id, newParticipantsHandle);

        return res.status(200).json(result);
    } catch (error) {
        console.error("Error updating conversation participants", error);
        return res.status(400).json({ error: error.message });
    }
}

export default {
    getConversations,
    createConversation,
    updateConversationParticipants
}