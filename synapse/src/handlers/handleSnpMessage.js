import { handlerDirectory } from './handlerDirectory.js'

export const handleSnpMessage = async (libp2p, message) => {
    const handler = handlerDirectory[message.messageType];
    if (handler) return await handler(libp2p, message);
    console.warn(`No handler found for: ${message.messageType}`);
};
