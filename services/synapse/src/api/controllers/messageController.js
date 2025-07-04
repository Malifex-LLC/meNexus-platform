// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

// Import Message model
import Message from "../models/message.js"

export const getMessages = async (req, res) => {
    if (!req.session || !req.session.user) {
        console.log("User not authenticated or session missing");
        return res.status(401).json({ error: "User not authenticated" });
    }

    const { publicKey } = req.session.user; // Get the current user's ID
    const { conversation_id } = req.query;

    if(!publicKey) {
        return res.status(401).json({ error: 'User not authenticated' });
    }

    const results = await Message.getMessages(conversation_id);

    return res.status(200).json(results);
}

export const createMessage = async (req, res) => {
    console.log('/createMessage called');
    console.log('/createMessage req.body:', req.body);
    console.log('/createMessage req.query:', req.query);

    if (!req.session || !req.session.user) {
        console.log("User not authenticated or session missing");
        return res.status(401).json({ error: "User not authenticated" });
    }

    const { publicKey } = req.session.user; // Get the current user's ID
    const { conversation_id } = req.body;
    const { message } = req.body;
    const participant_id = message.participant_id;
    const content = message.content;

    if(!publicKey) {
        return res.status(401).json({ error: 'User not authenticated' });
    }

    if (!conversation_id || !content) {
        return res.status(400).json({ error: 'Missing conversation_id or content' });
    }

    const result = await Message.createMessage(publicKey, conversation_id, message, participant_id, content);
    return res.status(200).json(result);
}

export const setMessagesAsRead = async (req, res) => {
    console.log('/setMessagesAsRead called for: ', req.body);
    const { conversation_id } = req.body;

    const results = await Message.setMessageAsRead(conversation_id);
    return res.status(200).json(results);
}

export default {
    getMessages,
    createMessage,
    setMessagesAsRead
}