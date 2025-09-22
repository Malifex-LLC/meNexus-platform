import Chat from '../models/chat.js'

export const createChatMessage = async (publicKey, activeChannel, content) => {
    return Chat.createChatMessage(publicKey, activeChannel, content)
}

export const getChannelChatMessages = async (channel) => {
    return Chat.getChannelChatMessages(channel);
}

export default {
    createChatMessage,
    getChannelChatMessages
}