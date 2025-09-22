// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

// Import Message model
import Chat from "../models/chat.js"

export const createChatMessage = async (req, res) => {
    const publicKey = req.user?.publicKey;
    const { activeChannel, content } = req.body;
    console.log('createChatMessage activeChannel: ', activeChannel);
    if (!publicKey || !activeChannel || !content) {
        return res.status(400).json({error: 'publicKey, activeChannel, or content not found.'});
    }

    try {
        const chatId = await Chat.createChatMessage(publicKey, activeChannel, content);
        res.status(200).json({ message: 'Chat created successfully.', chatId});
    } catch (error) {
        console.error('Error in createChat:', error);
        res.status(500).json({error: 'Failed to create chat.'});
    }
};

export const getChannelChatMessages = async (req, res) => {
    const { channel } = req.query;
    if (!channel) {
        return res.status(400).json({error: 'Channel not found.'});
    }
    try {
        const chats = await Chat.getChannelChatMessages(channel);
        res.status(200).json(chats);
    } catch (error) {
        console.error('Error in getChannelChats:', error);
        res.status(500).json({error: 'Failed to get channel chats.'})
    }
}

export default {
    createChatMessage,
    getChannelChatMessages,
}